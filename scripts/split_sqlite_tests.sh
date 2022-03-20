#!/bin/bash
set -eu
for f in ./external/sqlite/**/a*.test; do
  if ! bin/sqlite_test --input "$f" >>/tmp/test_results; then
    (
      tput setaf 1
      echo "$f"
      tput sgr0
    ) >&2
    exit 1
  fi
done
