use itertools::Itertools;
use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{abstract_syntax_tree::GlobalKind, expression::{Expression, Ident}, soul_type::{soul_type::SoulType}, spanned::Spanned, staments::{conditionals::{ForDecl, IfDecl, WhileDecl}, enum_likes::{EnumDecl, TypeEnumDecl, UnionDecl}, function::{ExtFnDecl, FnDecl}, objects::{ClassDecl, StructDecl, TraitDeclRef, TraitImpl}}}, utils::node_ref::NodeRef};

pub type Statment = Spanned<StmtKind>;
pub type DeleteList = String;

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    ExprStmt(Expression),
    VarDecl(VariableRef),

    FnDecl(FnDecl),
    ExtFnDecl(ExtFnDecl),

    StructDecl(StructDecl),
    ClassDecl(ClassDecl),
    TraitDecl(TraitDeclRef),

    EnumDecl(EnumDecl),
    UnionDecl(UnionDecl),
    TypeEnumDecl(TypeEnumDecl),

    TraitImpl(TraitImpl),

    Return(Return),

    Assignment(Assignment),
    If(IfDecl),
    While(WhileDecl),
    For(ForDecl),
    Block(Block),
    CloseBlock(CloseBlock),
}

pub trait InStmtKind{fn to_stmt_kind(self) -> StmtKind;}
macro_rules! impl_in_stmt_kind {
    ($($variant:ident => $type:ty),* $(,)?) => { 
        $(
            impl InStmtKind for $type { 
                fn to_stmt_kind(self) -> StmtKind {
                    StmtKind::$variant(self)
                }
            }
        )* 
    }; 
}
impl_in_stmt_kind!(
    ExprStmt => Expression, 
    VarDecl => VariableRef, 
    FnDecl => FnDecl, 
    ExtFnDecl => ExtFnDecl, 
    StructDecl => StructDecl, 
    ClassDecl => ClassDecl, 
    TraitDecl => TraitDeclRef, 
    EnumDecl => EnumDecl, 
    UnionDecl => UnionDecl, 
    TypeEnumDecl => TypeEnumDecl, 
    TraitImpl => TraitImpl, 
    Return => Return,
    Assignment => Assignment, 
    If => IfDecl, 
    While => WhileDecl,
    Block => Block, 
    CloseBlock => CloseBlock
);

impl Spanned<StmtKind> {
    pub fn from_kind<T: InStmtKind>(value: T, span: SoulSpan) -> Statment {
        Spanned::new(value.to_stmt_kind(), span)
    }
}

impl StmtKind {
    pub fn consume_as_global_kind(self) -> Option<GlobalKind> {
        match self {
            StmtKind::ClassDecl(decl) => Some(GlobalKind::ClassDecl(decl)),
            StmtKind::StructDecl(decl) => Some(GlobalKind::StructDecl(decl)),
            StmtKind::TraitDecl(decl) => Some(GlobalKind::TraitDecl(decl)),
            StmtKind::TraitImpl(impl_block) => Some(GlobalKind::TraitImpl(impl_block)),
            StmtKind::FnDecl(decl) => Some(GlobalKind::FuncDecl(decl)),
            StmtKind::ExtFnDecl(decl) => Some(GlobalKind::ExtFuncDecl(decl)),
            StmtKind::VarDecl(decl) => Some(GlobalKind::VarDecl(decl)),
            StmtKind::UnionDecl(decl) => Some(GlobalKind::UnionDecl(decl)),
            StmtKind::EnumDecl(decl) => Some(GlobalKind::EnumDecl(decl)),
            StmtKind::TypeEnumDecl(decl) => Some(GlobalKind::TypeEnumDecl(decl)),
            _ => None,
        }
    }

    pub fn get_varaint_name(&self) -> &'static str {
        match self {
            StmtKind::ExprStmt(_) => "ExprStmt",
            StmtKind::VarDecl(_) => "VarDecl",
            StmtKind::FnDecl(_) => "FnDecl",
            StmtKind::ExtFnDecl(_) => "ExtFnDecl",
            StmtKind::StructDecl(_) => "StructDecl",
            StmtKind::ClassDecl(_) => "ClassDecl",
            StmtKind::TraitDecl(_) => "TraitDecl",
            StmtKind::EnumDecl(_) => "EnumDecl",
            StmtKind::UnionDecl(_) => "UnionDecl",
            StmtKind::TypeEnumDecl(_) => "TypeEnumDecl",
            StmtKind::TraitImpl(_) => "TraitImpl",
            StmtKind::Return(_) => "Return",
            StmtKind::Assignment(_) => "Assignment",
            StmtKind::If(_) => "If",
            StmtKind::While(_) => "While",
            StmtKind::Block(_) => "Block",
            StmtKind::CloseBlock(_) => "CloseBlock",
            StmtKind::For(_) => "for",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lifetime {
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Block {
    pub statments: Vec<Statment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Return {
    pub value: Option<Expression>
}

#[derive(Debug, Clone, PartialEq)]
pub struct CloseBlock {
    pub delete_list: Vec<DeleteList>
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub target: Expression,
    pub value: Expression,
}

pub type VariableRef = NodeRef<VariableDecl>;

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub name: Ident,
    pub ty: SoulType,
    pub initializer: Option<Box<Expression>>,
    /// if 'foo := 1' and foo is not mutated yet lit_retention is Some and and is used instead of var
    pub lit_retention: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoulThis {
    pub ty: SoulType, 
    pub this: Option<SoulType>,
}

impl SoulThis {
    pub fn to_string(&self) -> String {
        match &self.this {
            Some(this) => format!("{} this{}", self.ty.to_string(), this.wrappers.iter().map(|wrap| wrap.to_str()).join("")),
            None => format!("{}", self.ty.to_string()),
        }
    }
}















































