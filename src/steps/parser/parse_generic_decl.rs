use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::parser::get_statments::parse_type_enum::get_type_enum_body;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::TypeKind;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{TypeConstraint};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::statment::GenericParam, scope::ScopeBuilder}, i_tokenizer::TokenStream};

pub struct GenericDecl {
    pub generics: Vec<GenericParam>,
    pub implements: Vec<Ident>,
}

pub fn get_generics_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<GenericDecl> {
    let mut generics_decl = GenericDecl{generics: vec![], implements: vec![]};

    if stream.current_text() != "<" {
        return Ok(generics_decl);
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    loop {

        check_name(stream.current_text())
            .map_err(|child| new_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("while trying to parse generics: {}", child)))?;
        
        let name = Ident(stream.current_text().clone());

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        let mut constraint = vec![];
        if stream.current_text() == ":" {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            add_generic_type_contraints(&mut constraint, stream, scopes)?;

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        let default = if stream.current_text() == "=" {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = SoulType::from_stream(stream, scopes)?;
            
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            Some(ty)
        }
        else {
            None
        };

        generics_decl.generics.push(GenericParam{name: name.clone(), constraint, default});
        
        scopes.insert_type(name.0.clone(), TypeKind::Generic(name))
            .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

        if stream.current_text() != "," {
            break;
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != ">" {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis, 
            stream.current_span(), 
            format!("while trying to get generics, generics should en with '>' but ends on '{}'", stream.current_text())
        ));
    } 

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    add_impl(&mut generics_decl, stream, scopes)?;
    add_where(&mut generics_decl, stream, scopes)?;

    Ok(generics_decl)
}

fn add_impl(generics_decl: &mut GenericDecl, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Impl) { 
        return Ok(());
    }
        
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        scopes.lookup_type(stream.current_text())
            .ok_or(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is not allowed in impl only traits are allowed", stream.current_text())))?;
        
        generics_decl.implements.push(Ident(stream.current_text().clone()));

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        if stream.current_text() != "+" {
            break;
        }
    }

    if stream.current_text() != "\n" && stream.current_text() != "{" && stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Where) {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), format!("token: '{}' invalid end should be '{{' or endline of '{}' ", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Where))))
    }

    Ok(())
}

fn add_where(generics_decl: &mut GenericDecl, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }
    
    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Where) { 
        return Ok(())
    }

    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        let kind = scopes.lookup_type(stream.current_text())
            .ok_or(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid should be generic (e.g. 'where T: trait,')", stream.current_text())))?;

        let generic_name = match kind {
            TypeKind::Generic(ident) => ident,
            _ => return Err(new_soul_error(SoulErrorKind::WrongType, stream.current_span(), format!("type: '{}' is not allowed in where should be generic (e.g. 'where T: trait,')", kind.get_variant()))),
        };

        let generic = generics_decl.generics.iter_mut().find(|gene| &gene.name == generic_name)
            .ok_or(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid should be generic (e.g. 'where T: trait,')", stream.current_text())))?;

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() != ":" {

        }
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        add_generic_type_contraints(&mut generic.constraint, stream, scopes)?;

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() != "," {
            break;
        }
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' invalid should be '{{'", stream.current_text())))
    }

    Ok(())
}

fn add_generic_type_contraints(contraints: &mut Vec<TypeConstraint>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    loop {

        let type_contraints = if let Some(kind) = scopes.lookup_type(stream.current_text()) {
            match kind {
                TypeKind::Trait(id) => TypeConstraint::Trait(id.clone()), 
                TypeKind::TypeEnum(_, types) => TypeConstraint::TypeEnum(types.clone()),
                _ => return Err(new_soul_error(SoulErrorKind::ArgError, stream.current_span(), format!("type: '{}' is '{}' only 'trait' and 'typeEnum' is allowed for generic contraint", stream.current_text(), kind.get_variant())))
            }
        }
        else if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
            let types = get_type_enum_body(stream, scopes)?;
            
            TypeConstraint::TypeEnum(types)
        }
        else {
            return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' is invalid in typeContrainets only allowed traits typeEnums and '+'", stream.current_text())))
        };

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        contraints.push(type_contraints);

        if stream.current_text() != "+" {
            break;
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }
    
    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() != ">" && stream.current_text() != "," && stream.current_text() != "{" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("token: '{}' in not allowed in generic", stream.current_text())))
    }

    if stream.next_multiple(-1).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    Ok(())
}

fn err_out_of_bounds(stream: &mut TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing generic ctor")
}























