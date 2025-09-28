use std::path::{Path, PathBuf};

use crate::steps::parser::statment::parse_block::get_use_block;
use crate::steps::parser::statment::parse_function::get_function;
use crate::steps::parser::statment::parse_variable::get_variable;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeKind;
use crate::soul_names::{NamesOtherKeyWords, ASSIGN_SYMBOOLS, SOUL_NAMES};
use crate::steps::parser::statment::parse_enum_like::{get_enum, get_union};
use crate::steps::parser::statment::parse_generics_decl::{get_type_enum_body};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::parser::statment::parse_object::{get_class, get_struct, get_trait};
use crate::steps::parser::statment::statment_type::{get_statement_type, StatementType};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::enum_like::TypeEnum;
use crate::steps::parser::expression::parse_expression::{get_expression, get_expression_statment};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{SoulType};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Expression, ExpressionKind, Ident};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::{Assignment, Block, StatementKind, STATMENT_END_TOKENS};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{ExternalPath, ExternalType, SoulPagePath, TypeKind};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::BlockBuilder, statement::Statement}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_statment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statement>> {
    
    if stream.next_if("\n").is_none() {
        return Ok(None)
    }

    if stream.current_is("{"){
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span_some(), "can not have a scope in global (consider making scope a function, struct or class)"))
        }

        let block = get_scope(stream, scopes)?;
        return Ok(Some(Statement::new_expression(ExpressionKind::Block(block.node), block.span)));
    }
    else if stream.current_is("}") {
        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span_some(), "there is a '}' without a '{'"))
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
            let variable_name = variable.node.name.clone();
            scopes.insert(variable_name.name.0.clone(), ScopeKind::Variable(variable.node), variable.span)?;

            Statement::new(StatementKind::Variable(variable_name), variable.span)
        },
        StatementType::Assignment => {
            let assignment = get_assignment(stream, scopes)?;
            Statement::new(StatementKind::Assignment(assignment.node), assignment.span)
        },

        StatementType::Use => {
            match get_use(stream, scopes)? {
                Some(statment) => statment,
                None => return Ok(None),
            }
        },
        StatementType::Function => {
            let function = get_function(stream, scopes)?;
            Statement::new(StatementKind::Function(function.node), function.span)
        },
        StatementType::FunctionCall => {
            Statement::from_expression(get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?)
        },
        StatementType::StructContructor => {
            Statement::from_expression(get_expression_statment(stream, scopes, STATMENT_END_TOKENS)?)
        },

        StatementType::Class => {
            let class_decl = get_class(stream, scopes)?;
            Statement::new(StatementKind::Class(class_decl.node), class_decl.span)
        },
        StatementType::Trait => {
            let trait_decl = get_trait(stream, scopes)?;
            Statement::new(StatementKind::Trait(trait_decl.node), trait_decl.span)
        },
        StatementType::Struct => {
            let struct_decl = get_struct(stream, scopes)?;
            Statement::new(StatementKind::Struct(struct_decl.node), struct_decl.span)
        },

        StatementType::Enum => {
            let enum_decl = get_enum(stream, scopes)?;
            Statement::new(StatementKind::Enum(enum_decl.node), enum_decl.span)
        },
        StatementType::Union => {
            let union_decl = get_union(stream, scopes)?;
            Statement::new(StatementKind::Union(union_decl.node), union_decl.span)
        },

        StatementType::If => {
            let expression = get_if(stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::Else => {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                stream.current_span_some(), 
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
            get_type_def_or_type_enum(stream, scopes)?;
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

fn get_use(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statement>> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Use));
    let use_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }
    
    if let Some(this) = SoulType::try_from_stream(stream, scopes)? {
        
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_is("{") {
            let use_block = get_use_block(stream, scopes, this, None)?;
            return Ok(Some(Statement::new(StatementKind::UseBlock(use_block.node), use_block.span.combine(&stream[use_i].span))))
        }
        
        if !stream.current_is(SOUL_NAMES.get_name(NamesOtherKeyWords::Impl)) {
            stream.go_to_index(use_i);
            get_use_path(stream, scopes)?;
            return Ok(None)
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        let impl_type = SoulType::from_stream(stream, scopes)?;
        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_is("{") {
            let use_block = get_use_block(stream, scopes, this, Some(impl_type))?;
            return Ok(Some(Statement::new(StatementKind::UseBlock(use_block.node), use_block.span.combine(&stream[use_i].span))))
        }
        else {
            let name = this.base.to_name_string();
            let typedef = ScopeKind::UseTypeDef{new_type: this, of_type: impl_type};
            scopes.insert(name, typedef, stream.current_span().combine(&stream[use_i].span))?;
            return Ok(None)
        }
    }

    stream.go_to_index(use_i);
    get_use_path(stream, scopes)?;
    return Ok(None)
}

fn get_use_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    
    let use_i = stream.current_index();

    debug_assert!(stream.current_is(SOUL_NAMES.get_name(NamesOtherKeyWords::Use)));
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let mut path = get_first_path(stream, scopes)?;

    loop {
        stream.next();
        if !stream.peek_multiple_is(2, ".") {
            break
        }
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
        path.push(Path::new(stream.current_text()));
    }

    let page_path = SoulPagePath::from_path(&path);
    if stream.current_is("\n") {
        let name = page_path.0.clone();
        
        let span = stream[use_i].span.combine(&stream.current_span());
        let path_type = ExternalPath{path: page_path.clone(), name: name.clone().into()};
        let ty = SoulType::from_type_kind(TypeKind::ExternalPath(Spanned::new(path_type, span)));
        
        scopes.insert(
            name, 
            ScopeKind::Type(ty), 
            span,
        )?;
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let names = if stream.current_text() == "[" {
        parse_multi_path(stream)?
    }
    else {
        vec![Spanned::new(stream.current_text().clone(), stream.current_span())]
    };

    for path_name in names {
        let span = stream[use_i].span.combine(&stream.current_span());

        let (ty, name) = if path_name.node == "this" {
            let mut path = page_path.clone();
            path.pop();
            let name = Ident::new(page_path.get_last_name());
            let path_type = ExternalPath{path, name: name.clone()};
            (SoulType::from_type_kind(TypeKind::ExternalPath(Spanned::new(path_type, path_name.span))), name)
        }
        else {
            let name = Ident::new(path_name.node);
            let path_type = ExternalType{path: page_path.clone(), name: name.clone()};
            (SoulType::from_type_kind(TypeKind::ExternalType(Spanned::new(path_type, path_name.span))), name)
        };  
        
        scopes.insert(
            name.0, 
            ScopeKind::Type(ty), 
            span,
        )?;
    }
    
    Ok(())
}

fn get_first_path(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<PathBuf> {
    if stream.current_is("this") {
        Ok(PathBuf::from(Path::new(&scopes.project_name)))
    }
    else {
        Ok(PathBuf::from(Path::new(stream.current_text())))
    }
}

fn parse_multi_path(stream: &mut TokenStream) -> Result<Vec<Spanned<String>>> {

    let mut names = vec![];
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.next_if("\n").is_none() {
            return Err(err_out_of_bounds(stream))
        }

        names.push(Spanned::new(stream.current_text().to_string(), stream.current_span()));

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }

        if stream.current_is(",") {
            continue
        }
        else if stream.current_is("]") {
            stream.next();
            break Ok(names)
        }
        else {

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span_some(), 
                format!("token: '{}' should be ',' or ']'", stream.current_text())
            ))
        }
    }
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
            stream.current_span_some(), 
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
            stream.current_span_some(), 
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
            stream.current_span_some(), 
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
            stream.current_span_some(), 
            "in get_if() function get_expression() did not return 'if' expression",
        ))
    }
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

