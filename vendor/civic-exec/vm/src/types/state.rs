//! Types for representing the VM state

use crate::{
    binary::Binary,
    types::{
        BultinFunction, ModuleId, ProgramCounter, StackValue,
    },
};

/// A stack frame
pub struct Frame {
    /// The [`ProgramCounter`]
    program_counter: ProgramCounter,
}

impl Frame {
    /// Create a new stack frame with the given program counter
    pub fn new(program_counter: ProgramCounter) -> Self {
        Self { program_counter }
    }

    /// Get the program counter stored in this frame
    pub fn program_counter(&self) -> &ProgramCounter {
        &self.program_counter
    }
}

/// A CiviC module
#[derive(Default, Clone)]
pub struct Module {
    /// The underlying binary representation
    binary: Binary,

    /// The list of global variables
    globals: Vec<StackValue>,

    /// The list of external functions
    importfuns: Vec<ImportedFunction>,

    /// The list of external variables
    importvars: Vec<ImportedVar>,
}

impl Module {
    /// Get the [`Binary`] of this module
    pub fn binary(&self) -> &Binary {
        &self.binary
    }

    /// Set the [`Binary`] for this module
    pub fn set_binary(&mut self, binary: Binary) {
        self.binary = binary;
    }

    /// Get a reference to the list of global variables
    pub fn globals(&self) -> &Vec<StackValue> {
        &self.globals
    }

    /// Get a mutable reference to the list of global variables
    pub fn globals_mut(&mut self) -> &mut Vec<StackValue> {
        &mut self.globals
    }

    /// Set the list of global variables
    pub fn set_globals(&mut self, globals: Vec<StackValue>) {
        self.globals = globals;
    }

    /// Get a reference to the list of imported functions
    pub fn importfuns(&self) -> &Vec<ImportedFunction> {
        &self.importfuns
    }

    /// Set the list of imported functions
    pub fn set_importfuns(&mut self, funimports: Vec<ImportedFunction>) {
        self.importfuns = funimports;
    }

    /// Get the list of imported variables
    pub fn importvars(&self) -> &Vec<ImportedVar> {
        &self.importvars
    }

    /// Set the list of imported variables
    pub fn set_importvars(&mut self, vars: Vec<ImportedVar>) {
        self.importvars = vars;
    }
}

/// A function that is declared but not defined in the current module.
///
/// An external/imported function can be either a built-in function or
/// a "real" external function
#[derive(Clone)]
pub enum FunctionImport {
    /// Built-in function from the standard library
    Builtin(BultinFunction),

    /// Function defined extern
    ///
    /// Contains the [`ModuleId`] where this function is defined as well as
    /// the offset in those modules function table
    Extern(ModuleId, usize),
}

/// Resolved function import entry
#[derive(Clone)]
pub struct ImportedFunction {
    /// The number of arguments this function expects
    number_of_args: usize,

    /// The function defintion
    function: FunctionImport,
}

impl ImportedFunction {
    /// Get the number of arguments this function expects
    pub fn number_of_args(&self) -> usize {
        self.number_of_args
    }

    /// Get a reference to the [function declaration](FunctionImport).
    pub fn function(&self) -> &FunctionImport {
        &self.function
    }

    /// Create a new function declaration for an imported function
    pub fn new(number_of_args: usize, function: FunctionImport) -> Self {
        Self {
            number_of_args,
            function,
        }
    }
}

/// A varaible declared but not defined in this module
#[derive(Clone, Default)]
pub struct ImportedVar {
    /// The [`ModuleId`] where this variable is defined
    module: ModuleId,

    /// The index in those modules varaible table
    index: usize,
}

impl ImportedVar {
    /// Get the [`ModuleId`] where this variable is defined
    pub fn module(&self) -> ModuleId {
        self.module
    }

    /// Get the index for the variable table of the referenced module
    pub fn index(&self) -> usize {
        self.index
    }

    /// Set the [`ModuleId`] where this variable is defined
    pub fn set_module(&mut self, module: ModuleId) {
        self.module = module;
    }

    /// Set the index for the variable table of the referenced module
    pub fn set_index(&mut self, index: usize) {
        self.index = index;
    }
}
