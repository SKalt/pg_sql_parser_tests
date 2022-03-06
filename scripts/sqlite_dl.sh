#!/usr/bin/env bash
### USAGE: dl.sh [-h|--help]
usage() { grep '^###' "$0" | sed 's/^### //g; s/^###//g'; }

this_dir="${BASH_SOURCE[0]%/*}"
repo_root="$(cd "$this_dir/.." && pwd)"
git_url=https://github.com/sqlite/sqlite.git
target_dir="${repo_root}/external/sqlite"

is_git_dir() { git rev-parse; }

init() {
  mkdir -p "$target_dir" && cd "$target_dir"
  if ! is_git_dir; then
    git init && git remote add -f origin "$git_url"
  fi

  git config core.sparseCheckout true &&
    git sparse-checkout init &&
    git sparse-checkout set test &&
    git pull origin master
}

main() {
  set -euo pipefail
  while [ -n "${1:-}" ]; do
    case "$1" in
    -h | --help) usage && exit 0 ;;
    esac
  done
  init
  tree --filelimit 10 "$target_dir"
}

if [ "${BASH_SOURCE[0]}" = "$0" ]; then main "$@"; fi