fn get_type_def_or_type_enum(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Type));
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let type_i = stream.current_index();
    let new_type = SoulType::from_stream(stream, scopes)?;
    if !matches!(new_type.base, TypeKind::Unknown(_)) {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span_some(), format!("type: '{}' is invalid", stream[type_i].text)))
    }

    let name = new_type.base.to_name_string();

    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Impl) {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span_some(), format!("token: '{}', should be '{}'", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Impl))))
    }
    
    if stream.peek_is("[") {
        let body = get_type_enum_body(stream, scopes)?;

        let span = stream[type_i].span.combine(&stream.current_span());
        return scopes.insert(name.clone(), ScopeKind::TypeEnum(TypeEnum{name: name.into(), body}), span)
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_is("\n") {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), format!("not type after '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::Impl))))
    }

    let of_type = SoulType::from_stream(stream, scopes)?;

    let span = stream[type_i].span.combine(&stream.current_span());
    scopes.insert(name, ScopeKind::TypeDef{new_type, of_type}, span)
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
            stream.current_span_some(), 
            "in get_return_like() function get_expression() did not return 'returnLike' expression",
        ))
    }
}
    
fn get_scope<'a>(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Block>> {
    scopes.push_scope();
    let mut block_builder = BlockBuilder::new(scopes.current_id(), stream.current_span());

    let result = loop {
        let statment = match get_statment(stream, scopes)? {
            Some(val) => val,
            None => break Ok(block_builder.into_block()),
        };

        if let StatementKind::CloseBlock = statment.node {
            block_builder.push(statment);
            break Ok(block_builder.into_block());
        }

        block_builder.push(statment);
    };

    scopes.pop_scope(stream.current_span())?;
    result
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpected end while trying to get statments")
}































