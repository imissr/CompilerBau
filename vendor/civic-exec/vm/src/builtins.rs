//! Definition of the standard libraray functions, that are built-in

use std::{
    io::{Write, stdin, stdout},
    sync::LazyLock,
};

use libcivexec::{BasicType, Type};

use crate::{
    state::VMState,
    types::{BultinFunction, Error, binary::Function},
};

/// Number of built-in functions
static NUM_BUILTINS: usize = 6;

/// The array of function definitions for the built-in functions
static BUILTINS: LazyLock<[Builtin; NUM_BUILTINS]> = LazyLock::new(|| {
    [
        Builtin {
            callback: print_int,
            name: "printInt",
            ret_type: BasicType::Void,
            num_parameters: 1,
            parameters: vec![Type::new(BasicType::Int, 0)],
        },
        Builtin {
            callback: print_float,
            name: "printFloat",
            ret_type: BasicType::Void,
            num_parameters: 1,
            parameters: vec![Type::new(BasicType::Float, 0)],
        },
        Builtin {
            callback: scan_int,
            name: "scanInt",
            ret_type: BasicType::Int,
            num_parameters: 0,
            parameters: vec![],
        },
        Builtin {
            callback: scan_float,
            name: "scanFloat",
            ret_type: BasicType::Float,
            num_parameters: 0,
            parameters: vec![],
        },
        Builtin {
            callback: print_newlines,
            name: "printNewlines",
            ret_type: BasicType::Void,
            num_parameters: 1,
            parameters: vec![Type::new(BasicType::Int, 0)],
        },
        Builtin {
            callback: print_spaces,
            name: "printSpaces",
            ret_type: BasicType::Void,
            num_parameters: 1,
            parameters: vec![Type::new(BasicType::Int, 0)],
        },
    ]
});

/// Print an integer on [`stdout`]
fn print_int(state: &mut VMState) -> Result<(), Error> {
    print!("{}", state.popi()?);
    state.do_return()?;
    Ok(())
}

/// Print a floating point number on [`stdout`]
fn print_float(state: &mut VMState) -> Result<(), Error> {
    print!("{0:.6}", state.popf()?);
    state.do_return()?;
    Ok(())
}

/// Read an integer from [`stdin`]
fn scan_int(state: &mut VMState) -> Result<(), Error> {
    let mut input = String::new();
    print!("Please enter an integer value: ");
    stdout()
        .flush()
        .map_err(|err| Error::Other(format!("{}", err)))?;
    stdin()
        .read_line(&mut input)
        .expect("Error on reading user input");
    _ = input.pop(); // last char is '\n'!
    let result: i64 = input.parse().unwrap();
    state.do_return()?;
    state.pushi(result);
    Ok(())
}

/// Read a floating point number from [`stdin`]
fn scan_float(state: &mut VMState) -> Result<(), Error> {
    let mut input = String::new();
    print!("Please enter an integer value: ");
    stdout()
        .flush()
        .map_err(|err| Error::Other(format!("{}", err)))?;
    stdin()
        .read_line(&mut input)
        .expect("Error on reading user input");
    _ = input.pop(); // last char is '\n'!
    let result: f64 = input.parse().unwrap();
    state.do_return()?;
    state.pushf(result);
    Ok(())
}

/// Print newline characters on [`stdout`].
///
/// The amount of the characters to print is determined by the top-of-stack value
fn print_newlines(state: &mut VMState) -> Result<(), Error> {
    let num = state.popi()?;
    for _i in 0..num {
        println!();
    }
    state.do_return()?;
    Ok(())
}

/// Print space characters on [`stdout`].
///
/// The amount of the characters to print is determined by the top-of-stack value
fn print_spaces(state: &mut VMState) -> Result<(), Error> {
    let num = state.popi()?;
    for _i in 0..num {
        print!(" ");
    }
    state.do_return()?;
    Ok(())
}

/// Function definition of a built-in function
pub struct Builtin {
    /// Which function to call
    pub callback: BultinFunction,

    /// The name of this function in the CiviC standard library
    pub name: &'static str,

    /// The return type of this function
    pub ret_type: BasicType,

    /// The number of parameters this function expects
    pub num_parameters: usize,

    /// The types of this parameters
    pub parameters: Vec<Type>,
}

/// Find a built-in function by a [`Function`] definition
///
/// A match is found, if name, return type and parameters (number and types)
/// are equal.
pub fn find_builtin(desc: &Function) -> Option<&Builtin> {
    for builtin in BUILTINS.iter() {
        if desc.name() == builtin.name
            && desc.return_type() == builtin.ret_type
            && *desc.parameters() == builtin.parameters
        {
            return Some(builtin);
        }
    }
    None
}
