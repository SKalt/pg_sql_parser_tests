use pest::Parser;
use std::rc::Rc;
use std::{cell::RefCell, time::Instant};

use crate::search::{contains, walk};
use pest::iterators::Pair;
use textwrap::{dedent, indent};

#[derive(Parser)]
#[grammar = "tcl_test.pest"]
pub struct TclTestParser;

fn unescape(s: &str) -> String {
    s.replace("\\\n", " ")
        .replace("\\n", "\n")
        .replace("\\r", "\r")
}

fn trim_sql_block(text: &str) -> String {
    let text = if text.len() <= 2 {
        ""
    } else {
        &text[1..text.len() - 1] // safe for a sql block {...} or dquote "..."
    };
    return dedent(text);
}
#[test]
fn test_trim_sql_block() {
    assert_eq!(trim_sql_block("{}"), "".to_owned());
    assert_eq!(trim_sql_block("{select 1}"), "select 1".to_owned());
    assert_eq!(trim_sql_block("{\nselect 1\n}"), "\nselect 1\n".to_owned());
}

pub fn extract_sql_candidate(
    sql_candidate: Pair<Rule>,
    start_line: usize,
) -> Result<(String, usize), String> {
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

pub fn find_test_cases(input: &str, path: &str) -> Result<Vec<(String, usize)>, String> {
    match TclTestParser::parse(Rule::main, input) {
        Err(e) => {
            let result = if let pest::error::LineColLocation::Pos((line, col)) = e.line_col {
                Err(format!("ERROR: {}:{}:{}", path, line, col))
            } else {
                Err(format!("{:?}", e))
            };
            result
        }
        Ok(mut statements) => {
            let mut line_number = 1usize;
            let mut test_cases = Vec::new();
            let shared = Rc::new(RefCell::new(&mut test_cases));

            let callback = |pair: &Pair<Rule>, n: usize| {
                // TODO: match classes of commands
                match pair.as_rule() {
                    Rule::sql_block => shared.borrow_mut().push((
                        indent(trim_sql_block(&unescape(pair.as_str())).as_str(), "  "),
                        n,
                    )),
                    _ => {}
                }
                return n + pair.as_str().matches("\n").count();
            };

            for stmt in statements.next().unwrap().into_inner() {
                line_number = walk(stmt, line_number, callback);
            }
            return Ok(test_cases);
        }
    }
}
