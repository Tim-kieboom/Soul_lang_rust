use itertools::Itertools;

use crate::steps::parser::statment::parse_function::get_function;
use crate::steps::parser::statment::parse_variable::get_variable;
use crate::soul_names::{check_name, NamesOtherKeyWords, ASSIGN_SYMBOOLS, SOUL_NAMES};
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeKind;
use crate::steps::parser::statment::parse_generics_decl::get_type_enum_body;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::parser::statment::statment_type::{get_statement_type, StatementType};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::enum_like::TypeEnum;
use crate::steps::parser::expression::parse_expression::{get_expression, get_expression_statment};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::TypeKind;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{SoulType};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::{Assignment, Block, StatementKind, STATMENT_END_TOKENS};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Expression, ExpressionKind, Ident, ReturnKind, ReturnLike, VariableName};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::BlockBuilder, statement::Statement}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_statment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statement>> {
    
    if stream.next_if("\n").is_none() {
        return Ok(None)
    }

    if stream.current_text() == "{" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not have a scope in global (consider making scope a function, struct or class)"))
        }

        let block = get_scope(stream, scopes)?;
        return Ok(Some(Statement::new_expression(ExpressionKind::Block(block.node), block.span)));
    }
    else if stream.current_text() == "}" {
        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "there is a '}' without a '{'"))
        }

        stream.next();
        return Ok(Some(Statement::new(StatementKind::CloseBlock, stream.current_span())));
    }


    let statment = match get_statement_type(stream)? {
        StatementType::Expression => {
            let expression = get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?;
            Statement::from_expression(expression)
        }

        StatementType::Variable => {
            let variable = get_variable(stream, scopes)?;
            let variable_name = VariableName::new(&variable.node.name);
            scopes.insert(variable_name.name.0.clone(), ScopeKind::Variable(variable.node), variable.span);

            Statement::new(StatementKind::Variable(variable_name), variable.span)
        },
        StatementType::Assignment => {
            let assignment = get_assignment(stream, scopes)?;
            Statement::new(StatementKind::Assignment(assignment.node), assignment.span)
        },

        StatementType::Use => {
            todo!("get Use");
            return Ok(None);
        },
        StatementType::Function => {
            let function = get_function(stream, scopes)?;
            Statement::new(StatementKind::Function(function.node), function.span)
        },
        StatementType::FunctionCall => {
            Statement::from_expression(get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?)
        }

        StatementType::Class => {
            todo!("get Class")
        },
        StatementType::Trait => {
            todo!("get Trait")
        },
        StatementType::Struct => {
            todo!("get Struct")
        },

        StatementType::Enum => {
            todo!("get Enum")
        },
        StatementType::Union => {
            todo!("get Union")
        },
        StatementType::TypeEnum => {
            get_type_enum(stream, scopes)?;
            return Ok(None);
        },

        StatementType::If => {
            let expression = get_if(stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::Else => {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                stream.current_span(), 
                "can not have 'else' without 'if'",
            ))
        },
        StatementType::For => {
            let expression = get_for(stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::While => {
            let expression = get_while(stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::Match => {
            let expression = get_match(stream, scopes)?;
            Statement::from_expression(expression)
        },

        StatementType::Type => {
            get_type_def(stream, scopes)?;
            return Ok(None);
        },
        StatementType::Return |
        StatementType::Break |
        StatementType::Fall => {
            let expression = get_return_like(stream, scopes)?;
            Statement::from_expression(expression)
        },
    };

    Ok(Some(statment))
}

fn get_match(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::MatchCase));

    let expression = get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?;

    if let ExpressionKind::Match(_) = &expression.node {
        Ok(expression)
    }
    else {
        Err(new_soul_error(
            SoulErrorKind::InternalError, 
            stream.current_span(), 
            "in get_match() function get_expression() did not return 'match' expression",
        ))
    }
}

fn get_while(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop));

    let expression = get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?;

    if let ExpressionKind::While(_) = &expression.node {
        Ok(expression)
    }
    else {
        Err(new_soul_error(
            SoulErrorKind::InternalError, 
            stream.current_span(), 
            "in get_while() function get_expression() did not return 'while' expression",
        ))
    }
}

fn get_for(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop));

    let expression = get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?;

    if let ExpressionKind::For(_) = &expression.node {
        Ok(expression)
    }
    else {
        Err(new_soul_error(
            SoulErrorKind::InternalError, 
            stream.current_span(), 
            "in get_for() function get_expression() did not return 'for' expression",
        ))
    }
}

fn get_if(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::If));

    let expression = get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?;

    if let ExpressionKind::If(_) = &expression.node {
        Ok(expression)
    }
    else {
        Err(new_soul_error(
            SoulErrorKind::InternalError, 
            stream.current_span(), 
            "in get_if() function get_expression() did not return 'if' expression",
        ))
    }
}

fn get_type_enum(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum));

    let type_def_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    check_name(&stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;

    let name_i = stream.current_index();
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    let type_enum_body = get_type_enum_body(stream, scopes)?; 

    if !STATMENT_END_TOKENS.iter().any(|sym| sym == stream.current_text()) {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedEnd, 
            stream.current_span(), 
            format!("token: '{}' is incorrect end of typeEnum should be one of these of ['{}']", stream.current_text(), STATMENT_END_TOKENS.iter().map(|el| if *el == "\n" {"\\n"} else {el}).join("' or '")),
        ));
    }

    let name = stream[name_i].text.as_str().into();
    let span = stream[type_def_i].span.combine(&stream.current_span());
    let type_enum = TypeEnum{name: Ident::new(&name), body: type_enum_body};

    scopes.insert(name, ScopeKind::TypeEnum(type_enum), span);
    Ok(())
}

fn get_assignment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Assignment>> {
    let assign_i = stream.current_index();
    let variable = get_expression(stream, scopes, ASSIGN_SYMBOOLS)?;
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let value = get_expression(stream, scopes, STATMENT_END_TOKENS)?;
    let span = stream[assign_i].span.combine(&stream.current_span());
    Ok(Spanned::new(Assignment{variable, value}, span))
}

fn get_type_def(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Type));
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let type_i = stream.current_index();
    let new_type = SoulType::from_stream(stream, scopes)?;
    if !matches!(new_type.base, TypeKind::Unknown(_)) {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("type: '{}' is invalid", stream[type_i].text)))
    }

    let name = new_type.base.to_name_string();

    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("token: '{}', should be '{}'", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof))))
    }
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let of_type = SoulType::from_stream(stream, scopes)?;

    let span = stream[type_i].span.combine(&stream.current_span());
    scopes.insert(name, ScopeKind::TypeDef{new_type, of_type}, span);
    Ok(())
}

fn get_return_like(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    debug_assert!(
        stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Return) ||
        stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::BreakLoop) ||
        stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Fall)
    );

    let expression = get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?;

    if let ExpressionKind::ReturnLike(_) = &expression.node {
        Ok(expression)
    }
    else {
        Err(new_soul_error(
            SoulErrorKind::InternalError, 
            stream.current_span(), 
            "in get_return_like() function get_expression() did not return 'returnLike' expression",
        ))
    }
}
    
fn get_scope<'a>(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Block>> {
    let mut block_builder = BlockBuilder::new(stream.current_span());

    loop {
        let statment = match get_statment(stream, scopes)? {
            Some(val) => val,
            None => return Ok(block_builder.into_block()),
        };

        if let StatementKind::CloseBlock = statment.node {
            block_builder.push(statment);
            return Ok(block_builder.into_block());
        }

        block_builder.push(statment);
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}































