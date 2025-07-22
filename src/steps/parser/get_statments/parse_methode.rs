use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::parser::get_statments::parse_function_decl::{get_bodyless_function_decl};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{FunctionSignature, SoulThis};

pub fn try_get_methode(this: &SoulThis, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Option<Result<Spanned<FunctionSignature>>> {

    if !stream.peek().is_some_and(|token| token.text == "(") {
        None
    }
    else {
        Some(get_bodyless_function_decl(Some(this), stream, scopes))
    }
} 

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing field")
}

































