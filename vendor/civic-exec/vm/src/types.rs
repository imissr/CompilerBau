//! Types for executing CiviC programs

use libcivexec::{BasicType, Type};

use crate::state::VMState;
use std::{
    cell::RefCell, fmt::{Debug, Display}, rc::Rc,
};

pub mod binary;
pub mod opcodes;
pub mod state;

pub type Pointer = usize;
pub type ModuleId = usize;
pub type BultinFunction = fn(&mut VMState) -> Result<(), Error>;

/// The program counter points to the current instruction the execute
#[derive(Copy, Clone, Debug, Default)]
pub struct ProgramCounter {
    /// The current module
    pub module: ModuleId,

    /// Offset in the bytecode
    pub offset: usize,
}

impl Display for ProgramCounter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "offset {} in module {}", self.offset, self.module)
    }
}

impl PartialEq<StackValue> for Type {
    fn eq(&self, other: &StackValue) -> bool {
        match other {
            StackValue::Int(_) => self.basic_type() == BasicType::Int && self.dimensions() == 0,
            StackValue::Float(_) => self.basic_type() == BasicType::Float && self.dimensions() == 0,
            StackValue::Bool(_) => self.basic_type() == BasicType::Bool && self.dimensions() == 0,
            StackValue::Array(x) => {
                let arr = x.borrow();
                let arr_type = arr.get_basic_type();
                let arr_dims = arr.get_dimensions();
                arr_type == self.basic_type() && arr_dims as u8 == self.dimensions()
            }
            _ => false,
        }
    }
}

impl PartialEq<StackValue> for &Type {
    fn eq(&self, other: &StackValue) -> bool {
        match other {
            StackValue::Int(_) => self.basic_type() == BasicType::Int && self.dimensions() == 0,
            StackValue::Float(_) => self.basic_type() == BasicType::Float && self.dimensions() == 0,
            StackValue::Bool(_) => self.basic_type() == BasicType::Bool && self.dimensions() == 0,
            _ => false,
        }
    }
}

/// Represenation of a typed stack value
#[derive(Clone)]
pub enum StackValue {
    /// An empty value
    None,

    /// 64 bit signed integer
    Int(i64),

    /// Floating point number of 64 bit precision
    Float(f64),

    /// Boolean value
    Bool(bool),

    // Array reference
    Array(Rc<RefCell<Array>>),

    /// Stack pointer, for state saving on frame change
    StackPointer(Pointer),

    /// Program counter, for state saving on frame change
    ProgramCounter(ProgramCounter),
}

impl StackValue {
    /// Create a _new_ array reference.
    ///
    /// This functions expects the internal representation of an array and
    /// moves it to the heap. Then a reference to this array is created and
    /// returned.
    pub fn array(array: Array) -> Self {
        Self::Array(Rc::new(RefCell::new(array)))
    }

    /// Get the stack representation of an integer value.
    pub fn int(int: i64) -> Self {
        Self::Int(int)
    }

    /// Get the stack representation of a floating point value.
    pub fn float(float: f64) -> Self {
        Self::Float(float)
    }

    /// Get the stack representation of a boolean value.
    pub fn bool(boolean: bool) -> Self {
        Self::Bool(boolean)
    }

    /// Get the number of dims for this value
    pub fn get_dims(&self) -> usize {
        match self {
            StackValue::Array(array) => array.borrow().bounds.len(),
            _ => 0,
        }
    }

    /// Compare the type of this stack value with the type of another.
    ///
    /// This function neither check if the inner values nor the dimensions are equal
    pub fn compare_type_with_other(&self, other: &Self) -> bool {
        match (self, other) {
            (StackValue::None, StackValue::None) => true,
            (StackValue::Int(_), StackValue::Int(_)) => true,
            (StackValue::Float(_), StackValue::Float(_)) => true,
            (StackValue::Bool(_), StackValue::Bool(_)) => true,
            (StackValue::Array(_), StackValue::Array(_)) => true,
            (StackValue::StackPointer(_), StackValue::StackPointer(_)) => true,
            (StackValue::ProgramCounter(_), StackValue::ProgramCounter(_)) => true,
            (_, _) => false,
        }
    }

    /// Compare the basic type of this value ith another basic type
    pub fn compare_with_basic_type(&self, other: &BasicType) -> bool {
        match (self, other) {
            (StackValue::Int(_), BasicType::Int) => true,
            (StackValue::Float(_), BasicType::Float) => true,
            (StackValue::Bool(_), BasicType::Bool) => true,
            (_, _) => false,
        }
    }
}

impl PartialEq<Type> for StackValue {
    fn eq(&self, other: &Type) -> bool {
        match self {
            StackValue::Int(_) => other.basic_type() == BasicType::Int && other.dimensions() == 0,
            StackValue::Float(_) => other.basic_type() == BasicType::Float && other.dimensions() == 0,
            StackValue::Bool(_) => other.basic_type() == BasicType::Bool && other.dimensions() == 0,
            _ => false,
        }
    }
}

impl PartialEq<Type> for &StackValue {
    fn eq(&self, other: &Type) -> bool {
        match self {
            StackValue::Int(_) => other.basic_type() == BasicType::Int && other.dimensions() == 0,
            StackValue::Float(_) => other.basic_type() == BasicType::Float && other.dimensions() == 0,
            StackValue::Bool(_) => other.basic_type() == BasicType::Bool && other.dimensions() == 0,
            _ => false,
        }
    }
}

impl Display for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::None => write!(f, ""),
            StackValue::Int(int) => write!(f, "{}: int", int),
            StackValue::Float(float) => write!(f, "{}: float", float),
            StackValue::Bool(bool) => write!(f, "{}: bool", bool),
            StackValue::Array(array) => write!(f, "{}", array.borrow()),
            StackValue::StackPointer(sp) => write!(f, "{}", sp),
            StackValue::ProgramCounter(pc) => write!(f, "{}", pc),
        }
    }
}

