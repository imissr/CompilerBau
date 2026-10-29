//! Types used for lexing, parsing and handling of pseudo instructions

use std::fmt::Display;
use libcivexec::{BasicType, Type};

use crate::{
    Location,
    program::tokens::{Token, TokenValue},
};

/// Jump mark reference
#[derive(Clone, PartialEq)]
pub enum Offset {
    /// Label reference
    Label(String),

    /// Offset
    Offset(i16),
}

impl Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Offset::Label(label) => write!(f, "{label}"),
            Offset::Offset(offset) => write!(f, "{offset}"),
        }
    }
}

/// A type carying a value
pub enum ValueType {
    /// Integer (64 bit)
    Int(i64),

    /// Floating point number (64 bit)
    Float(f64),

    /// Boolean
    Bool(bool),
}

/// A reference to a jump mark
#[derive(Clone, Default, PartialEq)]
pub struct LabelRef {
    /// Location of reference, for error reporting
    token_location: [Location; 2],

    /// Position of referenced label in binary code (might not exist)
    /// 
    /// This is either an label or an direct offset
    location: Option<Offset>,
}

impl LabelRef {
    /// Create new label reference from given location and name.
    ///
    /// - `location`: The location of the LabelRef token
    /// - `name`: The name of the jump mark, to reference
    pub fn new(token_location: [Location; 2], name: Offset) -> Self {
        Self {
            token_location,
            location: Some(name),
        }
    }

    /// Get the name of the jump mark to reference
    pub fn location(&self) -> &Offset {
        self.location.as_ref().unwrap()
    }

    /// Get location of the token in source code
    pub fn token_location(&self) -> &[Location; 2] {
        &self.token_location
    }
}

impl From<Token> for LabelRef {
    fn from(value: Token) -> Self {
        match value.value() {
            TokenValue::Int(i) => {
                Self {
                    token_location: value.position().clone(),
                    location: Some(Offset::Offset(*i as i16))
                }
            },
            TokenValue::String(str) => {
                Self {
                    token_location: value.position().clone(),
                    location: Some(Offset::Label(str.to_string())),
                }
            },
            _ => {
                panic!("Can not create a label reference out of a non-string token")
            }
        }
    }
}

impl Display for LabelRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string;
        if let Some(x) = &self.location {
            string = match x {
                Offset::Label(lbl) => format!("{lbl}"),
                Offset::Offset(offset) => format!("{offset}"),
            };
        } else {
            string = String::new();
        }
        write!(f, "{string} ({}, {})", self.token_location[0], self.token_location[1])
    }
}

/// Types of import or export variables
pub enum VarType {
    /// Import variable (type)
    Import(Type),
    /// Export variable (index in global table)
    Export(usize),
}

/// A variable.
///
/// Consists of name and type
pub struct Var {
    /// Name for export or import
    name: String,
    /// Import or export (with parameters)
    var_type: VarType,
}

impl Var {
    /// Create new variable from given name and type
    pub fn new(name: String, var_type: VarType) -> Self {
        Self { name, var_type }
    }

    /// Get the name of the variable
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the type of the variable
    pub fn var_type(&self) -> &VarType {
        &self.var_type
    }
}

/// Import and export functions
#[derive(Default)]
pub struct Function {
    /// Import or export name
    name: String,

    /// Parameter count and type
    parameters: Vec<Type>,

    /// Label (for exports)
    label: LabelRef,

    /// Return type
    return_type: BasicType,

    /// Resolve label (for exports)
    offset: u32,
}

impl Function {
    /// Get the name of the function
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Set the name of the function
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Add a parameter to the function.
    ///
    /// A parameter is defined by its [type](BasicType) and number of dimensions
    pub fn add_param(&mut self, base_type: BasicType, dims: u8) {
        let arr = Type::new(base_type, dims);
        self.parameters.push(arr);
    }

    /// Set the jump mark, where the functions code is defined
    pub fn set_label_ref(&mut self, label: LabelRef) {
        self.label = label;
    }

    /// Get all parameters
    pub fn params(&self) -> &Vec<Type> {
        &self.parameters
    }

    /// Get a mutable reference to all paramters
    pub fn params_mut(&mut self) -> &mut Vec<Type> {
        &mut self.parameters
    }

    /// Get the return type of the function
    pub fn return_type(&self) -> BasicType {
        self.return_type
    }

    /// Set the return type of the function
    pub fn set_return_type(&mut self, return_type: BasicType) {
        self.return_type = return_type;
    }

    /// Get the jump mark reference of the function
    pub fn label(&self) -> &LabelRef {
        &self.label
    }

    /// Get offset in the function table
    pub fn offset(&self) -> u32 {
        self.offset
    }

    /// Set the offset in the function table
    pub fn set_offset(&mut self, offset: u32) {
        self.offset = offset;
    }
}
