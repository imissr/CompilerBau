# CiviC Skeleton
This gives a base for your own civicc compiler.

## Getting started
First make sure you have all dependencies installed. The full dependencies of CoCoNut are listed on the main [GitHub](https://github.com/coconut/coconut) repository. On
Ubuntu these can be installed using:
```bash
sudo apt update && sudo apt install build-essential cmake bison flex graphviz
```
For macOS, see [macOS dependencies](#macos-dependencies).

To build the bundled CoCoNut sources and set up a compiler debug build, run
`./compiler/configure.sh` from the repository root, or `./configure.sh` from this
directory. Outputs go into `build/coconut/` and `build/compiler/` at the
repository root. The script builds CoCoNut in release mode and the compiler in
debug mode; running it again sets the compiler build type to `Debug`.

Alternatively, run `make debug` or `make release` from this directory (or
`make -C compiler debug` / `make -C compiler release` from the repository root).
Both targets build CoCoNut first and reuse `build/compiler/` for the compiler;
the last target determines its build type. Use `make help` to list all targets.
`make dist` runs the local packaging script and creates `build/dist/civicc.tgz`.
`make clean` deletes `build/compiler/`, `build/coconut/`, and `build/dist/`.
To disable AddressSanitizer, use `make debug CMAKE_ARGS=-DDISABLE_ASAN=ON`.

Build CoCoNut before configuring the compiler. Run these commands from the
repository root (one directory above `compiler/`) for a release build:

```bash
cmake -S vendor/coconut -B build/coconut -DCMAKE_BUILD_TYPE=Release
cmake --build build/coconut
cmake -S compiler -B build/compiler -DCMAKE_BUILD_TYPE=Release
cmake --build build/compiler
```

The compiler first looks for an installed CoCoNut CMake package. If none is
found, it uses `../vendor/coconut/coconut.cmake` and the generator built in
`../build/coconut/cocogen` (paths relative to `compiler/`). Older CoCoNut checkouts provide this
helper but do not install a `coconutConfig.cmake`, so setting `coconut_DIR` to
such a checkout will not work.

For a source checkout elsewhere, pass `-DCOCONUT_SOURCE_DIR=/path/to/coconut`
when configuring the compiler. For a different CoCoNut build location, also
pass `-DCOCONUT_BUILD_DIR=/path/to/build` or
`-DCOCOGEN_DIR=/path/to/build/cocogen`.

**IMPORTANT**: After cloning, you should change the remote to point to a **private** repo you control.

## Assignment 2

The pipeline is parsing → operator counting → demo traversals → optimisations
→ pretty printing. The AST now has a dedicated `Program` root containing the
statement list and arithmetic-operator statistics.

### 2.1: Strength reduction

The `Optimisations` phase contains subtraction optimisation, strength reduction,
and super strength reduction. Their implementations are in `src/optimisations/`.
The subtraction pass reduces `k - k` and equal integer-literal subtraction to
zero. Strength reduction rewrites `2 * k` and `k * 2` to `k + k`.

### 2.2: Parameterised super strength reduction

Set the maximum integer factor with `--strength-reduction-max N` (short form
`-r N`). The default is 2, preserving ordinary strength reduction. For example,
from the repository root:

```bash
./build/compiler/civicc --strength-reduction-max 4 compiler/test/super_strength_reduction.cvc
```

This additionally rewrites `3 * k` to `(k + k) + k` and `k * 4` to
`((k + k) + k) + k`. The maximum is inclusive. Both operand orders and nested
matches are supported. The limit must be a decimal integer from 2 to `INT_MAX`;
invalid, missing, and overflowing values produce an error.

Both reduction passes require a variable as the other operand. They leave
literal multiplication, boolean operands, arbitrary expressions, and factors
0 and 1 unchanged. Each repeated variable is a separate AST node. The starter
AST has no variable type information; checking that a variable has integer type
will require the type information introduced in later compiler work.

### 2.3: Collecting operator counts

`CountOperators` in `src/analysis/count_operators.c` counts `+`, `-`, `*`, `/`,
and `%`, including nested expressions and all statements. It uses CoCoNut
`travdata`, initialised for each traversal, and stores the five totals and a
`counts_valid` flag on the `Program` root. There are no global counters.
Counting runs before optimisations, so the totals describe the original source
even when a subtraction disappears or multiplication becomes addition.

The demo integer-sum traversal also uses the new root to print its summary
once per compilation.

### 2.4: Printing the root

The print traversal prints the statistics followed by the statement list:

```text
// Arithmetic operators (source): +: 2, -: 3, *: 2, /: 1, %: 1
```

The statistics are omitted if counting has not run yet, for example when
printing at the parser breakpoint:

```bash
./build/compiler/civicc -b SPdoScanParse compiler/test/test1.cvc
./build/compiler/civicc -b CountOperators compiler/test/test1.cvc
```

### Run the Assignment 2 regressions

From the repository root:

```bash
cmake --build build/compiler
ctest --test-dir build/compiler -R '^(strength_reduction|assignment2\.)' --output-on-failure
```

From `compiler/test/`, use `../../build/compiler` as the test directory instead.
The regressions cover both reduction passes, factor limits, invalid arguments,
all five operator counts, nested expressions, zero counts, and printing before
counts are available. The full public CiviC suites (`basic`, `nested_funs`, and
`arrays`) require language features beyond the starter parser and currently fail.

## VS Code support
For syntax highlighting of the CoCoNut DSL files (e.g. the `main.ccn` file), you can install the [nutcracker](https://github.com/CoCoNut-UvA/nutcracker/) extension from the Visual Studio Marketplace [here](https://marketplace.visualstudio.com/items?itemName=CoCoNut-UvA.nutcracker).

We provide a Linux configuration file in [`.vscode/c_cpp_properties.json`](../.vscode/c_cpp_properties.json) at the repository root that helps IntelliSense to find the header files generated by cocogen.
VS code should pick this up automatically; if you open a C file you should see CoCoNut-Linux in the bottom-right corner.


## Disabling the Address Sanitizer
If you just want ASAN to not report memory leaks (although we do recommend that you actually fix the leaks), you can add the following to src/main.c:
```c
/* Do not detect memory leaks when using AddressSanitizer. */
const char *__asan_default_options() {
    return "detect_leaks=0";
}
```

If you want to disable it entirely, edit the debug configuration in the [CMake Configuration File](./CMakeLists.txt).
You may need to remove the build directory and rerun the cmake configuration.

## macOS dependencies
The following build dependencies are required on macOS:
```bash
brew install cmake coreutils binutils bison graphviz gnu-tar
```

You might also need to run the following for cmake to find Bison:
```bash
echo "export PATH=\"$(brew --prefix)/opt/bison/bin:\$PATH\"" >> ~/.zshrc
```

**Note:** Make sure to fully restart your terminal after adding this line to `.zshrc`, and also delete the folder `coconut/build` if it exists. Otherwise cmake will keep using the old unsupported Bison version that macOS ships with.
