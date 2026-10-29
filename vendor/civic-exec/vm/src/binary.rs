//! Internal representation of a CiviC binary file

use std::io::BufRead;

use libcivexec::{BasicType, Type};

use crate::types::{
    Error, StackValue, binary::{Function, Variable},
};

/// Size of an [`f64`] value in bytes
const F64_SIZE: usize = size_of::<f64>();

/// Size of an [`i64`] value in bytes
const I64_SIZE: usize = size_of::<i64>();

/// Size of an [`u32`] value in bytes
const U32_SIZE: usize = size_of::<u32>();

/// Size of an [`u16`] value in bytes
const U16_SIZE: usize = size_of::<u16>();

/// The beginning bytes of an valid CiviC executable or library
const BINARY_SIGNATURE: [u8; 7] = [b'C', b'i', b'v', b'C', b'X', 1, 1];

/// The length of the byte file signature
const BINARY_SIGNATURE_SIZE: usize = size_of_val(&BINARY_SIGNATURE);

/// A CiviC binary file
#[derive(Clone, Default)]
pub struct Binary {
    /// The actual bytecode for execution
    bytecode: Vec<u8>,

    /// The list of external functions
    importfuns: Vec<Function>,

    /// The list of exported functions
    exportfuns: Vec<Function>,

    /// The list of external variables
    importvars: Vec<Variable>,

    /// The list of exported variables
    exportvars: Vec<Variable>,

    /// The list of constants
    constants: Vec<StackValue>,

    /// The list of global variables
    globals: Vec<Type>,

    /// The size of the bytecode in bytes
    code_size: usize,
}

impl Binary {
    /// Get a reference to the bytecode
    pub fn get_bytecode(&self) -> &Vec<u8> {
        &self.bytecode
    }

    /// Get a reference to the list of imported functions
    pub fn importfuns(&self) -> &Vec<Function> {
        &self.importfuns
    }

    /// Get a reference to the list of exported functions
    pub fn exportfuns(&self) -> &Vec<Function> {
        &self.exportfuns
    }

    /// Get a reference tot the list of the imported variables
    pub fn importvars(&self) -> &Vec<Variable> {
        &self.importvars
    }

    /// Get a reference to the list of the exported variables
    // pub fn exportvars(&self) -> &Vec<Variable> {
    //     &self.exportvars
    // }

    /// Get a reference to the constant values
    pub fn constants(&self) -> &Vec<StackValue> {
        &self.constants
    }

    /// Get a reference to the list of global variables
    pub fn globals(&self) -> &Vec<Type> {
        &self.globals
    }

    /// Get the size (in bytes) of the actual bytecode
    pub fn code_size(&self) -> usize {
        self.code_size
    }

    /// Find an exported function by its signature
    ///
    /// If there is an exported function in the current module that is equal in
    /// terms of name and signature, a reference to this function is returned in
    /// an [`Some`] value. Otherwise a [`None`] value is returned.
    pub fn find_exportfun(&self, desc: &Function) -> Option<&Function> {
        self.exportfuns
            .iter()
            .find(|&elem| -> bool { elem.check(&desc) })
    }

    /// Find an exported variable by its name
    ///
    /// If there is an exported function in the current module that is equal in
    /// terms of the name, a reference to this variable is returned in an [`Some`]
    /// value. Otherwise a [`None`] value is returned.
    pub fn find_exportvar(&self, desc: &Variable) -> Option<&Variable> {
        self.exportvars
            .iter()
            .find(|&elem| -> bool { elem.name() == desc.name() })
    }

