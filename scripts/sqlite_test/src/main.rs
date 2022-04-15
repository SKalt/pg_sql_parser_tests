extern crate pest;
#[macro_use]
extern crate pest_derive;
use core::panic;
use corpus::{Statement, StatementSource};
use pest::{iterators::Pair, Parser, RuleType};
use std::{cell::RefCell, fs, io, rc::Rc};
use textwrap::{dedent, indent};

mod sqlite_cli;
mod tcl_test;

#[derive(Debug)]
enum Failure<Rule: RuleType = tcl_test::Rule> {
    Io(io::Error),
    Parse(pest::error::Error<Rule>),
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::Io(e)
    }
}

fn unescape(s: &str) -> String {
    s.replace("\\\n", " ")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
}

fn walk<Callback, State, Rule>(pair: Pair<Rule>, initial: State, mut callback: Callback) -> State
where
    State: Copy,
    Rule: RuleType,
    Callback: Copy + FnMut(&Pair<Rule>, State) -> State,
{
    let mut inner = initial;
    for pair in pair.clone().into_inner() {
        inner = walk(pair, inner, callback);
    }
    return callback(&pair, initial);
}

fn contains<Rule: RuleType>(pair: Pair<Rule>, rules: Vec<Rule>) -> bool {
    let callback = |pair: &Pair<Rule>, n_found: usize| {
        for rule in rules.as_slice() {
            if &pair.as_rule() == rule {
                return n_found + 1;
            }
        }
        return n_found;
    };
    let n_found = walk(pair, 0, callback);
    return n_found > 0;
}

fn trim_sql_block(text: &str) -> String {
    let text = if text.len() <= 2 {
        ""
    } else {
        &text[1..text.len() - 2] // safe for a sql block {...} or dquote "..."
    };
    return dedent(text);
}

fn extract_sql_candidate(
    sql_candidate: Pair<tcl_test::Rule>,
    start_line: usize,
) -> Result<(String, usize), String> {
    use tcl_test::Rule;
    match sql_candidate.as_rule() {
        Rule::sql_block => Ok((trim_sql_block(sql_candidate.as_str()), start_line)),
        Rule::sql_dquote => {
            if contains(
                sql_candidate.clone(),
                vec![Rule::dollar_sub, Rule::bracket_sub],
            ) {
                return Err(format!("{} contains substitution", sql_candidate.as_str()));
            } else {
                Ok((
                    unescape(&trim_sql_block(sql_candidate.as_str())),
                    start_line,
                ))
            }
        }
        Rule::sql_word => Ok((sql_candidate.as_str().to_string(), start_line)),
        _ => unreachable!(),
    }
}

fn main() -> Result<(), Failure> {
    use std::time::Instant;
    let start_time = Instant::now();
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
    match tcl_test::TclTestParser::parse(tcl_test::Rule::main, input.as_str()) {
        Err(e) => {
            if let pest::error::LineColLocation::Pos((line, col)) = e.line_col {
                println!("{}:{}:{}", path, line, col)
            } else {
                panic!("{}", e);
            }
            return Err(Failure::Parse(e));
        }
        Ok(mut statements) => {
            use tcl_test::Rule;
            let mut line_number = 1usize;
            let mut test_cases = Vec::new();
            let shared = Rc::new(RefCell::new(&mut test_cases));

            let callback = |pair: &Pair<Rule>, n: usize| {
                // TODO: match classes of commands
                match pair.as_rule() {
                    Rule::sql_block => shared
                        .borrow_mut()
                        .push((indent(trim_sql_block(pair.as_str()).as_str(), "  "), n)),
                    _ => {}
                }
                return n + pair.as_str().matches("\n").count();
            };

            for stmt in statements.next().unwrap().into_inner() {
                line_number = walk(stmt, line_number, callback);
            }
            eprintln!(
                "{:5} test-cases found in {:>6.2} ms in {}",
                &test_cases.len(),
                start_time.elapsed().as_millis(),
                path,
            );
            if should_debug {
                for (sql, line_number) in test_cases {
                    println!("debug: {}:{} : {}", path, line_number, sql.as_str());
                }
            }
            return Ok(());
        }
    }
}
