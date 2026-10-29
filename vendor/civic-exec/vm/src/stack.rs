//! Definition of the CiviC VM stack

use crate::types::{Array, Error, Pointer, ProgramCounter, StackValue};
use std::{cell::RefCell, rc::Rc};

/// A stack representation
pub struct Stack {
    /// The data lying on the stack as LIFO list
    data: Vec<StackValue>,
}

impl Default for Stack {
    fn default() -> Self {
        Self { data: Vec::new() }
    }
}

impl Stack {
    /// Pop the topmost value from the stack
    fn pop(&mut self) -> Result<StackValue, Error> {
        if self.data.is_empty() {
            Err(Error::IndexNotValid(
                "Can not pop from an empty stack!".to_string(),
            ))
        } else {
            match self.data.pop() {
                Some(val) => Ok(val),
                None => Err(Error::Other("This should never occur!".to_string())),
            }
        }
    }

    /// Get a reference to the topmost stack value
    fn read(&self, index: Pointer) -> Result<&StackValue, Error> {
        match self.data.get(index) {
            Some(val) => Ok(val),
            None => Err(Error::IndexOutOfBounds(
                "trying to read out of bounds".to_string(),
            )),
        }
    }

    /// Store data at position `index`
    fn store(&mut self, index: Pointer, elem: StackValue) -> Result<(), Error> {
        if index >= self.data.len() {
            Err(Error::IndexOutOfBounds(
                "Trying to write outside of stack".to_string(),
            ))
        } else {
            self.data[index] = elem;
            Ok(())
        }
    }

    /// Push data on the stack
    fn push(&mut self, elem: StackValue) -> () {
        self.data.push(elem);
    }

    /// Check if top of stack is an integer and pop it
    ///
    /// If the top of stack value is not of integer type, this function returns
    /// an error
    pub fn popi(&mut self) -> Result<i64, Error> {
        match self.pop()? {
            StackValue::Int(val) => Ok(val),
            _ => Err(Error::InvalidType("Top of Stack is not an int".to_string())),
        }
    }

    /// Check if top of stack is floating point number and pop it
    ///
    /// If the top of stack value is not of floating point type, this function
    /// returns an error
    pub fn popf(&mut self) -> Result<f64, Error> {
        match self.pop()? {
            StackValue::Float(val) => Ok(val),
            _ => Err(Error::InvalidType(
                "Top of Stack is not an float".to_string(),
            )),
        }
    }

    /// Check if top of stack is a boolean and pop it
    ///
    /// If the top of stack value is not of boolean type, this function returns
    /// an error
    pub fn popb(&mut self) -> Result<bool, Error> {
        match self.pop()? {
            StackValue::Bool(val) => Ok(val),
            _ => Err(Error::InvalidType(
                "Top of Stack is not an boolean".to_string(),
            )),
        }
    }

    /// Check if top of stack is an array reference and pop it
    ///
    /// If the top of stack value is not of array reference type, this function
    /// returns an error
    pub fn popa(&mut self) -> Result<Rc<RefCell<Array>>, Error> {
        match self.pop()? {
            StackValue::Array(val) => Ok(val),
            _ => Err(Error::InvalidType(
                "Top of Stack is not an array".to_string(),
            )),
        }
    }

    /// Check if top of stack is an integer and return a reference to it
    ///
    /// If the top of stack value is not of integer type, this function returns
    /// an error
    pub fn readi(&self, index: Pointer) -> Result<&i64, Error> {
        match self.read(index)? {
            StackValue::Int(val) => Ok(val),
            _ => Err(Error::InvalidType("No integer at given index".to_string())),
        }
    }

    /// Check if top of stack is a floating point number and return a reference
    /// to it
    ///
    /// If the top of stack value is not of floating point type, this function
    /// returns an error
    pub fn readf(&self, index: Pointer) -> Result<&f64, Error> {
        match self.read(index)? {
            StackValue::Float(val) => Ok(val),
            _ => Err(Error::InvalidType("No float at given index".to_string())),
        }
    }

    /// Check if top of stack is a boolean and return a reference to it
    ///
    /// If the top of stack value is not of boolean type, this function returns
    /// an error
    pub fn readb(&self, index: Pointer) -> Result<&bool, Error> {
        match self.read(index)? {
            StackValue::Bool(val) => Ok(val),
            _ => Err(Error::InvalidType("No boolean at given index".to_string())),
        }
    }

