use serde::{Deserialize, Serialize};
use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{enum_like::{Enum, TypeEnum, Union}, expression::{Expression, ExpressionKind, VariableName}, function::Function, object::{Class, Struct, Trait, TraitSignature}, soul_type::soul_type::{Modifier, SoulType}, spanned::{SpannedAttribute}}};


pub type Statement = SpannedAttribute<StatementKind>;

pub const STATMENT_END_TOKENS: &[&str] = &["\n", "}"];

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum StatementKind {
    Expression(Expression),

    Variable(VariableName),
    Assignment(Assignment),
    
    Function(Function),
    Implement(Implement),

    Class(Class),
    Struct(Struct),
    Trait(Trait),
    
    Enum(Enum),
    Union(Union),
    TypeEnum(TypeEnum),

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
    pub fn new_expression(expr: ExpressionKind, span: SoulSpan) -> Self {
        Statement::new(StatementKind::Expression(Expression::new(expr, span)), span)
    }

    pub fn from_expression(expr: Expression) -> Self {
        Statement::new(StatementKind::Expression(Expression::new(expr.node, expr.span)), expr.span)
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

































