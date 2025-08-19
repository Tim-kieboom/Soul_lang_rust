use crate::prelude::*;
use my_macros::{CloneWithPool};
use serde::{Deserialize, Serialize};
use crate::{errors::soul_error::{new_soul_error, Result, SoulErrorKind}, steps::step_interfaces::i_parser::abstract_syntax_tree::{spanned::Spanned, staments::{enum_likes::{EnumDeclRef, TypeEnumDeclRef, UnionDeclRef}, function::{ExtFnDecl, FnDecl}, objects::{ClassDeclRef, StructDeclRef, TraitDeclRef, TraitImpl}, statment::{Block, Statment, StmtKind, VariableKind}}}, utils::serde_multi_ref::{MultiRef, MultiRefPool}};

#[derive(Debug, Clone, CloneWithPool, Serialize, Deserialize)]
pub struct AbstractSyntacTree {
    pub root: Vec<GlobalNode>,
}

pub type GlobalNode = Spanned<GlobalKind>;

#[derive(Debug)]
pub enum StatmentBuilder {
    Global(MultiRef<Vec<GlobalNode>>),
    Block(MultiRef<Spanned<Block>>),
}

#[derive(Debug, Clone, CloneWithPool, Serialize, Deserialize)]
pub enum GlobalKind {
    ClassDecl(ClassDeclRef),
    StructDecl(StructDeclRef),
    
    TraitDecl(TraitDeclRef),
    TraitImpl(TraitImpl),
    
    FuncDecl(FnDecl),
    ExtFuncDecl(ExtFnDecl),
    VarDecl(VariableKind),
    
    UnionDecl(UnionDeclRef),
    EnumDecl(EnumDeclRef),
    TypeEnumDecl(TypeEnumDeclRef),
}

impl StatmentBuilder {
    pub fn try_push(&mut self, ref_pool: &mut MultiRefPool, stament: Statment) -> Result<()> {
        match self {
            StatmentBuilder::Global(node_ref) => {
                let name = stament.node.get_varaint_name();
                let global_node = stament.node.consume_as_global_kind();
                if let Some(node) = global_node {
                    node_ref.borrow_mut(ref_pool).push(GlobalNode::new(node, stament.span));
                    return Ok(());
                } 

                Err(new_soul_error(SoulErrorKind::InvalidInContext, stament.span, format!("{} is not a valid global statment (only use this type of statment contexts of function, class, ect..)", name)))
            },
            StatmentBuilder::Block(node_ref) => {
                node_ref.borrow_mut(ref_pool).node.statments.push(stament);
                Ok(())
            },
        }
    }
}

impl GlobalKind {
    pub fn consume_as_stmt_kind(self) -> StmtKind {
        match self {
            GlobalKind::ClassDecl(decl) => StmtKind::ClassDecl(decl),
            GlobalKind::StructDecl(decl) => StmtKind::StructDecl(decl),
            GlobalKind::TraitDecl(decl) => StmtKind::TraitDecl(decl),
            GlobalKind::TraitImpl(impl_block) => StmtKind::TraitImpl(impl_block),
            GlobalKind::FuncDecl(decl) => StmtKind::FnDecl(decl),
            GlobalKind::ExtFuncDecl(decl) => StmtKind::ExtFnDecl(decl),
            GlobalKind::VarDecl(decl) => StmtKind::VarDecl(decl),
            GlobalKind::UnionDecl(decl) => StmtKind::UnionDecl(decl),
            GlobalKind::EnumDecl(decl) => StmtKind::EnumDecl(decl),
            GlobalKind::TypeEnumDecl(decl) => StmtKind::TypeEnumDecl(decl),
        }
    }

    pub fn get_varaint_name(&self) -> &'static str {
        match self {
            GlobalKind::VarDecl(_) => "VarDecl",
            GlobalKind::StructDecl(_) => "StructDecl",
            GlobalKind::ClassDecl(_) => "ClassDecl",
            GlobalKind::TraitDecl(_) => "TraitDecl",
            GlobalKind::EnumDecl(_) => "EnumDecl",
            GlobalKind::UnionDecl(_) => "UnionDecl",
            GlobalKind::TypeEnumDecl(_) => "TypeEnumDecl",
            GlobalKind::TraitImpl(_) => "TraitImpl",
            GlobalKind::FuncDecl(_) => "FnDecl",
            GlobalKind::ExtFuncDecl(_) => "ExtFnDecl",
        }
    }
}

impl AbstractSyntacTree {
    pub fn new() -> Self {
        Self { root: Vec::new() }
    }
}

