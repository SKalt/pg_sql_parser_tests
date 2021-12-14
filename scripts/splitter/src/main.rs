use lazy_static::lazy_static;
use pg_query_wrapper as pg_query;
use psql_splitter;
use regex::Regex;
mod sqlite;
use std::convert::{TryFrom, TryInto};
use std::io::Read;
use std::ptr::NonNull;
use std::{
    fs::{self, File},
    io::{self, Write},
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};
use xxhash_rust::xxh3::xxh3_64;
#[derive(Debug)]
enum Failure {
    IoErr(io::Error),
    PgQueryError(pg_query::Failure),
    DirDne,
    NotDir,
    Other(String),
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::IoErr(e)
    }
}
impl From<pg_query::Failure> for Failure {
    fn from(e: pg_query::Failure) -> Self {
        Self::PgQueryError(e)
    }
}

pub struct Statement {
    text: String,
    /// the xxhash3_64 of the text
    id: u64,
    /// might include line numbers inside a collection
    language: Language,
    // urls: Vec<String>,
    // start_line: usize,
    n_lines: usize,
}

impl Statement {
    fn new(text: String, language: Language) -> Self {
        let n_lines = text.matches("\n").count();
        Statement {
            id: xxh3_64(text.as_bytes()),
            text,
            language,
            n_lines,
        }
    }
    fn update(self: &mut Self, text: String, language: Language) -> &mut Self {
        self.language = language;
        self.text.push_str(text.as_str());
        return self.derive_id();
    }
    fn derive_id(self: &mut Self) -> &mut Self {
        self.id = self.digest();
        return self;
    }
    fn digest(self: &Self) -> u64 {
        return xxh3_64(self.text.as_bytes()); // TODO: may need to clone here
    }
    fn fingerprint(self: &Self) -> Result<u64, Failure> {
        let (fingerprint, _) = pg_query::fingerprint(self.text.clone().as_str())?;
        return Ok(fingerprint);
    }
    fn with_source(self: &Self, url: &str, start_line: usize) -> StatementSource {
        StatementSource {
            statement_id: self.id,
            url: url.to_owned(),
            start_line,
            n_lines: self.n_lines,
        }
    }
}

struct StatementSource {
    statement_id: u64,
    url: String, // not validated; can be ""
    start_line: usize,
    n_lines: usize,
}

fn extract_protobuf_string(node: &Box<pg_query::pbuf::Node>) -> String {
    use pg_query::pbuf::node::Node;
    match node.node.as_ref().unwrap() {
        Node::String(s) => return s.str.clone(),
        _ => panic!("node not string"),
    }
}
fn parse_pl(nodes: &Vec<pg_query::pbuf::Node>) -> (String, String) {
    use pg_query::pbuf::node::Node;

    let mut content: String = "".into();
    let mut lang: String = "plpgsql".into();
    for node in nodes {
        // unwrapping aggressively to catch unexpected structures via panics
        if let Node::DefElem(inner) = node.node.as_ref().unwrap() {
            match inner.defname.as_str() {
                "as" => match inner.arg.as_ref().unwrap().node.as_ref().unwrap() {
                    Node::String(s) => content = s.str.clone(),
                    Node::List(l) => {
                        assert_eq!(l.items.len(), 1);
                        let item = &l.items[0];
                        match item.node.as_ref().unwrap() {
                            Node::String(s) => content = s.str.clone(),
                            _ => panic!("unexpected list-item type {:?}", item),
                        }
                    }
                    _ => panic!("unexpected pl option {:?}", inner.as_ref()),
                },
                "language" => lang = extract_protobuf_string(inner.arg.as_ref().unwrap()),
                _ => {} // ignore
            }
        }
    }
    return (content, lang);
}
fn parse_do_stmt(d: &pg_query::pbuf::DoStmt) -> (String, String) {
    return parse_pl(d.args.as_ref());
}
fn parse_fn_stmt(f: &pg_query::pbuf::CreateFunctionStmt) -> (String, String) {
    return parse_pl(f.options.as_ref());
}

fn extract_pl(input: &str) -> Result<(String, String), Failure> {
    use pg_query::pbuf::node::Node;
    let stmts = pg_query::parse_to_protobuf(input)?.stmts;

    // for some reason there's a section in partition_prune that doesn't get
    // split when the entire document is passed via --input.  Passing the text
    // via stdin, however, causes the correct splits.
    // I'm ignoring it for now, since it only causes one snag in the entire regression
    // test suite.
    // if stmts.len() != 1 {
    //     println!("--------------------------------------------------");
    //     println!("{}", input);
    //     println!("==================================================");
    //     println!("{:?}", stmts);
    // }
    assert!(stmts.len() >= 1);
    if let Some(node) = &stmts[0].stmt {
        if let Some(node) = &node.node {
            match node {
                Node::DoStmt(stmt) => return Ok(parse_do_stmt(stmt)),
                Node::CreateFunctionStmt(stmt) => return Ok(parse_fn_stmt(stmt)),
                _ => return Err(Failure::Other(format!("unexpected node type {:?}", node))),
            }
        } else {
            return Err(Failure::Other("missing statement-node".to_string()));
        }
    } else {
        return Err(Failure::Other("empty stmt".to_string()));
    }
}

