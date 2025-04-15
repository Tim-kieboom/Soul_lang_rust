use std::io::{Error, ErrorKind};
use crate::tokenizer::token::Token;

#[cfg(feature = "throw_result")]
pub fn throw_result() {
    panic!("");
} 
#[cfg(not(feature = "throw_result"))]
pub fn throw_result() {}

pub fn new_soul_error(token: &Token, msg: &str) -> Error {
    throw_result();
    Error::new(ErrorKind::InvalidData, format!("at {}:{}; !!error!! {}", token.line_number, token.line_offset, msg))
}