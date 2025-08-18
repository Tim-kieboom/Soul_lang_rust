use once_cell::sync::Lazy;
use crate::steps::parser::get_expressions::parse_path::{get_page_path_type_stack, PagePathKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::soul_names::{NamesInternalType, NamesOtherKeyWords, SOUL_NAMES};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::scope::{ExternalPages, ScopeBuilder, ScopeVisibility, SoulPagePath, TypeScopeStack};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{ExternalPath, ExternalType, TypeKind, TypeSize};
use crate::utils::serde_multi_ref::MultiRefPool;

pub fn get_scope_from_type_stack(stream: &mut TokenStream, ref_pool: MultiRefPool, external_books: ExternalPages, project_name: String) -> Result<ScopeBuilder> {
    let mut types = TypeScopeStack::new();
    let mut scopes = ScopeBuilder::new(external_books, project_name, ref_pool);
    add_default_type_kind(&mut types);

    loop {

        parse_type(&mut types, &mut scopes, stream)?;

        if stream.next().is_none() {
            break;
        }
    }

    scopes.fill_with_type_stack(types);
    Ok(scopes)
}

fn parse_lambda_scope(types: &mut TypeScopeStack, scopes: &mut ScopeBuilder, stream: &mut TokenStream) -> Result<()> {
    let mut open_curly_bracket_stack = 0i64;
    let mut open_round_bracket_stack = 0i64;
    types.push_current(ScopeVisibility::All);

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() == "{" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    loop {

        match stream.current_text().as_str() {
            "(" => open_round_bracket_stack += 1,
            ")" => open_round_bracket_stack -= 1,
            "{" => open_curly_bracket_stack += 1,
            "}" => open_curly_bracket_stack += 1,
            "=>" => parse_lambda_scope(types, scopes, stream)?,
            "\n" => {
                if open_curly_bracket_stack == 0 && open_round_bracket_stack == 0 {
                    types.pop(stream.current_span());
                    break Ok(());
                }
            }
            _ => (),
        }


        if open_curly_bracket_stack < 0 || open_round_bracket_stack < 0 {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            types.pop(stream.current_span());
            break Ok(());
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

}

fn parse_type(types: &mut TypeScopeStack, scopes: &mut ScopeBuilder, stream: &mut TokenStream) -> Result<()> {

    if stream.current_text() == "=>" {
        parse_lambda_scope(types, scopes, stream)?;
    }

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
        
        types.try_pop(stream.current_span())?;

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }
    
    let duplicate = !match stream.current_text() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = TypeKind::TypeDefed(Ident(stream.current_text().clone()));
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
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Use) => {
            parse_use(stream, scopes, types)?
        },
        _ => true,
    };

    if duplicate {
        return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("a type of name: '{}' already exists", stream.current_text())))
    }

    Ok(())
}

fn parse_use(stream: &mut TokenStream, scopes: &mut ScopeBuilder, types: &mut TypeScopeStack) -> Result<bool> {
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let PagePathKind{path, is_page} = get_page_path_type_stack(stream, scopes, &types)?.node;
    if !is_page || stream.current_text() == "\n" || stream.current_text() == ";" {
        let name = Ident(path.get_last_name());
        let duplicate = !types.insert(name.0.clone(), TypeKind::ExternalPath(Spanned::new(ExternalPath{name, path: path.clone()}, stream.current_span())));
        if duplicate {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                stream.current_span(), 
                format!("path: '{}' is already included in scope", path.0)
            ));
        }
        return Ok(true)
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let names = if stream.current_text() == "[" {
        parse_multi_use(stream, types, &path)?
    }
    else {
        vec![Spanned::new(stream.current_text().clone(), stream.current_span())]
    };
    
    for Spanned{node: name, span} in names {
        
        if types.current_mut().symbols.contains_key(&name) {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, span, format!("a type of name: '{}' already exists", name)))
        }

        let ty = TypeKind::ExternalType(Spanned::new(ExternalType{name: Ident(name.clone()), path: path.clone()}, span));
        types.insert(name, ty);
    }

    Ok(true)
}

fn parse_multi_use(stream: &mut TokenStream, types: &mut TypeScopeStack, path: &SoulPagePath) -> Result<Vec<Spanned<String>>> {
    
    let mut names = vec![];
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        if stream.current_text() == "this" {
            let name = Ident(path.get_last_name());
            let duplicate = !types.insert(name.0.clone(), TypeKind::ExternalPath(Spanned::new(ExternalPath{name, path: path.clone()}, stream.current_span())));
            if duplicate {
                return Err(new_soul_error(
                    SoulErrorKind::InvalidInContext, 
                    stream.current_span(), 
                    format!("path: '{}' is already included in scope", path.0)
                ));
            }
        }
        else {
            names.push(Spanned::new(stream.current_text().clone(), stream.current_span()));
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "," {
            continue;
        }
        else if stream.current_text() == "]" {
            break Ok(names)
        }
        else {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token: '{}' should be ',' or ']'", stream.current_text())
            ))
        }
    }
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
    (SOUL_NAMES.get_name(NamesInternalType::Float8),        TypeKind::Float(TypeSize::Bit8)), 
    (SOUL_NAMES.get_name(NamesInternalType::Float16),       TypeKind::Float(TypeSize::Bit16)), 
    (SOUL_NAMES.get_name(NamesInternalType::Float32),       TypeKind::Float(TypeSize::Bit32)), 
    (SOUL_NAMES.get_name(NamesInternalType::Float64),       TypeKind::Float(TypeSize::Bit64)),
    
    (SOUL_NAMES.get_name(NamesInternalType::Float64),       TypeKind::Float(TypeSize::Bit64)),
]); 

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}













