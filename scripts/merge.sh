#!/bin/sh
### USAGE: merge.sh [-h|--help] [--dry-run] --out[=./corpus.db] INPUT_DBS...
### merge INPUT_DBS into a single test-corpus dtabase
###
### FLAGS:
###   -h, --help print this message and exit 0
###   --dry-run  print what would happen without actually doing it
###   --out      a path for the output database.  Must not yet exist.
###              [default ./corpus.db]
### ARGS:
###   INPUT_DBS: paths to the input databases.  Must all exist and have
###              schema_version 0.0

usage() { grep -e "^###" "$0" |  sed 's/^### //g' | sed 's/###//g'; }
get_absolute_path() { (cd "$(dirname "$1")" && pwd); }
get_db_schema_version() {
  sqlite3 "$1" "select major, minor from schema_version;";
}

validate_input_db_version() {
    get_db_schema_version "$1" | grep -q "0|0"
}
bulk_sql="
insert or ignore into main.languages              select * from other.languages;
insert or ignore into main.versions               select * from other.versions;
insert or ignore into main.language_versions      select * from other.language_versions;
insert or ignore into main.statements             select * from other.statements;
insert or ignore into main.statement_languages    select * from other.statement_languages;
insert or ignore into main.statement_fingerprints select * from other.statement_fingerprints;
insert or ignore into main.statement_versions     select * from other.statement_versions;
insert or ignore into main.documents              select * from other.documents;
insert or ignore into main.urls                   select * from other.urls;
insert or ignore into main.document_statements    select * from other.document_statements;
insert or ignore into main.licenses               select * from other.licenses;
insert or ignore into main.oracles                select * from other.oracles;
insert or ignore into main.predictions            select * from other.predictions;
"

main() {
  set -eu
  dry_run=false
  input_dbs="$(mktemp)"
  out=./corpus.db
  ifs_reset=$IFS

  # read in flags and args
  while test -n "${1:-}"; do
    case "$1" in
      -h|--help) usage && exit 0;;
      --dry-run) dry_run=true; shift;;
      --out) shift; out="$1"; shift;;
      --out=*) out="$(echo "$1" | cut -d= -f2)"; shift;;
      *) echo "$1" >> "$input_dbs"; shift;;
    esac
  done

  # validate flags, args
  validation_errors="$(mktemp)"

  if test -f "$out"; then
    echo "--out: $out is already present" >> "$validation_errors"
  fi

  if test "$(wc -l "$input_dbs" | awk '{print $1}')" -lt 2; then
    echo "at least two input dbs required" >> "$validation_errors"
  fi

  cat "$input_dbs" | while IFS= read -r input_db
  do
    if ! test -f "$input_db"; then
      echo "unable to read $input_db : $(ls -al "$input_db")" >> "$validation_errors"
    fi
    if ! validate_input_db_version "$input_db"; then
      printf "invalid schema version:" >> "$validation_errors"
      get_db_schema_version "$input_db" >> "$validation_errors"
    fi
  done
  IFS=$ifs_reset

  if test "$(wc -l "$validation_errors" | awk '{print $1}')" -gt 0; then
    cat "$validation_errors">&2 && exit 1
  fi

  schema="$(get_absolute_path "$(dirname "$0")/../..")/schema.sql"

  cmd="sqlite3 $out < $schema"
  if "$dry_run"; then
    echo "would run '$cmd'"
  else
    eval "$cmd"
  fi

  cat "$input_dbs" | while IFS= read -r input_db
  do
    printf "starting %s ... " "$input_db"
    sql="ATTACH DATABASE '$input_db' AS other; $bulk_sql; DETACH other;"
    cmd="sqlite3 $out \"$sql\""
    if "$dry_run"; then echo "would run '$cmd'"; else eval "$cmd"; fi
    echo "done"
  done
}

main "$@"