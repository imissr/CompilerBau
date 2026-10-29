//! Types that are parsed from a CiviC binary file

use crate::types::{BasicType, Type};

/// A function declaration
#[derive(Clone, Default)]
pub struct Function {
    /// The name of this function
    name: String,

    /// The return type of this function
    return_type: BasicType,

    /// The types of the parameters this function expects
    parameters: Vec<Type>,

    /// Offset of the function definition in the bytcode
    offset: usize,
}

impl Function {
    /// Check if name, return type and parameter types are equal
    pub fn check(&self, other: &Function) -> bool {
        self.name == other.name
            && self.return_type == other.return_type
            && self.parameters == other.parameters
    }

    /// Create a new function declaration
    pub fn new(name: String, return_type: BasicType, parameters: Vec<Type>, offset: usize) -> Self {
        Self {
            name,
            return_type,
            parameters,
            offset,
        }
    }

    /// Get the name of this function
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Get the return type of this function
    pub fn return_type(&self) -> BasicType {
        self.return_type
    }

    /// Get the parameter types this function expects
    pub fn parameters(&self) -> &Vec<Type> {
        &self.parameters
    }

    /// Get the offset of the function definition in the bytecode
    pub fn offset(&self) -> usize {
        self.offset
    }
}

#[derive(PartialEq, Eq, Clone, Default)]
pub struct Variable {
    name: String,
    value_type: Type,
    index: usize,
}

impl Variable {
    pub fn value_type(&self) -> Type {
        self.value_type
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn new(name: String, value_type: Type, index: usize) -> Self {
        Self {
            name,
            value_type,
            index,
        }
    }
}
