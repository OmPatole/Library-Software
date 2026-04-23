use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

// ─── Domain structs ───────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Book {
    pub accession_no: String,
    pub call_no: String,
    pub title: String,
    pub author: String,
    pub branch: String,
    pub publisher: String,
    pub price: f64,
    pub bill_no: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Member {
    pub id: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
    pub admission_year: Option<String>,
    pub course: Option<String>,
    pub current_year: Option<String>,
    pub branch: Option<String>,
    pub mobile_no: Option<String>,
    pub email: Option<String>,
    pub is_active: bool,
    pub total_due: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: Option<i64>,
    pub accession_no: String,
    pub user_id: String,
    pub user_name: String,
    pub issue_date: String,
    pub expected_return_date: String,
    pub actual_return_date: Option<String>,
    pub status: String,
}

// ─── Database wrapper ─────────────────────────────────────────────────────────

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch("
            PRAGMA journal_mode=WAL;

            CREATE TABLE IF NOT EXISTS books (
                accession_no TEXT PRIMARY KEY,
                call_no      TEXT NOT NULL,
                title        TEXT NOT NULL,
                author       TEXT NOT NULL,
                branch       TEXT NOT NULL DEFAULT '',
                publisher    TEXT NOT NULL,
                price        REAL NOT NULL DEFAULT 0.0,
                bill_no      TEXT NOT NULL DEFAULT '',
                status       TEXT NOT NULL DEFAULT 'Available'
            );

            CREATE TABLE IF NOT EXISTS members (
                id             TEXT PRIMARY KEY,
                first_name     TEXT NOT NULL,
                middle_name    TEXT,
                last_name      TEXT NOT NULL,
                admission_year TEXT,
                course         TEXT,
                current_year   TEXT,
                branch         TEXT,
                mobile_no      TEXT,
                email          TEXT,
                is_active      BOOLEAN NOT NULL DEFAULT 1,
                total_due      REAL    NOT NULL DEFAULT 0.0
            );

            CREATE TABLE IF NOT EXISTS faculty (
                uid          TEXT PRIMARY KEY,
                first_name   TEXT NOT NULL,
                middle_name  TEXT,
                last_name    TEXT NOT NULL,
                joining_date TEXT,
                branch       TEXT,
                mobile_no    TEXT,
                email        TEXT,
                is_active    BOOLEAN NOT NULL DEFAULT 1,
                total_due    REAL    NOT NULL DEFAULT 0.0
            );

            CREATE TABLE IF NOT EXISTS transactions (
                id                   INTEGER PRIMARY KEY AUTOINCREMENT,
                accession_no         TEXT NOT NULL,
                user_id              TEXT NOT NULL,
                issue_date           TEXT NOT NULL,
                expected_return_date TEXT NOT NULL,
                actual_return_date   TEXT,
                status               TEXT NOT NULL DEFAULT 'Issued',
                FOREIGN KEY(accession_no) REFERENCES books(accession_no)
            );
        ")?;
        Ok(())
    }

    // ── Book queries ──────────────────────────────────────────────────────────

    pub fn search_books(&self, query: &str) -> Result<Vec<Book>> {
        let pattern = format!("%{}%", query);
        let mut stmt = self.conn.prepare(
            "SELECT accession_no, call_no, title, author, branch, publisher, price, bill_no, status
             FROM books
             WHERE accession_no LIKE ?1 OR title LIKE ?1 OR author LIKE ?1 OR call_no LIKE ?1
             ORDER BY title LIMIT 100",
        )?;
        let books = stmt
            .query_map(params![pattern], |row| {
                Ok(Book {
                    accession_no: row.get(0)?,
                    call_no:      row.get(1)?,
                    title:        row.get(2)?,
                    author:       row.get(3)?,
                    branch:       row.get(4)?,
                    publisher:    row.get(5)?,
                    price:        row.get(6)?,
                    bill_no:      row.get(7)?,
                    status:       row.get(8)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(books)
    }

    pub fn add_book(&self, book: &Book) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO books
             (accession_no, call_no, title, author, branch, publisher, price, bill_no, status)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9)",
            params![
                book.accession_no, book.call_no, book.title, book.author,
                book.branch, book.publisher, book.price, book.bill_no, book.status
            ],
        )?;
        Ok(())
    }

    // ── Member queries ────────────────────────────────────────────────────────

    pub fn get_members(&self) -> Result<Vec<Member>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, first_name, middle_name, last_name, admission_year, course,
                    current_year, branch, mobile_no, email, is_active, total_due
             FROM members ORDER BY last_name, first_name",
        )?;
        let members = stmt
            .query_map([], |row| {
                Ok(Member {
                    id:             row.get(0)?,
                    first_name:     row.get(1)?,
                    middle_name:    row.get(2)?,
                    last_name:      row.get(3)?,
                    admission_year: row.get(4)?,
                    course:         row.get(5)?,
                    current_year:   row.get(6)?,
                    branch:         row.get(7)?,
                    mobile_no:      row.get(8)?,
                    email:          row.get(9)?,
                    is_active:      row.get(10)?,
                    total_due:      row.get(11)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(members)
    }

    pub fn add_member(&self, m: &Member) -> Result<()> {
        self.conn.execute(
            "INSERT OR REPLACE INTO members
             (id, first_name, middle_name, last_name, admission_year, course,
              current_year, branch, mobile_no, email, is_active, total_due)
             VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
            params![
                m.id, m.first_name, m.middle_name, m.last_name, m.admission_year,
                m.course, m.current_year, m.branch, m.mobile_no, m.email,
                m.is_active, m.total_due
            ],
        )?;
        Ok(())
    }

    // ── Transaction queries ───────────────────────────────────────────────────

    pub fn issue_book(&self, t: &Transaction) -> Result<()> {
        self.conn.execute(
            "INSERT INTO transactions (accession_no, user_id, issue_date, expected_return_date, status)
             VALUES (?1,?2,?3,?4,'Issued')",
            params![t.accession_no, t.user_id, t.issue_date, t.expected_return_date],
        )?;
        self.conn.execute(
            "UPDATE books SET status='Issued' WHERE accession_no=?1",
            params![t.accession_no],
        )?;
        Ok(())
    }

    pub fn return_book(&self, accession_no: &str, return_date: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE transactions SET actual_return_date=?1, status='Returned'
             WHERE accession_no=?2 AND status IN ('Issued','Renewed')",
            params![return_date, accession_no],
        )?;
        self.conn.execute(
            "UPDATE books SET status='Available' WHERE accession_no=?1",
            params![accession_no],
        )?;
        Ok(())
    }

    pub fn renew_book(&self, accession_no: &str, new_return_date: &str) -> Result<()> {
        self.conn.execute(
            "UPDATE transactions SET expected_return_date=?1, status='Renewed'
             WHERE accession_no=?2 AND status IN ('Issued','Renewed')",
            params![new_return_date, accession_no],
        )?;
        Ok(())
    }

    pub fn get_active_transactions(&self) -> Result<Vec<Transaction>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.id, t.accession_no, t.user_id,
                    COALESCE(m.first_name||' '||m.last_name,
                             f.first_name||' '||f.last_name,
                             t.user_id),
                    t.issue_date, t.expected_return_date,
                    t.actual_return_date, t.status
             FROM transactions t
             LEFT JOIN members m ON t.user_id = m.id
             LEFT JOIN faculty f ON t.user_id = f.uid
             WHERE t.status IN ('Issued','Renewed')
             ORDER BY t.issue_date DESC",
        )?;
        let txns = stmt
            .query_map([], |row| {
                Ok(Transaction {
                    id:                   row.get(0)?,
                    accession_no:         row.get(1)?,
                    user_id:              row.get(2)?,
                    user_name:            row.get::<_, Option<String>>(3)?.unwrap_or_default(),
                    issue_date:           row.get(4)?,
                    expected_return_date: row.get(5)?,
                    actual_return_date:   row.get(6)?,
                    status:               row.get(7)?,
                })
            })?
            .filter_map(|r| r.ok())
            .collect();
        Ok(txns)
    }
}
