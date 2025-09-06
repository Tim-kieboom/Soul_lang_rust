use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::errors::soul_error::SoulSpan;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub struct Spanned<T> {
    pub node: T,
    pub span: SoulSpan,
}

impl<T> Spanned<T> {
    pub fn new(inner: T, span: SoulSpan) -> Self {
        Self {node: inner, span}
    } 
}

impl<T> Default for Spanned<T>
where 
    T: Default
{
    fn default() -> Self {
        Self { node: Default::default(), span: SoulSpan::new(0,0,0) }
    }
}

pub type Attribute = u8/*dummy*/;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct SpannedAttribute<T> {
    pub node: T,
    pub span: SoulSpan,
    pub attribute: Option<Attribute>,
}

impl<T> SpannedAttribute<T> {
    pub fn new(node: T, span: SoulSpan) -> Self {
        Self{node, span, attribute: None}
    }
}

impl<T> Default for SpannedAttribute<T>
where 
    T: Default
{
    fn default() -> Self {
        Self{node: Default::default(), span: SoulSpan::new(0,0,0), attribute: None}
    }
}














