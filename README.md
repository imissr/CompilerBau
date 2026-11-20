# CompilerBau

CiviC compiler coursework using the CoCoNut framework.

## Repository layout

| Directory | Contents |
| --- | --- |
| [compiler/](compiler/) | Compiler sources, CMake configuration, demo inputs, and scripts |
| [vendor/coconut/](vendor/coconut/) | CoCoNut generator and runtime sources |
| [vendor/civic-exec/](vendor/civic-exec/) | CiviC assembler and virtual machine sources |
| [vendor/civic-toolchain-linux-static/](vendor/civic-toolchain-linux-static/) | Prebuilt Linux reference compiler, assembler, and virtual machine |
| [tests/public/](tests/public/) | Supplied compiler test suite |
| [docs/development/coconut.md](docs/development/coconut.md) | CoCoNut development guide |
| [docs/reference/](docs/reference/) | CiviC language and virtual machine manuals |
| [course/](course/) | Assignments, lectures, and compiler milestones |
| `build/` | Generated build files, excluded from Git |

## Build the compiler

Install CMake (>= 3.13), a C compiler (GCC or Clang), Flex, and Bison (>= 3.0.4).
Graphviz is needed for AST images. Open a terminal in the repository root,
`CompilerBau`, and run all commands below from there.

### First build

For an automatic setup using the bundled CoCoNut sources and a compiler debug
build, run:

```bash
./compiler/configure.sh
```

The script builds CoCoNut in `build/coconut/` and the compiler in
`build/compiler/`. It also works when called from another directory. Running it
again sets the compiler build type to `Debug`.

For a release build, run these commands instead:

```bash
cmake -S vendor/coconut -B build/coconut -DCMAKE_BUILD_TYPE=Release
cmake --build build/coconut

cmake -S compiler -B build/compiler -DCMAKE_BUILD_TYPE=Release
cmake --build build/compiler
```

CoCoNut must be built first. The compiler automatically uses the bundled
CoCoNut sources and the generator in `build/coconut/cocogen` when no installed
CoCoNut package is found.

### Make shortcuts

The compiler Makefile provides these commands from the repository root:

```bash
make -C compiler help
make -C compiler debug
make -C compiler release
make -C compiler dist
make -C compiler clean
```

Both build targets build CoCoNut first and then configure and build the
compiler. Debug and release reuse `build/compiler/`; the last target determines
the build type. `dist` runs `compiler/scripts/dist.sh` and writes
`build/dist/civicc.tgz`, containing `compiler/`, `vendor/`, and the root
`README.md`. You can also run `./compiler/scripts/dist.sh` directly from any
directory. `clean` deletes `build/compiler/`, `build/coconut/`, and
`build/dist/`.

### Rebuild after code changes

After changing your compiler code, you only need:

```bash
cmake --build build/compiler
```

Keep traversal implementations in `compiler/src/`; generated stubs under
`build/compiler/ccngen/user/` are overwritten during generation.

### Run the compiler

Run the supplied demo inputs:

```bash
./build/compiler/civicc compiler/test/test1.cvc
./build/compiler/civicc compiler/test/test2.cvc
```

For Assignment 2, use `--strength-reduction-max N` (or `-r N`) to expand
variable multiplications with integer factors up to `N` (default: 2).
The compiler also prints counts of the five arithmetic operators in the source.
See the [Assignment 2 implementation and tests](compiler/README.md#assignment-2).

Run its regression tests with:

```bash
ctest --test-dir build/compiler -R '^(strength_reduction|assignment2\.)' --output-on-failure
```

### Einstiegspunkt für die CiviC-VM

Beim Ausführen erwartet die CiviC-VM genau eine exportierte Funktion `main`
ohne Parameter und mit dem Rückgabetyp `void`:

```c
export void main() {
    return;
}
```

`int main()` wird von der VM nicht als Einstiegspunkt akzeptiert. Für Programme,
die mit der CiviC-VM ausgeführt werden sollen, muss daher `export void main()`
verwendet werden (siehe [CiviC-VM-Handbuch](docs/reference/civic-vm.pdf)).

### Generate an AST diagram

Run:

```bash
cmake --build build/compiler --target dot
```

The resulting image is `build/compiler/ast.png`.

## Documentation and tests

- [CoCoNut development guide](docs/development/coconut.md)
- [CiviC language reference](docs/reference/civic-language.pdf)
- [CiviC VM reference](docs/reference/civic-vm.pdf)
- [Compiler milestones](course/milestones/milestones.pdf)
- [Public test suite instructions](tests/README.md)

Open the repository root in VS Code to use its shared C/C++ configuration.

The supplied prebuilt Linux tools are in
[`vendor/civic-toolchain-linux-static/`](vendor/civic-toolchain-linux-static/).
Its `civicc` is the reference compiler; your own compiler is built as
`build/compiler/civicc`. Keep the prebuilt toolchain together so its run script
can find the binaries and the reference compiler can find `civic.h`.

Alternatively, build the assembler and virtual machine from their Rust sources
into `build/` with:

```bash
cargo build --release --manifest-path vendor/civic-exec/Cargo.toml --target-dir build/civic-exec
```
