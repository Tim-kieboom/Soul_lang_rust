use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, staments::statment::Block};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileDecl {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ForDecl {
    pub element: Ident,
    pub collection: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfDecl {
    pub condition: Expression,
    pub body: Block,
    pub else_branchs: Vec<ElseKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElseKind {
    ElseIf(Box<IfDecl>),
    Else(Block)
}











