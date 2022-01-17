#!/bin/sh

if test -t 1 || test -n "${FORCE_COLOR:-}"; then
  red="$(tput setaf   1)"
  green="$(tput setaf 2)"
  faint="$(tput setaf 7)"
  reset="$(tput setaf sgr0)"
else
  red=
  green=
  faint=
  reset=
fi
main() {
  set -eu
  splitter="$1"
  input_dir="$2"
  pg_version="$3"
  output_db="${4:-./corpus.db}"
  echo "reading postgres $pg_version from $input_dir; writing to $output_db"
  find "$input_dir" -name '*.sql' | sort | while read -r input_file
    do
      relative_path="$(echo "$input_file" | sed "s#$input_dir##g")"
      head="REL_${pg_version}_STABLE"

      gh_url="https://github.com/postgres/postgres/blob/${head}${relative_path}"
      pg_url="https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${relative_path};hb=refs/heads/${head}"
      if result="$(
        "$splitter" --count \
          --input "$input_file" --out "$output_db" \
          --license "$input_dir"/COPYRIGHT --spdx PostgreSQL \
          --url "$gh_url" \
          --url "$pg_url" 2>&1
      )"; then
        printf "%s%-4s%s %s %s%s\n" "$faint" "$pg_version" "$green" "$result" "$reset" "$relative_path"
      elif (echo "$result" | grep -q "stream did not contain valid UTF-8"); then
        printf "%s: %s%s%s\n" "$relative_path" "$red" "$result" "$reset" >&2;
      else
        echo "${relative_path}: ${red}${result}${reset}:"
      fi
    done;
}

main "$@"