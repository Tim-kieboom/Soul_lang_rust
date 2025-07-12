use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{soul_type::soul_type::SoulType, spanned::Spanned};

pub type Expression = Spanned<ExprKind>;
pub type BoxExpr = Box<Expression>;

pub type BinOp = Spanned<BinOpKind>;
pub type UnaryOp = Spanned<UnaryOpKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Literal(Literal),
    Variable(Ident),
    TypeOf(TypeOfExpr),
    Binary {
        left: BoxExpr,
        operator: BinOp,
        right: BoxExpr,
    },
    Index {
        collection: BoxExpr,
        index: BoxExpr,
    },
    Unary {
        operator: UnaryOp,
        expression: BoxExpr,
    },
    Call {
        callee: BoxExpr,
        arguments: Vec<Arguments>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeOfExpr {
    /// e.g. typeof int  (type reference)
    Type(SoulType),
    
    /// e.g. typeof Union1.Variant(data) (variant pattern match)
    VariantPattern {
        union_type: SoulType,
        variant_name: Ident,
        binding: Option<Ident>,
    },

    /// e.g. typeof [int, i8, i16] for typeEnum
    TypeEnum(Vec<SoulType>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    pub name: Option<Ident>,
    pub expression: Expression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    Str(String),
    Array(Vec<Literal>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOpKind {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Log, // log
    Pow, // **
    Root, // </ 

    BitAnd, // &
    BitOr, // |
    
    LogAnd, // &&
    LogOr, // ||
    Eq, // ==
    NotEq, // !=
    Lt, // <
    Gt, // >
    Le, // <=
    Ge, // >=
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpKind {
    Neg, // -
    Not, // !
    Incr{before_var: bool}, // ++
    Decr{before_var: bool}, // --
}

















