use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, spanned::Spanned, staments::statment::Block};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WhileDecl {
    pub condition: Option<Expression>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForDecl {
    pub element: Ident,
    pub collection: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IfDecl {
    pub condition: Expression,
    pub body: Block,
    pub else_branchs: Vec<Spanned<ElseKind>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchDecl {
    pub condition: Expression,
    pub cases: Vec<CaseSwitch>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CaseSwitch {
    pub if_expr: Expression,
    pub do_fn: CaseDoKind,
} 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaseDoKind {
    Block(Block),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElseKind {
    ElseIf(Box<Spanned<IfDecl>>),
    Else(Spanned<Block>)
}











