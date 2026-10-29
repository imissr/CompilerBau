# CoCoNut Developer Documentation

You can find the CoCoNut developer documentation here:

[CoCoNut Developer Documentation](https://coconut-uva.github.io/coconut/index.html)

This repository keeps compiler sources in `compiler/` and CoCoNut sources in
`vendor/coconut/`. Run build commands from the repository root. In the examples
below, `<build_directory>` means `build/compiler`, and the compiler DSL file is
`compiler/src/main.ccn`. See the [root README](../../README.md) for an overview.

### What is the CoCoNut framework doing for me?

From the CoCoNut documentation:

> CoCoNut is a framework to ease the construction of a compiler by providing a DSL to define the common compiler structure. From this specification, CoCoNut generates the boilerplate to work with the defined structure. The structure is defined using several compiler primitives: enums, nodes, nodesets, passes, traversals and phases.

### How do I add a traversal to my phases?

1. Define the traversal in the `.ccn` file.
2. Recompile your project (if not integrated in your IDE, perform the following steps):
   1. Run `cmake --build <build_directory>`.
   2. This will generate a linker error.
3. This will generate a file `trav_<traversal_name>.c` under `<build_directory>/ccngen/user`.
4. Copy that file to the place you want to have it.
5. Update your CMake configuration: Add the relative path to the copied file to the `add_executable` command in your `CMakeLists.txt` file.
6. Rebuild your project as in step 2. Now the linker error should no longer be present.

#### Why is this so complicated?

1. We do not know the name of the new traversal a priori.
2. The traversal skeletons under `<build_directory>/ccngen/user` are recreated every time you recompile your project.

### How do I control the traversal of a node?

CoCoNut generates functions that let you control how a node is traversed:

- `node_st *TRAVdo(node_st *node)` Performs the current traversal for the specified node.
- `node_st *TRAVopt(node_st *node)` Performs the current traversal for the specified node, but only if it is not NULL.
- `node_st *TRAVchildren(node_st *node)` Performs the current traversal for all children of the specified node and assigns the result of the traversals to each child respectively.
- `node_st *TRAV<name>(node_st *node)` Performs the current traversal for the child `<name>` of the specified node and assigns the result of this traversal to this child.

### How do I access my traversal data?

Use the macro `DATA_<UID>_GET()` where `<UID>` is the UID of your traversal. Please note that this will give you a pointer.

### How can I generate an image of my AST structure definition?

From the repository root, run:

```bash
cmake --build <build_directory> --target dot
```

### I want to work on a Windows system...

Well... We have two answers for you.

#### Short Answer

Have fun 😀

#### Long Answer

There are 2 (or more precisely, 3) show stoppers that you have to deal with:

- Command line argument parsing
- Lexing and Parsing of CiviC Code

Parsing of command line arguments is done via the `getopt_long` function of the *GNU libc*. Lexing and Parsing of CiviC code depends on *flex* and *GNU bison* in our toolchain. All three are not natively available on Windows. If you find a *good* alternative for them, feel free to try it. Nevertheless, we highly recommend using Linux (or Mac) systems. You can easily install them on your Windows machine via *WSL* or *MSYS/MinGW*. Please note that we cannot give you any support if you are not working with our toolchain.

### Building the CiviC Compiler Project

For building the CiviC compiler, you need the following tools:

- `cmake` (>= 3.13)
- `gcc` or `clang` (other C compilers may not be supported)
- `flex`
- `bison` (>= 3.0.4) — This is especially important for macOS users, who have a preinstalled old version of `bison`.
- `graphviz`/`dot` for visualizing the AST

When you have installed these programs on your system, you can use one of the following methods to build your project:

- Quick Start
- IDE with CMake integration

#### Quick Start

For a quick start with the compiler project, perform the following steps:

The compiler supports an installed CoCoNut CMake package, but this repository
includes an older source version that provides `coconut.cmake` instead of an
installable package. Build that bundled version first; no system installation
is needed.

Open a terminal in the repository root and run:

```bash
cmake -S vendor/coconut -B build/coconut -DCMAKE_BUILD_TYPE=Release
cmake --build build/coconut
cmake -S compiler -B build/compiler -DCMAKE_BUILD_TYPE=Release
cmake --build build/compiler
```

This puts generated files under `build/`. For subsequent compiler builds, run
`cmake --build build/compiler`.

#### Using an IDE with CMake integration

1. Clone this repository: `git clone <URL> [<path>]`.
2. Build CoCoNut using the first two Quick Start commands above.
3. Open the repository root in your IDE. Configure its CMake integration to use
   `compiler/` as the source directory and `build/compiler/` as the build
   directory.

If you are using Visual Studio Code (or derivatives), the repository provides
[`.vscode/c_cpp_properties.json`](../../.vscode/c_cpp_properties.json):

1. Its include paths already point to `compiler/src`, the bundled CoCoNut
   headers, and generated headers under `build/compiler/`. If you choose a
   different build directory, update the generated-header paths accordingly.
2. Set `compilerPath` to the C compiler you actually use (e.g. `/usr/bin/gcc`).

Zuletzt geändert: Montag, 26. Oktober 2026, 09:12
