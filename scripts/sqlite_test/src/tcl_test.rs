use pest::Parser;

#[derive(Parser)]
#[grammar = "tcl_test.pest"]
pub struct TclTestParser;
