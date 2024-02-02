use core::fmt;
use std::error::Error;

use crate::{token::Token, value::Value};


#[derive(Debug)]
pub struct ParserError{
    pub token: Token,
    pub message: String,
}

impl Error for ParserError{}

impl fmt::Display for ParserError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "[line {}] [token : {}] Error: {}", self.token.line, self.token, self.message)
    }
}


#[derive(Debug)]
pub enum RuntimeError {
    Error { token: Token, message: String },
    Return(Value),
}

impl Error for RuntimeError {}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RuntimeError::Error { token, message } => {
                write!(f, "[line {}] Error: {}", token.line, message)
            }
            RuntimeError::Return(value) => {
                write!(f, "Return value: {}", value)
            }
        }
    }
}