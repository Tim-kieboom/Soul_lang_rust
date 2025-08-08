use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use crate::{errors::soul_error::SoulSpan, soul_names::{NamesOperator, NamesOtherKeyWords, SOUL_NAMES}, steps::step_interfaces::i_parser::abstract_syntax_tree::{literal::Literal, pretty_format::PrettyPrint, soul_type::soul_type::SoulType, spanned::Spanned, staments::{conditionals::IfDecl, function::LambdaSignatureRef, statment::{Block, VariableKind, VariableRef}}}};

pub type Expression = Spanned<ExprKind>;
pub type BoxExpr = Box<Expression>;

pub type BinOp = Spanned<BinOpKind>;
pub type UnaryOp = Spanned<UnaryOpKind>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExprKind {
    Empty,
    Default,
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

    Ctor(FnCall),
    UnwrapVarDecl(Box<VariableKind>),

    Lambda(LambdaDecl),
    If(Box<IfDecl>),

    Ternary(Ternary),

    Deref(BoxExpr),
    MutRef(BoxExpr),
    ConstRef(BoxExpr),

    Array(Array),
    ArrayFiller(ArrayFiller),
    Tuple(Tuple),
    NamedTuple(NamedTuple),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Field {
    pub object: BoxExpr,
    pub field: Variable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticField {
    pub object: Spanned<SoulType>,
    pub field: Variable,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Array {
    pub collection_type: Option<SoulType>,
    pub element_type: Option<SoulType>,
    pub values: Vec<Expression>,
} 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArrayFiller {
    pub amount: BoxExpr,
    pub index: Option<VariableRef>,
    pub fill_expr: BoxExpr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tuple {
    pub values: Vec<Expression>,
} 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct NamedTuple {
    pub object_type: Option<SoulType>,
    pub values: BTreeMap<Ident, Expression>,
} 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Variable {
    pub name: Ident,
} 

impl ExprKind {
    pub fn to_string(&self, tab: usize) -> String {
        match self {
            ExprKind::Empty => "<EmptyExpression>".to_string(),
            ExprKind::Default => "<defaultExpression>".to_string(),
            ExprKind::Literal(literal) => literal.to_string(),
            ExprKind::Variable(Variable{name}) => name.0.clone(),
            ExprKind::TypeOf(TypeOfExpr{left, ty }) => format!("{} {} {}", left.node.to_string(tab), SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof), ty.to_string()),
            ExprKind::Binary(BinaryExpr{left, operator, right}) => format!("({} {} {})", left.node.to_string(tab), operator.node.to_str(), right.node.to_string(tab)),
            ExprKind::Index(Index{ collection, index }) => format!("{}[{}]", collection.node.to_string(tab), index.node.to_string(tab)),
            ExprKind::Unary(UnaryExpr{ operator, expression }) => {
                match operator.node {
                    UnaryOpKind::Incr{before_var} |
                    UnaryOpKind::Decr{before_var} => {
                        if before_var {
                            format!("{} {}", operator.node.to_str(), expression.node.to_string(tab))
                        }
                        else {
                            format!("{} {}", expression.node.to_string(tab), operator.node.to_str())
                        }
                    }
                    UnaryOpKind::Neg |
                    UnaryOpKind::Not |
                    UnaryOpKind::Invalid => format!("{} {}", operator.node.to_str(), expression.node.to_string(tab)),
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
                    format!("{}.{}{}({})", methode.node.to_string(tab), name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
                }
                else {
                    format!("{}{}({})", name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
                }
            },
            ExprKind::ConstRef(spanned) => format!("@{}", spanned.node.to_string(tab)),
            ExprKind::MutRef(spanned) => format!("&{}", spanned.node.to_string(tab)),
            ExprKind::Deref(spanned) => format!("*{}", spanned.node.to_string(tab)),
            ExprKind::Array(Array{collection_type, element_type, values}) => format!(
                "{}[{}{}]", 
                collection_type.as_ref().map(|ty| ty.to_string()).unwrap_or("".into()), 
                element_type.as_ref().map(|ty| format!("{};", ty.to_string())).unwrap_or("".into()),
                values.iter().map(|expr| expr.node.to_string(tab)).join(",")
            ),
            ExprKind::Tuple(Tuple{values}) => format!(
                "({})", 
                values.iter().map(|expr| expr.node.to_string(tab)).join(","),
            ),
            ExprKind::NamedTuple(NamedTuple{object_type, values}) => format!(
                "{}({})", 
                object_type.as_ref().map(|ty| ty.to_string()).unwrap_or("".into()), 
                if !values.is_empty() {values.iter().map(|(name, expr)| format!("{}: {}", name.0, expr.node.to_string(tab))).join(",")}
                else {":".into()},
            ),
            ExprKind::Field(Field{object, field}) => format!(
                "{}.{}",
                object.node.to_string(tab),
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
            ExprKind::Lambda(LambdaDecl{signature, arguments, body, capture:_}) => {
                let sig = signature.borrow();
                format!(
                    "{}({}): {} => {}",
                    sig.mode.get_lambda_name(),
                    arguments.iter().map(|el| el.node.to_string(tab)).join(","),
                    sig.return_type.as_ref().unwrap_or(&SoulType::none()).to_string(),
                    body.statments.iter().map(|el| el.node.to_pretty(0, false)).join(";")
                )
            },
            ExprKind::Ternary(Ternary{condition, if_branch, else_branch}) => format!(
                "({}) ? {} : {}",
                condition.node.to_string(tab),
                if_branch.node.to_string(tab),
                else_branch.node.to_string(tab),
            ),
            ExprKind::If(if_box) => {
                let IfDecl{condition, body, else_branchs} = &**if_box;
                let cond = condition.node.to_string(tab);
                let mut output = vec![format!("if ({})", cond)];

                if !body.statments.is_empty() {
                    output.push(body.to_pretty(tab + 1, true));
                }

                for (i, e) in else_branchs.iter().enumerate() {
                    let last = i == else_branchs.len() - 1;
                    output.push(e.node.to_pretty(tab + 1, last));
                }

                output.join("\n")
            },
            ExprKind::Ctor(FnCall{callee, name, generics, arguments}) => {
                let generics = if generics.is_empty() {
                    String::new()
                }
                else {
                    format!("<{}>", generics.iter().map(|ty| ty.to_string()).join(","))
                };

                if let Some(methode) = callee {
                    format!("{}.{}{}({})", methode.node.to_string(tab), name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
                }
                else {
                    format!("{}{}({})", name.0, generics, arguments.iter().map(|arg| arg.to_string()).join(","))
                }
            },
            ExprKind::UnwrapVarDecl(var) => {
                match var.as_ref() {
                    VariableKind::Variable(node_ref) => {
                        let node = node_ref.borrow();
                        format!(
                            "<unwrap>{} {} = {}",
                            node.ty.to_string(),
                            node.name.0,
                            node.initializer.as_ref().map(|init| init.node.to_string(tab)).unwrap_or(String::new())
                        )
                    },
                    VariableKind::MultiVariable{vars, ty, initializer, ..} => format!(
                        "<unwrap>{}({}) = {}", 
                        ty.to_string(), 
                        vars.iter().map(|(name, var)| format!("{}: {}", name.0, var.borrow().name.0.clone())).join(","),
                        initializer.as_ref().map(|init| init.node.to_string(tab)).unwrap_or(String::new()),
                    ),
                }
            },
            ExprKind::ArrayFiller(ArrayFiller{amount, index, fill_expr}) => format!(
                "[for {} {} => {}]",
                index.as_ref().map(|el| format!("{} in", el.borrow().name.0)).unwrap_or(String::new()),
                amount.node.to_string(0),
                fill_expr.node.to_string(0),
            ),
        }
    }

    pub fn is_any_ref(&self) -> bool {
        match self {
            ExprKind::MutRef(..) |
            ExprKind::ConstRef(..) => true,

            _ => false,
        }
    }

    pub fn get_variant_name(&self) -> &'static str {
        match self {
            ExprKind::Empty => "<empty>",
            ExprKind::Default => "<default>",

            ExprKind::If(_) => "If",
            ExprKind::Ctor{..} => "Ctor",
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
            ExprKind::Lambda(_) => "Lambda",
            ExprKind::Literal(_) => "Literal",
            ExprKind::Ternary(_) => "Ternary",
            ExprKind::Variable(_) => "Valiable",
            ExprKind::ConstRef(_) => "ConstRef",
            ExprKind::NamedTuple(_) => "NamedTuple",
            ExprKind::ArrayFiller(_) => "ArrayFiller",
            ExprKind::StaticField(_) => "StaticField",
            ExprKind::StaticMethode(_) => "StaticCall",
            ExprKind::UnwrapVarDecl(_) => "UnwrapVarDecl",
        }
    }

}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeOfExpr {
    pub left: BoxExpr,
    pub ty: SoulType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnCall {
    pub callee: Option<BoxExpr>,
    pub name: Ident,
    pub generics: Vec<SoulType>,
    pub arguments: Vec<Arguments>,
}

impl FnCall {
    pub fn consume_to_static_methode(self, ty: Spanned<SoulType>) -> StaticMethode {
        StaticMethode{
            callee: ty, 
            name: self.name, 
            generics: self.generics, 
            arguments: self.arguments,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaDecl {
    pub signature: LambdaSignatureRef,
    pub arguments: Vec<Expression>,
    pub body: Block,
    pub capture: Capture,
} 

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Ternary {
    pub condition: BoxExpr,
    pub if_branch: BoxExpr,
    pub else_branch: BoxExpr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capture {
    pub variable: VariableRef,
    pub kind: CaptureKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureKind {
    ConstRef,
    MutRef,
    Consume,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticMethode {
    pub callee: Spanned<SoulType>,
    pub name: Ident,
    pub generics: Vec<SoulType>,
    pub arguments: Vec<Arguments>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Index {
    pub collection: BoxExpr,
    pub index: BoxExpr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UnaryExpr {
    pub operator: UnaryOp,
    pub expression: BoxExpr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Arguments {
    pub name: Option<Ident>,
    pub expression: Expression,
}

impl Arguments {
    pub fn to_string(&self) -> String {
        if let Some(optional) = &self.name {
            format!("{}: {}", optional.0, self.expression.node.to_string(0))
        }
        else {
            self.expression.node.to_string(0)
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOpKind {
    Invalid,
    Neg, // -
    Not, // !
    Incr{before_var: bool}, // ++
    Decr{before_var: bool}, // --
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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
            Self::Range   => SOUL_NAMES.get_name(NamesOperator::Range),
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




























