# Public compiler tests

`public/` contains the supplied test cases and their expected outputs. The
original distribution instructions are preserved in
[public/UPSTREAM_README](public/UPSTREAM_README).

Build the compiler using the [root README](../README.md), then run a test group
from the repository root:

```bash
RUN_FUNCTIONAL=0 ctest --test-dir build/compiler -R '^basic$' --output-on-failure
```

The other registered groups are `nested_funs` and `arrays`. These tests cover
features to implement during the course; the initial skeleton does not pass
the full public suite. `RUN_FUNCTIONAL=0` skips assembling and executing output.

Once assembly generation is implemented, build the assembler and VM as
described in the root README, then enable functional tests:

```bash
CIVAS="$PWD/build/civic-exec/release/civicas" \
CIVVM="$PWD/build/civic-exec/release/civicvm" \
RUN_FUNCTIONAL=1 \
ctest --test-dir build/compiler -R '^basic$' --output-on-failure
```

CTest uses `compiler/test/run.bash`, which accepts the compiler path and
returns a failing status when test cases fail. `public/run.bash` is retained
as supplied with the test distribution. Small demonstration inputs remain
in `compiler/test/`.
