use rusqlite::{Connection, Result, params};
use crate::models::member::Member;

pub fn add_member(conn: &Connection, m: &Member) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO members (id, first_name, middle_name, last_name, admission_year,
         course, current_year, branch, mobile_no, email, address, is_active, total_due)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12,?13)",
        params![
            m.id, m.first_name, m.middle_name, m.last_name, m.admission_year,
            m.course, m.current_year, m.branch, m.mobile_no, m.email, m.address,
            m.is_active, m.total_due
        ],
    )?;
    Ok(())
}

pub fn get_member(conn: &Connection, id: &str) -> Result<Option<Member>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, middle_name, last_name, admission_year, course, current_year,
                branch, mobile_no, email, address, is_active, total_due
         FROM members WHERE id = ?1",
    )?;
    let mut rows = stmt.query(params![id])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Member {
            id: row.get(0)?,
            first_name: row.get(1)?,
            middle_name: row.get(2).unwrap_or_default(),
            last_name: row.get(3)?,
            admission_year: row.get(4).unwrap_or_default(),
            course: row.get(5).unwrap_or_default(),
            current_year: row.get(6).unwrap_or_default(),
            branch: row.get(7).unwrap_or_default(),
            mobile_no: row.get(8).unwrap_or_default(),
            email: row.get(9).unwrap_or_default(),
            address: row.get(10).unwrap_or_default(),
            is_active: row.get(11)?,
            total_due: row.get(12)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_all_members(conn: &Connection) -> Result<Vec<Member>> {
    let mut stmt = conn.prepare(
        "SELECT id, first_name, middle_name, last_name, admission_year, course, current_year,
                branch, mobile_no, email, address, is_active, total_due FROM members",
    )?;
    let members = stmt.query_map([], |row| {
        Ok(Member {
            id: row.get(0)?,
            first_name: row.get(1)?,
            middle_name: row.get(2).unwrap_or_default(),
            last_name: row.get(3)?,
            admission_year: row.get(4).unwrap_or_default(),
            course: row.get(5).unwrap_or_default(),
            current_year: row.get(6).unwrap_or_default(),
            branch: row.get(7).unwrap_or_default(),
            mobile_no: row.get(8).unwrap_or_default(),
            email: row.get(9).unwrap_or_default(),
            address: row.get(10).unwrap_or_default(),
            is_active: row.get(11)?,
            total_due: row.get(12)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    Ok(members)
}

pub fn delete_member(conn: &Connection, id: &str) -> Result<()> {
    conn.execute("DELETE FROM members WHERE id = ?1", params![id])?;
    Ok(())
}

pub fn set_all_active(conn: &Connection, active: bool) -> Result<()> {
    conn.execute("UPDATE members SET is_active = ?1", params![active])?;
    Ok(())
}
