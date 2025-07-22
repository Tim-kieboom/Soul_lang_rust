use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::errors::soul_error::{Result};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::parser::get_statments::parse_function_decl::{get_bodyless_function_decl};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{FunctionSignatureRef, SoulThis};

pub fn try_get_methode(this: &SoulThis, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<Spanned<FunctionSignatureRef>>> {

    if !stream.peek().is_some_and(|token| token.text == "(") {
        None
    }
    else {
        Some(get_bodyless_function_decl(Some(this), stream, scopes))
    }
} 

































