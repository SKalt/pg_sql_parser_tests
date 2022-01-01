use pg_query_wrapper::pbuf::ResTarget;
use rusqlite::{version, Connection, ToSql, Transaction};
use std::convert::TryFrom;
use std::path::PathBuf;

use crate::{Failure, Statement, StatementSource};

/// connect or else.
pub fn connect(path: &str) -> Result<Connection, Failure> {
    // TODO: check if path is a file. If a file, check if it's an empty sqlite db
    // else: chuck stuff in :memory:
    let output_path = PathBuf::from(path);
    if !output_path.exists() {
        let mut conn = Connection::open(path)?;
        init(&mut conn)?;
        return Ok(conn);
        // try to initialize the db
        // return Err(format!("output path {} does not exist", path).to_string());
    } else if output_path.is_file() {
        let conn = Connection::open(path)?;
        // check the schema version
        let version: (u32, u32) =
            conn.query_row("select major, minor from schema_version;", [], |row| {
                Ok((row.get(0).unwrap(), row.get(1).unwrap()))
            })?;
        assert_eq!(
            version,
            (0, 0),
            "unexpected version: got {}.{}, wanted 1",
            version.0,
            version.1
        );
        return Ok(conn);
    } else {
        return Err(Failure::Other(format!("non-file path: {}", path)));
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
        let insert = &mut txn.prepare(
            "INSERT INTO statements (id, text, language_id) VALUES (?, ?, ?) ON CONFLICT DO NOTHING"
        )?;
        for s in statements {
            insert.execute(rusqlite::params![s.id as i64, s.text, s.language as i32])?;
        }
    }
    return txn.commit();
}

pub fn insert_license_id(
    conn: &mut Connection,
    id: &str,
    license: String,
) -> Result<(), rusqlite::Error> {
    let txn = conn.transaction()?;
    {
        txn.execute(
            "INSERT INTO licenses (id, text) VALUES (?, ?) ON CONFLICT(id) DO UPDATE SET text = excluded.text",
            rusqlite::params![id, license],
        )?;
    }
    return txn.commit();
}

pub fn bulk_insert_urls(
    conn: &mut Connection,
    urls: &[&str],
    license_id: Option<&str>,
) -> Result<(), rusqlite::Error> {
    use xxhash_rust::xxh3::xxh3_64;
    let txn = conn.transaction()?;
    if let Some(license_id) = license_id {
        {
            let insert = &mut txn.prepare(
                "INSERT INTO urls (id, url, license_id) VALUES (?, ?, ?) ON CONFLICT DO NOTHING",
            )?;
            for url in urls {
                let id = xxh3_64(url.as_bytes()) as i64;
                let params = rusqlite::params![&id, *url, license_id];
                insert.execute(params)?;
            }
        }
    } else {
        {
            let insert = &mut txn
                .prepare("INSERT INTO urls (id, url) VALUES (?, ?) ON CONFLICT DO NOTHING")?;
            for url in urls {
                let id = xxh3_64(url.as_bytes()) as i64;
                let params = rusqlite::params![&id, *url];
                insert.execute(params)?;
            }
        }
    }
    return txn.commit();
}

pub fn bulk_insert_statement_sources(
    conn: &mut Connection,
    statement_sources: Vec<StatementSource>,
) -> Result<(), rusqlite::Error> {
    if statement_sources.len() > 0 {
        let txn = conn.transaction()?;
        {
            let insert = &mut txn.prepare(
                "INSERT INTO statement_sources (statement_id, url_id, locator) VALUES (?, ?, ?) ON CONFLICT DO NOTHING",
            )?;
            for src in statement_sources {
                insert.execute(rusqlite::params![
                    src.statement_id as i64,
                    src.url_id() as i64,
                    format!("#L{}-L{}", src.start_line, src.start_line + src.n_lines - 1)
                ])?;
            }
        }
        return txn.commit();
    } else {
        Ok(())
    }
}
