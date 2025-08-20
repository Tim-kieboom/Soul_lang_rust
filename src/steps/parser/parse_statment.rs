use crate::soul_names::{NamesOtherKeyWords, SOUL_NAMES};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExpressionKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::{Block, StatementKind};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::BlockBuilder, statement::Statement}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_statment(block_builder: &mut BlockBuilder, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statement>> {
    
    if stream.current_text() == "\n" || stream.current_text() == ";" {

        if stream.next().is_none() {
            return Ok(None)
        }
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
        return Ok(None);
    }
    
    match stream.current_text() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Return) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::BreakLoop) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Fall) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::SwitchCase) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Use) => {
            todo!()
        },
        _ => (),
    }

    if stream.current_text() == "(" {
        todo!("variable")
    }



    todo!()
}

fn get_scope<'a>(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Block>> {
    let mut block_builder = BlockBuilder::new(stream.current_span());

    loop {
        let statment = match get_statment(&mut block_builder, stream, scopes)? {
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
















