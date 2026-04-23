use rusqlite::{Connection, Result, params};
use crate::models::transaction::Transaction;

pub fn issue_book(conn: &Connection, t: &Transaction) -> Result<()> {
    conn.execute(
        "INSERT INTO transactions (accession_no, user_id, issue_date, expected_return_date, status)
         VALUES (?1, ?2, ?3, ?4, 'Issued')",
        params![t.accession_no, t.user_id, t.issue_date, t.expected_return_date],
    )?;
    conn.execute(
        "UPDATE books SET status = 'Issued' WHERE accession_no = ?1",
        params![t.accession_no],
    )?;
    Ok(())
}

pub fn return_book(conn: &Connection, accession_no: &str, return_date: &str) -> Result<()> {
    conn.execute(
        "UPDATE transactions SET actual_return_date = ?1, status = 'Returned'
         WHERE accession_no = ?2 AND status = 'Issued'",
        params![return_date, accession_no],
    )?;
    conn.execute(
        "UPDATE books SET status = 'Available' WHERE accession_no = ?1",
        params![accession_no],
    )?;
    Ok(())
}

pub fn renew_book(conn: &Connection, accession_no: &str, new_return_date: &str) -> Result<()> {
    conn.execute(
        "UPDATE transactions SET expected_return_date = ?1, status = 'Renewed'
         WHERE accession_no = ?2 AND status IN ('Issued','Renewed')",
        params![new_return_date, accession_no],
    )?;
    Ok(())
}

pub fn get_active_transactions(conn: &Connection) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.accession_no, t.user_id,
                COALESCE(m.first_name || ' ' || m.last_name, f.first_name || ' ' || f.last_name, t.user_id),
                t.issue_date, t.expected_return_date, t.actual_return_date, t.status
         FROM transactions t
         LEFT JOIN members m ON t.user_id = m.id
         LEFT JOIN faculty f ON t.user_id = f.uid
         WHERE t.status IN ('Issued','Renewed')
         ORDER BY t.issue_date DESC",
    )?;
    let txns = stmt.query_map([], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            accession_no: row.get(1)?,
            user_id: row.get(2)?,
            user_name: row.get(3).unwrap_or_default(),
            issue_date: row.get(4)?,
            expected_return_date: row.get(5)?,
            actual_return_date: row.get(6)?,
            status: row.get(7)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    Ok(txns)
}

pub fn get_transactions_in_range(conn: &Connection, from: &str, to: &str) -> Result<Vec<Transaction>> {
    let mut stmt = conn.prepare(
        "SELECT t.id, t.accession_no, t.user_id,
                COALESCE(m.first_name || ' ' || m.last_name, f.first_name || ' ' || f.last_name, t.user_id),
                t.issue_date, t.expected_return_date, t.actual_return_date, t.status
         FROM transactions t
         LEFT JOIN members m ON t.user_id = m.id
         LEFT JOIN faculty f ON t.user_id = f.uid
         WHERE t.issue_date BETWEEN ?1 AND ?2
         ORDER BY t.issue_date DESC",
    )?;
    let txns = stmt.query_map(params![from, to], |row| {
        Ok(Transaction {
            id: row.get(0)?,
            accession_no: row.get(1)?,
            user_id: row.get(2)?,
            user_name: row.get(3).unwrap_or_default(),
            issue_date: row.get(4)?,
            expected_return_date: row.get(5)?,
            actual_return_date: row.get(6)?,
            status: row.get(7)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    Ok(txns)
}
