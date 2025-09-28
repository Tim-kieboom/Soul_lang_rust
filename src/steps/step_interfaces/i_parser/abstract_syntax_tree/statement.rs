use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::{abstract_syntax_tree::{enum_like::{Enum, TypeEnum, Union}, expression::{Expression, ExpressionKind, VariableName}, function::Function, object::{Class, Struct, Trait}, soul_type::soul_type::{Modifier, SoulType}, spanned::SpannedAttribute}, scope_builder::ScopeId}};


pub type Statement = SpannedAttribute<StatementKind>;

pub const STATMENT_END_TOKENS: &[&str] = &["\n", "}"];

/// The different kinds of statements that can appear in the language.
///
/// Each variant corresponds to a syntactic construct, ranging from expressions
/// to type definitions and control structures.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum StatementKind {
    /// A standalone expression.
    Expression(Expression),

    /// A variable declaration.
    Variable(VariableName),
    /// An assignment to an existing variable.
    Assignment(Assignment),
    
    /// A function declaration (with body block).
    Function(Function),
    /// A scoped `use` block (soul version of rusts 'impl' with optional trait implementation).
    UseBlock(UseBlock),

    /// A class declaration.
    Class(Class),
    /// A struct declaration.
    Struct(Struct),
    /// A trait declaration.
    Trait(Trait),
    
    /// An enum declaration (c like enum).
    Enum(Enum),
    /// A union declaration (rust like enum).
    Union(Union),
    /// A type-enum declaration (a type that is the trait of all the overlapping traits of the types defined).
    TypeEnum(TypeEnum),

    /// Marker for closing a block (used during parsing).
    CloseBlock,
}

/// An assignment statement, e.g., `x = y + 1;`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Assignment {
    pub variable: Expression,
    pub value: Expression,
}

/// A `use` block, introducing a scope with optional trait implementation.
///
/// Example (hypothetical syntax):
/// ```soul
/// use MyType {
///     methode() { }
/// }
/// 
/// use MyType impl MyTrait {
///     traitMethode() { }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct UseBlock {
    pub impl_trait: Option<SoulType>,
    pub ty: SoulType,
    pub block: Block,
}

/// A block of statements grouped under a scope.
///
/// Blocks may represent function bodies, type bodies (class/struct/trait),
/// or the root of the program.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Block {
    pub ruleset: Modifier,
    pub statments: Vec<Statement>,
    pub scope_id: ScopeId,
}

impl Statement {
    pub fn new_expression(expr: ExpressionKind, span: SoulSpan) -> Self {
        Statement::new(StatementKind::Expression(Expression::new(expr, span)), span)
    }

    pub fn from_expression(expr: Expression) -> Self {
        Statement::new(StatementKind::Expression(Expression::new(expr.node, expr.span)), expr.span)
    }
}

impl StatementKind {
    pub fn get_scope_id(&self) -> Option<ScopeId> {
        Some(match self {
            StatementKind::Enum(enum_) => enum_.scope_id,
            StatementKind::Trait(trait_) => trait_.scope_id,
            StatementKind::Class(class) => class.scope_id,
            StatementKind::Union(union_) => union_.scope_id,
            StatementKind::Struct(struct_) => struct_.scope_id,
            StatementKind::Function(function) => function.block.scope_id,
            StatementKind::UseBlock(use_block) => use_block.block.scope_id,
            
            _ => return None, 
        })
    }
}

impl Block {
    pub fn new(scope_id: ScopeId) -> Self {
        Self{ruleset: Modifier::Default, statments: vec![], scope_id}
    }

    pub fn from_ruleset(scope_id: ScopeId, ruleset: Modifier) -> Self {
        Self{ruleset, statments: vec![], scope_id}
    }
}

































