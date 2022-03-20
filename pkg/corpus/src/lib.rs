use rusqlite::Connection;
use std::path::PathBuf;
use xxhash_rust::xxh3::xxh3_64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i64)]
pub enum Language {
    PgSql = 0,
    PlPgSql = 1,
    Psql = 2,
    PlPerl = 3,
    PlTcl = 4,
    PlPython2 = 5,
    PlPython3 = 6,
    Sqlite3 = 7,
    Other = -1,
}

/// connect or else.
pub fn connect(path: &str) -> Result<Connection, rusqlite::Error> {
    // TODO: check if path is a file. If a file, check if it's an empty sqlite db
    // else: chuck stuff in :memory:

    let mut conn = Connection::open(path)?;
    if !PathBuf::from(path).exists() {
        println!("initializing {}", path);
        init(&mut conn)?; // try to initialize the schema
        return Ok(conn);
    }

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
            "INSERT INTO statements (id, text) VALUES (?, ?) ON CONFLICT(id) DO NOTHING",
        )?;
        for statement in statements {
            insert.execute(rusqlite::params![statement.id as i64, statement.text])?;
        }
    }
    return txn.commit();
}

pub fn doc_already_processed(conn: &mut Connection, doc_id: i64) -> bool {
    return conn
        .query_row(
            "SELECT id FROM documents WHERE id = ?;",
            &[&doc_id],
            |row| {
                let id: i64 = row.get(0)?;
                return Ok(id);
            },
        )
        .is_ok();
}

pub fn bulk_insert_statement_languages(
    conn: &mut Connection,
    statement_languages: Vec<(i64, Language)>,
) -> Result<(), rusqlite::Error> {
    let txn = conn.transaction()?;
    {
        let value_tuples = ",(?,?)".repeat(statement_languages.len());
        let sql = format!(
            "INSERT INTO statement_languages(statement_id, language_id) VALUES {} ON CONFLICT DO NOTHING;",
            value_tuples.trim_start_matches(',')
        );
        let mut params: Vec<i64> = Vec::with_capacity(2 * statement_languages.len());

        for row in statement_languages {
            params.push(row.0);
            params.push(row.1 as i64)
        }
        txn.execute(sql.as_str(), rusqlite::params_from_iter(params.iter()))?;
    }
    return txn.commit();
}

pub fn insert_license(
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
) -> Result<Vec<i64>, rusqlite::Error> {
    let ids: Vec<i64> = urls
        .iter()
        .map(|url| xxh3_64(url.as_bytes()) as i64)
        .collect();
    let txn = conn.transaction()?;
    if let Some(license_id) = license_id {
        {
            let insert = &mut txn.prepare(
                "INSERT INTO urls (id, url, license_id) VALUES (?, ?, ?) ON CONFLICT DO NOTHING",
            )?;
            for (url, id) in urls.iter().zip(ids.as_slice().iter()) {
                let params = rusqlite::params![&id, *url, license_id];
                insert.execute(params)?;
            }
        }
    } else {
        {
            let insert = &mut txn
                .prepare("INSERT INTO urls (id, url) VALUES (?, ?) ON CONFLICT DO NOTHING")?;
            for (url, id) in urls.iter().zip(ids.as_slice().iter()) {
                let params = rusqlite::params![&id, *url];
                insert.execute(params)?;
            }
        }
    }
    txn.commit()?;
    return Ok(ids);
}

// TODO: insert document urls
pub fn bulk_insert_document_urls(
    conn: &mut Connection,
    document_id: i64,
    urls_ids: &[i64],
) -> Result<(), rusqlite::Error> {
    let txn = conn.transaction()?;
    {
        let insert = &mut txn.prepare(
            "INSERT INTO document_urls(document_id, url_id) VALUES (?, ?) ON CONFLICT DO NOTHING",
        )?;
        for url_id in urls_ids {
            insert.execute(&[&document_id, url_id])?;
        }
    }
    return txn.commit();
}

