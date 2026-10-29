# CompilerBau

CiviC compiler coursework using the CoCoNut framework.

## Repository layout

| Directory | Contents |
| --- | --- |
| [compiler/](compiler/) | Compiler sources, CMake configuration, demo inputs, and scripts |
| [vendor/coconut/](vendor/coconut/) | CoCoNut generator and runtime sources |
| [vendor/civic-exec/](vendor/civic-exec/) | CiviC assembler and virtual machine sources |
| [tests/public/](tests/public/) | Supplied compiler test suite |
| [docs/development/coconut.md](docs/development/coconut.md) | CoCoNut development guide |
| [docs/reference/](docs/reference/) | CiviC language and virtual machine manuals |
| [course/](course/) | Assignments, lectures, and compiler milestones |
| `build/` | Generated build files, excluded from Git |

## Build the compiler

Install CMake (>= 3.13), a C compiler (GCC or Clang), Flex, and Bison (>= 3.0.4).
Graphviz is needed for AST images. Run these commands from the repository root:

```bash
cmake -S vendor/coconut -B build/coconut -DCMAKE_BUILD_TYPE=Release
cmake --build build/coconut
cmake -S compiler -B build/compiler -DCMAKE_BUILD_TYPE=Release
cmake --build build/compiler
```

CoCoNut must be built first. The compiler automatically uses the bundled
CoCoNut sources and the generator in `build/coconut/cocogen` when no installed
CoCoNut package is found.

After changing compiler sources, rebuild with `cmake --build build/compiler`.
Keep traversal implementations in `compiler/src/`; generated stubs under
`build/compiler/ccngen/user/` are overwritten during generation.

Run the supplied demo inputs:

```bash
./build/compiler/civicc compiler/test/test1.cvc
./build/compiler/civicc compiler/test/test2.cvc
```

Generate an AST diagram with:

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

The optional assembler and virtual machine use Rust. Build them into `build/`
with:

```bash
cargo build --release --manifest-path vendor/civic-exec/Cargo.toml --target-dir build/civic-exec
```
