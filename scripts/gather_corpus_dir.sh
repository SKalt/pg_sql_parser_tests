#!/bin/sh

main() {
  set -eu
  splitter="$1"
  input_dir="$2"
  pg_version="$3"
  output_db="${4:-./corpus.db}"

  find "$input_dir" -name '*.sql' | sort | while read -r input_file
    do
      relative_path="$(echo "$input_file" | sed "s#$input_dir##g")"
      printf "%-4s %s\n" "$pg_version" "$relative_path"
      "$splitter" \
        --input "$input_file" --out "$output_db" \
        --license "$input_dir"/COPYRIGHT --spdx PostgreSQL \
        --url "https://github.com/postgres/postgres/blob/REL_${pg_version}_STABLE/${input_file##/pg/}" \
        --url "https://git.postgresql.org/gitweb/?p=postgresql.git;a=blob;f=${input_file##/pg/};hb=refs/heads/REL_${pg_version}_STABLE" \
      || true; # allow unicode failures to pass
    done;
}

main "$@"