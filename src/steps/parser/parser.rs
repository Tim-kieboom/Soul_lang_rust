use crate::errors::soul_error::{Result, SoulSpan};
use crate::steps::parser::statment::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::steps::step_interfaces::{i_parser::parser_response::ParserResponse, i_tokenizer::TokenizeResonse};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::{AbstractSyntacTree, BlockBuilder};


pub fn parse_ast(tokens: TokenizeResonse) -> Result<ParserResponse> {
    let mut stream = tokens.stream;
    let mut scopes = ScopeBuilder::new();

    let mut block_builder = BlockBuilder::new(SoulSpan::new(0,0,0));
    loop {

        if let Some(statment) = get_statment(&mut stream, &mut scopes)? {
            block_builder.push_global(statment)?;
        }

        if stream.peek().is_none() {
            break;
        }
    }
    
    let tree = AbstractSyntacTree::new(block_builder.into_block().node);
    Ok(ParserResponse{tree, scopes})
}






