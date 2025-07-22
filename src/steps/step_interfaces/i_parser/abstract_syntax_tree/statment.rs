use itertools::Itertools;
use std::sync::{Arc, RwLock, RwLockReadGuard, RwLockWriteGuard};
use crate::{errors::soul_error::{SoulSpan}, steps::step_interfaces::i_parser::abstract_syntax_tree::{abstract_syntax_tree::GlobalKind, expression::{Expression, Ident}, soul_type::{soul_type::SoulType, type_kind::{EnumVariant, Modifier, UnionVariant}}, spanned::Spanned}};

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
pub struct ForDecl {
    pub element: Ident,
    pub collection: Expression,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfDecl {
    pub condition: Expression,
    pub body: Block,
    pub else_branchs: Vec<ElseKind>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElseKind {
    ElseIf(Box<IfDecl>),
    Else(Block)
}

pub type TraitDeclRef = NodeRef<InnerTraitDecl>;

#[derive(Debug, Clone, PartialEq)]
pub struct InnerTraitDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub methodes: Vec<FunctionSignatureRef>,
    pub implements: Vec<Ident>,
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
    pub methodes: Vec<FnDecl>,
}

pub type VariableRef = NodeRef<VariableDecl>;

#[derive(Debug, Clone)]
pub struct NodeRef<T> 
{
    inner: Arc<RwLock<T>>
}

impl<T> NodeRef<T> 
{
    pub fn new(var: T) -> Self {
        Self { inner: Arc::new(RwLock::new(var)) }
    }

    pub fn borrow(&self) -> RwLockReadGuard<T> {
        self.inner.read().unwrap()
    } 

    pub fn borrow_mut(&self) -> RwLockWriteGuard<T> {
        self.inner.write().unwrap()
    } 

    pub fn consume(self) -> T {
        unsafe { Arc::try_unwrap(self.inner)
            .inspect_err(|_| panic!("internal error consumed nodeRef without this ref being the only owner")).unwrap_unchecked()
            .into_inner().unwrap() }
    }
}

impl<T> PartialEq for NodeRef<T> {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.inner, &other.inner)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub name: Ident,
    pub ty: SoulType,
    pub initializer: Option<Box<Expression>>,
    /// if 'foo := 1' and foo is not mutated yet lit_retention is Some and and is used instead of var
    pub lit_retention: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnDecl {
    pub signature: FunctionSignatureRef,
    pub body: Block,
    ///default = normal function, const = functional(can be compileTime), Literal = comileTime 
    pub modifier: Modifier, 
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExtFnDecl {
    pub signature: FunctionSignatureRef,
    pub body: Block,
    ///default = normal function, const = functional(can be compileTime), Literal = comileTime 
    pub modifier: Modifier, 
}

#[derive(Debug, Clone, PartialEq)]
pub enum FnDeclKind {
    Fn(FnDecl),
    ExtFn(ExtFnDecl),
}

impl FnDeclKind {
    pub fn consume_to_statment(self, span: SoulSpan) -> Statment {
        match self {
            FnDeclKind::Fn(fn_decl) => Statment::new(StmtKind::FnDecl(fn_decl), span),
            FnDeclKind::ExtFn(ext_fn_decl) => Statment::new(StmtKind::ExtFnDecl(ext_fn_decl), span),
        }
    } 
    pub fn consume_signature(self) -> FunctionSignatureRef {
        match self {
            FnDeclKind::Fn(this) => this.signature,
            FnDeclKind::ExtFn(this) => this.signature,
        }
    }

    pub fn get_signature(&self) -> &FunctionSignatureRef {
        match self {
            FnDeclKind::Fn(this) => &this.signature,
            FnDeclKind::ExtFn(this) => &this.signature,
        }
    }

    pub fn get_body(&self) -> &Block {
        match self {
            FnDeclKind::Fn(this) => &this.body,
            FnDeclKind::ExtFn(this) => &this.body,
        }
    }

    pub fn get_modifier(&self) -> &Modifier {
        match self {
            FnDeclKind::Fn(this) => &this.modifier,
            FnDeclKind::ExtFn(this) => &this.modifier,
        }
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
}

#[derive(Debug, Clone, PartialEq)]
pub struct SoulThis {
    pub ty: SoulType, 
    pub this: Option<SoulType>,
}

impl SoulThis {
    pub fn to_string(&self) -> String {
        match &self.this {
            Some(this) => format!("{} this{}", self.ty.to_string(), this.wrapper.iter().map(|wrap| wrap.to_str()).join("")),
            None => format!("{}", self.ty.to_string()),
        }
    }
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

#[derive(Debug, Clone, PartialEq)]
pub struct StructDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub implements: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassDecl {
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub methodes: Vec<Spanned<FunctionSignatureRef>>,
    pub implements: Vec<Ident>,
}

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

impl Default for FieldAccess  {
    fn default() -> Self {
        Self { get: None, set: None }
    }
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
    pub default: Option<SoulType>,
}

impl GenericParam {
    pub fn to_string(&self) -> String {
        let str = match self.constraint.is_empty() {
            true => format!("{}", self.name.0),
            false => format!("{}: {}", self.name.0, self.constraint.iter().map(|ty| ty.to_string()).join("+")),
        };

        match &self.default {
            Some(val) => format!("{} = {}", str, val.to_string()),
            None => str,
        }
    } 
}

#[derive(Debug, Clone, PartialEq)]
pub enum TypeConstraint {
    Trait(Ident),
    TypeEnum(Vec<SoulType>),
}

impl TypeConstraint {
    pub fn to_string(&self) -> String {
        match self {
            TypeConstraint::Trait(ident) => ident.0.clone(),
            TypeConstraint::TypeEnum(soul_types) => format!("typeof[{}]", soul_types.iter().map(|ty| ty.to_string()).join(",")),
        }
    }
}



























