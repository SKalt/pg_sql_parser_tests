#!/usr/bin/env sh
###
usage() { grep -e "^###" "$0" |  sed 's/^### //g' | sed 's/###//g'; }

main() {
  set -eu
  # expect pg_version environment variable to be set to a valid integer version
  case "$pg_version" in
    10|11|12|13|14) : ;; # pass
    *) echo "unrecognized pg version: $pg_version" && exit 1;;
  esac

  mkdir -p "/tmp/pg/"
  target_dir="/tmp/pg/$pg_version"
  tgz_file="/tmp/pg/$pg_version.tar.gz"
  # pull from gh since this is most likely to run in gh actions
  url="https://github.com/postgres/postgres/archive/refs/heads/REL_${pg_version}_STABLE.tar.gz"
  curl -Lo "$tgz_file" "$url"
  tar --extract -f "$tgz_file" --directory "/tmp/pg/"
  mv /tmp/pg/postgres-REL_${pg_version}_STABLE "$target_dir"
  # rely on /tmp/ to do the cleanup
  # find "$target_dir" -type f ! -name '*.sql' -exec rm '{}' ';'
}

main "$@"