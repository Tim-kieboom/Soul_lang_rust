use std::result;
use crate::tokenizer::token::Token;

pub type Result<T> = result::Result<T, SoulError>;

#[derive(Debug, Clone)]
pub struct SoulSpan {
    pub line_number: usize,
    pub line_offset: usize,
}
impl SoulSpan {
    pub fn from_token(token: &Token) -> Self {
        Self { line_number: token.line_number, line_offset: token.line_offset }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.line_number == other.line_number && self.line_offset == other.line_offset
    }
}
impl PartialEq for SoulSpan {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct SoulError {
    spans: Vec<SoulSpan>,
    msgs: Vec<String>,

    #[cfg(feature = "throw_result")]
    backtrace: String,
}

impl SoulError {
    #[cfg(not(feature = "throw_result"))]
    fn new(span: SoulSpan, msg: String) -> Self {
        Self { spans: Vec::from([span]), msgs: Vec::from([msg]) }
    }


    #[cfg(feature = "throw_result")]
    fn new(span: SoulSpan, msg: String, backtrace: String) -> Self {
        Self { spans: Vec::from([span]), msgs: Vec::from([msg]), backtrace }
    }

    fn get_message(&self) -> String {
        self.spans.iter()
            .zip(self.msgs.iter())
            .rev()
            .map(|(span, msg)| format!("at {}:{}; !!error!! {}", span.line_number, span.line_offset, msg))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn insert(mut self, span: SoulSpan, msg: String) -> Self {
        self.spans.push(span);
        self.msgs.push(msg);
        self
    }

    #[cfg(not(feature = "throw_result"))]
    pub fn to_err_message(&self) -> String {
        self.get_message()
    }

    #[cfg(feature = "throw_result")]
    pub fn to_err_message(&self) -> String {
        format!("{}\n\n{}", self.backtrace, self.get_message())
    }

    #[cfg(feature = "throw_result")]
    pub fn consume_backtrace(self) -> String {
        self.backtrace
    }
}

#[cfg(feature = "throw_result")]
pub fn pass_soul_error(token: &Token, msg: &str, child: SoulError) -> SoulError {
    child.insert(SoulSpan::from_token(token), msg.to_string())
}

#[cfg(not(feature = "throw_result"))]
pub fn pass_soul_error(token: &Token, msg: &str, child: SoulError) -> SoulError {
    child.insert(SoulSpan::from_token(token), msg.to_string())
}

#[cfg(feature = "throw_result")]
pub fn new_soul_error(token: &Token, msg: &str) -> SoulError {
    use std::backtrace::Backtrace;
    SoulError::new(SoulSpan::from_token(token), msg.to_string(), std::backtrace::Backtrace::capture().to_string())
}

#[cfg(not(feature = "throw_result"))]
pub fn new_soul_error(token: &Token, msg: &str) -> SoulError {
    SoulError::new(SoulSpan::from_token(token), msg.to_string())
}