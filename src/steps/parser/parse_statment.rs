use once_cell::sync::Lazy;
use std::collections::HashSet;
use crate::soul_names::{NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{Statment, StmtKind};

static ASSIGN_SYMBOOLS_SET: Lazy<HashSet<&&str>> = Lazy::new(|| {
    SOUL_NAMES.assign_symbools.iter().map(|(_, str)| str).collect::<HashSet<&&str>>()
});

pub fn get_statment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Statment> {

    if stream.current().text == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }

    if stream.current_text() == "{" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not have a scope in global (consider making scope a function, struct or class)"))
        }

        return Ok(Statment::new(StmtKind::Block(get_scope(stream, scopes)?), stream.current_span()));
    }
    else if stream.current_text() == "}" {
        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "there is a '}' without a '{'"))
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        // impl borrowchecker to get deletelist

        return Ok(Statment::new(StmtKind::CloseBlock(vec![]), stream.current_span()));
    }

    match stream.current_text() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Return) => {
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
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Interface) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            todo!()
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            todo!()
        }
        _ => (),
    }

    todo!();
}

fn get_scope(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Vec<Statment>> {
    let mut statments = Vec::new();

    loop {
        let statment = get_statment(stream, scopes)?;
        let should_end = matches!(statment.node, StmtKind::CloseBlock(_));

        statments.push(statment);
        
        if should_end {
            return Ok(statments);
        }
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}






















