use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, spanned::Spanned, staments::statment::Block};

#[derive(Debug, Clone, PartialEq)]
pub struct WhileDecl {
    pub condition: Option<Expression>,
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
pub struct SwitchDecl {
    pub condition: Expression,
    pub cases: Vec<CaseSwitch>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CaseSwitch {
    pub if_expr: Expression,
    pub do_fn: Block,
} 

#[derive(Debug, Clone, PartialEq)]
pub enum ElseKind {
    ElseIf(Box<Spanned<IfDecl>>),
    Else(Spanned<Block>)
}











