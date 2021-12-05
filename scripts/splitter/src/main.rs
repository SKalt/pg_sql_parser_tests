use lazy_static::lazy_static;
use pg_query_wrapper as pg_query;
use psql_splitter;
use regex::Regex;
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

fn extract_pl(input: &str) -> Result<(String, String), Failure> {
    use pg_query::pbuf::node::Node;
    let stmts = pg_query::parse_to_protobuf(input)?.stmts;

    assert_eq!(stmts.len(), 1);
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

fn idenify_language(lang: &str) -> Language {
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

fn split_psql(input: String, url: Option<Url>) -> Result<Vec<Statement>, Failure> {
    // TODO: trim leading space & comments? assign to prev?
    let mut statements = vec![];
    let mut rest = input.as_str();
    while let Ok((r, statement)) = psql_splitter::statement(rest) {
        statements.push(statement);
        rest = r;
    }
    assert_eq!(
        rest,
        "",
        "did not consume >>>{}<<<",
        &input[input.len() - rest.len()..]
    );
    assert_eq!(input, statements.join("").as_str());

    // pre-allocate a Vec close to the right size
    let mut result = Vec::<Statement>::with_capacity(statements.len());

    for text in statements {
        if psql_splitter::is_psql(text) {
            result.push(Statement::new(
                text.to_string(),
                Language::Psql,
                url.to_owned(),
            ))
        } else {
            result.push(Statement::new(
                text.trim().to_string(),
                Language::PgSql,
                url.to_owned(),
            ));
            if let Ok((inner, lang)) = extract_pl(text) {
                result.push(Statement::new(
                    inner,
                    idenify_language(lang.as_str()),
                    url.to_owned(),
                ))
            }
        }
    }
    Ok(result)
}

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
