//! CiviC Assembler
//!
//! For more details see the appropriate manual version

#[deny(missing_docs)]
use clap::Parser;
use std::{fmt::Display, fs::File, io::Read, path::PathBuf};

mod program;

/// The location of a token
#[derive(Clone, Copy, Default, PartialEq)]
pub struct Location {
    /// The starting line
    pub line: u32,

    /// The start column
    pub column: i32,
}

impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.line, self.column)
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct ProgramOptions {
    /// The assembler source code
    file: PathBuf,

    /// The optional output file
    #[arg(short, long, value_name = "OUTPUT_FILE")]
    output: Option<PathBuf>,

    /// Allow for multi-dimensional arrays
    #[arg(short, long, default_value_t = false)]
    multi_dimensional_arrays: bool,

    /// Allow direct offsets in assembly code
    #[arg(short, long, default_value_t = false)]
    direct_offsets: bool
}

fn main() -> std::io::Result<()> {
    let mut args = ProgramOptions::parse();

    if args.output.is_none() {
        args.output = Some(PathBuf::from("out.cvb")) // standard name for output is out.cvb
    }

    let mut infile = File::open(&args.file)?;
    let mut input = String::new();
    infile.read_to_string(&mut input)?; // read all lines into the string

   // open or create the output file
    let mut outfile = File::options()
        .write(true)
        .truncate(true)
        .create(true)
        .open(args.output.unwrap())?;

    // set up parser
    let mut program = program::Program::new(input, args.multi_dimensional_arrays);

    // do parsing
    program.parse(args.direct_offsets);
    program.resolve_export_labels();
    program.resolve_label_refs();

    // write to outfile
    program.write_byte_code(&mut outfile)?;

    Ok(())
}
