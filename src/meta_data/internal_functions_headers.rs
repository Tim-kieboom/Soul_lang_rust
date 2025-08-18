use std::{collections::HashMap, fs, io, path::PathBuf};

use once_cell::sync::Lazy;

use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{ExprKind, Ident}, generics::{GenericKind, GenericParam}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::{Modifier, TypeKind, TypeWrapper}}, spanned::Spanned, staments::{function::{FnDeclKind, FunctionSignatureRef, InnerFunctionSignature, Parameter}, objects::{FieldDecl, InnerStructDecl, StructDeclRef}}, visibility::FieldAccess}, external_header::Header, scope::{OverloadedFunctions, ScopeKind}}, i_sementic::sementic_scope::Byte}, utils::serde_multi_ref::MultiRefPool};

pub const INTERNAL_LIB_DIR: &str = "F:\\Code\\Github\\Soul_lang_rust\\output\\stdlibs";

pub static INTERNAL_FUNCTIONS: Lazy<&[FunctionSignatureRef]> = Lazy::new(|| &[
    
]);

static GENERIC_T: Lazy<GenericParam> = Lazy::new(|| GenericParam{name: Ident("T".into()), constraint: vec![], kind: GenericKind::Type, default: None});

fn get_fmt_ref_pool() -> MultiRefPool {
    MultiRefPool::new()
}

fn get_fmt_header(ref_pool: &mut MultiRefPool) -> Header {

    let span = SoulSpan::new(0,0,0);

    let format_args = ScopeKind::Functions(OverloadedFunctions::from_internal_fn(FunctionSignatureRef::new(Spanned::new(InnerFunctionSignature{
        name: Ident("FormatArgs".into()), 
        calle: None, 
        params: vec![Spanned::new(Parameter{name: Ident("args".into()), ty: SoulType::from_type_kind(TypeKind::Str).with_wrappers(vec![TypeWrapper::Array, TypeWrapper::ConstRef(None)]), default_value: None}, span)], 
        generics: vec![], 
        modifier: Modifier::Default, 
        return_type: Some(SoulType::from_type_kind(TypeKind::Str)), 
    }, span), ref_pool), ref_pool));

    let arg_struct = ScopeKind::Struct(StructDeclRef::new(InnerStructDecl{
        name: Ident("Arg".into()), 
        generics: vec![GENERIC_T.clone()], 
        fields: vec![
            Spanned::new(FieldDecl{ 
                name: Ident("value".into()), 
                ty: SoulType::from_type_kind(TypeKind::Generic(Ident("T".into()))),
                default_value: None, 
                vis: FieldAccess::new_public(),
            },span),
            Spanned::new(FieldDecl { 
                name: Ident("pretty".into()), 
                ty: SoulType::from_type_kind(TypeKind::Bool), 
                default_value: None, 
                vis: FieldAccess::new_public(), 
            }, span),
        ],
        size: Byte(0),
    }, ref_pool));

    let arg_ctor = ScopeKind::Functions(OverloadedFunctions::new(vec![FnDeclKind::InternalCtor(FunctionSignatureRef::new(Spanned::new(InnerFunctionSignature { 
        name: Ident("Arg".into()), 
        calle: None, 
        generics: vec![GENERIC_T.clone()], 
        params: vec![
            Spanned::new(Parameter{
                name: Ident("value".into()), 
                ty: SoulType::from_type_kind(TypeKind::Generic(Ident("T".into()))), 
                default_value: None,
            },span),
            Spanned::new(Parameter{
                name: Ident("pretty".into()), 
                ty: SoulType::from_type_kind(TypeKind::Bool), 
                default_value: Some(Spanned::new(ExprKind::Literal(Literal::Bool(false)), span)),
            },span),
        ],
        return_type: None, 
        modifier: Modifier::Const, 
    }, span), ref_pool))], ref_pool));

    Header{
        scope: HashMap::from([
            ("FormatArgs".into(), vec![format_args]),
            ("Arg".into(), vec![arg_struct, arg_ctor]),
        ]), 
        types: HashMap::from([

        ])
    }
}


pub fn load_std_headers() -> io::Result<()> {
    let mut fmt_path = {
        let mut path = PathBuf::from(INTERNAL_LIB_DIR);
        path.push("std");
        path
    };
    
    let mut ref_pool = get_fmt_ref_pool();
    let fmt_bin = bincode::serialize(&get_fmt_header(&mut ref_pool).to_serde_header(&mut ref_pool))
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err.to_string()))?;
    fs::create_dir_all(&fmt_path)?;
    fmt_path.push("fmt.soul.header");
    fs::write(fmt_path, fmt_bin)?;

    Ok(())
}












































