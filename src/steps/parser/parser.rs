use std::sync::Arc;

use hsoul::subfile_tree::SubFileTree;

use crate::errors::soul_error::{Result, SoulSpan};
use crate::steps::parser::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::{AbstractSyntacTree, BlockBuilder};
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::steps::step_interfaces::{i_parser::parser_response::ParserResponse, i_tokenizer::TokenizeResonse};


pub fn parse(tokens: TokenizeResonse, subfile_tree: Option<Arc<SubFileTree>>) -> Result<ParserResponse> {
    let mut tree = AbstractSyntacTree::new();
    let mut stream = tokens.stream;
    let mut scopes = ScopeBuilder::new();

    let mut block_builder = BlockBuilder::new(SoulSpan::new(0,0,0));
    loop {

        if let Some(statment) = get_statment(&mut block_builder, &mut stream, &mut scopes)? {
            block_builder.push_global(statment)?;
        }

        if stream.peek().is_none() {
            break;
        }
    }

    todo!()
}