// psql stuff ---------------------------------------------------

#[derive(Debug, Clone, Copy)]
pub enum Language {
    PgSql = 0,
    PlPgSql = 1,
    Psql = 2,
    PlPerl = 3,
    PlTcl = 4,
    PlPython2 = 5,
    PlPython3 = 6,
    Other = -1,
}

lazy_static! {
    static ref PLPGSQL_NAME: Regex = Regex::new("(?i)^plpgsql$").unwrap();
    static ref PLPERL_NAME: Regex = Regex::new("(?i)^plperl$").unwrap();
    static ref PLTCL_NAME: Regex = Regex::new("(?i)^pltcl$").unwrap();
    static ref PLPYTHON2_NAME: Regex = Regex::new("(?i)^plpython2?u$").unwrap();
    static ref PLPYTHON3_NAME: Regex = Regex::new("(?i)^plpython3u$").unwrap();
}

fn identify_language(lang: &str) -> Language {
    use Language::*;
    if let Some(_) = PLPGSQL_NAME.find(lang) {
        return PlPgSql;
    } else if let Some(_) = PLPERL_NAME.find(lang) {
        return PlPerl;
    } else if let Some(_) = PLTCL_NAME.find(lang) {
        return PlTcl;
    } else if let Some(_) = PLPYTHON2_NAME.find(lang) {
        return PlPython2;
    } else if let Some(_) = PLPYTHON3_NAME.find(lang) {
        return PlPython3;
    } else {
        return Other;
    }
}

fn text_to_statement(text: &str) -> Statement {
    if psql_splitter::is_psql(text) {
        return Statement::new(text.to_string(), Language::Psql);
    } else {
        return Statement::new(text.to_string(), Language::PgSql);
    }
}

fn extract_pl_from_statement(s: &Statement) -> Option<Statement> {
    if let Ok((text, lang)) = extract_pl(s.text.as_str()) {
        let stmt = Statement::new(text, identify_language(lang.as_str()));
        // stmt.start_line = s.start_line;
        return Some(stmt);
    } else {
        return None;
    }
}

fn split_psql_to_statements(input: String) -> Vec<String> {
    let mut statements: Vec<String> = vec![];
    let mut rest = input.as_str();
    while let Ok((r, text)) = psql_splitter::statement(rest) {
        statements.push(text.to_string());
        rest = r;
    }
    assert_eq!(
        rest,
        "",
        "did not consume >>>{}<<<",
        &input[..input.len() - rest.len()]
    );
    assert_eq!(input, statements.join("").as_str());
    let act_len = statements
        .iter()
        .map(|s| s.len())
        .reduce(|total, len| total + len)
        .unwrap();
    assert_eq!(input.len(), act_len);
    return statements;
}

// CLI stuff -------------------------------------------------------------------

fn validate_output_target(output: String) -> Result<(), String> {
    if output == "stdout" {
        // println!("writing to stdout");
        return Ok(());
    }
    let output_path = PathBuf::from(output.clone());
    if !output_path.exists() {
        return Err(format!("output path {} does not exist", output).to_string());
    }
    if !output_path.is_file() {
        return Err(format!("{} is not a file", output));
    }
    let file = File::open(output_path).unwrap();
    if file.metadata().unwrap().permissions().readonly() {
        return Err(format!("read-only file {}", output).to_string());
    }
    return Ok(());
}

fn is_flat_dir_of_readable_files(path: PathBuf) -> bool {
    if let Ok(mut dir_entries) = std::fs::read_dir(path) {
        return dir_entries.all(|f| {
            if let Ok(file) = f {
                let path = file.path();
                if !path.is_file() {
                    return false;
                }
                match file_is_readable(path) {
                    Ok(is_readable) => return is_readable,
                    _ => return false,
                }
            } else {
                return false;
            }
        });
    } else {
        return false;
    }
}
const READ_BITS: u32 = 0o444;
const WRITE_BITS: u32 = 0o222;

fn file_is_readable<File>(file: File) -> Result<bool, Failure>
where
    File: AsRef<Path>,
{
    return Ok(fs::metadata(file)?.permissions().mode() & READ_BITS > 0);
}

