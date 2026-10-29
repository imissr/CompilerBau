# CiviC Execution Environment

This repo contains the sources and documentation of the CiviC assembler and virtual machine.

## Build instructions

To build the tools you have to perform the following steps

1. Install Rust as recommended on the [Rust Homepage](https://rust-lang.org/)
2. Run `cargo build --release`

To build the documentation you need LaTeX installed on your system. Then you can navigate into the `doc` directory and run `latexmk -pdf manual.tex`.