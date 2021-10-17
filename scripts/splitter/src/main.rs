use md5;
use pg_query_wrapper;
use std::{
    fs,
    io::{self, Read, Write},
};

#[derive(Debug)]
enum Failure {
    IoErr(io::Error),
    PgQueryError(pg_query_wrapper::Failure),
    DirDne,
    NotDir,
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::IoErr(e)
    }
}
impl From<pg_query_wrapper::Failure> for Failure {
    fn from(e: pg_query_wrapper::Failure) -> Self {
        Self::PgQueryError(e)
    }
}

fn main() -> Result<(), Failure> {
    // TODO: accept --dry-run/--live flag
    let matches = clap::App::new("splitter")
        .arg(
            clap::Arg::with_name("live")
                .takes_value(false)
                .multiple(false),
        )
        .arg(
            clap::Arg::with_name("dry_run")
                .long("--dry-run")
                .multiple(false)
                .required(true)
                .conflicts_with("live"),
        )
        .arg(
            // the directory to link
            clap::Arg::with_name("outdir")
                .long("--out-dir")
                .short("-o")
                .default_value("."),
        )
        .arg(
            clap::Arg::with_name("input")
                .long("--input")
                .short("-i")
                .default_value("-")
                .help("the file or device from which to read SQL (`-` means stdin)"),
        )
        .get_matches();

    // read from stdin
    let mut buffer = String::new();
    match matches.value_of("input") {
        None | Some("-") => {
            io::stdin().read_to_string(&mut buffer)?;
        }
        Some(filename) => {
            buffer = fs::read_to_string(filename)?;
        }
    }

    let stmts = pg_query_wrapper::split_statements_with_scanner(buffer.as_str())?;

    if matches.is_present("dry_run") {
        for stmt in stmts.iter() {
            println!("----------------------------------------------------");
            println!("{}", *stmt);
        }
        return Ok(());
    } else {
        // write to ddd.sql in the current directory where d matches /[0-9]/
        let outdir = std::path::Path::new(matches.value_of("outdir").unwrap());
        if !outdir.exists() {
            return Err(Failure::DirDne);
        }
        if !outdir.is_dir() {
            return Err(Failure::NotDir);
        }

        for (i, stmt) in stmts.iter().enumerate() {
            let mut target_file = std::fs::File::create(outdir.join(format!("{:03}", i)))?;
            target_file.write_all(stmt.as_bytes())?;
        }
        return Ok(());
    }
}
