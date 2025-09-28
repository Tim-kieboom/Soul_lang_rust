use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{BinaryOperatorKind, UnaryOperatorKind}, spanned::Spanned}};

pub type Symbool = Spanned<SymboolKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum SymboolKind {
    BinaryOperator(BinaryOperatorKind),
    UnaryOperator(UnaryOperatorKind),
    Bracket(Bracket),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operator {
    Binary(BinaryOperatorKind),
    Unary(UnaryOperatorKind),
}

impl Operator {
    pub fn from_str(text: &str) -> Option<Self> {
        let bin = BinaryOperatorKind::from_str(text);
        if bin != BinaryOperatorKind::Invalid {
            return Some(Operator::Binary(bin));
        }

        let unary = UnaryOperatorKind::from_str(text);
        if unary != UnaryOperatorKind::Invalid {
            return Some(Operator::Unary(unary));
        }

        None
    }

    pub fn to_symbool(&self, span: SoulSpan) -> Symbool {
        match self.clone() {
            Operator::Binary(binary_operator_kind) => Symbool::new(SymboolKind::BinaryOperator(binary_operator_kind), span),
            Operator::Unary(unary_operator_kind) => Symbool::new(SymboolKind::UnaryOperator(unary_operator_kind), span),
        }
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            Operator::Binary(binary_operator_kind) => binary_operator_kind.get_precedence(),
            Operator::Unary(unary_operator_kind) => unary_operator_kind.get_precedence(),
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub enum Bracket {
    RoundOpen,
    RoundClose,
}






































