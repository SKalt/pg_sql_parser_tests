use pg_query_wrapper as pg_query;
use regex;
use std::{
    collections::HashSet,
    fs::{self, OpenOptions},
    io::{self, Read, Write},
    path,
};
use xxhash_rust as xxhash;

#[derive(Debug)]
enum Failure {
    IoErr(io::Error),
    PgQueryError(pg_query::Failure),
    DirDne,
    NotDir,
}
impl From<io::Error> for Failure {
    fn from(e: io::Error) -> Self {
        Self::IoErr(e)
    }
}
impl From<pg_query::Failure> for Failure {
    fn from(e: pg_query::Failure) -> Self {
        Self::PgQueryError(e)
    }
}

fn digest(input: &str) -> String {
    return format!("{:016x}", xxhash::xxh3::xxh3_64(input.as_bytes()));
}

/// append to a sorted, unique, single-column tsv
fn insert_sorted(list_file: &path::Path, text: &str) {
    if !list_file.exists() {
        let mut file = std::fs::File::create(list_file).expect("could not create file");
        file.write_all(text.as_bytes())
            .expect("could not write to file");
    } else {
        let mut file = OpenOptions::new()
            .append(true)
            .read(true)
            .open(list_file)
            .expect("could not open file as read-append");
        let mut current_text = String::new();
        file.read_to_string(&mut current_text)
            .expect("unable to read file");
        let mut values = current_text.split("\n").collect::<HashSet<&str>>();
        if values.insert(text) {
            let mut desired: Vec<String> = values.iter().map(|s| s.to_string()).collect();
            desired.sort();

            file.write_all(desired.join("\n").as_bytes())
                .expect("could not write to file");
        }
    }
    return ();
}

fn main() -> Result<(), Failure> {
    let psql_meta_pattern = regex::Regex::new(r"^\s*\\[a-z]").unwrap();

    let matches =
        clap::App::new("splitter")
            .arg(
                clap::Arg::with_name("live")
                    .takes_value(false)
                    .multiple(false),
            )
            .arg(
                clap::Arg::with_name("dry_run")
                    .long("--dry-run")
                    .multiple(false)
                    .conflicts_with("live"),
            )
            .arg(
                // the directory to link
                clap::Arg::with_name("outdir")
                    .long("--out-dir")
                    .short("-o")
                    .default_value("./fixtures/data"),
            )
            .arg(
                clap::Arg::with_name("input")
                    .long("--input")
                    .short("-i")
                    .default_value("-")
                    .help("the file or device from which to read SQL (`-` means stdin)"),
            )
            .arg(
                clap::Arg::with_name("url")
                    .long("--url")
                    .takes_value(true)
                    .help("a url at which the input may be found."),
            )
            .arg(clap::Arg::with_name("version").short("-v").takes_value(true).help(
                "the version of postgres AND plpgsql AND psql for which the input code is valid",
            ))
            .get_matches();

    // read from stdin or a file
    let mut buffer = String::new();
    match matches.value_of("input") {
        None | Some("-") => {
            io::stdin().read_to_string(&mut buffer)?;
        }
        Some(filename) => {
            buffer = fs::read_to_string(filename)?;
        }
    }

    let lines = buffer.split("\n");
    // let psql_meta = lines
    //     .clone()
    //     .filter(|line| psql_meta_pattern.is_match(line))
    //     .map(|s| s.to_string())
    //     .collect::<Vec<String>>();

    let normal = lines
        .filter(|line| !psql_meta_pattern.is_match(line))
        .collect::<Vec<&str>>()
        .join("\n");
    let stmts = pg_query::split_statements_with_scanner(normal.trim_end())?
        .iter()
        .map(|stmt| stmt.trim_start().to_string() + ";")
        .collect::<Vec<String>>();

    // write to {xxhash}.sql in the current directory where d matches /[0-9]/
    let outdir = std::path::Path::new(matches.value_of("outdir").unwrap());
    if !outdir.exists() {
        return Err(Failure::DirDne);
    }
    if !outdir.is_dir() {
        return Err(Failure::NotDir);
    }
    let dry_run = matches.is_present("dry_run");
    for stmt in stmts.iter() {
        let hash = digest(stmt);
        let target_dir = outdir.clone().join(hash.as_str());
        if dry_run {
            println!(
                "-- {} --------------------------------------",
                hash.as_str()
            );
            println!("{};", &stmt);
        } else {
            println!("{}", &hash)
        }
        if !target_dir.exists() {
            if !dry_run {
                fs::create_dir_all(&target_dir).expect("unable to create dir");
            } else {
                println!("-- would create dir {}", target_dir.display());
            }
        } else if !target_dir.is_dir() {
            if !dry_run {
                panic!("{} is not a directory", target_dir.display());
            } else {
                println!(
                    "-- would panic since {} is not a directory",
                    target_dir.display()
                );
            }
        }

        let target_file = target_dir.clone().join("input.sql");
        if !target_file.exists() {
            if !dry_run {
                fs::File::create(target_file)
                    .unwrap()
                    .write_all(stmt.as_bytes())?;
            } else {
                println!("-- would write to   {}", target_file.display());
            }
        }
        if matches.is_present("url") {
            let url = matches.value_of("url").unwrap();
            let urls_file = target_dir.clone().join("urls.tsv");
            insert_sorted(&urls_file, url);
        }
        if matches.is_present("version") {
            let version = matches.value_of("version").unwrap();
            let version_file = target_dir.clone().join("versions.tsv");
            insert_sorted(&version_file, version);
        }
    }
    return Ok(());
}