fn file_is_writeable<File>(file: File) -> Result<bool, Failure>
where
    File: AsRef<Path>,
{
    return Ok(fs::metadata(file)?.permissions().mode() & WRITE_BITS > 0);
}

fn validate_input_source(input: String) -> Result<(), String> {
    if input == "stdin" {
        // println!("reading from stdin");
        return Ok(());
    }
    let input_path = PathBuf::from(input.clone());
    if !input_path.exists() {
        return Err(format!("input path {} does not exist", input).into());
    }
    if input_path.is_dir() {
        if is_flat_dir_of_readable_files(input_path) {
            return Ok(());
        } else {
            return Err(format!("{} isn't a flat directory of readable files", input).into());
        }
    } else if input_path.is_file() {
        match file_is_readable(input_path) {
            Ok(readable) => {
                if readable {
                    return Ok(());
                } else {
                    return Err(format!("file {} is not readable", input).into());
                }
            }
            Err(e) => return Err(format!("{:?}", e).into()),
        }
    } else {
        return Err(format!("{} has an unknown filetype", input).into());
    }
}

fn main() -> Result<(), Failure> {
    let matches = clap::App::new("splitter")
        .arg(
            // the path to the sqlite file or stdout
            clap::Arg::with_name("out")
                .long("--out")
                .short("-o")
                .default_value("stdout")
                .help("where to write output")
                .long_help("the file or device to which to write output (default stdout)")
                .validator(validate_output_target),
        )
        .arg(
            clap::Arg::with_name("input")
                .long("--input")
                .short("-i")
                .default_value("stdin")
                .help("file or device from which to read input")
                .long_help("the file or device from which to read SQL")
                .validator(validate_input_source),
        )
        .arg(
            // should be multiple, e.g. multiple git hosts, each with a branch and commit
            clap::Arg::with_name("url")
                .long("--url")
                .multiple(true)
                .takes_value(true)
                .help("urls at which the input may be found.")
                .long_help(
                    "the url at which the input can be found.  If the input
                        is a directory of files and the url ends with a `/`, the
                        url will be considered the base path each file",
                ),
        )
        .arg(
            clap::Arg::with_name("count")
                .takes_value(false)
                .short("-c")
                .long("--count")
                .help("print the of count the number of statements"),
        )
        // .arg(
        //     clap::Arg::with_name("version")
        //         .short("-v")
        //         .long("--version")
        //         .takes_value(true)
        //         .help("the version of postgres")
        //         .long_help(
        //             "the version of postgres AND plpgsql AND psql for which
        //                 the input code is valid",
        //         ), // should be major-version only for now
        // )
        // ^ the oracles will decide whether each statement is valid
        .get_matches();

    // read from stdin or a file
    let mut buffer = String::new();
    match matches.value_of("input") {
        None | Some("stdin") => {
            io::stdin().read_to_string(&mut buffer)?;
        }
        Some(filename) => {
            buffer = fs::read_to_string(filename)?;
        }
    };
    let out = matches.value_of("out");
    // TODO: validate each URL
    let mut urls: Vec<&str> = Vec::with_capacity(matches.occurrences_of("url").try_into().unwrap());
    if let Some(url_args) = matches.values_of("url") {
        for url in url_args {
            urls.push(url);
        }
    };

    let splits = split_psql_to_statements(buffer);

    let mut statements = Vec::<Statement>::with_capacity(splits.capacity());
    let mut sources = Vec::<StatementSource>::with_capacity(urls.capacity() * splits.capacity());
    let mut line_number = 1usize;
    for split in splits {
        let stmt = text_to_statement(split.as_str());
        for url in urls.clone() {
            sources.push(stmt.with_source(url, line_number))
        }
        line_number += stmt.n_lines;
        statements.push(stmt);
    }
    let pl_blocks: Vec<Statement> = statements
        .iter()
        .filter_map(extract_pl_from_statement)
        .collect();
    if matches.is_present("count") {
        println!("{}", statements.len() + pl_blocks.len());
        return Ok(());
    }

    for s in statements {
        let id = s.id;
        println!(
            "-- {:?} {:x} --------------------------------------",
            s.language, s.id
        );
        for src in sources.iter().filter(|src| src.statement_id == id) {
            println!(
                "-- {}#L{}-L{}",
                src.url,
                src.start_line,
                src.start_line + src.n_lines - 1
            );
        }
        println!("---------------------------------------------");
        println!("{}", s.text);
    }

    for s in pl_blocks {
        println!(
            "-- {:?} {:x} --------------------------------------",
            s.language, s.id
        );
        println!("{}", s.text);
    }
    return Ok(());
}
