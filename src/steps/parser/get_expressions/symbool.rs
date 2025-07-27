use crate::{errors::soul_error::SoulSpan, steps::{step_interfaces::i_parser::abstract_syntax_tree::{expression::{BinOpKind, OperatorKind, UnaryOpKind}, spanned::Spanned}}};

pub const ROUND_BRACKET_OPEN: SymboolKind = SymboolKind::Parenthesis(Parenthesis::RoundOpen);
pub const ROUND_BRACKET_CLOSED: SymboolKind = SymboolKind::Parenthesis(Parenthesis::RoundClosed);

pub type Symbool = Spanned<SymboolKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum SymboolKind {
    BinOp(BinOpKind),
    UnaryOp(UnaryOpKind),
    Parenthesis(Parenthesis),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Parenthesis {
    RoundOpen,
    RoundClosed,
}

impl SymboolKind {
    pub fn from_str(name: &str) -> Option<Self> {
        let bin_op = BinOpKind::from_str(name);
        if bin_op != BinOpKind::Invalid {
            return Some(Self::BinOp(bin_op));
        }
        
        let un_op = UnaryOpKind::from_str(name);
        if un_op != UnaryOpKind::Invalid {
            return Some(Self::UnaryOp(un_op));
        }

        match name {
            "(" => Some(ROUND_BRACKET_OPEN),
            ")" => Some(ROUND_BRACKET_CLOSED),
            _ => None
        }
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            SymboolKind::Parenthesis(..) => 0,
            SymboolKind::BinOp(bin_op_kind) => bin_op_kind.get_precedence(),
            SymboolKind::UnaryOp(unary_op_kind) => unary_op_kind.get_precedence(),
        }
    }

    pub fn consume_to_symbool(self, span: SoulSpan) -> Symbool {
        Symbool::new(self, span)
    }
}

pub fn to_symbool(sy: OperatorKind, span: SoulSpan) -> Symbool {
    match sy {
        OperatorKind::BinOp(bin_op_kind) => Symbool::new(SymboolKind::BinOp(bin_op_kind), span),
        OperatorKind::UnaryOp(unary_op_kind) => Symbool::new(SymboolKind::UnaryOp(unary_op_kind), span),
    }
}