    /// Create a new binary out of a given input file
    pub fn try_new<T: BufRead>(input: &mut T, multi_dim_arrays_allowed: bool) -> Result<Self, Error> {
        let mut byte_count: usize = 0;
        let mut signature: [u8; 7] = [0; 7];
        Self::read(input, &mut signature, &mut byte_count);

        if signature != BINARY_SIGNATURE {
            return Err(Error::FileNotValid);
        }

        // Read instructions
        let length_bytecode = Self::read_u32(input, &mut byte_count);
        let mut bytecode = vec![0 as u8; length_bytecode as usize];
        Self::read(input, &mut bytecode, &mut byte_count);

        // Read exported functions
        let number_exportfuns = Self::read_u32(input, &mut byte_count);
        let mut exported_funs = vec![Function::default(); number_exportfuns as usize];

        for i in 0..number_exportfuns as usize {
            let name = Self::read_string(input, &mut byte_count);
            let ret_type = Self::read_type(input, true, &mut byte_count)?;

            let number_parameters = Self::read_u16(input, &mut byte_count);
            let mut parameters = vec![Type::default(); number_parameters as usize];

            for j in 0..number_parameters as usize {
                let base = Self::read_type(input, false, &mut byte_count)?;
                let dim = Self::read_u8(input, &mut byte_count);
                if dim > 1 && !multi_dim_arrays_allowed { return Err(Error::MultiDimensionArrayNotAllowed); }
                parameters[j] = Type::new(base, dim.into());
            }
            let offset = Self::read_u32(input, &mut byte_count);

            let function = Function::new(name, ret_type, parameters, offset as usize);
            exported_funs[i] = function;
        }

        // Read imported functions
        let number_importfuns = Self::read_u32(input, &mut byte_count);
        let mut imported_funs = vec![Function::default(); number_importfuns as usize];
        for i in 0..number_importfuns as usize {
            let name = Self::read_string(input, &mut byte_count);
            let ret_type = Self::read_type(input, true, &mut byte_count)?;
            let num_params = Self::read_u16(input, &mut byte_count);
            let mut parameters = vec![Type::default(); num_params as usize];
            for j in 0..num_params as usize {
                let base = Self::read_type(input, false, &mut byte_count)?;
                let dim = Self::read_u8(input, &mut byte_count);
                if dim > 1 && !multi_dim_arrays_allowed { return Err(Error::MultiDimensionArrayNotAllowed); }
                parameters[j] = Type::new(base, dim as u8);
            }
            let function = Function::new(name, ret_type, parameters, 0);
            imported_funs[i] = function;
        }

        // Read exported variables
        let num_exportvars = Self::read_u32(input, &mut byte_count);
        let mut exported_vars = vec![Variable::default(); num_exportvars as usize];
        for i in 0..num_exportvars as usize {
            let name = Self::read_string(input, &mut byte_count);
            let index = Self::read_u16(input, &mut byte_count);
            let var = Variable::new(name, Type::default(), index as usize);
            exported_vars[i] = var;
        }

        // Read imported vars
        let num_importvars = Self::read_u32(input, &mut byte_count);
        let mut imported_vars = vec![Variable::default(); num_importvars as usize];
        for i in 0..num_importvars as usize {
            let name = Self::read_string(input, &mut byte_count);
            let base_type = Self::read_type(input, false, &mut byte_count)?;
            let dim = Self::read_u8(input, &mut byte_count);
            if dim > 1 && !multi_dim_arrays_allowed { return Err(Error::MultiDimensionArrayNotAllowed); }
            let this_type = Type::new(base_type, dim as u8);
            imported_vars[i] = Variable::new(name, this_type, 0);
        }

        // Read globals
        let num_globals = Self::read_u32(input, &mut byte_count);
        let mut globals = vec![Type::default(); num_globals as usize];
        for i in 0..num_globals as usize {
            let base = Self::read_type(input, false, &mut byte_count)?;
            let dim = Self::read_u8(input, &mut byte_count);
            if dim > 1 { return Err(Error::MultiDimensionArrayNotAllowed); }
            let the_type = Type::new(base, dim as u8);
            globals[i] = the_type;
        }

        // Read constants
        let num_constants = Self::read_u32(input, &mut byte_count);
        let mut constants = vec![StackValue::default(); num_constants as usize];
        for i in 0..num_constants as usize {
            let base_type = Self::read_type(input, false, &mut byte_count)?;
            //let content = Self::read_i32(input, &mut byte_count);
            let val = match base_type {
                BasicType::Void => {
                    return Err(Error::InvalidType(
                        "Found void type where not allowed".to_string(),
                    ));
                }
                BasicType::Int => StackValue::Int(Self::read_i64(input, &mut byte_count)),
                BasicType::Float => StackValue::Float(Self::read_f64(input, &mut byte_count)),
                BasicType::Bool => StackValue::Bool(Self::read_u8(input, &mut byte_count) == 1),
            };
            constants[i] = val;
        }

        let code_size = byte_count - BINARY_SIGNATURE_SIZE; // pos - offset 

        let myself = Self {
            code_size,
            constants,
            exportfuns: exported_funs,
            exportvars: exported_vars,
            globals,
            importfuns: imported_funs,
            importvars: imported_vars,
            bytecode,
        };

        Ok(myself)
    }

