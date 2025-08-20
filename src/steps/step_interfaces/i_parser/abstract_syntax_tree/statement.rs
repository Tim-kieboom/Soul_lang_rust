use serde::{Deserialize, Serialize};
use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::{abstract_syntax_tree::{enum_like::{Enum, TypeEnum, Union}, expression::{Expression, ExpressionKind}, function::Function, object::{Class, Struct, Trait, TraitSignature}, soul_type::soul_type::{Modifier, SoulType}, spanned::Spanned}, scope_builder::Variable}};


pub type Statement = Spanned<StatementKind>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementKind {
    Expression(Expression),

    Variable(Variable),
    Assignment(Assignment),

    Function(Function),

    Class(Class),
    Struct(Struct),
    Trait(Trait),

    Enum(Enum),
    Union(Union),
    TypeEnum(TypeEnum),

    Implement(Implement),

    CloseBlock,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Assignment {
    pub variable: Expression,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Implement {
    pub impl_trait: Option<TraitSignature>,
    pub ty: SoulType,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub ruleset: Modifier,
    pub statments: Vec<Statement>,
}

impl Statement {
    pub fn new_expression(kind: ExpressionKind, span: SoulSpan) -> Self {
        Statement::new(StatementKind::Expression(Expression::new(kind, span)), span)
    }
}

impl Block {
    pub fn new() -> Self {
        Self{ruleset: Modifier::Default, statments: vec![]}
    }

    pub fn from_ruleset(ruleset: Modifier) -> Self {
        Self{ruleset, statments: vec![]}
    }
}










