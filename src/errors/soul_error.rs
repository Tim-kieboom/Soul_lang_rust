use std::{io::{BufRead, BufReader, Read}, result};

use serde::{Deserialize, Serialize};

use crate::utils::show_diff::{generate_highlighted_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoulSpan {
    pub line_number: usize,
    ///for multiline span
    pub end_line_number: Option<usize>,
    ///lineoffset from last line
    pub line_offset: usize,
    ///length from from last line line_offset to end
    pub len: usize,
}
impl SoulSpan {
    pub fn new(line_number: usize, line_offset: usize, len: usize) -> Self {
        Self { line_number, end_line_number: None, line_offset, len }
    }

    pub fn eq(&self, other: &Self) -> bool {
        self.line_number == other.line_number && self.line_offset == other.line_offset && self.len == other.len
    }

    pub fn combine(&self, other: &Self) -> Self {
        if self.line_number != other.line_number {
            let line_number = self.line_number.min(other.line_number);
            let end_line_number = self.line_number.max(other.line_number);
            
            let mut this = if self.line_number > other.line_number {
                self.clone()
            }
            else {
                other.clone()
            };

            this.line_number = line_number;
            this.end_line_number = Some(end_line_number);
            
            return this;
        }

        let line_number = self.line_number;
        let line_offset = self.line_offset.min(other.line_offset);
        let max_offset = self.line_offset.max(other.line_offset);
        let len = if self.line_offset == max_offset {
            max_offset + self.len - line_offset
        }
        else {
            max_offset + other.len - line_offset
        };

        Self{line_number, end_line_number: None, line_offset, len}
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

    fn get_message_stack(&self) -> Vec<String> {
        self.spans.iter()
            .zip(self.msgs.iter())
            .rev()
            .map(|(span, msg)| format!("at {}:{}; !!error!! {}", span.line_number, span.line_offset, msg))
            .collect::<Vec<_>>()
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
    pub fn to_err_message(&self) -> Vec<String> {
        self.get_message_stack()
    }

    pub fn to_highlighed_message<R: Read>(&self, reader: BufReader<R>) -> String {

        //an error that is not in any line number in the source code
        const NON_SPANABLE_ERROR: usize = 0;
        
        let first_span = self.spans[0];
        if first_span.line_number == NON_SPANABLE_ERROR {
            return String::new();
        } 
        
        let start = first_span.line_number;
        let end = if let Some(end) = first_span.end_line_number {
            end
        }
        else {
            first_span.line_number
        };

        let lines: Vec<String> = reader.lines()
            .enumerate()
            .filter_map(|(idx, line)| {
                if idx+1 >= start && idx < end {
                    line.ok()
                } else {
                    None
                }
            })
            .collect();


        generate_highlighted_string(first_span.line_number, lines.as_slice(), &[(start, first_span.line_offset, first_span.line_offset+first_span.len)])
    }

    #[cfg(feature = "throw_result")]
    pub fn to_err_message(&self) -> Vec<String> {
        let mut stack = vec![self.backtrace.clone(), "\n", "\n"];
        stack.extend_from_slice(self.get_message_stack());
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
    SoulError::new(kind, span, msg.into(), std::backtrace::Backtrace::capture().to_string())
}

#[cfg(not(feature = "throw_result"))]
pub fn new_soul_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S) -> SoulError {
    SoulError::new(kind, span, msg.into())
}