    /// Read bytes from the binary file until the buffer `dest` is filled
    fn read<T: BufRead>(input: &mut T, dest: &mut [u8], bytecounter: &mut usize) {
        _ = input.read_exact(dest).unwrap();
        *bytecounter += dest.len();
    }

    /// Read 8 bytes from the binary file and intepret the result as [`f64`]
    fn read_f64<T: BufRead>(input: &mut T, bytecounter: &mut usize) -> f64 {
        let mut buf: [u8; F64_SIZE] = [0; F64_SIZE];
        Self::read(input, &mut buf, bytecounter);
        f64::from_le_bytes(buf)
    }

    /// Read 8 bytes from the binary file and interpret the result as [`i64`]
    fn read_i64<T: BufRead>(input: &mut T, bytecounter: &mut usize) -> i64 {
        let mut buf: [u8; I64_SIZE] = [0; I64_SIZE];
        Self::read(input, &mut buf, bytecounter);
        i64::from_le_bytes(buf)
    }

    /// Read 4 bytes from the binary file and interpret the result as [`u32`]
    fn read_u32<T: BufRead>(input: &mut T, bytecounter: &mut usize) -> u32 {
        let mut buf: [u8; U32_SIZE] = [0; U32_SIZE];
        Self::read(input, &mut buf, bytecounter);
        u32::from_le_bytes(buf)
    }

    /// Read 2 bytes from the binary file and interpret the result as [`u16`]
    fn read_u16<T: BufRead>(input: &mut T, bytecounter: &mut usize) -> u16 {
        let mut buf: [u8; U16_SIZE] = [0; U16_SIZE];
        Self::read(input, &mut buf, bytecounter);
        u16::from_le_bytes(buf)
    }

    /// Read one byte from the binary file and interpret the result as [`u8`]
    fn read_u8<T: BufRead>(input: &mut T, bytecounter: &mut usize) -> u8 {
        let mut val: [u8; 1] = [0];
        Self::read(input, &mut val, bytecounter);

        val[0]
    }

    /// Read a string from the binary file
    ///
    /// A string is encoded as UTF-8 bytes and does not necessary terminate with
    /// a 0-byte. However, it is prefixed with the number of characers (bytes) it
    /// consists of.
    fn read_string<T: BufRead>(input: &mut T, bytecounter: &mut usize) -> String {
        let number_of_chars = Self::read_u16(input, bytecounter);
        let mut chars = vec![0 as u8; number_of_chars as usize];

        Self::read(input, &mut chars, bytecounter);

        String::from_utf8(chars).unwrap()
    }

    /// Read a type from the binary file
    ///
    /// A type is encoded as single byte, containing the enumeration value that
    /// matches this type.
    fn read_type<T: BufRead>(
        input: &mut T,
        allow_void: bool,
        bytecounter: &mut usize,
    ) -> Result<BasicType, Error> {
        let byte = Self::read_u8(input, bytecounter);
        let type_id = BasicType::try_from(byte).map_err(|err| Error::InvalidType(err))?;

        if type_id == BasicType::Void && !allow_void {
            Err(Error::InvalidType(
                "Found void type where not allowed".to_string(),
            ))
        } else {
            Ok(type_id)
        }
    }
}
