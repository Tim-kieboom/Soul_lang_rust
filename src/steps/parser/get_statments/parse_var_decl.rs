use crate::soul_names::check_name;
use crate::steps::step_interfaces::i_parser::scope::ScopeKind;
use crate::steps::parser::get_expressions::parse_expression::get_expression;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::{i_parser::scope::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExprKind, Ident};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::Modifier;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::{VariableDecl, VariableRef};
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind};

pub fn get_var_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<VariableRef>> {
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get initialization of variable")
    }

    let begin_i = stream.current_index();
    let possible_type = match SoulType::try_from_stream(stream, scopes) {
        Some(val) => {
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }
            Some(val?)
        },
        None => None,
    };

    let is_type_invered = possible_type.is_none();

    let modifier = if is_type_invered {
        let modi = Modifier::from_str(stream.current_text());
        if modi != Modifier::Default {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        modi
    }
    else {
        possible_type.as_ref()
            .unwrap()
            .modifier
            .clone()
    };

    let var_name_index = stream.current_index();
    if let Err(msg) = check_name(&stream[var_name_index].text) {
        return Err(new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))
    }

    let possible_scope_kinds = scopes.flat_lookup(&stream[var_name_index].text);
    let possible_var = possible_scope_kinds
        .filter(|scope_kinds| {
            scope_kinds.iter().any(|kind| matches!(kind, ScopeKind::Variable(_)))
        });

    if possible_var.is_some() {
        return Err(new_soul_error(
            SoulErrorKind::NotFoundInScope, 
            stream[var_name_index].span, 
            format!("variable '{}' already exists in scope", &stream[var_name_index].text)
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" || stream.current_text() == ";" {
        if is_type_invered {
            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                stream.current_span(), 
                format!("variable '{}' can not have no type and no assignment (add type 'int foo' or assignment 'foo := 1')", &stream[var_name_index].text)
            ));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                stream.current_span(), 
                format!("global variables HAVE TO BE assigned at init, variable '{}' is not assigned", &stream[var_name_index].text)
            ));
        }

        let ty = possible_type.unwrap();
        let name = Ident(stream[var_name_index].text.clone());
        let var_decl: VariableRef = VariableRef::new(
            VariableDecl{name, ty, initializer: None, lit_retention: None}
        );

        scopes.insert(stream[var_name_index].text.clone(), ScopeKind::Variable(var_decl.clone()));
        return Ok(Spanned::new(var_decl, stream[begin_i].span.combine(&stream.current_span())));
    }

    if is_type_invered {

        if modifier == Modifier::Default &&
           stream.current_text() != ":=" 
        {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("'{}' is not allowed at end of default type invered initialize variable (use ':=')", stream.current_text())
            ));
        }
    }
    else if stream.current_text() != "=" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("'{}' is not allowed at end of initialize variable (use '=')", &stream.current().text)
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if is_type_invered {

        let begin_i = stream.current_index();
        let expr = get_expression(stream, scopes, &[";", "\n"])
            .map_err(|err| pass_soul_error(err.get_last_kind(), stream[begin_i].span, format!("while trying to get assignment of variable: '{}'", &stream[var_name_index].text).as_str(), err))?;

        let (lit_retention, mut ty) = match &expr.node {
            ExprKind::Literal(literal) => (Some(expr.clone()), literal.to_soul_type()),
            _ => (None, SoulType::new()),
        };

        ty.modifier = modifier;
        ty.base = ty.base.untyped_to_typed();

        let var = VariableRef::new(VariableDecl{name: Ident(stream[var_name_index].text.clone()), ty, initializer: Some(Box::new(expr)), lit_retention});
        
        scopes.insert(stream[var_name_index].text.clone(), ScopeKind::Variable(var.clone()));
        Ok(Spanned::new(var, stream[begin_i].span.combine(&stream.current_span())))
    }
    else {
        let begin_i = stream.current_index();
        let expr = get_expression(stream, scopes, &[";", "\n"])
            .map_err(|err| pass_soul_error(err.get_last_kind(), stream[begin_i].span, format!("while trying to get assignment of variable: '{}'", &stream[var_name_index].text).as_str(), err))?;

        let lit_retention = match &expr.node {
            ExprKind::Literal(..) => Some(expr.clone()),
            _ => None,
        };

        let mut ty = possible_type.unwrap();
        ty.modifier = modifier;

        let var = VariableRef::new(VariableDecl{name: Ident(stream[var_name_index].text.clone()), ty, initializer: Some(Box::new(expr)), lit_retention});
        
        scopes.insert(stream[var_name_index].text.clone(), ScopeKind::Variable(var.clone()));
        Ok(Spanned::new(var, stream[begin_i].span.combine(&stream.current_span())))
    }
}









