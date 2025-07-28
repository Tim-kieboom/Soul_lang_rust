use std::collections::BTreeMap;

use itertools::Itertools;
use crate::{errors::soul_error::SoulSpan, soul_names::{NamesOperator, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{literal::Literal, soul_type::{soul_type::SoulType, type_kind::TypeKind}, spanned::Spanned}};

pub type Expression = Spanned<ExprKind>;
pub type BoxExpr = Box<Expression>;

pub type BinOp = Spanned<BinOpKind>;
pub type UnaryOp = Spanned<UnaryOpKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprKind {
    Empty,
    Call(FnCall),
    Index(Index),
    Field(Field),
    Literal(Literal),
    Unary(UnaryExpr),
    Variable(Variable),
    TypeOf(TypeOfExpr),
    Binary(BinaryExpr),
    StaticField(StaticField),
    StaticMethode(StaticMethode),

    Deref(BoxExpr),
    MutRef(BoxExpr),
    ConstRef(BoxExpr),

    Array(Array),
    Tuple(Tuple),
    NamedTuple(NamedTuple),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Field {
    pub object: BoxExpr,
    pub field: Variable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaticField {
    pub object: Spanned<TypeKind>,
    pub field: Variable,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
    pub collection_type: Option<SoulType>,
    pub element_type: Option<SoulType>,
    pub values: Vec<Expression>,
} 

#[derive(Debug, Clone, PartialEq)]
pub struct Tuple {
    pub values: Vec<Expression>,
} 

#[derive(Debug, Clone, PartialEq)]
pub struct NamedTuple {
    pub object_type: Option<SoulType>,
    pub values: BTreeMap<Ident, Expression>,
} 

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: Ident,
} 

impl ExprKind {
    pub fn to_string(&self) -> String {
        match self {
            ExprKind::Empty => "<EmptyExpression>".to_string(),
            ExprKind::Literal(literal) => literal.to_string(),
            ExprKind::Variable(Variable{name}) => name.0.clone(),
            ExprKind::TypeOf(type_of_expr) => type_of_expr.to_string(),
            ExprKind::Binary(BinaryExpr{left, operator, right}) => format!("({} {} {})", left.node.to_string(), operator.node.to_str(), right.node.to_string()),
            ExprKind::Index(Index{ collection, index }) => format!("{}[{}]", collection.node.to_string(), index.node.to_string()),
            ExprKind::Unary(UnaryExpr{ operator, expression }) => {
                match operator.node {
                    UnaryOpKind::Incr{before_var} |
                    UnaryOpKind::Decr{before_var} => {
                        if before_var {
                            format!("{} {}", operator.node.to_str(), expression.node.to_string())
                        }
                        else {
                            format!("{} {}", expression.node.to_string(), operator.node.to_str())
                        }
                    }
                    UnaryOpKind::Neg |
                    UnaryOpKind::Not |
                    UnaryOpKind::Invalid => format!("{} {}", operator.node.to_str(), expression.node.to_string()),
                }
            },
            ExprKind::Call(FnCall{callee, name, generics, arguments}) => {
                let generics = if generics.is_empty() {
                    String::new()
                }
                else {
                    format!("<{}>", generics.iter().map(|ty| ty.to_string()).join(","))
                };

                if let Some(methode) = callee {
                    format!("{}.{}{}({})", methode.node.to_string(), name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
                }
                else {
                    format!("{}{}({})", name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
                }
            },
            ExprKind::ConstRef(spanned) => format!("@{}", spanned.node.to_string()),
            ExprKind::MutRef(spanned) => format!("&{}", spanned.node.to_string()),
            ExprKind::Deref(spanned) => format!("*{}", spanned.node.to_string()),
            ExprKind::Array(Array{collection_type, element_type, values}) => format!(
                "{}[{}{}]", 
                collection_type.as_ref().map(|ty| ty.to_string()).unwrap_or("".into()), 
                element_type.as_ref().map(|ty| format!("{};", ty.to_string())).unwrap_or("".into()),
                values.iter().map(|expr| expr.node.to_string()).join(",")
            ),
            ExprKind::Tuple(Tuple{values}) => format!(
                "({})", 
                values.iter().map(|expr| expr.node.to_string()).join(","),
            ),
            ExprKind::NamedTuple(NamedTuple{object_type, values}) => format!(
                "{}({})", 
                object_type.as_ref().map(|ty| ty.to_string()).unwrap_or("".into()), 
                if !values.is_empty() {values.iter().map(|(name, expr)| format!("{}: {}", name.0, expr.node.to_string())).join(",")}
                else {":".into()},
            ),
            ExprKind::Field(Field{object, field}) => format!(
                "{}.{}",
                object.node.to_string(),
                field.name.0
            ),
            ExprKind::StaticField(StaticField{object, field}) => format!(
                "{}.{}",
                object.node.to_string(),
                field.name.0
            ),
            ExprKind::StaticMethode(StaticMethode{ callee, name, generics, arguments}) => {
                let generics = if generics.is_empty() {
                    String::new()
                }
                else {
                    format!("<{}>", generics.iter().map(|ty| ty.to_string()).join(","))
                };

                format!("{}.{}{}({})", callee.node.to_string(), name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
            },
        }
    }

    pub fn is_any_ref(&self) -> bool {
        match self {
            ExprKind::Empty |
            ExprKind::Call(..) |
            ExprKind::Unary(..) |
            ExprKind::Index(..) |
            ExprKind::Deref(..) |
            ExprKind::Array(..) |
            ExprKind::Tuple(..) |
            ExprKind::Field(..) |
            ExprKind::TypeOf(..) |
            ExprKind::Binary(..) |
            ExprKind::Literal(..) |
            ExprKind::Variable(..) |
            ExprKind::NamedTuple(..) |
            ExprKind::StaticMethode(..) |
            ExprKind::StaticField(..) => false,
            
            ExprKind::MutRef(..) |
            ExprKind::ConstRef(..) => true,
        }
    }

    pub fn get_variant_name(&self) -> &'static str {
        match self {
            ExprKind::Empty => "<empty>",
            ExprKind::Index(_) => "index",
            ExprKind::Unary(_) => "unary",
            ExprKind::Call(_) => "FnCall",
            ExprKind::Deref(_) => "DeRef",
            ExprKind::Array(_) => "Array",
            ExprKind::Tuple(_) => "Tuple",
            ExprKind::Field(_) => "Field",
            ExprKind::TypeOf(_) => "typeof",
            ExprKind::Binary(_) => "binary",
            ExprKind::MutRef(_) => "MutRef",
            ExprKind::Literal(_) => "Literal",
            ExprKind::Variable(_) => "Valiable",
            ExprKind::ConstRef(_) => "ConstRef",
            ExprKind::NamedTuple(_) => "NamedTuple",
            ExprKind::StaticMethode(_) => "StaticCall",
            ExprKind::StaticField(_) => "StaticField",
        }
    }

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

impl TypeOfExpr {
    pub fn to_string(&self) -> String {
        match self {
            TypeOfExpr::Type(soul_type) => format!("typeof {}", soul_type.to_string()),
            TypeOfExpr::VariantPattern { union_type, variant_name, binding } => format!("typeof {}.{}({})", union_type.to_string(), variant_name.0, binding.as_ref().map(|bind| bind.0.as_str()).unwrap_or("")),
            TypeOfExpr::TypeEnum(soul_types) => format!("typeof[{}]", soul_types.iter().map(|ty| ty.to_string()).join(",")),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall {
    pub callee: Option<BoxExpr>,
    pub name: Ident,
    pub generics: Vec<SoulType>,
    pub arguments: Vec<Arguments>,
}

impl FnCall {
    pub fn consume_to_static_methode(self, ty: Spanned<TypeKind>) -> StaticMethode {
        StaticMethode{
            callee: ty, 
            name: self.name, 
            generics: self.generics, 
            arguments: self.arguments,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StaticMethode {
    pub callee: Spanned<TypeKind>,
    pub name: Ident,
    pub generics: Vec<SoulType>,
    pub arguments: Vec<Arguments>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Index {
    pub collection: BoxExpr,
    pub index: BoxExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub expression: BoxExpr,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryExpr {
    pub left: BoxExpr,
    pub operator: BinOp,
    pub right: BoxExpr,
}

impl BinaryExpr {
    pub fn new(left: Expression, operator: BinOp, right: Expression) -> Self {
        Self {left: Box::new(left), operator, right: Box::new(right)}
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
    pub name: Option<Ident>,
    pub expression: Expression,
}

impl Arguments {
    pub fn to_string(&self) -> String {
        if let Some(optional) = &self.name {
            format!("{}: {}", optional.0, self.expression.node.to_string())
        }
        else {
            self.expression.node.to_string()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Ident(pub String);

impl UnaryExpr {
    pub fn consume_to_expression(self, span: SoulSpan) -> Expression {
        Expression::new(ExprKind::Unary(self), span)
    }
}

impl BinaryExpr {
    pub fn consume_to_expression(self, span: SoulSpan) -> Expression {
        Expression::new(ExprKind::Binary(self), span)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinOpKind {
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOpKind {
    Invalid,
    Neg, // -
    Not, // !
    Incr{before_var: bool}, // ++
    Decr{before_var: bool}, // --
}

#[derive(Debug, Clone, PartialEq)]
pub enum OperatorKind {
    BinOp(BinOpKind),
    UnaryOp(UnaryOpKind),
}

impl OperatorKind {
    pub fn from_str(text: &str) -> Option<OperatorKind> {
        let bin = BinOpKind::from_str(text);
        if bin != BinOpKind::Invalid {
            return Some(OperatorKind::BinOp(bin));
        }

        let unary = UnaryOpKind::from_str(text);
        if unary != UnaryOpKind::Invalid {
            return Some(OperatorKind::UnaryOp(unary));
        }

        None
    }
}

impl BinOpKind {
    pub fn get_precedence(&self) -> u8 {
        match self {
            BinOpKind::Invalid => 0,
            
            BinOpKind::Range |
            BinOpKind::LogAnd |
            BinOpKind::LogOr => 1,

            BinOpKind::BitAnd |
            BinOpKind::BitOr |
            BinOpKind::BitXor => 2,

            BinOpKind::Eq |
            BinOpKind::NotEq => 3,

            BinOpKind::Lt |
            BinOpKind::Gt |
            BinOpKind::Le |
            BinOpKind::Ge => 4,

            BinOpKind::Sub |
            BinOpKind::Add => 5,
            
            BinOpKind::Mul |
            BinOpKind::Div |
            BinOpKind::Mod => 6,

            BinOpKind::Log |
            BinOpKind::Pow |
            BinOpKind::Root => 7,
        }
    }
}

impl UnaryOpKind {
    pub fn get_precedence(&self) -> u8 {
        match self {
            UnaryOpKind::Invalid => 0,
            UnaryOpKind::Neg |
            UnaryOpKind::Not => 8,
            UnaryOpKind::Incr{..} |
            UnaryOpKind::Decr{..} => 9,
        }
    }
}

impl OperatorKind {
    pub fn get_precedence(&self) -> u8 {
        match self {
            OperatorKind::BinOp(bin_op_kind) => bin_op_kind.get_precedence(),
            OperatorKind::UnaryOp(unary_op_kind) => unary_op_kind.get_precedence(),
        }       
    }
}

impl BinOpKind {
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
            Self::Range  => SOUL_NAMES.get_name(NamesOperator::Range),
            Self::Invalid => "<invalid>",
        }
    }


}

impl UnaryOpKind {
    pub fn from_str(name: &str) -> Self {
        match name {
            "-" => Self::Neg,
            val if val == SOUL_NAMES.get_name(NamesOperator::Not) => Self::Not,
            val if val == SOUL_NAMES.get_name(NamesOperator::Increment) => Self::Incr{before_var: true},
            val if val == SOUL_NAMES.get_name(NamesOperator::Decrement) => Self::Decr{before_var: true},
            _ => Self::Invalid,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            UnaryOpKind::Invalid => "<invalid>",
            UnaryOpKind::Neg => "-",
            UnaryOpKind::Not => "!",
            UnaryOpKind::Incr{..} => "++",
            UnaryOpKind::Decr{..} => "--",
        }
    }
}




























