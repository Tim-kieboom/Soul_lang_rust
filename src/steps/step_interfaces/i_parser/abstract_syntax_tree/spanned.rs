use serde::{Deserialize, Serialize};
use crate::errors::soul_error::SoulSpan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Spanned<T> {
    pub node: T,
    pub span: SoulSpan,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: SoulSpan) -> Self {
        Self {node: inner, span}
    } 
}


