pub mod books;
pub mod faculty;
pub mod members;
pub mod schema;
pub mod transactions;

use rusqlite::{Connection, Result};
use std::path::Path;

pub fn get_db_connection() -> Result<Connection> {
    let db_path = Path::new("libsoft.db");
    let conn = Connection::open(db_path)?;
    schema::init_db(&conn)?;
    Ok(conn)
}
