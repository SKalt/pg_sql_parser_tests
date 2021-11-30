use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::Slice;

use pg_query_wrapper as pg_query;
use regex;
use std::convert::TryFrom;
use std::io::Read;
use std::{
    fs::{self, File},
    io::{self, Write},
    os::unix::prelude::PermissionsExt,
    path::{Path, PathBuf},
};
use url::{ParseError, Url};
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

struct Statement {
    text: String,
    /// the xxhash3_64 of the text
    id: u64,
    /// might include line numbers inside a collection
    url: Option<Url>,
    language: Language,
}

/// a collection of Statements under a shared url, e.g. https://github.com/postgres/postgres/blob/REL_14_STABLE/src/test/regress/sql/aggregates.sql
struct MultiStatement {
    /// an ordered array of statement ids within the collection
    statements: Vec<u64>,
    url: Option<Url>,
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
fn extract_pl(input: &str) -> Result<String, Failure> {
    use pg_query::pbuf::node::Node;
    let stmts = pg_query_wrapper::parse_to_protobuf(input)?.stmts;
    assert_eq!(stmts.len(), 1);

    let mut content: String = "".into();
    let mut lang: String = "".into();

    if let Some(node) = &stmts[0].stmt {
        if let Some(node) = &node.node {
            match node {
                Node::DoStmt(stmt) => {
                    let (content_, lang_) = parse_do_stmt(stmt);
                    content = content_;
                    lang = lang_;
                    Ok(content)
                }
                Node::CreateFunctionStmt(stmt) => {
                    let (content_, lang_) = parse_fn_stmt(stmt);
                    content = content_;
                    lang = lang_;
                    return Ok(content);
                }
                _ => panic!("unexpected node type {:?}", node),
            }
        } else {
            return Err(Failure::Other("missing statement-node".to_string()));
        }
    } else {
        return Err(Failure::Other("empty stmt".to_string()));
    }
}
impl Statement {
    fn new(text: String, language: Language, url: Option<Url>) -> Self {
        Statement {
            id: xxh3_64(text.as_bytes()),
            text,
            language,
            url,
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
}

// psql stuff ---------------------------------------------------
use lazy_static::lazy_static;
use regex::Regex;

fn nested_psql(input: &str) -> nom::IResult<&str, &str> {
    Ok(recognize(delimited(
        tag(r"\if"),
        many0(alt((nested_psql, recognize(anychar)))),
        tag(r"\endif"),
    ))(input)?)
}

lazy_static! {
    static ref PSQL_START: Regex = Regex::new(r"(?m)^\s*\\[a-z]+ .*$").unwrap();
    static ref PSQL_END: Regex = Regex::new(r"(?m)\\[a-z]+.*$").unwrap();
    static ref PSQL_IF: Regex = Regex::new(r"\\if ").unwrap();
    static ref PSQL_COPY: Regex = Regex::new(r"(?m)^\\\.$").unwrap();
    static ref PSQL_VARIABLE_HEURISTIC: Regex = Regex::new("^[^-:']*:['\"]?[a-z]").unwrap();
    // static ref LEADING_WHITESPACE: Regex = Regex::new(r"^\s*(--.+)?$").unwrap();
    static ref LEADING_LINE_COMMENT: Regex = Regex::new(r"^\s*--").unwrap();
    // r#^[^-:']*:['"]?[a-z]# seems to match all the psql variables in the postgres
    // regression tests
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    PgSql = 0,
    PlPgSql = 1,
    Psql = 2,
    PlPerl = 3,
    PlPython = 4,
}

fn split_psql(input: String, url: Option<Url>) -> Result<Vec<Statement>, Failure> {
    // TODO: trim leading space & comments? assign to prev?
    let sql_statements = pg_query::split_statements_with_scanner(input.as_str())?;
    // pre-allocate a Vec close to the right size
    let mut result = Vec::<Statement>::with_capacity(sql_statements.len());

    for mut text in sql_statements {
        if text.len() == 0 {
            let statement = Statement::new(";".to_string(), Language::PgSql, None);
            result.push(statement);
        }
        if result.len() > 0 {
            // prev-append leading comments
            if text.chars().nth(0).unwrap() != '\n' {
                let mut line_number: usize = 0;
                let lines: Vec<&str> = text.split("\n").collect();
                loop {
                    // println!("stripping leading comments");
                    if line_number >= lines.len() {
                        break;
                    }
                    if let Some(_) = LEADING_LINE_COMMENT.find(lines[line_number]) {
                        line_number += 1;
                    } else {
                        break;
                    }
                }
                if line_number > 0 {
                    // tack it onto the prev statement
                    let prev = result.last_mut().unwrap();
                    prev.update(lines[0..line_number].join("\n"), prev.language);
                    // and shorten the current text
                    let rest = lines.join("\n");
                    text = &text[rest.as_str().len()..]
                }
            } else {
                let prev = result.last_mut().unwrap();
                prev.update("\n".to_string(), prev.language);
                text = text.trim_start();
            }

            // check for copy-in terminators
            loop {
                if let Some(terminator) = PSQL_COPY.find(text) {
                    let copied = &text[terminator.end() + 1..];
                    let prev = result.last_mut().unwrap();
                    prev.update(copied.to_string(), Language::Psql);
                    text = &text[terminator.end() + 1..];
                } else {
                    break;
                }
            }
        }

        // handle psql meta-commands
        loop {
            // println!("stripping leading psql meta commands");

            if let Some(m) = PSQL_START.find(text) {
                // check for \if \elif \else \endif
                let recognized = &text[m.start()..m.end()];
                if recognized.contains(r"\if") {
                    break;
                } else if recognized.contains(r"\elif") {
                    break;
                } else if recognized.contains(r"\else") {
                    break;
                } else if recognized.contains(r"\endif") {
                    break;
                } else {
                    // ayyy, it's a normal psql meta command. Snap it off as a
                    // separate statement.
                    text = &text[m.end()..];
                    result.push(Statement::new(recognized.to_string(), Language::Psql, None));
                }
                // if recognized
            } else {
                break;
            }
        }

        loop {
            if let Some(m) = PSQL_END.find(text) {
                // TODO: chop off up through the tail into a new statement
                let recognized = &text[m.start()..m.end()];
                if recognized.contains(r"\if") {
                    break;
                } else if recognized.contains(r"\elif") {
                    break;
                } else if recognized.contains(r"\else") {
                    break;
                } else if recognized.contains(r"\endif") {
                    break;
                } else {
                    let tail = &text[..m.end()];
                    text = &text[m.end()..];
                    result.push(Statement::new(tail.to_string(), Language::Psql, None));
                }
            } else {
                break;
            }
        }
        // TODO: check for inline \if, \elsif, empty
        let mut txt = text.to_string();
        txt.push_str(";");
        let statement = Statement::new(txt, Language::PgSql, url.to_owned());
        result.push(statement);
    }
    Ok(result)
}
// #[test]
// fn test_split_psql() {
//     let input = include_str!("../../../pg/psql.sql");
//     // let input = "select 1; select 2;".to_string();
//     let result = split_psql(input.to_string());
//     println!("{:?}", result);
//     assert!(result.is_ok());
//     // let expected: Vec<String> = vec!["select 1".to_string(), "select 2".to_string()];
//     // assert_eq!(result.unwrap(), expected);
// }
// output stuff ------------------------------------------------
trait Writable {
    fn write(self: &mut Self, statement: Statement) -> Result<u64, Failure>;
}

struct Tar<W: Write> {
    tar: tar::Builder<W>,
}

impl<W: Write> Tar<W> {
    fn new(obj: W) -> Self {
        let mut result = Tar {
            tar: tar::Builder::new(obj),
        };

        // TODO: add README on directory structure
        // result.write_file("/README.md", "TODO");
        return result;
    }

    fn write_file<P: AsRef<Path>>(self: &mut Self, path: P, data: &str) {
        let bytes: &[u8] = data.as_bytes();
        let size = u64::try_from(bytes.len()).unwrap(); // may panic on 32-bit systems
        let mut header = tar::Header::new_gnu();
        header.set_size(size);
        header.set_cksum();
        self.tar.append_data(&mut header, path, bytes);
    }
}

// impl<W: Write> Writable for Tar<W> {
//     fn write(self: &mut Self, statement: Statement) -> Result<u64, Failure> {

//         let path = format!("/{}-13/{}.{}.sql", statement.version, statement);
//     }
// }

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
    // let pg = fs::read_dir("./pg")?;
    // for ls in pg {
    //     if let Ok(file) = ls {
    //         let sql = fs::read_to_string(file.path())?;
    //         match split_psql(sql, None) {
    //             Ok(_) => println!("OK:\t{}", &file.path().to_str().unwrap()),
    //             Err(e) => println!("ERR:\t{}\t{:?}", &file.path().to_str().unwrap(), e),
    //         }
    //     }
    // }

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
            clap::Arg::with_name("url")
                .long("--url")
                .takes_value(true)
                .help("a url at which the input may be found.")
                .long_help(
                    "the url at which the input can be found.  If the input
                        is a directory of files and the url ends with a `/`, the
                        url will be considered the base path each file",
                ),
        )
        .arg(
            clap::Arg::with_name("version")
                .short("-v")
                .long("--version")
                .takes_value(true)
                .help("the version of postgres")
                .long_help(
                    "the version of postgres AND plpgsql AND psql for which
                        the input code is valid",
                ),
        )
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

    let splits = split_psql(buffer, None)?;
    for s in splits {
        println!("-- {:?} --------------------------------------", s.language);
        println!("{}", s.text);
    }
    return Ok(());
}
