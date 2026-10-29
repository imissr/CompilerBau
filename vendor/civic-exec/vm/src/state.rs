//! The state of the virtual machine
//!
//! This mosule contains the main logic of the VM, handling program execution
//! and stack maintenance.
//!
//!
//! Stack layout:
//! ```text
//!           ...
//! Old FP -> ...
//!           ...
//!           ...
//!           Return address \
//!           Old FP         |- Activation Record
//!           Static link    /
//!     FP -> arg 0          \
//!           arg 1          |- Arguments
//!           arg 2          /
//!           local 0        \
//!           local 1        |- Locals
//!           local 2        /
//!           ...            \
//!           ...            |- Stack space for expressions, etc
//!           ...            /
//!     SP ->
//!```
//! `SP`` is the vectors length.
//!

use std::{cell::RefCell, rc::Rc};

use libcivexec::{BasicType, Type, operation::{Instr, OperandType, Operation}};

use crate::{
    binary::Binary,
    builtins,
    stack::Stack,
    types::{
        Array, Error, ModuleId, Pointer, ProgramCounter, StackValue,
        state::{Frame, FunctionImport, ImportedFunction, ImportedVar, Module},
    },
};

// FP based activation record indices

/// Size of an activation record
const ACT_REC_SIZE: usize = 3;

/// Offset of the return address in an activation record
const ACT_REC_RET_ADDR: usize = 3;

/// Offset of the frame pointer in an activation record
const ACT_REC_DLINK: usize = 2;

/// Offset of the stack pointer in an activation record
const ACT_REC_SLINK: usize = 1;

/// Identifier of an invalid module
const INVALID_MODULE: ModuleId = ModuleId::MAX;

/// Types of a call instruction
enum Call {
    /// Call of a function of an higher frame
    HigherFrame(u8),

    /// Call of a (nested) local function
    NestedLocal,

    /// Call of a global function
    Global,
}

/// The current state of the VM
#[derive(Default)]
pub struct VMState {
    /// The currently loaded modules
    modules: Vec<Module>,

    /// The stack
    stack: Stack,

    /// The program counter pointing to the current instruction
    pc: ProgramCounter,

    /// The frame pointer specifying the current activation record
    fp: Pointer,

    /// How many instructions were executed
    instr_counter: usize,

    /// Should we print the executed instructions?
    verbose_mode: bool,

    /// Are multi-dimensional arrays allowed?
    mult_dim_arrays_allowed: bool
}

impl VMState {
    /// Perform the integer binary operation specified by `op`
    fn binop_i<T>(&mut self, op: impl Fn(i64, i64) -> T) -> Result<T, Error> {
        let right = self.popi()?;
        let left = self.popi()?;
        Ok(op(left, right))
    }

    /// Perform the floating point binary operation specified by `op`
    fn binop_f<T>(&mut self, op: impl Fn(f64, f64) -> T) -> Result<T, Error> {
        let right = self.popf()?;
        let left = self.popf()?;
        Ok(op(left, right))
    }

    /// Perform the boolean binary  operation specified by `op`
    fn binop_b<T>(&mut self, op: impl Fn(bool, bool) -> T) -> Result<T, Error> {
        let right = self.popb()?;
        let left = self.popb()?;
        Ok(op(left, right))
    }

    /// Check, whether the type of the top-of-stack value matches the given [`BasicType`]
    fn compare_value_type(value: &StackValue, basic_type: BasicType) -> bool {
        match (value, basic_type) {
            (StackValue::Int(_), BasicType::Int) => true,
            (StackValue::Float(_), BasicType::Float) => true,
            (StackValue::Bool(_), BasicType::Bool) => true,
            (_, _) => false,
        }
    }

    /// Traverses the static links by N hops and returns that frame pointer
    fn get_frame(&self, mut n: usize) -> Result<Pointer, Error> {
        // Follow the static link
        let mut fp = self.fp;
        while n > 0 {
            if fp == 0 {
                return Err(Error::InvalidFP("Invalid static depth".to_string()));
            }
            fp = *self.stack.read_sp(fp - ACT_REC_SLINK)?;
            if fp <= ACT_REC_SLINK {
                return Err(Error::InvalidFP(
                    "There should be at least an entire activation context before FP".to_string(),
                ));
            }
            n -= 1;
        }

        Ok(fp)
    }

    /// Get the module specified by `index`
    fn get_module_from_index(&self, index: usize) -> Result<&Module, Error> {
        match self.modules.get(index) {
            Some(value) => Ok(value),
            None => Err(Error::IndexOutOfBounds(format!(
                "{} is not a valid module index",
                index
            ))),
        }
    }

    /// Get a mutable reference to the module specified by `index`
    fn get_module_from_index_mut(&mut self, index: usize) -> Result<&mut Module, Error> {
        match self.modules.get_mut(index) {
            Some(value) => Ok(value),
            None => Err(Error::IndexOutOfBounds(format!(
                "{} is not a valid module index",
                index
            ))),
        }
    }

    /// Loads the constant specified by `index` from the current module's
    /// constants.
    ///
    /// This function only returns a reference to the constant, if the type of
    /// the constant matches the type specified by `the_type`.
    fn readc(&self, index: u16, the_type: BasicType) -> Result<&StackValue, Error> {
        let module = self.get_module_from_index(self.pc.module)?;

        match module.binary().constants().get(index as usize) {
            Some(value) => {
                if !Self::compare_value_type(value, the_type) {
                    Err(Error::InvalidType(format!(
                        "reading constant {}, expected {}",
                        value, the_type
                    )))
                } else {
                    Ok(value)
                }
            }
            None => Err(Error::IndexOutOfBounds(
                "constants index out of range".to_string(),
            )),
        }
    }

    /// Loads the value of the global variable specified by `index` from the
    /// current module's globals.
    ///
    /// This function only returns a reference to the constant, if the type of
    /// the global matches the type specified by `the_type`.
    fn readg(&self, index: u16, the_type: Type) -> Result<&StackValue, Error> {
        let module = self.get_module_from_index(self.pc.module)?;

        match module.globals().get(index as usize) {
            Some(value) => {
                if (the_type.dimensions() == 0 && !Self::compare_value_type(value, the_type.basic_type()))
                    || (the_type.dimensions() > 0 && value.get_dims() == 0)
                {
                    Err(Error::InvalidType(format!(
                        "reading constant {}, expected type {}",
                        value,
                        the_type.basic_type()
                    )))
                } else {
                    Ok(value)
                }
            }
            None => Err(Error::IndexOutOfBounds(
                "Globals index out of range".to_string(),
            )),
        }
    }

