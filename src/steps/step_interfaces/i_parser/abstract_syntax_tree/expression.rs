use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};
use crate::{errors::soul_error::SoulSpan, soul_names::{NamesOperator, NamesOtherKeyWords, SOUL_NAMES}, steps::step_interfaces::i_parser::{abstract_syntax_tree::{function::{FunctionCall, Lambda, StaticMethod, StructConstructor}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::SoulPagePath}, spanned::Spanned, statement::Block}, scope_builder::ScopeId}};

pub type Expression = Spanned<ExpressionKind>;

pub type BoxExpression = Box<Expression>;
pub type UnaryOperator = Spanned<UnaryOperatorKind>;
pub type BinaryOperator = Spanned<BinaryOperatorKind>;

/// The different kinds of expressions in the language.
///
/// Expressions can be values, control-flow structures, function calls,
/// or more complex constructs like lambdas and blocks.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ExpressionKind {
    /// An empty expression (placeholder).
    Empty,
    /// A `default` literal or default value.
    Default,
    /// A literal value (number, string, etc.).
    Literal(Literal),
    
    /// Indexing into a collection, e.g., `arr[i]`.
    Index(Index),
    /// A lambda `() => {}`.
    Lambda(Lambda),
    /// A function call, e.g., `foo(x, y)`.
    FunctionCall(FunctionCall),
    /// Constructing a struct, e.g., `Point { x: 1, y: 2 }`.
    StructConstructor(StructConstructor),

    /// Accessing a field on an instance, e.g., `obj.field`.
    AccessField(AccessField),
    /// Accessing a static field, e.g., `Type.field`.
    StaticField(StaticField),
    /// Calling a static method, e.g., `Type.method()`.
    StaticMethod(StaticMethod),

    /// Referring to a variable `var`.
    Variable(VariableName),
    /// Variable unwrapping / pattern destructuring `Data{field1, field2} := data`.
    UnwrapVariable(UnwrapVariable),
    /// An external expression from another page/module `path.to.something.expression`.
    ExternalExpression(ExternalExpression),

    /// A unary operation (negation, increment, etc.) `!1`.
    Unary(Unary),
    /// A binary operation (addition, multiplication, comparison, etc.) `1 + 2`.
    Binary(Binary),

    /// An `if` expression `if true {Println("is true")}`.
    If(If),
    /// A `for` loop `for i in 1..2 {Println(f"num:{i}")}`.
    For(For),
    /// A `while` loop `while true {Println("loop")}`.
    While(While),
    /// A `match` expression `match booleanVar {true => (), false => }`.
    Match(Match),
    /// A ternary expression `cond ? a : b`.
    Ternary(Ternary),

    /// A dereference, e.g., `*ptr`.
    Deref(BoxExpression),
    /// A mutable reference, e.g., `&x`.
    MutRef(BoxExpression),
    /// A constant reference, e.g., `@x`.
    ConstRef(BoxExpression),

    /// A block of statements, returning the last expression `{/*stuff*/}`.
    Block(Block),
    /// Return-like expressions (`return`, `break`, `fall`) `return 1`.
    ReturnLike(ReturnLike),
    /// A grouped expression, e.g., tuples, namedTuples or arrays.
    ExpressionGroup(ExpressionGroup),
}

impl Default for ExpressionKind {
    fn default() -> Self {
        Self::Empty
    }
} 

pub type DeleteList = Vec<String>;

/// A `return`, `fall`, or `break`-like expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct ReturnLike {
    pub value: Option<BoxExpression>,
    pub delete_list: DeleteList,
    pub kind: ReturnKind
}

/// The kind of return-like expression.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ReturnKind {
    /// A returns function.
    Return,
    /// A returns current block.
    Fall,
    /// A returns current loop.
    Break,
}

/// A grouped expression type, such as tuple, array, or named tuple.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ExpressionGroup {
    /// A tuple, e.g., `(1, 2, 3)`.
    Tuple(Tuple),
    /// An array literal, e.g., `[1, 2, 3]`.
    Array(Array),
    /// A named tuple, e.g., `{x: 1, y: 2}`.
    NamedTuple(NamedTuple),
    /// An array filler expression, e.g., `[5 => 1] //makes [1,1,1,1,1]`.
    ArrayFiller(ArrayFiller),
}

