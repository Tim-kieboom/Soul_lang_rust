use std::result;
use crate::tokenizer::token::Token;

pub type Result<T> = result::Result<T, SoulError>;

#[derive(Debug)]
pub struct ErrorSpan {
    pub line_number: usize,
    pub line_offset: usize,
}
impl ErrorSpan {
    pub fn from_token(token: &Token) -> Self {
        Self { line_number: token.line_number, line_offset: token.line_offset }
    }
}

#[derive(Debug)]
pub struct SoulError {
    span: ErrorSpan,
    msg: String,

    #[cfg(feature = "throw_result")]
    backtrace: String,
}

impl SoulError {
    #[cfg(not(feature = "throw_result"))]
    fn new(span: ErrorSpan, msg: String) -> Self {
        Self { span, msg }
    }


    #[cfg(feature = "throw_result")]
    fn new(span: ErrorSpan, msg: String, backtrace: String) -> Self {
        Self { span, msg, backtrace }
    }

    fn get_message(&self) -> String {
        format!("at {}:{}; !!error!! {}", self.span.line_number, self.span.line_offset, self.msg)
    }

    #[cfg(not(feature = "throw_result"))]
    pub fn to_string(&self) -> String {
        self.get_message()
    }

    #[cfg(feature = "throw_result")]
    pub fn to_string(&self) -> String {
        format!("{}\n\n{}", self.backtrace, self.get_message())
    }
}

#[cfg(feature = "throw_result")]
pub fn pass_soul_error(token: &Token, msg: &str, child: &SoulError) -> SoulError {
    use std::backtrace::Backtrace;
    SoulError::new(ErrorSpan::from_token(token), msg.to_string(), format!("{}", std::backtrace::Backtrace::capture()))
}

#[cfg(not(feature = "throw_result"))]
pub fn pass_soul_error(token: &Token, msg: &str, child: &SoulError) -> SoulError {
    SoulError::new(ErrorSpan::from_token(token), format!("{}\n{}", msg, child.get_message()))
}

#[cfg(feature = "throw_result")]
pub fn new_soul_error(token: &Token, msg: &str) -> SoulError {
    use std::backtrace::Backtrace;
    SoulError::new(ErrorSpan::from_token(token), msg.to_string(), format!("{}", std::backtrace::Backtrace::capture()))
}

#[cfg(not(feature = "throw_result"))]
pub fn new_soul_error(token: &Token, msg: &str) -> SoulError {
    SoulError::new(ErrorSpan::from_token(token), msg.to_string())
}