    /// Check if top of stack is an array reference and return a reference to it
    ///
    /// If the top of stack value is not of array reference type, this function
    /// returns an error
    pub fn reada(&self, index: Pointer) -> Result<&Rc<RefCell<Array>>, Error> {
        match self.read(index)? {
            StackValue::Array(val) => Ok(val),
            _ => Err(Error::InvalidType("No array at given index".to_string())),
        }
    }

    /// Check if top of stack is a program counter and return a reference to it
    ///
    /// If the top of stack value is not of program counter type, this function
    /// returns an error
    pub fn read_pc(&self, index: Pointer) -> Result<&ProgramCounter, Error> {
        match self.read(index)? {
            StackValue::ProgramCounter(val) => Ok(val),
            _ => Err(Error::InvalidType(
                "No program counter at given index".to_string(),
            )),
        }
    }

    /// Check if top of stack is a stack pointer and return a reference to it
    ///
    /// If the top of stack value is not of stack pointer type, this function
    /// returns an error
    pub fn read_sp(&self, index: Pointer) -> Result<&Pointer, Error> {
        match self.read(index)? {
            StackValue::StackPointer(val) => Ok(val),
            _ => Err(Error::InvalidType(
                "No stack pointer at given index".to_string(),
            )),
        }
    }

    /// Store an integer at the given position
    pub fn storei(&mut self, index: Pointer, elem: i64) -> Result<(), Error> {
        let val = StackValue::Int(elem);
        self.store(index, val)
    }

    /// Store a floating point number at the given position
    pub fn storef(&mut self, index: Pointer, elem: f64) -> Result<(), Error> {
        let val = StackValue::Float(elem);
        self.store(index, val)
    }

    /// Store a boolean value at the given position
    pub fn storeb(&mut self, index: Pointer, elem: bool) -> Result<(), Error> {
        let val = StackValue::Bool(elem);
        self.store(index, val)
    }

    /// Store an array reference at the given position
    pub fn storea(&mut self, index: Pointer, elem: Rc<RefCell<Array>>) -> Result<(), Error> {
        let val = StackValue::Array(elem);
        self.store(index, val)
    }

    /// Store a program counter at the given position
    pub fn store_pc(&mut self, index: Pointer, elem: ProgramCounter) -> Result<(), Error> {
        let val = StackValue::ProgramCounter(elem);
        self.store(index, val)
    }

    /// Push an integer on the stack
    pub fn pushi(&mut self, elem: i64) -> () {
        let val = StackValue::Int(elem);
        self.push(val);
    }

    /// Push a floating point value on the stack
    pub fn pushf(&mut self, elem: f64) -> () {
        let val = StackValue::Float(elem);
        self.push(val);
    }

    /// Push a boolean value on the stack
    pub fn pushb(&mut self, elem: bool) -> () {
        let val = StackValue::Bool(elem);
        self.push(val);
    }

    /// Push an array reference on the stack
    pub fn pusha(&mut self, elem: Rc<RefCell<Array>>) -> () {
        let val = StackValue::Array(elem);
        self.push(val);
    }

    /// Push a program counter on the stack
    pub fn push_pc(&mut self, elem: ProgramCounter) -> () {
        let val = StackValue::ProgramCounter(elem);
        self.push(val);
    }

    /// Push a stack pointer on the stack
    pub fn push_sp(&mut self, elem: Pointer) -> () {
        let val = StackValue::StackPointer(elem);
        self.push(val);
    }

    /// Allocatate additional space on the stack
    ///
    /// The newly created space is filled with [`StackValue::None`] values
    pub fn allocate(&mut self, n: u32) -> Result<(), Error> {
        let space = self.data.len();
        if space > u32::MAX as usize || space + n as usize > u32::MAX as usize {
            Err(Error::IndexOutOfBounds(
                "Can only handle 2^32 stack elements!".to_string(),
            ))
        } else {
            self.data
                .resize_with(self.data.len() + n as usize, || StackValue::None);
            Ok(())
        }
    }

    /// Reset stack size to given stack pointer
    pub fn reset(&mut self, sp: Pointer) -> () {
        self.data.resize(sp, StackValue::None);
    }

    /// Get length of the stack
    pub fn size(&self) -> Pointer {
        self.data.len()
    }
}
