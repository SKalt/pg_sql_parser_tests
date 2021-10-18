#!/usr/bin/env sh


main() {
  src="${1:-./pg}"
  version="${2:-13}"
  zero_ver="$(printf "%03d" "$version")"
  url="https://github.com/postgres/postgres/blob/REL_${version}_STABLE/src/test/regress/sql"

  for f in "$src"/*.sql; do
    filename="$(basename "$f")"
    suite="./fixtures/versions/$zero_ver/regress/${filename%.*}"
    i=0
    for hash in $(./target/debug/splitter --input "$f" --url "$url/$filename" -v 013); do
      ref="$(printf "%04d" $i)"
      mkdir -p "$suite"
      ln -s "../../../../data/$hash" "$suite/$ref"
      i="$((i+1))"
    done
  done
}

main "$@"