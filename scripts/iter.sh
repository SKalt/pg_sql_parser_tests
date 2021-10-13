#!/bin/bash
declare -a todo;
todo=()
for i in $(find ./fixtures -name 'err.txt' | sort); do
  todo+=("$i")
done
j=0

for i in "${todo[@]}"; do
  j=$((j+1));
  echo "$j/${#todo[@]}";
  code --wait "${i%/*}/input.sql";
done