impl Debug for StackValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StackValue::None => write!(f, ""),
            StackValue::Int(int) => write!(f, "{}: int", int),
            StackValue::Float(float) => write!(f, "{}: float", float),
            StackValue::Bool(bool) => write!(f, "{}: bool", bool),
            StackValue::Array(array) => write!(f, "{}", array.borrow()),
            StackValue::StackPointer(sp) => write!(f, "{}", sp),
            StackValue::ProgramCounter(pc) => write!(f, "{}", pc),
        }
    }
}

// Default implementation required for constant loading on binary initialisiation
impl Default for StackValue {
    fn default() -> Self {
        StackValue::None
    }
}

/// The representation of an CiviC array.
#[derive(Clone, Debug)]
pub struct Array {
    /// The type of the elements
    basic_type: BasicType,

    /// The length of each dimension
    bounds: Vec<u32>,

    /// The actual values as 1d-array
    values: Vec<StackValue>,
}

impl Array {
    /// Create a new array of given element type and bounds.
    ///
    /// The array is created and filles with dummy values.
    pub fn new(value_type: BasicType, bounds: Vec<u32>) -> Self {
        let mut size: usize = 1;
        for x in &bounds {
            size *= *x as usize;
        }
        let values = match value_type {
            BasicType::Void => vec![StackValue::None; size],
            BasicType::Int => vec![StackValue::Int(i64::MIN); size],
            BasicType::Float => vec![StackValue::Float(f64::MIN); size],
            BasicType::Bool => vec![StackValue::Bool(false); size],
        };

        Self {
            bounds,
            values,
            basic_type: value_type,
        }
    }

    /// Get the values of the array
    pub fn values(&self) -> &Vec<StackValue> {
        &self.values
    }

    /// Get a mutalbe reference to the values of the array
    pub fn values_mut(&mut self) -> &mut Vec<StackValue> {
        &mut self.values
    }

    /// Get the length of each dimension
    pub fn get_bounds(&self) -> &Vec<u32> {
        &self.bounds
    }

    /// Get the type of the array elements
    pub fn get_basic_type(&self) -> BasicType {
        self.basic_type
    }

    /// Get the number of dimensions of this array
    pub fn get_dimensions(&self) -> usize {
        self.bounds.len()
    }

    /// Get the a string representation of a given range
    fn get_string_from_range(&self, lower: usize, upper: usize) -> String {
        let mut string = String::from("[ ");
        for i in lower..upper {
            string.push_str(&format!("{}, ", self.values[i]));
        }
        /*self.values
        .iter()
        .for_each(|x| string.push_str(&format!("{}, ", x)));*/
        string = string[0..string.len() - 2].to_string(); // strip the trailing ", "
        string.push_str(" ]");
        string
    }

    /// Get the string representation of a multi-dimensional array
    fn get_string_multi_dim(&self) -> String {
        let mut str = String::from("[ ");
        let mut lower: usize = 0;
        let mut upper: usize;
        for bound in &self.bounds {
            upper = lower + *bound as usize;
            let new_str = self.get_string_from_range(lower, upper);
            str.push_str(&new_str);
            lower = upper;
            str.push_str(", ");
        }
        str = str[0..str.len() - 2].to_string();
        str.push_str(" ]");

        str
    }
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self.bounds.len() {
            1 => self.get_string_from_range(0, self.values.len()),
            _ => self.get_string_multi_dim(),
        };
        write!(f, "{}", str)
    }
}

/// The errors that can occur during execution
pub enum Error {
    FunctionNotFound(String),
    Other(String),

    /// The input file is not a valid civic executable
    FileNotValid,

    /// A value has a type that was not expected
    InvalidType(String),

    /// The frame pointer is invalid
    InvalidFP(String),

    /// Try to access a list with an out-of-bounds index
    IndexOutOfBounds(String),

    /// Try to dereference a nullpointer
    EmptyDeref,

    /// (Dimension got, Dimension max)
    DimensionOutOfBounds(usize, usize),

    /// Got a negative index (Description)
    IndexNotValid(String),

    /// Got an u8 that as opcode that does not match to an opcode
    UnknownOpCode(u8),

    /// Got an operand that is empty
    InvalidOperand,

    /// Got an mutli-dimensional array where but the currect execution mode does not allow for it
    MultiDimensionArrayNotAllowed
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::FunctionNotFound(s) => write!(f, "[Function not Found] {}", s),
            Error::Other(s) => write!(f, "[ERROR] {}", s),
            Error::FileNotValid => write!(f, "Reading a file that is not a civic executable"),
            Error::InvalidType(s) => write!(f, "[Type mismatch] {}", s),
            Error::InvalidFP(s) => write!(f, "[Invalid Framepointer] {}", s),
            Error::IndexOutOfBounds(s) => write!(f, "[Index out of bounds] {}", s),
            Error::EmptyDeref => write!(f, "[Array Deref] Trying to dereference an empty array"),
            Error::DimensionOutOfBounds(got, max) => write!(
                f,
                "[Dimension mismatch] Got dimension {}, but only {} dimensions exists.",
                got, max
            ),
            Error::IndexNotValid(s) => write!(f, "[Invalid Index] {}", s),
            Error::UnknownOpCode(x) => write!(f, "[Invalid Opcode] {}", x),
            Error::InvalidOperand => write!(f, "[Invalid Operand]"),
            Error::MultiDimensionArrayNotAllowed => write!(f, "[Invalid Array] multi-dimensional array not allowed in current execution mode")
        }
    }
}
