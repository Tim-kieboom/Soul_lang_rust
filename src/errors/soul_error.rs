use std::{io::{BufRead, BufReader, Read}, result};

use crate::utils::show_diff::{generate_highlighted_string, show_str_diff};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoulErrorKind {
    NoKind, // no kind selected

    InternalError,

    ArgError, // error with program args
    ReaderError, // e.g. could not read line

    UnterminatedStringLiteral, // e.g., string not closed
    InvalidEscapeSequence, // e.g., "\q" in a string
    EndingWithSemicolon, // if line ends with ';'
    UnmatchedParenthesis, // e.g., "(" without ")"
    
    WrongType,

    UnexpectedToken, // e.g., found ";" but expected "\n"
    
    NotFoundInScope,

    InvalidStringFormat, // if f"..." has incorrect argument
    InvalidInContext,
    InvalidName,
    InvalidType,

    UnexpectedEnd,
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
pub struct SoulSpan {
    pub line_number: usize,
    pub line_offset: usize,
    pub len: usize,
}
impl SoulSpan {
    pub fn new(line_number: usize, line_offset: usize, len: usize) -> Self {
        Self { line_number, line_offset, len }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.line_number == other.line_number && self.line_offset == other.line_offset && self.len == other.len
    }
}

pub type Result<T> = result::Result<T, SoulError>;

#[derive(Debug)]
pub struct SoulError {
    kinds: Vec<SoulErrorKind>,
    spans: Vec<SoulSpan>,
    msgs: Vec<String>,

    #[cfg(feature = "throw_result")]
    backtrace: String,
}

impl SoulError {
    #[cfg(not(feature = "throw_result"))]
    fn new(kind: SoulErrorKind, span: SoulSpan, msg: String) -> Self {
        Self { kinds: vec![kind], spans: Vec::from([span]), msgs: Vec::from([msg]) }
    }


    #[cfg(feature = "throw_result")]
    fn new(kind: SoulErrorKind, span: SoulSpan, msg: String, backtrace: String) -> Self {
        Self { kinds: vec![kind], spans: Vec::from([span]), msgs: Vec::from([msg]), backtrace }
    }

    fn get_message(&self) -> String {
        self.spans.iter()
            .zip(self.msgs.iter())
            .rev()
            .map(|(span, msg)| format!("at {}:{}; !!error!! {}", span.line_number, span.line_offset, msg))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn get_kinds(&self) -> &Vec<SoulErrorKind> {
        &self.kinds
    }    
    
    pub fn get_last_kind(&self) -> SoulErrorKind {
        self.kinds[self.kinds.len()-1].clone()
    }

    fn insert(mut self, kind: SoulErrorKind, span: SoulSpan, msg: String) -> Self {
        self.spans.push(span);
        self.msgs.push(msg);
        self.kinds.push(kind);
        self
    }

    #[cfg(not(feature = "throw_result"))]
    pub fn to_err_message(&self) -> String {
        self.get_message()
    }

    pub fn to_highlighed_message<R: Read>(&self, reader: BufReader<R>) -> String {
        let first_span = self.spans[0];
        let line = reader.lines().nth(first_span.line_number-1).expect("soulspan linenumber not found in reader").expect("soulspan linenumber not found in reader");
        generate_highlighted_string(&line, &[(first_span.line_offset, first_span.line_offset+first_span.len)])
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
pub fn pass_soul_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S, child: SoulError) -> SoulError {
    child.insert(kind, span, msg.into())
}

#[cfg(not(feature = "throw_result"))]
pub fn pass_soul_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S, child: SoulError) -> SoulError {
    child.insert(kind, span, msg.into())
}

#[cfg(feature = "throw_result")]
pub fn new_soul_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S) -> SoulError {
    use std::backtrace::Backtrace;
    SoulError::new(kind, span, msg.into(), std::backtrace::Backtrace::capture().to_string())
}

#[cfg(not(feature = "throw_result"))]
pub fn new_soul_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S) -> SoulError {
    SoulError::new(kind, span, msg.into())
}











