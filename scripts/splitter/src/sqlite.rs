use rusqlite::{Connection, ToSql, Transaction};

use crate::Statement;

/// connect or else.
pub fn connect(path: &str) -> Connection {
    // TODO: check if path is a file, not a dir
    // else: chuck stuff in :memory:

    match Connection::open(path) {
        Ok(conn) => return conn,
        Err(e) => panic!("{}", e),
    }
}

pub fn init(conn: &mut Connection) -> Result<&mut Connection, rusqlite::Error> {
    const SCHEMA: &str = include_str!("../../../schema.sql");
    conn.execute_batch(SCHEMA)?;
    return Ok(conn);
}

pub fn bulk_insert_statements(
    conn: &mut Connection,
    statements: Vec<Statement>,
) -> Result<(), rusqlite::Error> {
    let txn = conn.transaction()?;
    {
        // block required for lifetime of borrow of txn
        let insert =
            &mut txn.prepare("INSERT INTO statements (id, text, language_id) VALUES (?, ?, ?)")?;
        for s in statements {
            insert.execute(rusqlite::params![s.id, s.text, s.language as i32])?;
        }
    }

    return txn.commit();
}
