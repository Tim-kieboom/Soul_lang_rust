use serde::{Deserialize, Serialize};

use crate::{steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Array, ArrayFiller, BinaryExpr, Expression, ExternalExpression, Field, FnCall, Ident, Index, LambdaDecl, NamedTuple, StaticField, StaticMethode, Ternary, Tuple, TypeOfExpr, UnaryExpr, Variable}, literal::Literal, soul_type::soul_type::SoulType, spanned::Spanned, staments::{conditionals::IfDecl, enum_likes::{EnumDeclRef, InnerEnumDecl, InnerTypeEnumDecl, InnerUnionDecl, TypeEnumDeclRef, UnionDeclRef}, function::{ExtFnDecl, FnDecl}, objects::{InnerClassDecl, InnerStructDecl, InnerTraitDecl, TraitImpl}, statment::{Assignment, Block, DeleteList, ReturnKind, ReturnLike, VariableDecl, VariableKind}}}, utils::serde_multi_ref::SerdeMultiRef};

pub struct SerdeAbstractSyntacTree {

}