pub fn bulk_insert_statement_documents(
    conn: &mut Connection,
    statement_sources: Vec<StatementSource>,
) -> Result<(), rusqlite::Error> {
    if statement_sources.len() == 0 {
        return Ok(());
    }
    let mut params: Vec<i64> = Vec::with_capacity(6 * statement_sources.len());

    let txn = conn.transaction()?;
    {
        let insert_document_statement = &mut txn.prepare(
                format!(
                    "INSERT INTO document_statements (document_id, statement_id, start_line, end_line, start_offset, end_offset) VALUES {} ON CONFLICT DO NOTHING",
                    ",(?,?,?,?,?,?)".repeat(statement_sources.len()).trim_start_matches(",")
                ).as_str())?;

        let insert_document_url = &mut txn.prepare(
            "INSERT INTO document_urls(document_id, url_id) VALUES (?, ?) ON CONFLICT DO NOTHING",
        )?;

        for src in statement_sources.as_slice() {
            // insert_document_statement.execute(rusqlite::params![
            //     src.document_id as i64,
            //     src.statement_id as i64,
            //     src.start_line,
            //     src.start_line + src.n_lines - 1,
            //     src.start_offset,
            //     src.end_offset,
            //     // format!("#L{}-L{}", src.start_line, src.start_line + src.n_lines - 1)
            // ])?;
            let url_id = src.url_id();
            params.push(src.document_id);
            params.push(src.statement_id);
            params.push(src.start_line as i64);
            params.push((src.start_line + src.n_lines) as i64);
            params.push(src.start_offset as i64);
            params.push(src.end_offset as i64);
            insert_document_url.execute(rusqlite::params![src.document_id, url_id,])?;
        }
        insert_document_statement.execute(rusqlite::params_from_iter(params.iter()))?;
    }
    return txn.commit();
}

pub fn bulk_insert_statement_fingerprints(
    conn: &mut Connection,
    statement_fingerprints: Vec<(i64, i64)>,
) -> Result<(), rusqlite::Error> {
    if statement_fingerprints.len() <= 0 {
        return Ok(());
    }
    let txn = conn.transaction()?;
    {
        let insert = &mut txn.prepare(
            format!(
                "INSERT INTO statement_fingerprints(statement_id, fingerprint) VALUES {} ON CONFLICT DO NOTHING;",
                ",(?,?)".repeat(statement_fingerprints.len()).trim_start_matches(","),
            ).as_str()
        )?;
        let mut params: Vec<i64> = Vec::with_capacity(statement_fingerprints.len() * 2);
        for (statement_id, fingerprint) in statement_fingerprints {
            params.push(statement_id);
            params.push(fingerprint);
        }
        insert.execute(rusqlite::params_from_iter(params.iter()))?;
    }
    return txn.commit();
}

#[derive(Clone, Debug)]
pub struct Statement {
    pub text: String,
    /// the xxhash3_64 of the overall utf-8 document that this statement is
    /// drawn from
    pub document_id: i64,
    /// the xxhash3_64 of the text
    pub id: i64,
    /// might include line numbers inside a collection
    pub language: Language,
    // urls: Vec<String>,
    // start_line: usize,
    pub n_lines: usize,
}

impl Statement {
    pub fn new(text: String, language: Language, document_id: i64) -> Self {
        let n_lines = text.matches("\n").count();
        Statement {
            id: xxh3_64(text.as_bytes()) as i64,
            document_id,
            text,
            language,
            n_lines,
        }
    }
    pub fn with_source(
        self: &Self,
        url: &str,
        start_line: usize,
        start_offset: usize,
    ) -> StatementSource {
        StatementSource {
            statement_id: self.id,
            url: url.to_owned(),
            document_id: self.document_id,
            start_line,
            start_offset,
            end_offset: start_offset + self.text.len(),
            n_lines: self.n_lines,
        }
    }
}

#[derive(Clone)]
pub struct StatementSource {
    pub statement_id: i64,   //
    pub start_line: usize,   // 1-indexed
    pub n_lines: usize,      // can be 0
    pub start_offset: usize, // 0-indexed length in bytes, **not** unicode code points
    pub end_offset: usize,   // = start_offset + statement.len()
    pub document_id: i64, // xxhash3_64 of the overall document from which this statement is drawn
    pub url: String,      // TODO: validate; can currently be "" or "file://"
}
impl StatementSource {
    pub fn url_id(&self) -> i64 {
        xxh3_64(self.url.as_bytes()) as i64
    }
}
