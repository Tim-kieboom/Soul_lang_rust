use serde::{Deserialize, Serialize};

use crate::errors::soul_error::{new_soul_error, pass_soul_error, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserResponse {
    pub tree: AbstractSyntacTree,
    pub scopes: ScopeBuilder,
}

pub trait FromTokenStream<T> {
    ///Result::Err means that stream is of type but is invalid
    ///Result::OK(None) means that stream is not of type
    fn try_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<T>, SoulError>;
    ///Result::Err means that stream is invalid
    fn from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<T, SoulError>;
}

#[derive(Debug, Clone)]
///mean as helper for impl of FromTokenStream
pub struct FromStreamError {
    pub err: SoulError,
    pub kind: FromStreamErrorKind,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum FromStreamErrorKind {
    IsOfType,
    IsNotOfType,
}

pub fn new_from_stream_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S, err_kind: FromStreamErrorKind) -> FromStreamError {
    FromStreamError{err: new_soul_error(kind, span, msg), kind: err_kind}
}

pub fn pass_from_stream_error<S: Into<String>>(kind: SoulErrorKind, span: SoulSpan, msg: S, child: SoulError, err_kind: FromStreamErrorKind) -> FromStreamError {
    FromStreamError{err: pass_soul_error(kind, span, msg, child), kind: err_kind}
}















