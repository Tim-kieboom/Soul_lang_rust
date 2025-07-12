use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{spanned::Spanned, statment::{ClassDecl, EnumDecl, FunctionDecl, ImplBlock, InterfaceDecl, StmtKind, StructDecl, TraitDecl, TypeEnumDecl, UnionDecl, VariableDecl}};


pub struct AbstractSyntacTree {
    pub root: Vec<GlobalNode>,
}

pub type GlobalNode = Spanned<GlobalKind>;

#[derive(Debug, Clone)]
pub enum GlobalKind {
    ClassDecl(ClassDecl),
    StructDecl(StructDecl),
    
    TraitDecl(TraitDecl),
    TraitImpl(ImplBlock),
    InterfaceDecl(InterfaceDecl),
    
    FuncDecl(FunctionDecl),
    ExtFuncDecl(FunctionDecl),
    VarDecl(VariableDecl),
    
    UnionDecl(UnionDecl),
    EnumDecl(EnumDecl),
    TypeEnumDecl(TypeEnumDecl),
}

impl GlobalKind {
    pub fn consume_as_stmt_kind(self) -> StmtKind {
        match self {
            GlobalKind::ClassDecl(decl) => StmtKind::ClassDecl(decl),
            GlobalKind::StructDecl(decl) => StmtKind::StructDecl(decl),
            GlobalKind::TraitDecl(decl) => StmtKind::TraitDecl(decl),
            GlobalKind::TraitImpl(impl_block) => StmtKind::TraitImpl(impl_block),
            GlobalKind::InterfaceDecl(decl) => StmtKind::InterfaceDecl(decl),
            GlobalKind::FuncDecl(decl) => StmtKind::FnDecl(decl),
            GlobalKind::ExtFuncDecl(decl) => StmtKind::ExtFnDecl(decl),
            GlobalKind::VarDecl(decl) => StmtKind::VarDecl(decl),
            GlobalKind::UnionDecl(decl) => StmtKind::UnionDecl(decl),
            GlobalKind::EnumDecl(decl) => StmtKind::EnumDecl(decl),
            GlobalKind::TypeEnumDecl(decl) => StmtKind::TypeEnumDecl(decl),
        }
    }
}

impl AbstractSyntacTree {
    pub fn new() -> Self {
        Self { root: Vec::new() }
    }

    pub fn to_pretty_string(&self) -> String {
        todo!()
    }
}

