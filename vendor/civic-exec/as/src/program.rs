//! Parser and AST definition

mod lex;
mod tokens;
pub mod types;

use std::{collections::HashMap, io::Write};

use libcivexec::{BasicType, Type, operation::{OperandType, Operation}};

use crate::program::{
        lex::Lexer, tokens::{Token, TokenValue}, types::{Function, LabelRef, Offset, ValueType, Var, VarType},
    };

/// The magic bytes of a CiviC binary file
const FILE_SIGNATURE: [u8; 7] = [b'C', b'i', b'v', b'C', b'X', 1, 1];

/// The representation of a CiviC binary program
pub struct Program {
    /// The lexer to use for parsing the assembly file
    lexer: Lexer,

    /// The list of opcodes
    op_codes: Vec<Operation<LabelRef>>,

    /// The list of exported function
    exportfuns: Vec<Function>,

    /// The list of imported function
    importfuns: Vec<Function>,

    /// The list of exported variables
    exportvars: Vec<Var>,

    /// The list of imported variables
    importvars: Vec<Var>,

    /// The list of constants
    constants: Vec<ValueType>,

    /// The list of global variables
    globals: Vec<Type>,

    /// The map for turning label names into offsets
    labels: HashMap<String, u32>,

    /// The number of bytes the binary consists of
    byte_counter: u32,

    /// The last token returned from the parser
    last_token: Option<Token>,
}

impl Program {
    /// Intitialise an empty AST with giving the source to the lexer
    pub fn new(input: String, multi_dim_arrays_allowed: bool) -> Self {
        Self {
            lexer: Lexer::new(input, multi_dim_arrays_allowed),
            op_codes: Vec::new(),
            exportfuns: Vec::new(),
            importfuns: Vec::new(),
            exportvars: Vec::new(),
            importvars: Vec::new(),
            constants: Vec::new(),
            globals: Vec::new(),
            labels: HashMap::new(),
            byte_counter: 0,
            last_token: None,
        }
    }

