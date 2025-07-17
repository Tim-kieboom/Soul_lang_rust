use std::{cell::{Ref, RefCell, RefMut}, rc::Rc};

use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_parser::abstract_syntax_tree::{abstract_syntax_tree::GlobalKind, expression::{Expression, Ident}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::{EnumVariant, Modifier, UnionVariant}}, spanned::Spanned}};

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
    TraitDecl(TraitDecl),

    EnumDecl(EnumDecl),
    UnionDecl(UnionDecl),
    TypeEnumDecl(TypeEnumDecl),

    TraitImpl(TraitImpl),

    Return(Return),

    Assignment(Assignment),
    If(IfDecl),
    While(WhileDecl),
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
    TraitDecl => TraitDecl, 
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

#[derive(Debug, Clone, PartialEq)]
pub struct WhileDecl {
    pub condition: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfDecl {
    pub condition: Expression,
    pub then_branch: Block,
    pub else_branch: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub name: Ident,
    pub methods: Vec<FunctionSignature>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub name: Ident,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub name: Ident,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnumDecl {
    pub name: Ident,
    pub types: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitImpl {
    pub trait_name: Ident,
    pub for_type: SoulType,
    pub methods: Vec<FnDecl>,
}

pub type VariableRef = NodeRef<VariableDecl>;

#[derive(Debug, Clone, PartialEq)]
pub struct NodeRef<T> {
    inner: Rc<RefCell<T>>
}

impl<T> NodeRef<T> {
    pub fn new(var: T) -> Self {
        Self { inner: Rc::new(RefCell::new(var)) }
    }

    pub fn borrow(&self) -> Ref<T> {
        self.inner.borrow()
    } 

    pub fn borrow_mut(&mut self) -> RefMut<T> {
        self.inner.borrow_mut()
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub name: Ident,
    pub ty: SoulType,
    pub initializer: Option<Box<Expression>>,
    /// if 'foo := 1' and foo is not mutated yet lit_retention is Some and and is used instead of var
    pub lit_retention: Option<Literal>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub signature: FunctionSignature,
    pub body: Block,
    ///default = normal function, const = functional(can be compileTime), Literal = comileTime 
    pub modifier: Modifier, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtFnDecl {
    pub signature: FunctionSignature,
    pub body: Block,
    ///default = normal function, const = functional(can be compileTime), Literal = comileTime 
    pub modifier: Modifier, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: Ident,
    /// Some() = an extension method
    pub receiver: Option<SoulType>, 
    pub params: Vec<Parameter>,
    pub return_type: Option<SoulType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
    pub name: Ident,
    pub ty: SoulType,
    pub default_value: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub implements: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub signature: ClassSignature,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<Spanned<FunctionSignature>>,
    pub implements: Vec<SoulType>,
}

pub type ClassSignature = Ident;

#[derive(Debug, Clone, PartialEq)]
pub struct FieldDecl {
    pub name: Ident,
    pub ty: SoulType,
    pub default_value: Option<Expression>,
    pub vis: FieldAccess
}
 
#[derive(Debug, Clone, PartialEq)]
pub struct FieldAccess {
    /// None = use default (e.g. pub)
    pub get: Option<Visibility>, 
    // None = disallow set
    pub set: Option<Visibility>, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
}

#[derive(Debug, Clone, PartialEq)]
pub struct GenericParam {
    pub name: Ident,
    pub constraint: Vec<TypeConstraint>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    Trait(Ident),
    Interface(Ident),
    TypeEnum(Ident),
}



























