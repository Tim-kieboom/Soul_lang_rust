use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use crate::{soul_names::{NamesOperator, NamesOtherKeyWords, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{function::{StructConstructor, FunctionCall, Lambda, StaticMethod}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::SoulPagePath}, spanned::{Spanned}, statement::Block}};

pub type Expression = Spanned<ExpressionKind>;

pub type BoxExpression = Box<Expression>;
pub type UnaryOperator = Spanned<UnaryOperatorKind>;
pub type BinaryOperator = Spanned<BinaryOperatorKind>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExpressionKind {
    Empty,
    Default,
    Literal(Literal),
    
    Index(Index),
    Lambda(Lambda),
    FunctionCall(FunctionCall),
    StructConstructor(StructConstructor),

    AccessField(AccessField),
    StaticField(StaticField),
    StaticMethod(StaticMethod),

    Variable(VariableName),
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
    ReturnLike(ReturnLike),
    ExpressionGroup(ExpressionGroup),
}

impl Default for ExpressionKind {
    fn default() -> Self {
        Self::Empty
    }
} 

pub type DeleteList = Vec<String>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReturnLike {
    pub value: Option<BoxExpression>,
    pub delete_list: DeleteList,
    pub kind: ReturnKind
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ReturnKind {
    Return,
    Fall,
    Break,
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
    pub collection_type: Option<SoulType>,
    pub element_type: Option<SoulType>,
    pub amount: BoxExpression,
    pub index: Option<VariableName>,
    pub fill_expr: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedTuple {
    pub values: HashMap<Ident, Expression>,
    
    // 'Foo(field: 1, ..)' is true if '..' meaning that all other fields use default value
    pub insert_defaults: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Tuple {
    pub values: Vec<Expression>
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
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct For {
    pub element: Option<BoxExpression>,
    pub collection: BoxExpression,
    pub block: Block,
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
    Block(Spanned<Block>),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct If {
    pub condition: BoxExpression,
    pub block: Block,
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
    Increment{before_var: bool}, // ++
    Decrement{before_var: bool}, // --
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

impl VariableName {
    pub fn new<T: Into<Ident>>(ident: T) -> Self {
        Self{name: ident.into()}
    }
}

impl Ident {
    pub fn new<T: Into<String>>(ident: T) -> Self {
        Self(ident.into())
    }

    pub fn empty() -> Self {
        Self(String::new())
    }
}

impl Binary {
    pub fn new(left: Expression, operator: BinaryOperator, right: Expression) -> Self {
        Self { left: Box::new(left), operator, right: Box::new(right) }
    }
}

impl ReturnKind {
    pub fn to_str(&self) -> &'static str {
        match self {
            ReturnKind::Return => "return",
            ReturnKind::Fall => "fall",
            ReturnKind::Break => "break",
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<Ident> for String {
    fn into(self) -> Ident {
        Ident::new(self)
    }
}

impl Into<Ident> for &Ident {
    fn into(self) -> Ident {
        self.clone()
    }
}

impl Into<Ident> for &String {
    fn into(self) -> Ident {
        Ident::new(self)
    }
} 

impl Into<Ident> for &str {
    fn into(self) -> Ident {
        Ident::new(self)
    }
} 

impl ExpressionKind {

    pub fn get_variant_name(&self) -> &'static str {
        match self {
            ExpressionKind::ReturnLike(return_kind) => return_kind.kind.to_str(),
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
            ExpressionKind::Variable(_) => "Variable",
            ExpressionKind::StructConstructor(_) => "Constructor",
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

impl BinaryOperatorKind {
    pub fn from_str(name: &str) -> Self {
        match name {
            val if val == SOUL_NAMES.get_name(NamesOperator::Equals) => Self::Eq,
            val if val == SOUL_NAMES.get_name(NamesOperator::NotEquals) => Self::NotEq,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsSmaller) => Self::Le,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsSmallerEquals) => Self::Lt,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsBigger) => Self::Ge,
            val if val == SOUL_NAMES.get_name(NamesOperator::IsBiggerEquals) => Self::Gt,
            val if val == SOUL_NAMES.get_name(NamesOperator::Addition) => Self::Add,
            val if val == SOUL_NAMES.get_name(NamesOperator::Subtract) => Self::Sub,
            val if val == SOUL_NAMES.get_name(NamesOperator::Multiple) => Self::Mul,
            val if val == SOUL_NAMES.get_name(NamesOperator::Divide) => Self::Div,
            val if val == SOUL_NAMES.get_name(NamesOperator::Modulo) => Self::Mod,
            val if val == SOUL_NAMES.get_name(NamesOperator::Power) => Self::Pow,
            val if val == SOUL_NAMES.get_name(NamesOperator::Root) => Self::Root,
            val if val == SOUL_NAMES.get_name(NamesOperator::Logarithm) => Self::Log,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseOr) => Self::BitOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseAnd) => Self::BitOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::BitWiseXor) => Self::BitXor,
            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalOr) => Self::LogOr,
            val if val == SOUL_NAMES.get_name(NamesOperator::LogicalAnd) => Self::LogAnd,
            val if val == SOUL_NAMES.get_name(NamesOperator::Range) => Self::Range,
            val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) => Self::TypeOf,
            _ => Self::Invalid, 
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Eq      => SOUL_NAMES.get_name(NamesOperator::Equals),
            Self::NotEq   => SOUL_NAMES.get_name(NamesOperator::NotEquals),
            Self::Le      => SOUL_NAMES.get_name(NamesOperator::IsSmaller),
            Self::Lt      => SOUL_NAMES.get_name(NamesOperator::IsSmallerEquals),
            Self::Ge      => SOUL_NAMES.get_name(NamesOperator::IsBigger),
            Self::Gt      => SOUL_NAMES.get_name(NamesOperator::IsBiggerEquals),
            Self::Add     => SOUL_NAMES.get_name(NamesOperator::Addition),
            Self::Sub     => SOUL_NAMES.get_name(NamesOperator::Subtract),
            Self::Mul     => SOUL_NAMES.get_name(NamesOperator::Multiple),
            Self::Div     => SOUL_NAMES.get_name(NamesOperator::Divide),
            Self::Mod     => SOUL_NAMES.get_name(NamesOperator::Modulo),
            Self::Pow     => SOUL_NAMES.get_name(NamesOperator::Power),
            Self::Root    => SOUL_NAMES.get_name(NamesOperator::Root),
            Self::Log     => SOUL_NAMES.get_name(NamesOperator::Logarithm),
            Self::BitOr   => SOUL_NAMES.get_name(NamesOperator::BitWiseOr),
            Self::BitAnd  => SOUL_NAMES.get_name(NamesOperator::BitWiseAnd),
            Self::BitXor  => SOUL_NAMES.get_name(NamesOperator::BitWiseXor),
            Self::LogOr   => SOUL_NAMES.get_name(NamesOperator::LogicalOr),
            Self::LogAnd  => SOUL_NAMES.get_name(NamesOperator::LogicalAnd),
            Self::Range   => SOUL_NAMES.get_name(NamesOperator::Range),
            Self::TypeOf  => SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof),
            Self::Invalid => "<invalid>",
        }
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            BinaryOperatorKind::Invalid => 0,
            
            BinaryOperatorKind::Range |
            BinaryOperatorKind::LogAnd |
            BinaryOperatorKind::LogOr => 1,

            BinaryOperatorKind::BitAnd |
            BinaryOperatorKind::BitOr |
            BinaryOperatorKind::BitXor => 2,

            BinaryOperatorKind::Eq |
            BinaryOperatorKind::NotEq => 3,

            BinaryOperatorKind::Lt |
            BinaryOperatorKind::Gt |
            BinaryOperatorKind::Le |
            BinaryOperatorKind::Ge => 4,

            BinaryOperatorKind::Sub |
            BinaryOperatorKind::Add => 5,
            
            BinaryOperatorKind::Mul |
            BinaryOperatorKind::Div |
            BinaryOperatorKind::Mod => 6,

            BinaryOperatorKind::Log |
            BinaryOperatorKind::Pow |
            BinaryOperatorKind::Root |
            BinaryOperatorKind::TypeOf => 7,
        }
    }
}

impl UnaryOperatorKind {
    pub fn from_str(name: &str) -> Self {
        match name {
            "-" => Self::Neg,
            val if val == SOUL_NAMES.get_name(NamesOperator::Not) => Self::Not,
            val if val == SOUL_NAMES.get_name(NamesOperator::Increment) => Self::Increment{before_var: true},
            val if val == SOUL_NAMES.get_name(NamesOperator::Decrement) => Self::Decrement{before_var: true},
            _ => Self::Invalid,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::Invalid => "<invalid>",
            Self::Neg => "-",
            Self::Not => "!",
            Self::Increment{..} => "++",
            Self::Decrement{..} => "--",
        }
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            UnaryOperatorKind::Invalid => 0,
            UnaryOperatorKind::Neg |
            UnaryOperatorKind::Not => 8,
            UnaryOperatorKind::Increment{..} |
            UnaryOperatorKind::Decrement{..} => 9,
        }
    }
}

#[macro_export]
macro_rules! soul_tuple {
    () => (
        Tuple{values: vec![]}
    );
    ($($x:expr),+ $(,)?) => (
        Tuple{values: vec![$($x),+]}
    );
}
































