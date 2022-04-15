extern crate pest;
#[macro_use]
extern crate pest_derive;
use corpus::{Statement, StatementSource};
use pest::RuleType;
use std::convert::TryInto;
use std::{fs, io, time::Instant};
use tcl_test::find_test_cases;
use xxhash_rust::xxh3::xxh3_64;

mod search;
mod sqlite_cli;
mod tcl_test;
// use search::{contains, walk};

#[derive(Debug)]
enum Failure {
    Io(io::Error),
    Parse(String),
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
impl From<String> for Failure {
    fn from(e: String) -> Self {
        Self::Parse(e)
    }
}

fn main() -> Result<(), Failure> {
    let cli = clap::App::new("sqlite_test_parser")
        .arg(
            clap::Arg::with_name("input")
                .short("-i")
                .long("--input")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("debug")
                .long("--debug")
                .takes_value(false),
        );
    let args = cli.get_matches();
    let path = args.value_of("input").unwrap();
    let should_debug = args.is_present("debug");

    let input = fs::read_to_string(path)?;
    let mut urls: Vec<&str> = Vec::with_capacity(args.occurrences_of("url").try_into().unwrap());
    if let Some(url_args) = args.values_of("url") {
        for url in url_args {
            urls.push(url);
        }
    };
    let document_id = xxh3_64(input.as_bytes()) as i64;
    let start_time = Instant::now();
    let test_cases = find_test_cases(input.as_str(), path)?;
    eprintln!(
        "{:5} test-cases found in {:>6.2} ms in {}",
        &test_cases.len(),
        start_time.elapsed().as_millis(),
        path,
    );
    let mut statements: Vec<Statement> = Vec::new();

    for (sql, line_number) in test_cases {
        let mut stmts = sqlite_cli::split(sql.as_str(), path, document_id, line_number)?;
        statements.append(&mut stmts);
        println!("debug: {}:{} : {}", path, line_number, sql.as_str());
    }

    Ok(())
}