    /// Parse the assemby code
    pub fn parse(&mut self, allow_direct_offsets: bool) {
        let mut token = self.lexer.get_next_token();

        while *token.value() != TokenValue::EoF {
            // we are finished if we ot an EoF token
            match token.value_mut() {
                TokenValue::OpCode(op_code) => {
                    // we got an instruction code
                    self.byte_counter += 1;
                    for op in op_code.operands_mut() {
                        // parse operands
                        match op {
                            // A type argument ->  check for u8 operand
                            OperandType::ArgumentCount(None) => {
                                let token_2 = self.lexer.get_next_token();
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        *op = OperandType::ArgumentCount(Some(*int as u8));
                                        self.byte_counter += 1;
                                    }
                                    _ => panic!(
                                        "Expected token of type int, got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }

                            // L type argument -> check for u8 argument
                            OperandType::Local(None) => {
                                let token_2 = self.lexer.get_next_token();
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        *op = OperandType::Local(Some(*int as u8));
                                        self.byte_counter += 1;
                                    }
                                    _ => panic!(
                                        "Expected token of type int, got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }

                            // N type argument -> check for u8 operand
                            OperandType::LinkCount(None) => {
                                let token_2 = self.lexer.get_next_token();
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        *op = OperandType::LinkCount(Some(*int as u8));
                                        self.byte_counter += 1;
                                    }
                                    _ => panic!(
                                        "Expected token of type int, got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }

                            // C type argument -> check for u16 operand
                            OperandType::ConstantIndex(None) => {
                                let token_2 = self.lexer.get_next_token();
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        *op = OperandType::ConstantIndex(Some(*int as u16));
                                        self.byte_counter += 2;
                                    }
                                    _ => panic!(
                                        "Expected token of type int, got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }

                            // G type argument -> check for u16 operand
                            OperandType::GlobalIndex(None) => {
                                let token_2 = self.lexer.get_next_token();
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        *op = OperandType::GlobalIndex(Some(*int as u16));
                                        self.byte_counter += 2;
                                    }
                                    _ => panic!(
                                        "Expected token of type int, got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }

                            // I type argument -> check for u16 operand
                            OperandType::ImportIndex(None) => {
                                let token_2 = self.lexer.get_next_token();
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        *op = OperandType::ImportIndex(Some(*int as u16));
                                        self.byte_counter += 2;
                                    }
                                    _ => panic!(
                                        "Expected token of type int, got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }

                            // O type argument -> check for label reference
                            OperandType::Offset(None) => {
                                let token_2 = self.lexer.get_next_token();
                                // offsets are specified direct as i16 or as label (string)
                                match token_2.value() {
                                    TokenValue::Int(int) => {
                                        if !allow_direct_offsets {
                                            panic!("Direct ofsets are not allowed in current execution mode.")
                                        }
                                        let label_ref = LabelRef::new(*token_2.position(), Offset::Offset(*int as i16));
                                        *op = OperandType::Offset(Some(label_ref));
                                        self.byte_counter += 2;
                                    }
                                    TokenValue::String(str) => {
                                        let label_ref = LabelRef::new(*token_2.position(), Offset::Label(str.clone()));
                                        *op = OperandType::Offset(Some(label_ref));
                                        self.byte_counter += 2;
                                    }
                                    _ => panic!(
                                        "Expected token of type int or string (label), got {}",
                                        token_2.value().get_type()
                                    ),
                                }
                            }
                            _ => {}
                        }
                    }
                    self.op_codes.push(op_code.clone());
                }

                // Found label -> put into map with current offset
                TokenValue::Label(string) => {
                    if None != self.labels.insert(string.clone(), self.byte_counter) {
                        panic!("Label {} already exists", string);
                    }
                }

                // Found pseudo instructions -> special treatment
                TokenValue::Pseudo(string) => self.last_token = self.handle_pseudo(string),

                // Found EoF -> do nothing
                TokenValue::EoF => {}

                // Found some other token -> this is an error!
                _ => panic!("This should not occur here!"),
            }

            // if we have stored a token, set the old token as current token and
            // delete the saved one
            token = match &self.last_token {
                Some(x) => {
                    let this_token = x.clone();
                    self.last_token = None;
                    this_token
                }
                None => self.lexer.get_next_token(),
            };
        }
    }

    /// Handle pseudo instructions (given as argument)
    fn handle_pseudo(&mut self, value: &mut String) -> Option<Token> {
        match value.as_str() {
            // Imported function -> parse signature and push into the list
            "importfun" => {
                let mut func = Function::default();
                let last_token = self.parse_func_signature(&mut func);

                let ret_type = func.params_mut().remove(0);
                func.set_return_type(ret_type.basic_type());

                self.importfuns.push(func);

                return Some(last_token);
            }

            // Exported function -> parse signature and link with jump mark and
            // push it into the list
            "exportfun" => {
                let mut func = Function::default();
                let token = self.parse_func_signature(&mut func);

                let ret_type = func.params_mut().remove(0);
                func.set_return_type(ret_type.basic_type());

                match token.value() {
                    TokenValue::String(_) | TokenValue::Int(_) => {
                        let label_ref = token.into();
                        func.set_label_ref(label_ref);
                        self.exportfuns.push(func);
                    },
                    _=> // No jump mark reference found -> Error
                    panic!("Not a valid export function signature (Label reference or offset missing)")
                }

                return None;
            }

            // Imported variable -> parse name and type and push it into the list
            "importvar" => {
                let token = self.lexer.get_next_token();
                let token_value = token.value();
                if let TokenValue::String(name) = token_value {
                    // name found
                    let token_2 = self.lexer.get_next_token();
                    let token_value_2 = token_2.value();

                    match token_value_2 {
                        // parse type
                        TokenValue::DimensionalType(t, d) => {
                            let dim_type = Type::new(*t, *d);
                            let var_type = VarType::Import(dim_type);
                            let var = Var::new(name.to_string(), var_type);
                            self.importvars.push(var);
                        }
                        TokenValue::BasicType(t) => {
                            let dim_type = Type::new(*t, 0);
                            let var_type = VarType::Import(dim_type);
                            let var = Var::new(name.to_string(), var_type);
                            self.importvars.push(var);
                        }
                        _ => panic!("Not a valid importvar signature"),
                    }
                } else {
                    panic!("Not a valid importvar signature")
                }
                return None;
            }

            // Exported variable -> parse name and index in global variables table
            // and push it into the list
            "exportvar" => {
                let token = self.lexer.get_next_token();
                let token_value = token.value();
                if let TokenValue::String(name) = token_value {
                    // name found
                    let token_2 = self.lexer.get_next_token();
                    let token_value_2 = token_2.value();
                    if let TokenValue::Int(i) = token_value_2 {
                        // parse index
                        if *i < 0 {
                            panic!("Index below 0 not allowed");
                        } else {
                            let var = Var::new(name.to_string(), VarType::Export(*i as usize));
                            self.exportvars.push(var);
                        }
                    } else {
                        panic!("Not a valid exportvar signature (index missing)");
                    }
                } else {
                    panic!("Not a valid exportvar signature (name missing)");
                }
                return None;
            }

            // Global variable -> parse name and type and push it into the list
            "global" => {
                let token = self.lexer.get_next_token();
                let token_value = token.value();
                if let TokenValue::DimensionalType(t, d) = token_value {
                    let val = Type::new(*t, *d);
                    self.globals.push(val);
                } else if let TokenValue::BasicType(t) = token_value {
                    let val = Type::new(*t, 0);
                    self.globals.push(val);
                } else {
                    panic!("Not a valid global signature (type missing)");
                }
                return None;
            }

            // Constant -> parse type and value and push it into the list
            "const" => {
                let token = self.lexer.get_next_token();
                let token_value = token.value();
                if let TokenValue::BasicType(t) = token_value {
                    let token_2 = self.lexer.get_next_token();
                    let token_value_2 = token_2.value();
                    match t {
                        BasicType::Int => {
                            if let TokenValue::Int(i) = token_value_2 {
                                self.constants.push(ValueType::Int(*i));
                            }
                            return None;
                        }

                        BasicType::Float => {
                            if let TokenValue::Float(f) = token_value_2 {
                                self.constants.push(ValueType::Float(*f));
                            }
                            return None;
                        }

                        BasicType::Bool => {
                            if let TokenValue::Bool(b) = token_value_2 {
                                self.constants.push(ValueType::Bool(*b));
                            }
                            return None;
                        }

                        BasicType::Void => panic!("void type not allowed for constants"),
                    }
                } else {
                    panic!("Not a valid const signature (expected basic type)");
                }
            }
            x => panic!("Expected pseudo instruction, got {}.", x),
        }
    }

    /// Parse signature of function given as argument
    fn parse_func_signature(&mut self, fun: &mut Function) -> Token {
        // parse name
        let name = self.lexer.get_next_token();
        if let TokenValue::String(str) = name.value() {
            fun.set_name(str.replace("\"", ""));

            let mut sig_end = false;
            let mut sig_token = Token::default();

            // parse signature
            while !sig_end {
                sig_token = self.lexer.get_next_token(); // invoke lexer

                // Token is type -> add to function signature
                match *sig_token.value() {
                    TokenValue::DimensionalType(t, d) => {
                        fun.add_param(t, d);
                    }
                    TokenValue::BasicType(t) => {
                        fun.add_param(t, 0);
                    }
                    _ => {
                        // other token -> signature end found
                        sig_end = true;
                    }
                }
            }
            sig_token
        } else {
            panic!("An imported function must at least specify its name");
        }
    }

    /// Turn export labels into offsets
    pub fn resolve_export_labels(&mut self) {
        for exportfun in &mut self.exportfuns {
            let label = exportfun.label().location();
            match label {
                Offset::Label(lbl) => {
                    match self.labels.get(lbl) {
                        Some(offset) => {
                            // label found -> lookup offset
                            exportfun.set_offset(*offset);
                        }
                        None => {
                            panic!("{} is not a valid label reference", label);
                        }
                    }        
                },
                Offset::Offset(int) => exportfun.set_offset(*int as u32),
            }
        }
    }

    /// Turn label reference offset
    ///
    /// Traverse through all instructions and simulatanously increase the offset
    /// counter. If we found a jump mark reference, look up its offset. Calculate
    /// the actual offset based on the current pc (the current offset counter).
    pub fn resolve_label_refs(&mut self) {
        let mut offset: usize = 0;
        for op in &mut self.op_codes {
            offset += 1;
            for arg in op.operands_mut() {
                match arg {
                    OperandType::None => {}
                    OperandType::ArgumentCount(_) => offset += 1,
                    OperandType::Local(_) => offset += 1,
                    OperandType::LinkCount(_) => offset += 1,
                    OperandType::ConstantIndex(_) => offset += 2,
                    OperandType::GlobalIndex(_) => offset += 2,
                    OperandType::ImportIndex(_) => offset += 2,
                    OperandType::Offset(label_ref) => {
                        if let Some(reference) = label_ref.as_mut() {
                            if let Offset::Label(label) = reference.location() {
                                if let Some(label_position) = self.labels.get(label) {
                                    let new_offset: i16 = *label_position as i16 - offset as i16;
                                    *reference = LabelRef::new(*reference.token_location(), Offset::Offset(new_offset));
                                    offset += 2;
                                } else {
                                    panic!("Got reference to undefined label {} ({} - {})", label, reference.token_location()[0], reference.token_location()[1])
                                }
                            }
                        } else {
                            panic!("Got empty reference")
                        }
                    }
                }
            }
        }
    }

    /// Wirte bytecode into specified buffer
    pub fn write_byte_code<T>(self, out: &mut T) -> std::io::Result<()>
    where
        T: Write,
    {
        //let mut counter: usize = 0;
        _ = out.write(&FILE_SIGNATURE)?;
        _ = out.write(&(self.byte_counter).to_le_bytes())?;
        for op in &self.op_codes {
            _ = out.write(&[op.instruction().into()])?;
            for operand in op.operands() {
                match operand {
                    OperandType::ArgumentCount(Some(val)) => _ = out.write(&[*val])?,
                    OperandType::Local(Some(val)) => _ = out.write(&[*val])?,
                    OperandType::LinkCount(Some(val)) => _ = out.write(&[*val])?,
                    OperandType::ConstantIndex(Some(val)) => _ = out.write(&val.to_le_bytes())?,
                    OperandType::GlobalIndex(Some(val)) => _ = out.write(&val.to_le_bytes())?,
                    OperandType::ImportIndex(Some(val)) => _ = out.write(&val.to_le_bytes())?,
                    OperandType::Offset(Some(label_ref)) => {
                        if let Offset::Offset(int) = label_ref.location() {
                            _ = out.write(&int.to_le_bytes())?
                        }
                        else {
                            panic!("Got reference {}, that is not an offset!", label_ref)
                        }
                    }
                    OperandType::None => {}
                    _ => panic!("This should never occur!!!"),
                }
            }
        }

        // Write exported functions
        _ = out.write(&(self.exportfuns.len() as u32).to_le_bytes());
        for fun in &self.exportfuns {
            _ = self.write_string(out, fun.name())?;

            let ret_type: u8 = fun.return_type().into();
            _ = out.write(&[ret_type])?;

            let num_args = fun.params().len() as u16;
            _ = out.write(&num_args.to_le_bytes())?;

            for param in fun.params() {
                _ = out.write(&[param.basic_type().into()]);
                _ = out.write(&[param.dimensions()]);
            }
            _ = out.write(&(fun.offset() as u32).to_le_bytes())?;
        }

        // Write imported functions
        _ = out.write(&(self.importfuns.len() as u32).to_le_bytes())?;
        for fun in &self.importfuns {
            _ = self.write_string(out, fun.name())?;
            _ = out.write(&[fun.return_type().into()])?;
            _ = out.write(&(fun.params().len() as u16).to_le_bytes())?;
            for param in fun.params() {
                _ = out.write(&[param.basic_type().into()])?;
                _ = out.write(&[param.dimensions()]);
            }
        }

        // Write exported vars
        _ = out.write(&(self.exportvars.len() as u32).to_le_bytes())?;
        for var in &self.exportvars {
            _ = self.write_string(out, var.name());
            if let VarType::Export(index) = var.var_type() {
                _ = out.write(&(*index as u16).to_le_bytes());
            } else {
                panic!("Imported varialbe found in export variables");
            }
        }

        // Write imported vars
        _ = out.write(&(self.importvars.len() as u32).to_le_bytes())?;
        for var in &self.importvars {
            _ = self.write_string(out, var.name())?;
            if let VarType::Import(x) = var.var_type() {
                _ = out.write(&[x.basic_type().into()])?;
                _ = out.write(&[x.dimensions()])?;
            }
        }

        // Write the globals
        _ = out.write(&(self.globals.len() as u32).to_le_bytes())?;
        for global in &self.globals {
            _ = out.write(&[global.basic_type().into()])?;
            _ = out.write(&[global.dimensions()])?;
        }

        // Write constants
        _ = out.write(&(self.constants.len() as u32).to_le_bytes())?;
        for constant in &self.constants {
            match constant {
                ValueType::Int(i) => {
                    _ = out.write(&[BasicType::Int.into()])?;
                    _ = out.write(&i.to_le_bytes())?;
                }
                ValueType::Float(f) => {
                    _ = out.write(&[BasicType::Float.into()])?;
                    _ = out.write(&f.to_le_bytes())?;
                }
                ValueType::Bool(b) => {
                    _ = out.write(&[BasicType::Bool.into()])?;
                    let val: u8 = match b {
                        true => 1,
                        false => 0,
                    };
                    _ = out.write(&val.to_le_bytes())?;
                }
            }
        }

        /* This makes no sense. We check, if all bytes from the _instruction_ bytecode are written out.
         * But we do not check for the other bytes (import/exprort functions/variables, constants, globals).
         * However, they are equally important for correctnes.
         * As we no longer calculate the size of the bytecode for the instructions, we omit this check until we
         * implement such a check for _all_ parts of the binary.
         */
        /*if counter == self.instr_size {
            Ok(())
        }
        else {
            Err(Error::other("Not all bytes are written!"))
        }*/

        Ok(())
    }

    /// Write a string into the specified buffer
    ///
    /// A string is wirtten as its UTF-8 bytes prefixed by it length as 16 bit
    /// unsigned integer
    fn write_string<T>(&self, out: &mut T, string: &String) -> std::io::Result<()>
    where
        T: Write,
    {
        let len = string.len() as u16;
        let bytes = string.as_bytes();
        _ = out.write(&len.to_le_bytes())?;
        _ = out.write(bytes)?;

        Ok(())
    }
}
