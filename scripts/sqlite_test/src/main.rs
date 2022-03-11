extern crate pest;
#[macro_use]
extern crate pest_derive;
use core::panic;
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

fn unescape(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\r", "\r")
}

fn extract_test_body(test_body: pest::iterators::Pair<Rule>) {
    for inner in test_body.into_inner() {
        match inner.as_rule() {
            Rule::body_scan => {
                println!("{}", inner.as_str());
            }
            _ => {
                println!("{:?}", inner.as_rule());
            }
        }
    }
}
// should extract test name, test sql
fn extract_test(do_test_stmt: pest::iterators::Pair<Rule>) -> () {
    if do_test_stmt.as_rule() != Rule::do_test_stmt {
        panic!(
            "passed {:?}, only `do_test_statement` allowed",
            do_test_stmt.as_rule(),
        );
    }
    for stmt in do_test_stmt.into_inner() {
        match stmt.as_rule() {
            Rule::do_test => {}
            Rule::test_name => {
                println!("test name: {}", stmt.as_str());
            }
            Rule::test_body => {
                println!("test body:");
                extract_test_body(stmt)
            }
            _ => {
                println!("{:?}", stmt.as_rule());
            }
        }
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
    let statements = SqliteTestParser::parse(Rule::statements, input.as_str())
        .expect("failed to parse")
        .next()
        .unwrap();
    // almost infallible, but might fail
    let mut unparsed = String::new();
    let mut line_number = 1usize;
    for stmt in statements.into_inner() {
        match stmt.as_rule() {
            // only valid children are statement and EOI
            Rule::EOI => {
                break;
            }
            Rule::statement => {
                for s in stmt.into_inner() {
                    match s.as_rule() {
                        Rule::other => {
                            unparsed.push_str(unescape(s.as_str()).as_str());
                        }
                        Rule::do_test_stmt => {
                            extract_test(s.clone());
                        }
                        _ => {
                            if unparsed.len() > 0 {
                                println!("unparsed:\n{:?}\n", unparsed.as_str());
                                unparsed.clear();
                            }
                            if s.as_rule() == Rule::comment {
                                println!("{}:{} : {:?}", path, line_number, s.as_rule());
                            } else {
                                println!("{}:{} : {:?}", path, line_number, s.as_rule());
                                // println!("parsed: ");
                                // println!("{}", unescape(s.as_str()));
                            }
                        }
                    }
                    line_number += s.as_str().matches("\n").count();
                }
            }
            _ => {
                println!("{:?}", stmt.as_rule());
            }
        };
    }
    if unparsed.len() > 0 {
        println!("unparsed:\n{:?}\n", unparsed.as_str());
        unparsed.clear();
    }
    return Ok(());
}
