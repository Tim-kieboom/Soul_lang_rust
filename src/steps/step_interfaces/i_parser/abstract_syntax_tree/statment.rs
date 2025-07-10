use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Expression, Ident}, soul_type::{EnumVariant, SoulType, UnionVariant}, spanned::Spanned};

pub type Statment = Spanned<StmtKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum StmtKind {
    ExprStmt(Expression),
    VarDecl(VariableDecl),

    FnDecl(FunctionDecl),
    ExtFnDecl(FunctionDecl),

    StructDecl(StructDecl),
    ClassDecl(ClassDecl),
    TraitDecl(TraitDecl),
    InterfaceDecl(InterfaceDecl),

    EnumDecl(EnumDecl),
    UnionDecl(UnionDecl),
    TypeEnumDecl(TypeEnumDecl),

    TraitImpl(ImplBlock),

    Return(Option<Expression>),

    Assignment {
        target: Expression,
        value: Expression
    },
    If {
        condition: Expression,
        then_branch: Vec<Statment>,
        else_branch: Option<Vec<Statment>>,
    },
    While {
        condition: Expression,
        body: Vec<Statment>,
    },
    Block(Vec<Statment>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceDecl {
    pub signature: InterfaceSignature,
    pub methods: Vec<FunctionSignature>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InterfaceSignature {
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitDecl {
    pub signature: TraitSignature,
    pub methods: Vec<FunctionSignature>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TraitSignature {
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumDecl {
    pub signature: EnumSignature,
    pub variants: Vec<EnumVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct EnumSignature {
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionDecl {
    pub signature: UnionSignature,
    pub variants: Vec<UnionVariant>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UnionSignature {
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnumDecl {
    pub signature: TypeEnumSignature,
    pub types: Vec<SoulType>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TypeEnumSignature {
    pub name: Ident,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImplBlock {
    pub trait_name: Ident,
    pub for_type: SoulType,
    pub methods: Vec<FunctionDecl>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDecl {
    pub name: Ident,
    pub ty: Option<SoulType>,
    pub initializer: Option<Box<Expression>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDecl {
    pub signature: FunctionSignature,
    pub body: Vec<Statment>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionSignature {
    pub name: Ident,
    pub receiver: Option<SoulType>, // Some() = an extension method
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
    pub name: Ident,
    pub generics: Vec<GenericParam>,
    pub fields: Vec<FieldDecl>,
    pub methods: Vec<Spanned<FunctionSignature>>,
    pub implements: Vec<SoulType>,
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
    pub get: Option<Visibility>, // None = use default (e.g. pub)
    pub set: Option<Visibility>, // None = disallow set
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