    /// Loads the variable specified by `ìndex` from the current module's imported variables
    ///
    /// This function only returns a reference to the constant, if the type of
    /// the variable matches the type specified by `the_type`.
    fn reade(&self, index: u16, the_type: Type) -> Result<&StackValue, Error> {
        let module = self.get_module_from_index(self.pc.module)?;

        let imp = match module.importvars().get(index as usize) {
            Some(x) => Ok(x), // get the variable reference
            None => Err(Error::IndexOutOfBounds(
                "importvar index out of range".to_string(),
            )),
        }?;

        // lookup the actual value in the module specified by the reference
        let new_module = self.get_module_from_index(imp.module())?;
        match new_module.globals().get(imp.index()) {
            Some(value) => {
                if (the_type.dimensions() == 0 && !Self::compare_value_type(value, the_type.basic_type()))
                    || (the_type.dimensions() > 0 && value.get_dims() == 0)
                {
                    Err(Error::InvalidType(format!(
                        "reading value {}, expected type {}",
                        value,
                        the_type.basic_type()
                    )))
                } else {
                    Ok(value)
                }
            }
            None => Err(Error::IndexOutOfBounds(
                "globals index out of range".to_string(),
            )),
        }
    }

    /// Writes `value` into the global variable of the current module specified
    /// by `index`.
    ///
    /// Includes a typecheck.
    fn storeg(&mut self, index: u16, value: StackValue) -> Result<(), Error> {
        let module = self.get_module_from_index_mut(self.pc.module)?;

        match module.globals_mut().get_mut(index as usize) {
            Some(myval) => {
                if myval.compare_type_with_other(&value) {
                    *myval = value;
                    Ok(())
                } else {
                    Err(Error::InvalidType(format!(
                        "trying write global {}, expected type {}",
                        value, myval
                    )))
                }
            }
            None => Err(Error::IndexOutOfBounds(
                "globals index out of bounds".to_string(),
            )),
        }
    }

    /// Writes `value` into the variable specified by `ìndex` from the current
    /// module's imported variables.
    ///
    /// Includes a typecheck.
    fn storee(&mut self, index: u16, value: StackValue) -> Result<(), Error> {
        let module = self.get_module_from_index(self.pc.module)?; // load the module to write in

        // perform a typecheck and write the value
        match module.importvars().get(index as usize) {
            Some(importvar) => {
                let mod_index = importvar.module();
                let import_index = importvar.index();
                let ext_module = self.get_module_from_index_mut(mod_index)?;
                match ext_module.globals_mut().get_mut(import_index) {
                    Some(val) => {
                        if val.compare_type_with_other(&value) {
                            *val = value;
                            Ok(())
                        } else {
                            Err(Error::InvalidType(format!(
                                "writing external {}, expected type {}",
                                val, value
                            )))
                        }
                    }
                    None => Err(Error::IndexOutOfBounds(
                        "external globals index out of range".to_string(),
                    )),
                }
            }
            None => Err(Error::IndexOutOfBounds(
                "imports index out of range".to_string(),
            )),
        }
    }

