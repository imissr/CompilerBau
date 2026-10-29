#!/usr/bin/env bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
"$DIR/civicc" -o _tmp.s "$*" &&
"$DIR/civicas" -o _tmp.out _tmp.s &&
"$DIR/civicvm" _tmp.out
rm -f _tmp.s _tmp.out
