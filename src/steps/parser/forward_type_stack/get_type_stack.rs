use once_cell::sync::Lazy;

use crate::soul_names::{NamesInternalType, NamesOtherKeyWords, SOUL_NAMES};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::parser::get_statments::parse_type_enum::get_type_enum_body;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{TypeKind, TypeSize};
use crate::steps::step_interfaces::i_parser::scope::{ScopeVisibility, TypeScopeStack};
use crate::steps::step_interfaces::i_tokenizer::TokenStream;

pub fn forward_declarde_type_stack(stream: &mut TokenStream) -> Result<TypeScopeStack> {
    let mut types = TypeScopeStack::new();
    add_default_type_kind(&mut types);

    loop {

        parse_type(&mut types, stream)?;

        if stream.next().is_none() {
            break;
        }
    }

    Ok(types)
}

fn parse_type(types: &mut TypeScopeStack, stream: &mut TokenStream) -> Result<()> {
    
    if stream.current_text() == "{}" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }
    else if stream.current_text() == "{" {
        types.push_current(ScopeVisibility::All);
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    } 
       
    if stream.current_text() == "}" {
        types.pop();
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }
    
    let duplicate = !match stream.current_text() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::Custom(Ident(stream.current_text().clone()));
            types.insert(stream.current_text().clone(), ty)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::Trait(Ident(stream.current_text().clone()));
            types.insert(stream.current_text().clone(), ty)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::TypeEnum(Ident(stream.current_text().clone()), vec![]);
            types.insert(stream.current_text().clone(), ty)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::Union(Ident(stream.current_text().clone()));
            types.insert(stream.current_text().clone(), ty)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::Enum(Ident(stream.current_text().clone()));
            types.insert(stream.current_text().clone(), ty)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::Struct(Ident(stream.current_text().clone()));
            types.insert(stream.current_text().clone(), ty)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::Class(Ident(stream.current_text().clone()));
            types.insert(stream.current_text().clone(), ty)
        },
        _ => true,
    };

    if duplicate {
        return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("a type of name: '{}' already exists", stream.current_text())))
    }

    Ok(())
}

fn add_default_type_kind(types: &mut TypeScopeStack) {

    for (name, type_kind) in INTERNAL_TYPES.iter() {
        types.insert(name.to_string(), type_kind.clone());
    }
}

static INTERNAL_TYPES: Lazy<Vec<(&str, TypeKind)>> = Lazy::new(|| vec![
    (SOUL_NAMES.get_name(NamesInternalType::None),          TypeKind::None), 
    (SOUL_NAMES.get_name(NamesInternalType::Boolean),       TypeKind::Bool), 
    (SOUL_NAMES.get_name(NamesInternalType::Character),     TypeKind::Char(TypeSize::Bit8)), 
    (SOUL_NAMES.get_name(NamesInternalType::String),        TypeKind::Str), 

    (SOUL_NAMES.get_name(NamesInternalType::Int),           TypeKind::SystemInt), 
    (SOUL_NAMES.get_name(NamesInternalType::UntypedInt),    TypeKind::UntypedInt), 
    (SOUL_NAMES.get_name(NamesInternalType::Int8),          TypeKind::Int(TypeSize::Bit8)), 
    (SOUL_NAMES.get_name(NamesInternalType::Int16),         TypeKind::Int(TypeSize::Bit16)), 
    (SOUL_NAMES.get_name(NamesInternalType::Int32),         TypeKind::Int(TypeSize::Bit32)), 
    (SOUL_NAMES.get_name(NamesInternalType::Int64),         TypeKind::Int(TypeSize::Bit64)), 

    (SOUL_NAMES.get_name(NamesInternalType::Uint),          TypeKind::SystemUint), 
    (SOUL_NAMES.get_name(NamesInternalType::UntypedUint),   TypeKind::UntypedUint), 
    (SOUL_NAMES.get_name(NamesInternalType::Uint8),         TypeKind::Uint(TypeSize::Bit8)), 
    (SOUL_NAMES.get_name(NamesInternalType::Uint16),        TypeKind::Uint(TypeSize::Bit16)), 
    (SOUL_NAMES.get_name(NamesInternalType::Uint32),        TypeKind::Uint(TypeSize::Bit32)), 
    (SOUL_NAMES.get_name(NamesInternalType::Uint64),        TypeKind::Uint(TypeSize::Bit64)), 

    (SOUL_NAMES.get_name(NamesInternalType::UntypedFloat),  TypeKind::UntypedFloat), 
    (SOUL_NAMES.get_name(NamesInternalType::Float8),       TypeKind::Float(TypeSize::Bit8)), 
    (SOUL_NAMES.get_name(NamesInternalType::Float16),       TypeKind::Float(TypeSize::Bit16)), 
    (SOUL_NAMES.get_name(NamesInternalType::Float32),       TypeKind::Float(TypeSize::Bit32)), 
    (SOUL_NAMES.get_name(NamesInternalType::Float64),       TypeKind::Float(TypeSize::Bit64)),
]);

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}