    /// Loads a value from the array `array` and push it on the stack.
    ///
    /// Includes a typecheck.
    fn array_deref<'a>(
        &mut self,
        array: &'a mut Array,
        the_type: BasicType,
    ) -> Result<&'a mut StackValue, Error> {
        if array.values().is_empty() {
            return Err(Error::EmptyDeref);
        }
        match array.values().get(0) {
            Some(stack_ref) => {
                if !stack_ref.compare_with_basic_type(&the_type) {
                    return Err(Error::InvalidType(format!(
                        "Trying to dereference array of type {}, expected {}",
                        stack_ref, the_type
                    )));
                }
            }
            None => return Err(Error::Other("This should never occur!".to_string())),
        }

        // Pop dimensions and calculate flat index
        let mut index: i64 = 0;
        let mut base: i64 = 1;
        for bound in array.get_bounds() {
            let idx = self.popi()?;
            if idx < 0 || idx >= *bound as i64 {
                return Err(Error::IndexOutOfBounds(
                    "Array index out of bounds".to_string(),
                ));
            }
            index += idx * base;
            base *= *bound as i64;
        }

        Ok(array.values_mut().get_mut(index as usize).unwrap())
    }

    /*fn array_size(array: &Array, dim: usize) -> Result<u32, Error> {
        match array.get_bounds().get(dim) {
            Some(x) => Ok(*x),
            None => Err(Error::DimensionOutOfBounds(dim, array.get_bounds().len()))
        }
    }*/

    /// Create a new array of given dimensions and type.
    ///
    /// Creates a multi-dimensional array with `dim` dimensions. The size of
    /// each dimension is poped from the stack.
    ///
    /// The array is stored on the stack as heap reference
    fn array_new(&mut self, dim: usize, the_type: BasicType) -> Result<Rc<RefCell<Array>>, Error> {
        if the_type == BasicType::Void {
            return Err(Error::InvalidType(
                "Can not create array of void type.".to_string(),
            ));
        }

        if dim > 1 && !self.mult_dim_arrays_allowed { return Err(Error::MultiDimensionArrayNotAllowed); }

        // Pop bounds
        let mut bounds = vec![0 as u32; dim];
        for i in 0..dim {
            let dim_size = self.popi()?;
            if dim_size <= 0 {
                return Err(Error::IndexNotValid(
                    "Can not create array with negative lenght".to_string(),
                ));
            }
            bounds[i] = dim_size as u32;
        }

        Ok(Rc::new(RefCell::new(Array::new(the_type, bounds))))
    }

    /// Performs a conditional branch to the given offset.
    ///
    /// - `taken`: Whether the brach should be performed or not
    /// - `offset`: The value to add to the current program counter
    fn branch(&mut self, taken: bool, abs_offset: i16) -> Result<(), Error> {
        if taken {
            if let Some(module) = self.modules.get(self.pc.module) {
                let inst_size = module.binary().get_bytecode().len();
                if abs_offset < 0 || (abs_offset as usize) >= inst_size {
                    return Err(Error::IndexNotValid(
                        "Attemping to jump outside of function".to_string(),
                    ));
                }
                self.pc.offset = abs_offset as usize;
            } else {
                return Err(Error::IndexOutOfBounds(format!(
                    "Module index {} out of bounds",
                    self.pc.module
                )));
            }
        } else {
            self.pc.offset += 2;
        }

        Ok(())
    }

    /// Prepare jump to subroutine in scope of the given nesting level
    fn isr(&mut self, nesting_level: Call) -> Result<(), Error> {
        // Push activation frame
        self.stack.push_pc(self.pc); // Return address (value unused)
        self.stack.push_sp(self.fp); // dynamic link

        // global call; no static link
        let link: Pointer = match nesting_level {
            Call::HigherFrame(val) => self.get_frame(val as usize + 1)?,
            Call::NestedLocal => self.fp,
            Call::Global => 0,
        };

        self.stack.push_sp(link);

        Ok(())
    }

    /// Jump into the subroutine specified by module id and offset with given
    /// number of arguments.
    ///
    /// This function increases the program counter by 2.
    fn jsr(&mut self, number_args: u8, mod_id: ModuleId, offset: usize) -> Result<(), Error> {
        self.jump_subroutine(number_args, mod_id, offset, true)
    }

    /// Jump into the subroutine specified by module id and offset with given
    /// number of arguments.
    ///
    /// The calee can decide whether to increase the program counter of not.
    fn jump_subroutine(
        &mut self,
        number_args: u8,
        mod_id: ModuleId,
        offset: usize,
        increase_pc: bool,
    ) -> Result<(), Error> {
        let module = self.get_module_from_index(mod_id)?;
        let instr_size = module.binary().get_bytecode().len();
        if offset as usize >= instr_size {
            return Err(Error::IndexNotValid(
                "Attemping to jump outside the program".to_string(),
            ));
        }

        if increase_pc {
            self.pc.offset += 2; // We've read the the offset -> increase PC now
        }
        // if clause necessary since jsre may using this
        // function. However, a jsre instruction must have
        // different implications on the program counter

        let fp = self.stack.size() - number_args as usize;
        self.stack.store_pc(fp - ACT_REC_RET_ADDR, self.pc)?;

        // Verify that the new FP points to the activation frame.
        // We do this by reading it and having the typecheck verify them.
        self.stack.read_sp(fp - ACT_REC_DLINK)?;
        self.stack.read_sp(fp - ACT_REC_SLINK)?;

        self.fp = fp;
        self.pc.module = mod_id;
        self.pc.offset = offset;

        Ok(())
    }

    /// Jump into imported subroutine specified by `index`
    fn jsre(&mut self, index: u16) -> Result<(), Error> {
        // Get the imported function by its index
        let module = match self.modules.get(self.pc.module) {
            Some(value) => Ok(value),
            None => Err(Error::IndexOutOfBounds(format!(
                "{} is not a valid module index",
                index
            ))),
        }?;

        // Jump into the function of the specified module
        match module.importfuns().get(index as usize) {
            Some(imported_fun) => {
                // We've got a builtin function
                if let FunctionImport::Builtin(fun) = imported_fun.function() {
                    let fp = (self.stack.size() - imported_fun.number_of_args()) as u8;
                    self.stack
                        .store_pc(fp as usize - ACT_REC_RET_ADDR, self.pc)?;

                    // Verify that the new FP points to the activation frame.
                    // We do this by reading it and having the typecheck verify them.
                    self.stack.read_sp(fp as usize - ACT_REC_DLINK)?;
                    self.stack.read_sp(fp as usize - ACT_REC_SLINK)?;

                    self.fp = fp.into();

                    // Call the builtin
                    fun(self)?;
                // We got an actually imported function -> Do a inter-module procedure call
                } else if let FunctionImport::Extern(mod_id, offset) = imported_fun.function() {
                    self.jump_subroutine(
                        imported_fun.number_of_args() as u8,
                        *mod_id,
                        *offset,
                        false,
                    )?;
                }
            }
            // Index not valid
            None => {
                return Err(Error::IndexOutOfBounds(
                    "Function imports index".to_string(),
                ));
            }
        }

        Ok(())
    }

    /// Read an integer value from the stack at given index
    fn readi(&self, index: Pointer) -> Result<&i64, Error> {
        self.stack.readi(index)
    }

    /// Read a floating point value from the stack at given index
    fn readf(&self, index: Pointer) -> Result<&f64, Error> {
        self.stack.readf(index)
    }

    /// Read a boolean value from the stack at given index
    fn readb(&self, index: Pointer) -> Result<&bool, Error> {
        self.stack.readb(index)
    }

    /// Read an array reference from the stack at given index
    fn reada(&self, index: Pointer) -> Result<&Rc<RefCell<Array>>, Error> {
        self.stack.reada(index)
    }

    /// Store an integer value on the stack at given index
    fn storei(&mut self, index: Pointer, value: i64) -> Result<(), Error> {
        self.stack.storei(index, value)
    }

    /// Read a floating point value on the stack at given index
    fn storef(&mut self, index: Pointer, value: f64) -> Result<(), Error> {
        self.stack.storef(index, value)
    }

    /// Read a boolean value on the stack at given index
    fn storeb(&mut self, index: Pointer, value: bool) -> Result<(), Error> {
        self.stack.storeb(index, value)
    }

    /// Store an array reference on the stack at given index
    fn storea(&mut self, index: Pointer, array_ref: Rc<RefCell<Array>>) -> Result<(), Error> {
        self.stack.storea(index, array_ref)
    }

    /// Read a 8 bit unsigned integer from the modules bytecode
    fn read_u8_from_bytecode(&mut self) -> Result<u8, Error> {
        let module_id = self.pc.module;
        if let Some(module) = self.modules.get(module_id) {
            let byte = module.binary().get_bytecode()[self.pc.offset];
            Ok(byte)
        } else {
            Err(Error::IndexOutOfBounds(format!(
                "No module with index {} found",
                module_id
            )))
        }
    }

    /// Read a 16 bit unsigned integer from the modules bytecode
    fn read_u16_from_bytecode(&mut self) -> Result<u16, Error> {
        let module_id = self.pc.module;
        if let Some(module) = self.modules.get(module_id) {
            let bytecode = module.binary().get_bytecode();
            let (byte_1, byte_2) = (bytecode[self.pc.offset], bytecode[self.pc.offset + 1]);
            Ok(u16::from_le_bytes([byte_1, byte_2]))
        } else {
            Err(Error::IndexOutOfBounds(format!(
                "No module with index {} found",
                module_id
            )))
        }
    }

    /// Read a 16 bit signed integer from the modules bytecode
    fn read_i16_from_bytecode(&mut self) -> Result<i16, Error> {
        let module_id = self.pc.module;
        if let Some(module) = self.modules.get(module_id) {
            let bytecode = module.binary().get_bytecode();
            let (byte_1, byte_2) = (bytecode[self.pc.offset], bytecode[self.pc.offset + 1]);
            Ok(i16::from_le_bytes([byte_1, byte_2]))
        } else {
            Err(Error::IndexOutOfBounds(format!(
                "No module with index {} found",
                module_id
            )))
        }
    }

    /// Execute the bytecode
    ///
    /// This is the main logic of our VM. It loads the next instruction from the
    /// bytecode and decodes it. If necessary, it loads the operands. Then the
    /// instruction is executed.
    pub fn run(&mut self) -> Result<(), Error> {
        while self.pc.module != INVALID_MODULE {
            let verb_mode = self.verbose_mode;

            // Get the module we are executing from
            let module = self.get_module_from_index(self.pc.module)?;

            let op_code = module.binary().get_bytecode()[self.pc.offset];
            let instr = Instr::try_from(op_code).map_err(|_e| Error::UnknownOpCode(op_code))? ;
            let mut op = Operation::from(instr);

            // Remember the old offset
            let old_offset = self.pc.offset;

            self.instr_counter += 1;
            self.pc.offset += 1;

            // Load operands (if any)
            for i in 0..2 {
                match op.operands_mut()[i] {
                    OperandType::None => {}
                    OperandType::ArgumentCount(_) => {
                        let res = self.read_u8_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::ArgumentCount(Some(res));
                        self.pc.offset += 1;
                    }
                    OperandType::Local(_) => {
                        let res = self.read_u8_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::Local(Some(res));
                        self.pc.offset += 1;
                    }
                    OperandType::LinkCount(_) => {
                        let res = self.read_u8_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::LinkCount(Some(res));
                        self.pc.offset += 1;
                    }
                    OperandType::ConstantIndex(_) => {
                        let res = self.read_u16_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::ConstantIndex(Some(res));
                        self.pc.offset += 2;
                    }
                    OperandType::GlobalIndex(_) => {
                        let res = self.read_u16_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::GlobalIndex(Some(res));
                        self.pc.offset += 2;
                    }
                    OperandType::ImportIndex(_) => {
                        let res = self.read_u16_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::ImportIndex(Some(res));
                        self.pc.offset += 2;
                    }
                    OperandType::Offset(_) => {
                        let res = self.read_i16_from_bytecode()?;
                        op.operands_mut()[i] = OperandType::Offset(Some(res));
                        //self.pc.offset += 2; don't do this or we got an error when jumping or branching
                    }
                }
            }

            // If we are in verbose mode, print the decoded instruction
            if verb_mode {
                crate::types::opcodes::print_instr(old_offset, &op);
            }

            let operands = op.operands();

            // Execute the instruction
            match op.instruction() {
                Instr::IntAdd => {
                    let result = self.binop_i(|a, b| a + b)?;
                    self.pushi(result);
                }
                Instr::IntSub => {
                    let result = self.binop_i(|a, b| a - b)?;
                    self.pushi(result);
                }
                Instr::IntMul => {
                    let result = self.binop_i(|a, b| a * b)?;
                    self.pushi(result);
                }
                Instr::IntDiv => {
                    let result = self.binop_i(|a, b| a / b)?;
                    self.pushi(result);
                }
                Instr::IntRem => {
                    let result = self.binop_i(|a, b| a % b)?;
                    self.pushi(result);
                }
                Instr::IntNeg => {
                    let result = self.popi()?;
                    self.pushi(-result);
                }
                Instr::IntNewArray => {
                    let array = self.array_new(1, BasicType::Int)?;
                    self.pusha(array);
                }
                Instr::IntPop => _ = self.popi(),
                Instr::IntNotEq => {
                    let result = self.binop_i(|a, b| a != b)?;
                    self.pushb(result);
                }
                Instr::IntEq => {
                    let result = self.binop_i(|a, b| a == b)?;
                    self.pushb(result);
                }
                Instr::IntLess => {
                    let result = self.binop_i(|a, b| a < b)?;
                    self.pushb(result);
                }
                Instr::IntLessEqual => {
                    let result = self.binop_i(|a, b| a <= b)?;
                    self.pushb(result);
                }
                Instr::IntGreater => {
                    let result = self.binop_i(|a, b| a > b)?;
                    self.pushb(result);
                }
                Instr::IntGreaterEqual => {
                    let result = self.binop_i(|a, b| a >= b)?;
                    self.pushb(result);
                }
                Instr::IntReturn => {
                    let result = self.popi()?;
                    self.do_return()?;
                    self.pushi(result);
                }
                Instr::IntLoadLocal0 => {
                    let result = self.readi(self.fp + 0)?;
                    self.pushi(*result);
                }
                Instr::IntLoadLocal1 => {
                    let result = self.readi(self.fp + 1)?;
                    self.pushi(*result);
                }
                Instr::IntLoadLocal2 => {
                    let result = self.readi(self.fp + 2)?;
                    self.pushi(*result);
                }
                Instr::IntLoadLocal3 => {
                    let result = self.readi(self.fp + 3)?;
                    self.pushi(*result);
                }
                Instr::IntLoad0 => self.pushi(0),
                Instr::IntLoad1 => self.pushi(1),
                Instr::IntLoadM1 => self.pushi(-1),
                Instr::IntLoadConstant => {
                    match operands[0] {
                        OperandType::ConstantIndex(Some(index)) => {
                            let result = self.readc(index, BasicType::Int)?;
                            if let StackValue::Int(i) = result {
                                self.pushi(*i);
                            }
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntLoadLocal => {
                    match operands[0] {
                        OperandType::Local(Some(local)) => {
                            let index = self.fp + local as usize;
                            let result = self.readi(index)?;
                            self.pushi(*result);
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntLoadHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(frame_index)), OperandType::Local(Some(index))) => {
                            let frame = self.get_frame(frame_index as usize)?;
                            let index = frame + index as usize;
                            let result = self.readi(index)?;
                            self.pushi(*result);
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                },
                Instr::IntLoadGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let result = self.readg(index, Type::new(BasicType::Int, 0))?;
                            if let StackValue::Int(i) = result {
                                self.pushi(*i);
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntLoadExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let result = self.reade(index, Type::new(BasicType::Int, 0))?;
                            if let StackValue::Int(i) = result {
                                self.pushi(*i);
                            }
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntLoadFromArray => {
                    let array = self.popa()?;
                    let mut borrow = array.borrow_mut();
                    let deref = self.array_deref(&mut borrow, BasicType::Int)?;
                    if let StackValue::Int(i) = deref {
                        self.pushi(*i);
                    }
                }
                Instr::IntStoreLocal => {
                    match operands[0] {
                        OperandType::Local(Some(local_index)) => {
                            let index = self.fp + local_index as usize;
                            let result = self.popi()?;
                            self.storei(index, result)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntStoreHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(base_index)), OperandType::Local(Some(frame_index))) => {
                                let index = self.get_frame(base_index as usize)? + frame_index as usize;
                                let result = self.popi()?;
                                self.storei(index, result)?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntStoreGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let result = self.popi()?;
                            self.storeg(index, StackValue::Int(result))?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntStoreExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let result = self.popi()?;
                            self.storee(index, StackValue::Int(result))?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntStoreToArray => {
                    let array = self.popa()?;
                    let mut borrow = array.borrow_mut();
                    let deref = self.array_deref(&mut borrow, BasicType::Int)?;
                    let result = self.popi()?;
                    if let StackValue::Int(val) = deref {
                        *val = result;
                    }
                }
                Instr::FloatAdd => {
                    let func = |a, b| a + b;
                    let result = self.binop_f(func)?;
                    self.pushf(result);
                }
                Instr::FloatSub => {
                    let func = |a, b| a - b;
                    let result = self.binop_f(func)?;
                    self.pushf(result);
                }
                Instr::FloatMul => {
                    let func = |a, b| a * b;
                    let result = self.binop_f(func)?;
                    self.pushf(result);
                }
                Instr::FloatDiv => {
                    let func = |a, b| a / b;
                    let result = self.binop_f(func)?;
                    self.pushf(result);
                }
                Instr::FloatNeg => {
                    let result = self.popf()?;
                    self.pushf(-result);
                }
                Instr::FloatNewArray => {
                    let reuslt = self.array_new(1, BasicType::Float)?;
                    self.pusha(reuslt);
                }
                Instr::FloatPop => _ = self.popf(),
                Instr::FloatNotEq => {
                    let func = |a, b| a != b;
                    let result = self.binop_f(func)?;
                    self.pushb(result);
                }
                Instr::FloatEq => {
                    let func = |a, b| a == b;
                    let result = self.binop_f(func)?;
                    self.pushb(result);
                }
                Instr::FloatLess => {
                    let func = |a, b| a < b;
                    let result = self.binop_f(func)?;
                    self.pushb(result);
                }
                Instr::FloatLessEqual => {
                    let func = |a, b| a <= b;
                    let result = self.binop_f(func)?;
                    self.pushb(result);
                }
                Instr::FloatGreater => {
                    let func = |a, b| a > b;
                    let result = self.binop_f(func)?;
                    self.pushb(result);
                }
                Instr::FloatGreaterEqual => {
                    let func = |a, b| a >= b;
                    let result = self.binop_f(func)?;
                    self.pushb(result);
                }
                Instr::FloatReturn => {
                    let result = self.popf()?;
                    self.do_return()?;
                    self.pushf(result);
                }
                Instr::FloatLoadLocal0 => {
                    let result = self.readf(self.fp + 0)?;
                    self.pushf(*result);
                }
                Instr::FloatLoadLocal1 => {
                    let result = self.readf(self.fp + 1)?;
                    self.pushf(*result);
                }
                Instr::FloatLoadLocal2 => {
                    let result = self.readf(self.fp + 2)?;
                    self.pushf(*result);
                }
                Instr::FloatLoadLocal3 => {
                    let result = self.readf(self.fp + 3)?;
                    self.pushf(*result);
                }
                Instr::FloatLoad0 => self.pushf(0.0),
                Instr::FloatLoad1 => self.pushf(1.0),
                Instr::FloatLoadConstant => {
                    match operands[0] {
                        OperandType::ConstantIndex(Some(index)) => {
                            let result = self.readc(index, BasicType::Float)?;
                            if let StackValue::Float(f) = result {
                                self.pushf(*f);
                            }
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatLoadLocal => {
                    match operands[0] {
                        OperandType::Local(Some(basic_index)) => {
                            let index = self.fp + basic_index as usize;
                            let result = self.readf(index)?;
                            self.pushf(*result);
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatLoadHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(basic_index)), OperandType::Local(Some(local_index))) => {
                            let frame_index = self.get_frame(basic_index as usize)?;
                            let index = frame_index + local_index as usize;
                            let result = self.readf(index)?;
                            self.pushf(*result);
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatLoadGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let result = self.readg(index, Type::new(BasicType::Float, 0))?;
                            if let StackValue::Float(f) = result {
                                self.pushf(*f);
                            }
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatLoadExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let result = self.reade(index, Type::new(BasicType::Float, 0))?;
                            if let StackValue::Float(f) = result {
                                self.pushf(*f);
                            }
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatLoadFromArray => {
                    let array = self.popa()?;
                    let mut borrow = array.borrow_mut();
                    let deref = self.array_deref(&mut borrow, BasicType::Float)?;
                    if let StackValue::Float(f) = deref {
                        self.pushf(*f);
                    }
                }
                Instr::FloatStoreLocal => {
                    match operands[0] {
                        OperandType::Local(Some(index)) => {
                            let res = self.popf()?;
                            let new_fp = self.fp + index as usize;
                            self.storef(new_fp, res)?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatStoreHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(basic_index)), OperandType::Local(Some(local_idx))) => {
                            let curr_frame = self.get_frame(basic_index as usize)?;
                            let index = curr_frame + local_idx as usize;
                            let result = self.popf()?;
                            self.storef(index, result)?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatStoreGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let result = self.popf()?;
                            self.storeg(index, StackValue::Float(result))?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatStoreExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let result = self.popf()?;
                            self.storee(index, StackValue::Float(result))?;
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::FloatStoreToArray => {
                    let array = self.popa()?;
                    let mut borrow = array.borrow_mut();
                    let value = self.array_deref(&mut borrow, BasicType::Float)?;
                    let result = self.popf()?;
                    if let StackValue::Float(f) = value {
                        *f = result;
                    }
                }
                Instr::BoolAdd => {
                    let op = |a, b| a | b;
                    let res = self.binop_b(op)?;
                    self.pushb(res);
                }
                Instr::BoolNot => {
                    let result = self.popb()?;
                    self.pushb(!result);
                }
                Instr::BoolNewArray => {
                    let array = self.array_new(1, BasicType::Bool)?;
                    self.pusha(array);
                }
                Instr::BoolPop => _ = self.popb(),
                Instr::BoolNotEq => {
                    let op = |a, b| a != b;
                    let res = self.binop_b(op)?;
                    self.pushb(res);
                }
                Instr::BoolEq => {
                    let op = |a, b| a == b;
                    let res = self.binop_b(op)?;
                    self.pushb(res);
                }
                Instr::BitMul => {
                    let op = |a, b| a & b;
                    let res = self.binop_b(op)?;
                    self.pushb(res);
                }
                Instr::BoolReturn => {
                    let result = self.popb()?;
                    self.do_return()?;
                    self.pushb(result);
                }
                Instr::BoolLoadLocal0 => {
                    let result = self.readb(self.fp + 0)?;
                    self.pushb(*result);
                }
                Instr::BoolLoadLocal1 => {
                    let result = self.readb(self.fp + 1)?;
                    self.pushb(*result);
                }
                Instr::BoolLoadLocal2 => {
                    let result = self.readb(self.fp + 2)?;
                    self.pushb(*result);
                }
                Instr::BoolLoadLocal3 => {
                    let result = self.readb(self.fp + 3)?;
                    self.pushb(*result);
                }
                Instr::BoolLoadTrue => self.pushb(true),
                Instr::BoolLoadFalse => self.pushb(false),
                Instr::BoolLoadConstant => {
                    match operands[0] {
                        OperandType::ConstantIndex(Some(index)) => {
                            let result = self.readc(index, BasicType::Bool)?;
                            if let StackValue::Bool(b) = result {
                                self.pushb(*b);
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolLoadLocal => {
                    match operands[0] {
                        OperandType::Local(Some(local_index)) => {
                            let index = self.fp + local_index as usize;
                            let result = self.readb(index)?;
                            self.pushb(*result);
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolLoadHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(basic_index)), OperandType::Local(Some(local_index))) => {
                            let frame = self.get_frame(basic_index as usize)?;
                            let index = frame + local_index as usize;
                            let result = self.readb(index)?;
                            self.pushb(*result);
                        }
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolLoadGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let result = self.readg(index, Type::new(BasicType::Bool, 0))?;
                            if let StackValue::Bool(b) = result {
                                self.pushb(*b);
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolLoadExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let result = self.reade(index, Type::new(BasicType::Bool, 0))?;
                            if let StackValue::Bool(b) = result {
                                self.pushb(*b);
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolLoadFromArray => {
                    let array = self.popa()?;
                    let mut borrow = array.borrow_mut();
                    let deref = self.array_deref(&mut borrow, BasicType::Bool)?;
                    if let StackValue::Bool(b) = deref {
                        self.pushb(*b);
                    }
                }
                Instr::BoolStoreLocal => {
                    match operands[0] {
                        OperandType::Local(Some(local_index)) => {
                            let index = self.fp + local_index as usize;
                            let result = self.popb()?;
                            self.storeb(index, result)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolStoreHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(basic_index)), OperandType::Local(Some(local_index))) => {
                            let index = self.get_frame(basic_index as usize)? + local_index as usize;
                            let result = self.popb()?;
                            self.storeb(index, result)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolStoreGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let result = self.popb()?;
                            self.storeg(index, StackValue::Bool(result))?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolStoreExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let result = self.popb()?;
                            self.storee(index, StackValue::Bool(result))?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BoolStoreToArray => {
                    let array = self.popa()?;
                    let mut borrow = array.borrow_mut();
                    let deref = self.array_deref(&mut borrow, BasicType::Bool)?;
                    let result = self.popb()?;
                    if let StackValue::Bool(b) = deref {
                        *b = result;
                    }
                }

                Instr::IntIncr => {
                    match (operands[0], operands[1]) {
                        (OperandType::Local(Some(basic_index)), OperandType::ConstantIndex(Some(const_index))) => {
                            let index = self.fp + basic_index as usize;
                            let incr_val = self.readc(const_index, BasicType::Int)?;
                            let val = self.readi(index)?;
                            if let StackValue::Int(i) = incr_val {
                                self.storei(index, (*val) + i)?;
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntIncrBy1 => {
                    match operands[0] {
                        OperandType::Local(Some(basic_index)) => {
                            let index = self.fp + basic_index as usize;
                            let val = self.readi(index)?;
                            self.storei(index, (*val) + 1)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntDecr => {
                    match (operands[0], operands[1]) {
                        (OperandType::Local(Some(basic_index)), OperandType::ConstantIndex(Some(const_index))) => {
                            let index = self.fp + basic_index as usize;
                            let incr_val = self.readc(const_index, BasicType::Int)?;
                            let val = self.readi(index)?;
                            if let StackValue::Int(i) = incr_val {
                                self.storei(index, (*val) - i)?;
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntDecrBy1 => {
                    match operands[0] {
                        OperandType::Local(Some(basic_index)) => {
                            let index = self.fp + basic_index as usize;
                            let val = self.readi(index)?;
                            self.storei(index, (*val) - 1)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::Jump => {
                    match operands[0] {
                        OperandType::Offset(Some(offset)) => {
                            self.branch(true, self.pc.offset as i16 + offset)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BranchTrue => {
                    let cond_true = self.popb()?;
                    match operands[0] {
                        OperandType::Offset(Some(offset)) => {
                            self.branch(cond_true, self.pc.offset as i16 + offset)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::BranchFalse => {
                    let cond_true = self.popb()?;
                    match operands[0] {
                        OperandType::Offset(Some(offset)) => {
                            self.branch(!cond_true, self.pc.offset as i16 + offset)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::InitiateLocalSubRoutine => self.isr(Call::HigherFrame(0))?,
                Instr::InitiateHigherLocalSubRoutine => {
                    match operands[0] {
                        OperandType::LinkCount(Some(number)) => {
                            self.isr(Call::HigherFrame(number))?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntiateBelowLocalSubRoutine => self.isr(Call::NestedLocal)?,
                Instr::InitiateGlobalSubRoutine => self.isr(Call::Global)?,
                Instr::EnterSubRoutine => {
                    match operands[0] {
                        OperandType::Local(Some(index)) => {
                            self.stack.allocate(index as u32)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::JumpToSubRoutine => {
                    match (operands[0], operands[1]) {
                        (OperandType::ArgumentCount(Some(num_args)), OperandType::Offset(Some(offset))) => {
                            let new_offset = (self.pc.offset as i64 + offset as i64) as usize;
                            self.jsr(num_args.into(), self.pc.module, new_offset)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::JumpToExternalSubRoutine => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            self.jsre(index.into())?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::Return => self.do_return()?,

                Instr::ArrayLoadLocal0 => {
                    let arr = self.reada(self.fp + 0)?;
                    self.pusha(arr.clone());
                }
                Instr::ArrayLoadLocal1 => {
                    let arr = self.reada(self.fp + 1)?;
                    self.pusha(arr.clone());
                }
                Instr::ArrayLoadLocal2 => {
                    let arr = self.reada(self.fp + 2)?;
                    self.pusha(arr.clone());
                }
                Instr::ArrayLoadLocal3 => {
                    let arr = self.reada(self.fp + 3)?;
                    self.pusha(arr.clone());
                }

                /*Instr::ArraySize => {
                    let arr = self.popa()?;
                    let index = self.read_u8() as usize;
                    let size = Self::array_size(&arr, index)?;
                    self.pushi(size as i32);
                }*/
                Instr::ArrayLoadLocal => {
                    match operands[0] {
                        OperandType::Local(Some(offset)) => {
                            let value = self.reada(self.fp + offset as usize)?;
                            self.pusha(value.clone());
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::ArrayLoadHigherLocal => {
                    match (operands[0], operands[1]) {
                        (OperandType::LinkCount(Some(basic_index)), OperandType::Local(Some(local_index))) => {
                            let index = self.get_frame(basic_index as usize)? + local_index as usize;
                            let arr = self.reada(index)?;
                            self.pusha(arr.clone());
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::ArrayLoadGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let res = self.readg(index, Type::new(BasicType::Int, 1))?;
                            if let StackValue::Array(arr) = res {
                                self.pusha(arr.clone());
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::ArrayLoadExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let res = self.reade(index, Type::new(BasicType::Int, 1))?;
                            if let StackValue::Array(arr) = res {
                                self.pusha(arr.clone());
                            }
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::ArrayStoreLocal => {
                    match operands[0] {
                        OperandType::Local(Some(local)) => {
                            let index = self.fp + local as usize;
                            let arr = self.popa()?;
                            self.storea(index, arr)?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::ArrayStoreGlobal => {
                    match operands[0] {
                        OperandType::GlobalIndex(Some(index)) => {
                            let arr = self.popa()?;
                            self.storeg(index, StackValue::Array(arr))?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::ArrayStoreExternal => {
                    match operands[0] {
                        OperandType::ImportIndex(Some(index)) => {
                            let arr = self.popa()?;
                            self.storee(index, StackValue::Array(arr))?;
                        },
                        _ => { return Err(Error::InvalidOperand); }
                    }
                }
                Instr::IntToFloat => {
                    let result = self.popi()?;
                    self.pushf(result as f64);
                }
                Instr::FloatToInt => {
                    let result = self.popf()?;
                    self.pushi(result as i64);
                }
            }
        }

        Ok(())
    }

    /*fn find_exportfun(&self, name: &str, arguments: &[StackValue], ret_type: BasicType) -> Result<(&Function, usize), Error> {
        let num_args = arguments.len();
        for module in &self.modules {
            for exportfun in module.binary().exportfuns()
            {
                // Verify signature
                if    exportfun.name() == name
                   && exportfun.parameters().len() == num_args
                   && exportfun.return_type() == ret_type
                {
                    let mut i = 0;
                    while i < num_args {
                        match exportfun.parameters().get(i) {
                            Some(val) => {
                                    if val != arguments[i] {
                                    return Err(Error::InvalidType(format!("Type mismach on argument {} of function '{}'. Expected {}. Got {}", i, exportfun.name(), arguments[i], val.base_type())));
                                }
                                i += 1;
                            },
                            None => return Err(Error::IndexOutOfBounds(format!("Function {} has only {} arguments. Tried to access no. {}", exportfun.name(), exportfun.parameters().len(), i))),
                        }
                    }
                    if i == exportfun.parameters().len() {
                        return Ok((exportfun, i));
                    }
                }
            }
        }
        Err(Error::Other(format!("Function {} with {} not found.", name, num_args)))
    }*/

    /// Perform a function call
    ///
    /// - `name`: The name of the fucntion to call
    /// - `arguments`: The arguments to this function call
    /// - `ret_type`: The expected type of the return value
    pub fn call(
        &mut self,
        name: &str,
        arguments: &[StackValue],
        ret_type: BasicType,
    ) -> Result<(), Error> {
        // Stack should be empty
        assert!(self.stack.size() == 0);
        assert!(self.fp == 0);

        let num_args = arguments.len();

        // Check if there is an exported function with the given properties in
        // any of our modules
        let mut exportfun_opt = None;
        for mod_id in 0..self.modules.len() {
            let module = self.modules.get(mod_id).unwrap();
            for exportfun in module.binary().exportfuns() {
                // Verify signature
                if exportfun.name() == name
                    && exportfun.parameters().len() == num_args
                    && exportfun.return_type() == ret_type
                {
                    let mut j = 0;
                    while j < num_args {
                        match exportfun.parameters().get(j) {
                            Some(param) => {
                                if param != arguments[j] {
                                    return Err(Error::InvalidType(format!(
                                        "Type mismach on argument {} of function '{}'",
                                        j,
                                        exportfun.name()
                                    )));
                                } else {
                                    j += 1;
                                }
                            }
                            None => {
                                return Err(Error::IndexOutOfBounds(format!(
                                    "Exported function {} has only {} parameters. Got index {}",
                                    exportfun.name(),
                                    exportfun.parameters().len(),
                                    j
                                )));
                            }
                        }
                    }
                    if j == exportfun.parameters().len() {
                        exportfun_opt = Some((exportfun, mod_id))
                    }
                }
            }
        }

        // Iff we have found a function, perform a call to it
        if let Some((exported_function, mod_id)) = exportfun_opt {
            let num_args = arguments.len();
            let offset = exported_function.offset();

            // Signature matches, do a global call to the function
            self.isr(Call::Global)?;
            for arg in arguments {
                match arg {
                    StackValue::Int(i) => self.pushi(*i),
                    StackValue::Float(f) => self.pushf(*f),
                    StackValue::Bool(b) => self.pushb(*b),
                    x => {
                        return Err(Error::InvalidType(format!(
                            "Got type {}, expected int, float or bool",
                            x
                        )));
                    }
                }
            }
            self.jsr(num_args as u8, mod_id, offset)?;
            //self.run();
            return Ok(());
        }
        Err(Error::FunctionNotFound(format!(
            "Function {} for parameters {:?} not found",
            name, arguments
        )))
    }

    /// Perform an return operation
    ///
    /// This function does the stack maintenance on return from a soubroutine
    pub fn do_return(&mut self) -> Result<(), Error> {
        // We should have at least an entire activation record on the stack
        let fp = self.fp;
        assert!(fp >= ACT_REC_SIZE.into());

        // Restore PC and FP
        _ = self.stack.read_sp(fp - ACT_REC_SLINK as usize);
        self.fp = *self.stack.read_sp(fp - ACT_REC_DLINK as usize)?;
        self.pc = *self.stack.read_pc(fp - ACT_REC_RET_ADDR as usize)?;

        self.stack.reset(fp - ACT_REC_SIZE);

        Ok(())
    }

    pub fn get_backtrace(&self) -> Result<Vec<Frame>, Error> {
        let mut frames = Vec::<Frame>::new();

        // Walk the stack
        let mut pc = self.pc;
        let mut fp = self.fp;
        while pc.module != INVALID_MODULE {
            // Store current frame
            let frame = Frame::new(pc);
            frames.push(frame);

            // Go to parent frame
            pc = *self.stack.read_pc(fp - ACT_REC_RET_ADDR as usize)?;
            fp = *self.stack.read_sp(fp - ACT_REC_DLINK as usize)?;
        }

        Ok(frames)
    }

    /// Pop an integer value from the stack
    pub fn popi(&mut self) -> Result<i64, Error> {
        self.stack.popi()
    }

    /// Pop a floating point value from the stack
    pub fn popf(&mut self) -> Result<f64, Error> {
        self.stack.popf()
    }

    /// Pop a boolena value from the stack
    pub fn popb(&mut self) -> Result<bool, Error> {
        self.stack.popb()
    }

    /// Pop a array reference from the stack
    pub fn popa(&mut self) -> Result<Rc<RefCell<Array>>, Error> {
        self.stack.popa()
    }

    /// Push an integer value on the stack
    pub fn pushi(&mut self, i: i64) -> () {
        self.stack.pushi(i);
    }

    /// Push a floating point value on the stack
    pub fn pushf(&mut self, f: f64) -> () {
        self.stack.pushf(f);
    }

    /// Push a boolean value on the stack
    pub fn pushb(&mut self, b: bool) -> () {
        self.stack.pushb(b);
    }

    /// Push a array reference on the stack
    pub fn pusha(&mut self, a: Rc<RefCell<Array>>) -> () {
        self.stack.pusha(a);
    }

    /// Get the number of executed instructions
    pub fn instr_executed(&self) -> usize {
        self.instr_counter
    }

    /// Create a new VM state of of given binaries
    ///
    /// This function initialises all global and imported variables and functions
    /// and prepare the stack for the bytecode execution by executing the `__init`
    /// function(s) (if any).
    pub fn try_new(binaries: Vec<Binary>, verbose: bool, multi_dim_array_allowed: bool) -> Result<Self, Error> {
        let mut myself = Self::default();
        myself.verbose_mode = verbose;
        myself.mult_dim_arrays_allowed = multi_dim_array_allowed;

        // We first have to load all binaries, otherwise the next steps could fail
        for bin in binaries {
            let mut module = Module::default();
            module.set_binary(bin);

            let mut values = Vec::<StackValue>::new();

            for global in module.binary().globals() {
                let glob_type = global.clone();
                let val = match glob_type.dimensions() {
                    0 => match glob_type.basic_type() {
                        BasicType::Void => {
                            return Err(Error::InvalidType(
                                "Cannot initialise a value with void type".to_string(),
                            ));
                        }
                        BasicType::Int => StackValue::Int(i64::MIN),
                        BasicType::Float => StackValue::Float(f64::MIN),
                        BasicType::Bool => StackValue::Bool(false),
                    },
                    _ => {
                        let array = match glob_type.basic_type() {
                            BasicType::Void => {
                                return Err(Error::InvalidType(
                                    "Cannot initialise a value with void array type".to_string(),
                                ));
                            }
                            BasicType::Int => Array::new(BasicType::Int, vec![0]),
                            BasicType::Float => Array::new(BasicType::Float, vec![0]),
                            BasicType::Bool => Array::new(BasicType::Bool, vec![0]),
                        };
                        StackValue::Array(Rc::new(RefCell::new(array)))
                    }
                };
                values.push(val);
            }
            module.set_globals(values);
            myself.modules.push(module);
        }

        let number_of_modules = myself.modules.len();

        for i in 0..number_of_modules {
            let mut tmp_module = myself.modules[i].clone();
            let mut funimports = Vec::<ImportedFunction>::new();

            for fun in tmp_module.binary().importfuns() {
                let mut importfun: Option<ImportedFunction> = None;
                if let Some(builtin) = builtins::find_builtin(fun) {
                    let result = FunctionImport::Builtin(builtin.callback);
                    let num_args = builtin.num_parameters;
                    importfun = Some(ImportedFunction::new(num_args, result));
                } else {
                    // Not a builtin function, find it among the other modules
                    let mut j: usize = 0;
                    while j < number_of_modules {
                        let module_ref = &myself.modules[j];
                        if let Some(exp) = module_ref.binary().find_exportfun(fun) {
                            importfun = Some(ImportedFunction::new(
                                exp.parameters().len(),
                                FunctionImport::Extern(j, exp.offset()),
                            ));
                            break;
                        }
                        j = j + 1;
                    }
                    if j == number_of_modules {
                        return Err(Error::InvalidType(format!(
                            "Unable to find '{}' in any of the modules",
                            fun.name()
                        )));
                    }
                }
                //module.importfuns_mut().push(importfun.unwrap());
                funimports.push(importfun.unwrap()); // we can be sure, that importfun contains a Some value, otherwise we would have already returned an error
            }
            tmp_module.set_importfuns(funimports);

            let mut importvars = Vec::<ImportedVar>::new();
            for j in 0..tmp_module.binary().importvars().len() {
                let mut import_var = ImportedVar::default(); //&mut module.importvars[j];
                let orig = &tmp_module.binary().importvars()[j];

                let mut k: usize = 0;
                while k < number_of_modules {
                    let curr_mod = &myself.modules[k];
                    if let Some(exp) = curr_mod.binary().find_exportvar(orig) {
                        let orig_type = orig.value_type();
                        let other = &curr_mod.globals()[exp.index()];
                        let typecheck = orig_type.eq(other);
                        if typecheck {
                            import_var.set_module(k);
                            import_var.set_index(exp.index());
                            break;
                        } else {
                            return Err(Error::InvalidType(format!(
                                "Type mismatch for imported variable '{}'.",
                                orig.name()
                            )));
                        }
                    }
                    k += 1;
                }
                if k == myself.modules.len() {
                    return Err(Error::IndexOutOfBounds(format!(
                        "Unable to find '{}' in any of the modules",
                        orig.name()
                    )));
                }
                //module.importvars_mut().push(import_var);
                importvars.push(import_var);
            }
            tmp_module.set_importvars(importvars);
            myself.modules[i] = tmp_module;
        }

        myself.fp = 0;
        myself.pc = ProgramCounter {
            module: INVALID_MODULE,
            offset: 0,
        };

        // Now call __init functions of all modules (if any)
        for i in 0..number_of_modules {
            let module = myself.modules[i].clone(); // clone here, otherwise we have borrow problems
            // Stack should be empty
            assert!(myself.stack.size() == 0);
            assert!(myself.fp == 0);

            for exportfun in module.binary().exportfuns() {
                if exportfun.name() == "__init"
                    && exportfun.parameters().is_empty()
                    && exportfun.return_type() == BasicType::Void
                {
                    // This is a correct __init and working already on the modules in myself
                    myself.isr(Call::Global)?;
                    myself.jsr(0, i, exportfun.offset())?;
                    myself.run()?;
                }
            }
        }

        Ok(myself)
    }
}
