use std::{collections::HashMap};
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{function::{Constructor, FunctionCall, Lambda, StaticMethod}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::SoulPagePath}, spanned::Spanned, statement::Block};

pub type BoxExpression = Box<Expression>;
pub type Expression = Spanned<ExpressionKind>;
pub type UnaryOperator = Spanned<UnaryOperatorKind>;
pub type BinaryOperator = Spanned<BinaryOperatorKind>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionKind {
    Empty,
    Default,
    Literal(Literal),
    
    Index(Index),
    Lambda(Lambda),
    Constructor(Constructor),
    FunctionCall(FunctionCall),

    AccessField(AccessField),
    StaticField(StaticField),
    StaticMethod(StaticMethod),

    UnwrapVariable(UnwrapVariable),
    ExternalExpression(ExternalExpression),

    Unary(Unary),
    Binary(Binary),

    If(If),
    For(For),
    While(While),
    Match(Match),
    Ternary(Ternary),

    Deref(BoxExpression),
    MutRef(BoxExpression),
    ConstRef(BoxExpression),

    Block(Block),
    ExpressionGroup(ExpressionGroup),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionGroup {
    Tuple(Tuple),
    Array(Array),
    NamedTuple(NamedTuple),
    ArrayFiller(ArrayFiller),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Array {
    pub collection_type: Option<SoulType>,
    pub element_type: Option<SoulType>,
    pub values: Vec<Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayFiller {
    pub amount: BoxExpression,
    pub index: Option<VariableName>,
    pub fill_expr: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedTuple {
    pub values: HashMap<Ident, Expression>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tuple {
    pub values: Vec<TupleElement>
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TupleElement {
    Element{value: Expression},
    Optional{name: Ident, value: Expression}
}

impl TupleElement {
    pub fn to_string(&self) -> String {
        match self {
            TupleElement::Element{value} => value.node.to_string(),
            TupleElement::Optional{name, value} => format!("{} = {}", name.0, value.node.to_string()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ternary {
    pub condition: BoxExpression,
    pub if_branch: BoxExpression,
    pub else_branch: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct While {
    pub condition: Option<BoxExpression>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct For {
    pub element: Ident,
    pub collection: BoxExpression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Match {
    pub condition: BoxExpression,
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
pub struct If {
    pub condition: BoxExpression,
    pub body: Block,
    pub else_branchs: Vec<Spanned<ElseKind>>,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ElseKind {
    ElseIf(Box<Spanned<If>>),
    Else(Spanned<Block>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExternalExpression {
    pub path: SoulPagePath,
    pub expr: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum UnwrapVariable {
    Variable(VariableName),
    MultiVariable{vars: Vec<VariableName>, ty: SoulType, initializer: Option<BoxExpression>}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Binary {
    pub left: BoxExpression,
    pub operator: BinaryOperator,
    pub right: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CompareTypeOf {
    pub left: BoxExpression,
    pub ty: SoulType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticField {
    pub object: SoulType,
    pub field: VariableName,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccessField {
    pub object: BoxExpression,
    pub field: VariableName,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    pub collection: BoxExpression,
    pub index: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct VariableName {
    pub name: Ident
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOperatorKind {
    Invalid,
    Neg, // -
    Not, // !
    Incr{before_var: bool}, // ++
    Decr{before_var: bool}, // --
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryOperatorKind {
    Invalid,
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Log, // log
    Pow, // **
    Root, // </ 
    Mod, // % 

    BitAnd, // &
    BitOr, // |
    BitXor, // |
    
    LogAnd, // &&
    LogOr, // ||
    Eq, // ==
    NotEq, // !=
    Lt, // <
    Gt, // >
    Le, // <=
    Ge, // >=

    Range, //..
    TypeOf, // <variable> typeof <type>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Ident(pub String);

impl Ident {
    pub fn new<T: Into<String>>(ident: T) -> Self {
        Self(ident.into())
    }
}

impl ExpressionKind {
    pub fn to_string(&self) -> String {
        match self {
            ExpressionKind::Empty => "<empty>".into(),
            ExpressionKind::Default => "<default>".into(),
            _ => todo!(),
        }
    }

    pub fn get_variant_name(&self) -> &'static str {
        match self {
            ExpressionKind::If(_) => "If",
            ExpressionKind::For(_) => "For",
            ExpressionKind::While(_) => "While",
            ExpressionKind::Match(_) => "Match",
            ExpressionKind::Empty => "Empty",
            ExpressionKind::Default => "Default",
            ExpressionKind::Index(_) => "Index",
            ExpressionKind::Unary(_) => "Unary",
            ExpressionKind::Deref(_) => "Deref",
            ExpressionKind::Block(_) => "Block",
            ExpressionKind::MutRef(_) => "MutRef",
            ExpressionKind::Binary(_) => "Binary",
            ExpressionKind::Lambda(_) => "Lambda",
            ExpressionKind::Ternary(_) => "Ternary",
            ExpressionKind::Literal(_) => "Literal",
            ExpressionKind::ConstRef(_) => "ConstRef",
            ExpressionKind::Constructor(_) => "Constructor",
            ExpressionKind::AccessField(_) => "AccessField",
            ExpressionKind::StaticField(_) => "StaticField",
            ExpressionKind::FunctionCall(_) => "FunctionCall",
            ExpressionKind::StaticMethod(_) => "StaticMethode",
            ExpressionKind::UnwrapVariable(_) => "UnwrapVariable",
            ExpressionKind::ExpressionGroup(_) => "ExpressionGroup",
            ExpressionKind::ExternalExpression(_) => "ExternalExpression",
        }
    }
}



































