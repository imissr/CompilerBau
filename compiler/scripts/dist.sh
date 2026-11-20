#!/usr/bin/env bash
set -euo pipefail

repo_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../.." && pwd)"
archive="$repo_dir/build/dist/civicc.tgz"

mkdir -p -- "$(dirname -- "$archive")"
tar -czf "$archive" -C "$repo_dir" -- compiler vendor README.md

printf 'Created %s\n' "$archive"
