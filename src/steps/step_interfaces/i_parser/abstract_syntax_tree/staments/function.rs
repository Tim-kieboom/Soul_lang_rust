use itertools::Itertools;
use serde::{Deserialize, Serialize};
use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, generics::GenericParam, soul_type::{soul_type::SoulType, type_kind::Modifier}, spanned::Spanned, staments::statment::{Block, SoulThis, Statment, StmtKind}}, utils::serde_multi_ref::{MultiRef, MultiRefPool}};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FnDecl {
    pub signature: FunctionSignatureRef,
    pub body: Block, 
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExtFnDecl {
    pub signature: FunctionSignatureRef,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum FnDeclKind {
    InternalFn(FunctionSignatureRef),
    InternalCtor(FunctionSignatureRef),
    Fn(FnDecl),
    Ctor(FnDecl),
    ExtFn(ExtFnDecl),
}

impl FnDeclKind {

    pub fn consume_to_statment(self, span: SoulSpan) -> Statment {
        match self {
            FnDeclKind::Fn(fn_decl) => Statment::new(StmtKind::FnDecl(fn_decl), span),
            FnDeclKind::ExtFn(ext_fn_decl) => Statment::new(StmtKind::ExtFnDecl(ext_fn_decl), span),
            FnDeclKind::Ctor(fn_decl) => Statment::new(StmtKind::FnDecl(fn_decl), span),
            FnDeclKind::InternalFn(..) => panic!("trying to consume_to_statment but is internalfn"),
            FnDeclKind::InternalCtor(..) => panic!("trying to consume_to_statment but is internalCtor"),
        }
    }

    pub fn consume_signature(self) -> FunctionSignatureRef {
        match self {
            FnDeclKind::Fn(this) => this.signature,
            FnDeclKind::ExtFn(this) => this.signature,
            FnDeclKind::Ctor(this) => this.signature,
            FnDeclKind::InternalFn(..) => panic!("trying to consume_signature but is internalfn"),
            FnDeclKind::InternalCtor(..) => panic!("trying to consume_signature but is internalCtor"),
        }
    }

    pub fn consume_body(self) -> Block {
        match self {
            FnDeclKind::Fn(this) => this.body,
            FnDeclKind::ExtFn(this) => this.body,
            FnDeclKind::Ctor(this) => this.body,
            FnDeclKind::InternalFn(..) => panic!("trying to consume_signature but is internalfn"),
            FnDeclKind::InternalCtor(..) => panic!("trying to consume_signature but is internalCtor"),
        }
    }

    pub fn get_signature(&self) -> &FunctionSignatureRef {
        match self {
            FnDeclKind::Fn(this) => &this.signature,
            FnDeclKind::ExtFn(this) => &this.signature,
            FnDeclKind::Ctor(this) => &this.signature,
            FnDeclKind::InternalFn(fn_ref) => &fn_ref,
            FnDeclKind::InternalCtor(fn_ref) => &fn_ref,
        }
    }

    pub fn get_body(&self) -> &Block {
        match self {
            FnDeclKind::Fn(this) => &this.body,
            FnDeclKind::ExtFn(this) => &this.body,
            FnDeclKind::Ctor(this) => &this.body,
            FnDeclKind::InternalFn(..) => panic!("trying to get_body but is internalfn"),
            FnDeclKind::InternalCtor(..) => panic!("trying to get_body but is internalCtor"),
        }
    }

    pub fn get_body_mut(&mut self) -> &mut Block {
        match self {
            FnDeclKind::Fn(this) => &mut this.body,
            FnDeclKind::ExtFn(this) => &mut this.body,
            FnDeclKind::Ctor(this) => &mut this.body,
            FnDeclKind::InternalFn(..) => panic!("trying to get_body but is internalfn"),
            FnDeclKind::InternalCtor(..) => panic!("trying to get_body but is internalCtor"),
        }
    }

    pub fn get_modifier(&self, ref_pool: &MultiRefPool) -> Modifier {
        self.get_signature().borrow(ref_pool).node.modifier.clone()
    }
}

pub type FunctionSignatureRef = MultiRef<Spanned<InnerFunctionSignature>>; 
pub type LambdaSignatureRef = MultiRef<InnerLambdaSignature>;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LambdaMode {
    Mut,
    Const,
    Consume,
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerLambdaSignature {
    pub params: Vec<Spanned<Parameter>>,
    pub return_type: Option<SoulType>,
    pub mode: LambdaMode, 
}

impl LambdaSignatureRef {
    pub fn to_type_string(&self, ref_pool: &MultiRefPool) -> String {
        let this = self.borrow(ref_pool);
        format!(
            "{}<{}>{}",
            this.mode.get_lambda_name(),
            this.params.iter().map(|el| el.node.ty.to_string(ref_pool)).join(","),
            this.return_type.as_ref().map(|el| el.to_string(ref_pool)).unwrap_or("".into())
        )
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerFunctionSignature {
    pub name: Ident,
    /// Some() = an extension method
    pub calle: Option<Spanned<SoulThis>>, 
    pub generics: Vec<GenericParam>,
    pub params: Vec<Spanned<Parameter>>,
    pub return_type: Option<SoulType>,
    ///default = normal function, const = functional(can be compileTime), Literal = comileTime 
    pub modifier: Modifier,
}

impl FunctionSignatureRef {
    pub fn to_string(&self, ref_pool: &MultiRefPool) -> String {
        let this = &match self.try_borrow(ref_pool) {
            Ok(val) => val,
            Err(_) => return String::new(),
        }.node;
        if this.generics.is_empty() {
            format!(
                "{}{}({}){}", 
                this.calle.as_ref().map(|ty| format!("{} ", ty.node.to_string(ref_pool))).unwrap_or("".to_string()),
                this.name.0, 
                this.params.iter().map(|par| par.node.to_string(ref_pool)).join(","),
                this.return_type.as_ref().map(|ty| format!("{} ", ty.to_string(ref_pool))).unwrap_or("".to_string()),
            )
        }
        else {
            format!(
                "{}{}<{}>({}){}", 
                this.calle.as_ref().map(|ty| format!("{} ", ty.node.to_string(ref_pool))).unwrap_or("".to_string()),
                this.name.0, 
                this.generics.iter().map(|gene| gene.to_string(ref_pool)).join(","), 
                this.params.iter().map(|par| par.node.to_string(ref_pool)).join(","),
                this.return_type.as_ref().map(|ty| format!("{} ", ty.to_string(ref_pool))).unwrap_or("".to_string()),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: Ident,
    pub ty: SoulType,
    pub default_value: Option<Expression>,
}

impl Parameter {
    pub fn to_string(&self, ref_pool: &MultiRefPool) -> String {
        match &self.default_value {
            Some(value) => format!("{} {} = {}", self.ty.to_string(ref_pool), self.name.0, value.node.to_string(ref_pool, 0)),
            None => format!("{} {}", self.ty.to_string(ref_pool), self.name.0),
        }
    }
}




























