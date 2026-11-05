#!/usr/bin/env sh

set -eu

repo_dir=$(CDPATH= cd "$(dirname "$0")/.." && pwd)

echo "Checking prerequisites"
for tool in cmake flex bison; do
    command -v "$tool" > /dev/null 2>&1 || {
        echo "Could not find $tool in PATH" >&2
        exit 1
    }
done

echo "--------------------------------------------------"
echo "Building bundled CoCoNut in build/coconut"
echo "--------------------------------------------------"
cmake -S "$repo_dir/vendor/coconut" -B "$repo_dir/build/coconut" \
    -DCMAKE_BUILD_TYPE=Release
cmake --build "$repo_dir/build/coconut"

echo "--------------------------------------------------"
echo "Setting up compiler debug build in build/compiler"
echo "--------------------------------------------------"
cmake -S "$repo_dir/compiler" -B "$repo_dir/build/compiler" \
    -DCMAKE_BUILD_TYPE=Debug
cmake --build "$repo_dir/build/compiler"

echo "--------------------------------------------------"
echo "Project configured and built."
