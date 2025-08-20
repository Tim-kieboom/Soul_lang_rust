use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Literal {
    // basic
    Int(i64),
    Uint(u64),
    Float(OrderedFloat<f64>),
    Bool(bool),
    Char(char),
    Str(String),

    // complex
    Array{ty: LiteralType, values: Vec<Literal>},
    Tuple{values: Vec<Literal>},
    NamedTuple{values: BTreeMap<Ident, Literal>},

    // a type of Literal variable for complex literals and refing basic literals 
    ProgramMemmory(Ident, LiteralType),
}

#[derive(Debug, Clone, Hash, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub enum LiteralType {
    Int,
    Uint,
    Float,
    Bool,
    Char,
    Str,
    Array(Box<LiteralType>),
    Tuple(Vec<LiteralType>),
    NamedTuple(BTreeMap<Ident, LiteralType>),
    ProgramMemmory(Box<LiteralType>),
}