/// An array literal, e.g., `[1, 2, 3]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Array {
    pub collection_type: Option<SoulType>,
    pub element_type: Option<SoulType>,
    pub values: Vec<Expression>,
}

/// An array filler expression, e.g., `[5 => 1] //makes [1,1,1,1,1]`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct ArrayFiller {
    pub collection_type: Option<SoulType>,
    pub element_type: Option<SoulType>,
    pub amount: BoxExpression,
    pub index: Option<VariableName>,
    pub fill_expr: BoxExpression,
    pub scope_id: ScopeId,
}

/// A named tuple, e.g., `{x: 1, y: 2}`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct NamedTuple {
    pub values: HashMap<Ident, Expression>,
    
    // 'Foo{field: 1, ..}' is true if '..' meaning that all other fields use default value
    pub insert_defaults: bool,
}

/// A tuple, e.g., `(1, 2, 3)`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Tuple {
    pub values: Vec<Expression>
}

/// A ternary conditional expression, e.g., `cond ? a : b`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Ternary {
    pub condition: BoxExpression,
    pub if_branch: BoxExpression,
    pub else_branch: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct While {
    pub condition: Option<BoxExpression>,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct For {
    pub element: Option<BoxExpression>,
    pub collection: BoxExpression,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Match {
    pub condition: BoxExpression,
    pub cases: Vec<CaseSwitch>,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct CaseSwitch {
    pub if_kind: IfCaseKind,
    pub do_fn: CaseDoKind,
    pub scope_id: ScopeId,
} 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum IfCaseKind {
    Expression(Expression),
    Variant{name: Ident, params: Tuple},
    NamedVariant{name: Ident, params: NamedTuple},
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum CaseDoKind {
    Block(Spanned<Block>),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct If {
    pub condition: BoxExpression,
    pub block: Block,
    pub else_branchs: Vec<Spanned<ElseKind>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum ElseKind {
    ElseIf(Box<Spanned<If>>),
    Else(Spanned<Block>)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct ExternalExpression {
    pub path: SoulPagePath,
    pub expr: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub enum UnwrapVariable {
    Variable(VariableName),
    MultiVariable{vars: Vec<VariableName>, ty: SoulType, initializer: Option<BoxExpression>}
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub expression: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Binary {
    pub left: BoxExpression,
    pub operator: BinaryOperator,
    pub right: BoxExpression,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct CompareTypeOf {
    pub left: BoxExpression,
    pub ty: SoulType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct StaticField {
    pub object: SoulType,
    pub field: VariableName,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct AccessField {
    pub object: BoxExpression,
    pub field: VariableName,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Encode, Decode)]
pub struct Index {
    pub collection: BoxExpression,
    pub index: BoxExpression,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode)]
pub struct VariableName {
    pub name: Ident,
    pub span: SoulSpan,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
pub enum UnaryOperatorKind {
    Invalid,
    Neg, // -
    Not, // !
    Increment{before_var: bool}, // ++
    Decrement{before_var: bool}, // --
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Encode, Decode)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize, Encode, Decode)]
pub struct Ident(pub String);

impl VariableName {
    pub fn new<T: Into<Ident>>(ident: T, span: SoulSpan) -> Self {
        Self{name: ident.into(), span}
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

impl AsRef<str> for Ident {
    fn as_ref(&self) -> &str {
        &self.0
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

    pub fn get_scope_id(&self) -> Option<ScopeId> {
        
        Some(match self {
            ExpressionKind::If(if_) => if_.block.scope_id,
            ExpressionKind::For(for_) => for_.block.scope_id,
            ExpressionKind::While(while_) => while_.block.scope_id,
            ExpressionKind::Match(match_) => match_.scope_id,
            ExpressionKind::Block(block) => block.scope_id,
            ExpressionKind::Lambda(lambda_) => lambda_.scope_id,
            ExpressionKind::ExpressionGroup(ExpressionGroup::ArrayFiller(array)) => array.scope_id,

            _ => return None,
        })
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
































