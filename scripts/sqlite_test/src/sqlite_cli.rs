use std::{cell::RefCell, panic::PanicInfo, rc::Rc};

use corpus::{Language, Statement};
use pest::{iterators::Pair, Parser};
#[derive(Parser)]
#[grammar = "sqlite_cli.pest"]
pub struct SqliteCliParser;

pub fn split(
    sql: &str,
    path: &str,
    document_id: i64,
    start_line: usize,
) -> Result<Vec<Statement>, String> {
    match SqliteCliParser::parse(Rule::program, sql) {
        Ok(mut program) => {
            let mut statements = Vec::new();

            for chunk in program.next().unwrap().into_inner() {
                match chunk.as_rule() {
                    Rule::statement => {
                        for stmt in chunk.into_inner().into_iter() {
                            let lang = match stmt.as_rule() {
                                Rule::sql_statement => Language::Sqlite3,
                                Rule::cli_comment | Rule::dot_command => Language::Sqlite3Cli,
                                _ => panic!(
                                    "unexpected rule type: {:?} for {}",
                                    stmt.as_rule(),
                                    stmt.as_str()
                                ),
                            };
                            statements.push(Statement::new(
                                stmt.as_str().to_owned(),
                                lang,
                                document_id,
                            ));
                        }
                    }
                    Rule::newline | Rule::EOI => {} // ignore
                    _ => panic!(
                        "unexpected rule type: {:?} for {}",
                        chunk.as_rule(),
                        chunk.as_str()
                    ),
                }
            }
            return Ok(statements);
        }
        Err(e) => {
            let result = if let pest::error::LineColLocation::Pos((line, col)) = e.line_col {
                format!(
                    "ERROR parsing sqlite: {}:{}:{}: {:?} \n>>>{}<<<\n",
                    path,
                    start_line + line,
                    col,
                    e,
                    sql
                )
            } else {
                format!("ERROR parsing sqlite: {}", e)
            };
            return Err(result);
        }
    }
}
