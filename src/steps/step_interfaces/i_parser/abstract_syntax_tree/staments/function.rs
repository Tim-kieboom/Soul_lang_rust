use itertools::Itertools;

use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, generics::GenericParam, soul_type::{soul_type::SoulType, type_kind::Modifier}, spanned::Spanned, staments::statment::{Block, SoulThis, Statment, StmtKind}}, utils::node_ref::NodeRef};

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub signature: FunctionSignatureRef,
    pub body: Block, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtFnDecl {
    pub signature: FunctionSignatureRef,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnDeclKind {
    InternalFn(FunctionSignatureRef),
    Fn(FnDecl),
    ExtFn(ExtFnDecl),
}

impl FnDeclKind {

    pub fn consume_to_statment(self, span: SoulSpan) -> Statment {
        match self {
            FnDeclKind::Fn(fn_decl) => Statment::new(StmtKind::FnDecl(fn_decl), span),
            FnDeclKind::ExtFn(ext_fn_decl) => Statment::new(StmtKind::ExtFnDecl(ext_fn_decl), span),
            FnDeclKind::InternalFn(..) => panic!("trying to consume_to_statment but is internalfn"),
        }
    }

    pub fn consume_signature(self) -> FunctionSignatureRef {
        match self {
            FnDeclKind::Fn(this) => this.signature,
            FnDeclKind::ExtFn(this) => this.signature,
            FnDeclKind::InternalFn(..) => panic!("trying to consume_signature but is internalfn"),
        }
    }

    pub fn consume_body(self) -> Block {
        match self {
            FnDeclKind::Fn(this) => this.body,
            FnDeclKind::ExtFn(this) => this.body,
            FnDeclKind::InternalFn(..) => panic!("trying to consume_signature but is internalfn"),
        }
    }

    pub fn get_signature(&self) -> &FunctionSignatureRef {
        match self {
            FnDeclKind::Fn(this) => &this.signature,
            FnDeclKind::ExtFn(this) => &this.signature,
            FnDeclKind::InternalFn(fn_ref) => &fn_ref,
        }
    }

    pub fn get_body(&self) -> &Block {
        match self {
            FnDeclKind::Fn(this) => &this.body,
            FnDeclKind::ExtFn(this) => &this.body,
            FnDeclKind::InternalFn(..) => panic!("trying to get_body but is internalfn"),
        }
    }

    pub fn get_modifier(&self) -> Modifier {
        self.get_signature().borrow().modifier.clone()
    }
}

pub type FunctionSignatureRef = NodeRef<InnerFunctionSignature>; 

#[derive(Debug, Clone, PartialEq)]
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
    pub fn to_string(&self) -> String {
        let this = self.borrow();
        if this.generics.is_empty() {
            format!(
                "{}{}({}){}", 
                this.calle.as_ref().map(|ty| format!("{} ", ty.node.to_string())).unwrap_or("".to_string()),
                this.name.0, 
                this.params.iter().map(|par| par.node.to_string()).join(","),
                this.return_type.as_ref().map(|ty| format!("{} ", ty.to_string())).unwrap_or("".to_string()),
            )
        }
        else {
            format!(
                "{}{}<{}>({}){}", 
                this.calle.as_ref().map(|ty| format!("{} ", ty.node.to_string())).unwrap_or("".to_string()),
                this.name.0, 
                this.generics.iter().map(|gene| gene.to_string()).join(","), 
                this.params.iter().map(|par| par.node.to_string()).join(","),
                this.return_type.as_ref().map(|ty| format!("{} ", ty.to_string())).unwrap_or("".to_string()),
            )
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Ident,
    pub ty: SoulType,
    pub default_value: Option<Expression>,
}

impl Parameter {
    pub fn to_string(&self) -> String {
        match &self.default_value {
            Some(value) => format!("{} {} = {}", self.ty.to_string(), self.name.0, value.node.to_string()),
            None => format!("{} {}", self.ty.to_string(), self.name.0),
        }
    }
}




























