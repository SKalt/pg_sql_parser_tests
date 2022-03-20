extern crate pest;
#[macro_use]
extern crate pest_derive;
use core::panic;
use std::{cell::RefCell, fs, io, rc::Rc};
use textwrap::dedent;

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

fn walk<F>(pair: Pair<Rule>, n: usize, mut callback: F) -> usize
where
    F: Copy + FnMut(&Pair<Rule>, usize) -> usize,
{
    callback(&pair, n);
    let mut inner_line_number = n;
    for pair in pair.clone().into_inner() {
        let n_lines = pair.as_str().matches("\n").count();
        walk(pair, inner_line_number, callback);
        inner_line_number += n_lines;
    }
    return inner_line_number;
}

fn contains_rule(pair: Pair<Rule>, rule: Rule) -> bool {
    let callback = |pair: &Pair<Rule>, n: usize| {
        if pair.as_rule() == rule {
            return 1;
        } else {
            return 0;
        }
    };
    let mut n_found = walk(pair, 0, callback);
    return n_found > 0;
}

fn trim_word() {}

fn main() -> Result<(), Failure> {
    use std::time::Instant;
    let start_time = Instant::now();
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
            let callback = |pair: &Pair<Rule>, line_number: usize| {
                match pair.as_rule() {
                    Rule::sql_block => shared
                        .borrow_mut()
                        .push((pair.as_str().to_owned(), line_number)),
                    _ => {}
                }
                return 0;
            };
            for stmt in statements.next().unwrap().into_inner() {
                match stmt.as_rule() {
                    // only valid children are SOI, cmd, and EOI
                    Rule::EOI => {
                        break;
                    }
                    _ => {
                        walk(stmt, line_number, callback);
                    }
                };
            }
            println!(
                "{:5} test-cases found in {} in {:.2?}",
                &sqls.len(),
                path,
                start_time.elapsed()
            );
            // for (sql, line_number) in sqls {
            //     // println!("{}:{} : {}", path, line_number, sql.as_str());
            // }
            return Ok(());
        }
    }
}
