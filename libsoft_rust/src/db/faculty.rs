use rusqlite::{Connection, Result, params};
use crate::models::faculty::Faculty;

pub fn add_faculty(conn: &Connection, f: &Faculty) -> Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO faculty (uid, first_name, middle_name, last_name, joining_date,
         joining_under, branch, mobile_no, email, address, is_active, total_due)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8,?9,?10,?11,?12)",
        params![
            f.uid, f.first_name, f.middle_name, f.last_name, f.joining_date,
            f.joining_under, f.branch, f.mobile_no, f.email, f.address,
            f.is_active, f.total_due
        ],
    )?;
    Ok(())
}

pub fn get_faculty(conn: &Connection, uid: &str) -> Result<Option<Faculty>> {
    let mut stmt = conn.prepare(
        "SELECT uid, first_name, middle_name, last_name, joining_date, joining_under,
                branch, mobile_no, email, address, is_active, total_due
         FROM faculty WHERE uid = ?1",
    )?;
    let mut rows = stmt.query(params![uid])?;
    if let Some(row) = rows.next()? {
        Ok(Some(Faculty {
            uid: row.get(0)?,
            first_name: row.get(1)?,
            middle_name: row.get(2).unwrap_or_default(),
            last_name: row.get(3)?,
            joining_date: row.get(4).unwrap_or_default(),
            joining_under: row.get(5).unwrap_or_default(),
            branch: row.get(6).unwrap_or_default(),
            mobile_no: row.get(7).unwrap_or_default(),
            email: row.get(8).unwrap_or_default(),
            address: row.get(9).unwrap_or_default(),
            is_active: row.get(10)?,
            total_due: row.get(11)?,
        }))
    } else {
        Ok(None)
    }
}

pub fn get_all_faculty(conn: &Connection) -> Result<Vec<Faculty>> {
    let mut stmt = conn.prepare(
        "SELECT uid, first_name, middle_name, last_name, joining_date, joining_under,
                branch, mobile_no, email, address, is_active, total_due FROM faculty",
    )?;
    let faculty = stmt.query_map([], |row| {
        Ok(Faculty {
            uid: row.get(0)?,
            first_name: row.get(1)?,
            middle_name: row.get(2).unwrap_or_default(),
            last_name: row.get(3)?,
            joining_date: row.get(4).unwrap_or_default(),
            joining_under: row.get(5).unwrap_or_default(),
            branch: row.get(6).unwrap_or_default(),
            mobile_no: row.get(7).unwrap_or_default(),
            email: row.get(8).unwrap_or_default(),
            address: row.get(9).unwrap_or_default(),
            is_active: row.get(10)?,
            total_due: row.get(11)?,
        })
    })?
    .filter_map(|r| r.ok())
    .collect();
    Ok(faculty)
}

pub fn delete_faculty(conn: &Connection, uid: &str) -> Result<()> {
    conn.execute("DELETE FROM faculty WHERE uid = ?1", params![uid])?;
    Ok(())
}
