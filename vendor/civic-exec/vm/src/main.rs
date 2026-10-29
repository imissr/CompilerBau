//! # The CiviC Virtual Machine.
//!
//! This is a stack-based virtual machine for executing CiviC programs

use std::io::ErrorKind;
#[deny(missing_docs)]
use std::{fs::File, io::BufReader};

use clap::Parser;
use libcivexec::BasicType;

use crate::{
    binary::Binary,
    state::VMState,
    types::{Error, StackValue},
};

mod binary;
mod builtins;
mod stack;
mod state;
mod types;

#[derive(Parser)]
#[command(version, about, long_about)]
/// The possible command line args
struct Args {
    #[arg(short, long)]
    /// Print instructions during execution
    verbose: bool,

    #[arg(short, long)]
    /// Display code size in bytes
    size: bool,

    #[arg(short, long)]
    /// Display number of executed instructions
    instrs: bool,

    #[arg(required = true)]
    /// The file(s) to execute
    files: Vec<String>,

    #[arg(short, long, default_value_t = false)]
    /// Allow for multi-dimensional arrays
    multi_dimensional_arrays: bool,
}

fn main() -> std::io::Result<()> {
    let cli = Args::parse(); // parse arguments

    let mut binaries = Vec::<Binary>::new();

    for filename in &cli.files {
        // create internal representation for each file given
        let fileref = File::open(filename)?;

        let mut filereader = BufReader::new(fileref);
        match Binary::try_new(&mut filereader, cli.multi_dimensional_arrays) {
            Ok(binary) => {
                if cli.size {
                    eprintln!("Size of code in {}: {} bytes", filename, binary.code_size())
                }
                binaries.push(binary);
            }
            Err(err) => {
                return handle_error(err, None, &cli.files);
            }
        }
    }

    // create new vm state for the parsed binaries
    //
    // this includes an invocation of the `__init` function (if any)
    let res = match VMState::try_new(binaries, cli.verbose, cli.multi_dimensional_arrays) {
        Ok(mut vm) => match run(&cli, &mut vm) {
            Ok(_) => Ok(()),
            Err(err) => handle_error(err, Some(&vm), &cli.files),
        },
        Err(err) => handle_error(err, None, &cli.files),
    };

    res
}

/// Start the bytecode execution with given command line arguments and state
fn run(cli: &Args, vm: &mut VMState) -> Result<(), Error> {
    let args: [StackValue; 0] = []; // Construct aruments of main function (empty)

    vm.call("main", &args, BasicType::Void)?; // prepare running of main function
    vm.run()?; // run the execution

    if cli.instrs {
        eprintln!("Instructions executed: {}", vm.instr_executed()); // print number of executed instructions
    }

    Ok(())
}

/// Display an execution error
fn handle_error(error: Error, vm: Option<&VMState>, files: &Vec<String>) -> std::io::Result<()> {
    //eprintln!("{}", error);
    if let Some(state) = vm {
        let frames = match state.get_backtrace() {
            Ok(f) => f,
            Err(err) => {
                return Err(std::io::Error::new(
                    ErrorKind::Unsupported,
                    format!("{}", err),
                ));
            }
        };
        for frame in frames {
            let pc = frame.program_counter();
            eprintln!(
                "Error occured at {}, offset {}",
                files[pc.module], pc.offset
            );
        }
    }
    Err(std::io::Error::new(
        ErrorKind::Unsupported,
        format!("{}", error),
    ))
}
