//! Shared library for civic assembler and vm

pub mod operation;

use std::fmt::{Debug, Display};

/// A basic type of the VM
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum BasicType {
    /// Integer (64 bit)
    Int,

    /// Float (64 bit)
    Float,

    /// Boolean
    Bool,

    /// Unit type for function signatures
    #[default]
    Void,
}

impl TryFrom<u8> for BasicType {
    type Error = String;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Void),
            1 => Ok(Self::Int),
            2 => Ok(Self::Float),
            3 => Ok(Self::Bool),
            _ => Err(format!(
                "Can not convert {} to BasicType",
                value
            )),
        }
    }
}

impl Into<u8> for BasicType {
    fn into(self) -> u8 {
        match self {
            BasicType::Void => 0,
            BasicType::Int => 1,
            BasicType::Float => 2,
            BasicType::Bool => 3,
        }
    }
}

impl Display for BasicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BasicType::Void => write!(f, "void"),
            BasicType::Int => write!(f, "int"),
            BasicType::Float => write!(f, "float"),
            BasicType::Bool => write!(f, "bool"),
        }
    }
}

impl Debug for BasicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

/// Type of an array.
///
/// Consists of the unserlying basic type and the number of dimensions.
#[derive(Default, PartialEq, Clone, Copy, Eq)]
pub struct Type {
    /// The [`BasicType`] of the values
    basic_type: BasicType,

    /// Number of dimensions
    dim: u8,
}

impl Type {
    /// Get the underlying basic type
    pub fn basic_type(&self) -> BasicType {
        self.basic_type
    }

    /// Get the number of dimensions
    pub fn dimensions(&self) -> u8 {
        self.dim
    }
    
    /// Create new array type from given [`BasicType`] and number of dimensions
    pub fn new(basic_type: BasicType, dim: u8) -> Self {
        Self { basic_type, dim }
    }
}
