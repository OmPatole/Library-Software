use rusqlite::{Connection, Result, params};
use crate::models::book::Book;

pub fn add_book(conn: &Connection, book: &Book) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO books (accession_no, call_no, title, author, branch, publisher, price, bill_no, status)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            book.accession_no, book.call_no, book.title, book.author,
            book.branch, book.publisher, book.price, book.bill_no, book.status
        ],
    )?;
    Ok(())
}

pub fn get_book(conn: &Connection, accession_no: &str) -> Result<Option<Book>> {
    let mut stmt = conn.prepare(
        "SELECT accession_no, call_no, title, author, branch, publisher, price, bill_no, status
         FROM books WHERE accession_no = ?1",
    )?;
    let mut rows = stmt.query(params![accession_no])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Book {
            accession_no: row.get(0)?,
            call_no: row.get(1)?,
            title: row.get(2)?,
            author: row.get(3)?,
            branch: row.get(4)?,
            publisher: row.get(5)?,
            price: row.get(6)?,
            bill_no: row.get(7)?,
            status: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_all_books(conn: &Connection) -> Result<Vec<Book>> {
    let mut stmt = conn.prepare(
        "SELECT accession_no, call_no, title, author, branch, publisher, price, bill_no, status FROM books",
    )?;
    let books = stmt.query_map([], |row| {
        Ok(Book {
            accession_no: row.get(0)?,
            call_no: row.get(1)?,
            title: row.get(2)?,
            author: row.get(3)?,
            branch: row.get(4)?,
            publisher: row.get(5)?,
            price: row.get(6)?,
            bill_no: row.get(7)?,
            status: row.get(8)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    Ok(books)
}

pub fn update_book_status(conn: &Connection, accession_no: &str, status: &str) -> Result<()> {
    conn.execute(
        "UPDATE books SET status = ?1 WHERE accession_no = ?2",
        params![status, accession_no],
    )?;
    Ok(())
}

pub fn delete_book(conn: &Connection, accession_no: &str) -> Result<()> {
    conn.execute("DELETE FROM books WHERE accession_no = ?1", params![accession_no])?;
    Ok(())
}

pub fn search_books(conn: &Connection, query: &str) -> Result<Vec<Book>> {
    let like = format!("%{}%", query);
    let mut stmt = conn.prepare(
        "SELECT accession_no, call_no, title, author, branch, publisher, price, bill_no, status
         FROM books
         WHERE accession_no LIKE ?1 OR title LIKE ?1 OR author LIKE ?1 OR call_no LIKE ?1 OR publisher LIKE ?1",
    )?;
    let books = stmt.query_map(params![like], |row| {
        Ok(Book {
            accession_no: row.get(0)?,
            call_no: row.get(1)?,
            title: row.get(2)?,
            author: row.get(3)?,
            branch: row.get(4)?,
            publisher: row.get(5)?,
            price: row.get(6)?,
            bill_no: row.get(7)?,
            status: row.get(8)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    Ok(books)
}
