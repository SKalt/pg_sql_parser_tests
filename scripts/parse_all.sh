#!/bin/bash
make bin/parse
find ./fixtures -name 'err.txt' | xargs rm
all_example_dirs=($(find ./fixtures -maxdepth 2 -mindepth 2 -type d))
printf "" > /tmp/out
successes=0
failures=0
for example in "${all_example_dirs[@]}"; do
  if ! [ -e "$example/whitelist" ]; then
    if (bin/parse "$example" >> /tmp/out); then
      successes=$((successes + 1))
    else
      echo "FAIL: $example/input.sql; $example/err.txt"
      failures=$((failures + 1))
    fi
  fi
done
echo "successes: $successes / ${#all_example_dirs[@]}"
echo "failures: $failures / ${#all_example_dirs[@]}"
