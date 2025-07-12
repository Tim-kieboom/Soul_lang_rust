use crate::errors::soul_error::Result;
use crate::steps::parser::parse_statment::get_statment;
use crate::steps::step_interfaces::i_tokenizer::TokenizeResonse;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::steps::parser::forward_type_stack::get_type_stack::forward_declarde_type_stack;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::AbstractSyntacTree;

pub fn parse_tokens(tokens: TokenizeResonse) -> Result<ParserResponse> {
    let mut tree = AbstractSyntacTree{root: Vec::new()};
    let mut stream = tokens.stream;

    #[cfg(feature="dev_mode")]
    println!( // this print is to be able to see what token is at what index because rust debugger suckss
        "\ntokenizer:\n{:?}\n", 
        stream
            .iter()
            .map(|token| token.text.as_str())
            .enumerate()
            .collect::<Vec<(usize, &str)>>()
    );

    let type_stack = forward_declarde_type_stack(&mut stream)?;
    #[cfg(feature="dev_mode")]
    println!(
        "\nforward_declarde_type_stack\n{}\n", 
        type_stack.scopes
            .iter()
            .map(|scope| format!("{}.{:#?}", scope.self_index, scope.symbols))
            .join("\n-------------\n")
    );

    let mut scopes = ScopeBuilder::new(type_stack);
    
    loop {

       get_statment(&mut stream, &mut scopes)?; 

        if stream.peek().is_none() {
            break;
        }
    }

    Ok(ParserResponse{tree, scopes})
}




















