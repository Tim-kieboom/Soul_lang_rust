use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{BinaryOperatorKind, UnaryOperatorKind}, spanned::Spanned}};

pub type Symbool = Spanned<SymboolKind>;

pub enum SymboolKind {
    BinaryOperator(BinaryOperatorKind),
    UnaryOperator(UnaryOperatorKind),
    Bracket(Bracket),
}

impl Symbool {
    pub fn new_bracket(node: Bracket, span: SoulSpan) -> Self  {
        Self { node: SymboolKind::Bracket(node), span }
    }

    pub fn new_binary(node: BinaryOperatorKind, span: SoulSpan) -> Self {
        Self { node: SymboolKind::BinaryOperator(node), span }
    }

    pub fn new_unary(node: UnaryOperatorKind, span: SoulSpan) -> Self {
        Self { node: SymboolKind::UnaryOperator(node), span }
    }
}

impl SymboolKind {
    pub fn from_str(name: &str) -> Option<Self> {
        let bin_op = BinaryOperatorKind::from_str(name);
        if bin_op != BinaryOperatorKind::Invalid {
            return Some(Self::BinaryOperator(bin_op));
        }
        
        let un_op = UnaryOperatorKind::from_str(name);
        if un_op != UnaryOperatorKind::Invalid {
            return Some(Self::UnaryOperator(un_op));
        }

        match name {
            "(" => Some(Self::Bracket(Bracket::RoundOpen)),
            ")" => Some(Self::Bracket(Bracket::RoundClose)),
            _ => None
        }
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            SymboolKind::Bracket(..) => 0,
            SymboolKind::UnaryOperator(unary_op_kind) => unary_op_kind.get_precedence(),
            SymboolKind::BinaryOperator(bin_op_kind) => bin_op_kind.get_precedence(),
        }
    }

    pub fn consume_to_symbool(self, span: SoulSpan) -> Symbool {
        Symbool::new(self, span)
    }
}

pub enum Bracket {
    RoundOpen,
    RoundClose,
}






































