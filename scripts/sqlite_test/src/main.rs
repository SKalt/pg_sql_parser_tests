extern crate pest;
#[macro_use]
extern crate pest_derive;
use core::panic;
use std::{cell::RefCell, fs, io, rc::Rc};

use pest::{
    iterators::{Pair, Pairs},
    Parser,
};

#[derive(Parser)]
#[grammar = "test_grammar.pest"]
struct SqliteTestParser;

#[derive(Debug)]
enum Failure {
    Io(io::Error),
    Parse(pest::error::Error<Rule>),
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

fn unescape(s: &str) -> String {
    s.replace("\\n", "\n").replace("\\r", "\r")
}

fn walk<F>(pair: Pair<Rule>, path: &str, line_number: usize, mut callback: F)
where
    F: Copy + FnMut(&Pair<Rule>, &str, usize),
{
    callback(&pair, path, line_number);
    let mut inner_line_number = line_number;
    for pair in pair.clone().into_inner() {
        let n_lines = pair.as_str().matches("\n").count();
        walk(pair, path, inner_line_number, callback);
        inner_line_number += n_lines;
    }
}

fn extract_test_body(test_body: pest::iterators::Pair<Rule>) {
    for inner in test_body.into_inner() {
        match inner.as_rule() {
            // Rule::body_scan => {
            //     println!("{}", inner.as_str());
            // }
            _ => {
                println!("{:?}", inner.as_rule());
            }
        }
    }
}

// should extract test name, test sql
fn extract_test(do_test_stmt: pest::iterators::Pair<Rule>, path: &str, line_number: usize) -> () {
    // if do_test_stmt.as_rule() != Rule::do_test_stmt {
    //     panic!(
    //         "passed {:?}, only `do_test_statement` allowed",
    //         do_test_stmt.as_rule(),
    //     );
    // }
    // for stmt in do_test_stmt.into_inner() {
    //     match stmt.as_rule() {
    //         Rule::do_test => {
    //             println!("{}", stmt.as_str());
    //         }
    //         Rule::test_name => {
    //             println!("{}:{} test name: {}", path, line_number, stmt.as_str());
    //         }
    //         Rule::test_body => {
    //             println!("test body:");
    //             extract_test_body(stmt)
    //         }
    //         _ => {
    //             println!("{:?}", stmt.as_rule());
    //         }
    //     }
    // }
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
    match SqliteTestParser::parse(Rule::main, input.as_str()) {
        Err(e) => {
            if let pest::error::LineColLocation::Pos((line, col)) = e.line_col {
                println!("{}:{}:{}", path, line, col)
            } else {
                panic!("{}", e);
            }
            return Err(Failure::Parse(e));
            // panic!("{:?}", e);
        }
        Ok(mut statements) => {
            let line_number = 1usize;
            let mut sqls = Vec::new();
            let shared = Rc::new(RefCell::new(&mut sqls));
            let callback = |pair: &Pair<Rule>, path: &str, line_number: usize| match pair.as_rule()
            {
                Rule::sql_block => shared
                    .borrow_mut()
                    .push((pair.as_str().to_owned(), line_number)),
                _ => {}
            };
            for stmt in statements.next().unwrap().into_inner() {
                match stmt.as_rule() {
                    // only valid children are statement and EOI
                    Rule::EOI => {
                        break;
                    }
                    _ => {
                        walk(stmt, path, line_number, callback);
                    }
                };
            }
            for (sql, line_number) in sqls {
                println!("{}:{} : {}", path, line_number, sql.as_str());
            }
            return Ok(());
        }
    }
}
