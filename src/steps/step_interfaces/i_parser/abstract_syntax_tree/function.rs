use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{BoxExpression, Ident, NamedTuple, Tuple, VariableName}, generic::GenericParameter, soul_type::soul_type::{Modifier, SoulType}, spanned::Spanned, statement::Block};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Function {
    pub signature: FunctionSignature,
    pub block: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Lambda {
    pub signature: LambdaSignature,
    pub arguments: Tuple,
    pub body: LambdaBody,
    pub capture: Capture,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LambdaBody {
    Block(Block),
    Expression(BoxExpression),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCall {
    pub name: Ident,
    pub callee: Option<BoxExpression>,
    pub generics: Vec<SoulType>,
    pub arguments: Tuple,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StructConstructor {
    pub calle: SoulType,
    pub arguments: NamedTuple,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct StaticMethod {
    pub callee: Spanned<SoulType>,
    pub name: Ident,
    pub generics: Vec<SoulType>,
    pub arguments: Tuple,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionSignature {
    pub name: Ident,
    pub callee: Option<Spanned<FunctionCallee>>,
    pub generics: Vec<GenericParameter>,
    pub parameters: Vec<Spanned<Parameter>>,
    pub ruleset: Modifier,
    pub return_type: Option<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FunctionCallee {
    pub extention_type: SoulType,
    pub this: Option<SoulType>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Ident,
    pub ty: SoulType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LambdaSignature {
    pub params: Vec<Spanned<Parameter>>,
    pub return_type: Option<Box<SoulType>>,
    pub mode: LambdaMode, 
    pub has_return: bool,
}

impl LambdaSignature {
    pub fn to_type_string(&self) -> String {
        format!(
            "{}<{}>{}",
            self.mode.get_lambda_name(),
            self.params.iter().map(|el| el.node.ty.to_string()).join(","),
            self.return_type.as_ref().map(|el| el.to_string()).unwrap_or("".into())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capture {
    pub variable: VariableName,
    pub kind: CaptureKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CaptureKind {
    ConstRef,
    MutRef,
    Consume,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LambdaMode {
    Mut,
    Const,
    Consume,
}

impl FunctionCallee {
    pub fn new_static(ty: SoulType) -> Self {
        Self{extention_type: ty, this: None}
    }

    pub fn new(ty: SoulType, this: Option<SoulType>) -> Self {
        Self{extention_type: ty, this}
    }
}

impl LambdaMode {
    pub fn get_lambda_name(&self) -> &'static str {
        match self {
            LambdaMode::Mut => "MutFn",
            LambdaMode::Const => "ConstFn",
            LambdaMode::Consume => "OnceFn",
        } 
    }
}








