extern crate pest;
#[macro_use]
extern crate pest_derive;
use std::{fs, io};

use pest::Parser;

#[derive(Parser)]
#[grammar = "test_grammar.pest"]
struct SqliteTestParser;

#[derive(Debug)]
enum Failure {
    Io(io::Error),
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}
fn main() -> Result<(), Failure> {
    let cli = clap::App::new("sqlite_test_parser").arg(
        clap::Arg::with_name("input")
            .short("-i")
            .long("--input")
            .takes_value(true),
    );
    let args = cli.get_matches();
    let path = args.value_of("input").unwrap();
    let input = fs::read_to_string(path)?;
    let x = SqliteTestParser::parse(Rule::statements, input.as_str()).unwrap();
    return Ok(());
}
