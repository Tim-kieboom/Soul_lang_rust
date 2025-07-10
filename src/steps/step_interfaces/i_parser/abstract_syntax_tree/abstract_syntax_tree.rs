use itertools::Itertools;

use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{pretty_format::PrettyFormat, spanned::Spanned, statment::{ClassDecl, EnumDecl, FunctionDecl, InterfaceDecl, Statment, StmtKind, StructDecl, TypeEnumDecl, UnionDecl, VariableDecl}};

pub struct AbstractSyntacTree {
    pub root: Vec<GlobalNode>,
}

pub enum GlobalNode {
    Statment(Statment),
    EnumDecl(Spanned<EnumDecl>),
    ClassDecl(Spanned<ClassDecl>),
    UnionDecl(Spanned<UnionDecl>),
    VarDecl(Spanned<VariableDecl>),
    StructDecl(Spanned<StructDecl>),
    FuncDecl(Spanned<FunctionDecl>),
    ExtFuncDecl(Spanned<FunctionDecl>),
    TypeEnumDecl(Spanned<TypeEnumDecl>),
    InterfaceDecl(Spanned<InterfaceDecl>),
}

impl AbstractSyntacTree {
    pub fn new() -> Self {
        Self { root: Vec::new() }
    }

    pub fn to_pretty_string(&self) -> String {
        self.root
            .iter()
            .map(|node| 
                // convert Function and VarDecl to Statment because i can not be asked to impl to_pretty_string in GlobalNode
                match node {
                    GlobalNode::VarDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::VarDecl(decl.clone()), span.clone()),
                    GlobalNode::FuncDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::FnDecl(decl.clone()), span.clone()),
                    GlobalNode::EnumDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::EnumDecl(decl.clone()), span.clone()),
                    GlobalNode::ClassDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::ClassDecl(decl.clone()), span.clone()),
                    GlobalNode::UnionDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::UnionDecl(decl.clone()), span.clone()),
                    GlobalNode::StructDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::StructDecl(decl.clone()), span.clone()),
                    GlobalNode::ExtFuncDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::ExtFnDecl(decl.clone()), span.clone()),
                    GlobalNode::TypeEnumDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::TypeEnumDecl(decl.clone()), span.clone()),
                    GlobalNode::InterfaceDecl(Spanned { node: decl, span }) => Statment::new(StmtKind::InterfaceDecl(decl.clone()), span.clone()),
                    GlobalNode::Statment(stmt) => stmt.clone(),
                }.to_pretty_string(0)
            )
            .join("\n")
    }
}


















