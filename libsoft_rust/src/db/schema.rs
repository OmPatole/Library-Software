use rusqlite::{Connection, Result};

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            accession_no TEXT PRIMARY KEY,
            call_no TEXT NOT NULL,
            title TEXT NOT NULL,
            author TEXT NOT NULL,
            branch TEXT NOT NULL,
            publisher TEXT NOT NULL,
            price REAL NOT NULL,
            bill_no TEXT NOT NULL,
            status TEXT NOT NULL
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS members (
            id TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            middle_name TEXT,
            last_name TEXT NOT NULL,
            admission_year TEXT,
            course TEXT,
            current_year TEXT,
            branch TEXT,
            mobile_no TEXT,
            email TEXT,
            address TEXT,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            total_due REAL NOT NULL DEFAULT 0.0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS faculty (
            uid TEXT PRIMARY KEY,
            first_name TEXT NOT NULL,
            middle_name TEXT,
            last_name TEXT NOT NULL,
            joining_date TEXT,
            joining_under TEXT,
            branch TEXT,
            mobile_no TEXT,
            email TEXT,
            address TEXT,
            is_active BOOLEAN NOT NULL DEFAULT 1,
            total_due REAL NOT NULL DEFAULT 0.0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS transactions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            accession_no TEXT NOT NULL,
            user_id TEXT NOT NULL,
            issue_date TEXT NOT NULL,
            expected_return_date TEXT NOT NULL,
            actual_return_date TEXT,
            status TEXT NOT NULL, -- e.g., Issued, Returned, Renewed
            FOREIGN KEY(accession_no) REFERENCES books(accession_no)
        )",
        [],
    )?;

    Ok(())
}
