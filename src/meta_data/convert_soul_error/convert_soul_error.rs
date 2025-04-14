use std::io::{Error, ErrorKind};
use crate::tokenizer::token::Token;

pub fn new_soul_error(token: &Token, msg: &str) -> Error {
    Error::new(ErrorKind::InvalidData, format!("at{}:{}; !!error!! {}", token.line_number, token.line_offset, msg))
}