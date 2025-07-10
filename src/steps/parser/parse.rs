use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;

pub fn parse_tokens() -> Result<ParserResponse> {
    let mut tree = AbstractSyntacTree{root: Vec::new()};
    

    Ok(ParserResponse{tree})
}