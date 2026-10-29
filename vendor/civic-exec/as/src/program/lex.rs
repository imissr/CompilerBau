//! The lexer
//!
//! The lexer reads the input character by character until a delimiter was found.
//! Then it trys to generate a token out of the crafted string by simple pattern matching.

use libcivexec::{BasicType, operation::{Instr, Operation}};

use crate::{
    Location,
    program::tokens::{Token, TokenValue}
};

/// A simple assembly lexer
pub struct Lexer {
    /// The program to lex
    input: String,

    /// The current location
    location: Location,

    /// Ignore all following characters?
    ///
    /// This is set for ignoring comments on lexing
    ignore: bool,

    /// Whether multi-dimensional arrays are allowed or not
    multi_dim_arrays: bool
}

impl Lexer {
    /// Get the next token from the current location
    ///
    /// This function reads all characters of the input. It begins at the current
    /// location and reads until a delimiter character is reached. The characters
    /// are collected in a string. This string is converted into a [`Token`].
    pub fn get_next_token(&mut self) -> Token {
        let start_loc = Location {
            column: self.location.column,
            line: self.location.line,
        };
        let mut token = String::new();

        while let Some(char) = self.input.chars().nth(0) {
            assert_eq!(self.input.remove(0), char);
            self.location.column += 1;
            match char {
                ' ' => {
                    // Got a space, token is ready to be parsed!
                    if token.is_empty() {
                        continue;
                    }
                    // If the string is empty, do not convert it.
                    else {
                        let end_loc = Location {
                            column: self.location.column,
                            line: self.location.line,
                        };
                        return self.get_token_from_str(token, [start_loc, end_loc]);
                    }
                }
                '\n' => {
                    // Got a new line space, token is ready to be parsed!
                    self.ignore = false; // No longer in comment environment
                    let end_loc = Location {
                        column: self.location.column,
                        line: self.location.line,
                    };
                    self.location.line += 1; // go to next line
                    self.location.column = -1; // go to first character
                    if token.is_empty() {
                        continue;
                    }
                    // If the string is empty, do not convert it.
                    else {
                        return self.get_token_from_str(token, [start_loc, end_loc]);
                    }
                }
                ';' => {
                    // Comment -> Token is ready to be parsed; ignore all following until newline
                    let end_loc = Location {
                        column: self.location.column - 1,
                        line: self.location.line,
                    };
                    self.ignore = true; // Ignore all following tokens
                    if token.is_empty() {
                        continue;
                    }
                    // If the string is empty, do not convert it.
                    else {
                        return self.get_token_from_str(token, [start_loc, end_loc]);
                    }
                }
                x => {
                    if !self.ignore {
                        token.push(x);
                    }
                } // If we are not in ignore mode, push the character at the end of the string
            }
        }
        Token::new(TokenValue::EoF, [start_loc; 2])
    }

    /// Helper function for lexing. Converts a string into a [`Token`]
    fn get_token_from_str(&self, mut str: String, position: [Location; 2]) -> Token {
        let instr = Instr::try_from(str.as_str());
        match instr {
            // Have we found an [`Operation`]?
            Ok(x) => { // Yes -> Convert into Operation token
                let operation = Operation::from(x);
                Token::new(TokenValue::OpCode(operation), position)
            }, 
            Err(_msg) => {
                let token_value;
                // String ends with colon -> found a label
                if str.ends_with(':') {
                    _ = str.pop();
                    token_value = TokenValue::Label(str);
                }
                // String starts with dot -> Found a pseudo instruction
                else if str.starts_with('.') {
                    token_value = TokenValue::Pseudo(str[1..].to_string());
                }
                // Found type marker for integer
                else if str.starts_with("int") {
                    let dims = Self::get_dims_from_str(str[3..].to_string(), self.multi_dim_arrays);
                    token_value = match dims > 0 {
                        // Array or scalar?
                        true => TokenValue::DimensionalType(BasicType::Int, dims),
                        false => TokenValue::BasicType(BasicType::Int),
                    };
                }
                // Found type marker for floating point number
                else if str.starts_with("float") {
                    let dims = Self::get_dims_from_str(str[5..].to_string(), self.multi_dim_arrays);
                    token_value = match dims > 0 {
                        // Array or scalar?
                        true => TokenValue::DimensionalType(BasicType::Float, dims),
                        false => TokenValue::BasicType(BasicType::Float),
                    };
                }
                // Found type marker for boolean
                else if str.starts_with("bool") {
                    let dims = Self::get_dims_from_str(str[4..].to_string(), self.multi_dim_arrays);
                    token_value = match dims > 0 {
                        // Array or scalar?
                        true => TokenValue::DimensionalType(BasicType::Bool, dims),
                        false => TokenValue::BasicType(BasicType::Bool),
                    };
                }
                // Found type marker for unit type
                else if str == "void" {
                    token_value = TokenValue::BasicType(BasicType::Void);
                }
                // Found `true` literal -> Convert into value token
                else if str == "true" {
                    token_value = TokenValue::Bool(true)
                }
                // Found `false` literal -> Convert into value token
                else if str == "false" {
                    token_value = TokenValue::Bool(false)
                }
                // Found integer -> Convert into value token
                else if let Ok(int) = str.parse::<i64>() {
                    token_value = TokenValue::Int(int)
                }
                // Found floating point number -> Convert into value token
                else if let Ok(float) = str.parse::<f64>() {
                    token_value = TokenValue::Float(float)
                }
                // Found arbitrary string
                else {
                    token_value = TokenValue::String(str);
                }

                // Return token consisting of value and position
                Token::new(token_value, position)
            }
        }
    }

    /// Check, how many dimensions an array has.
    ///
    /// A dimensional (or array) expression consists of starting and closing
    /// square brackets and a number of comma. Each comma introduces another
    /// dimension.
    ///
    /// Panics if the closing bracket is missing.
    fn get_dims_from_str(str: String, multi_dims_allowed: bool) -> u8 {
        let mut dims: u8 = 0;
        if let Some(index) = str.find('[') {
            let dim_expr = &str[index..];
            let mut chars = dim_expr.chars();
            let end = dim_expr.len() - 1;
            for i in 1..end {
                if Some(',') == chars.nth(i) {
                    dims += 1
                } else {
                    panic!("Not a valid array expression")
                }
            }
            if Some(']') != chars.nth(end) {
                panic!("Not a valid array expression")
            }
            dims += 1; // iff we have a valid array expression, we have at least one dimension

            if !multi_dims_allowed && dims > 1 {
                panic!("Multi-dimensional arrays are not allowed!")
            }

        }
        dims
    }

    /// Create a new (and empty) lexer
    pub fn new(input: String, multi_dims_allowed: bool) -> Self {
        Self {
            input,
            location: Location { line: 0, column: 0 },
            ignore: false,
            multi_dim_arrays: multi_dims_allowed
        }
    }
}
