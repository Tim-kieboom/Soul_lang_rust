use crate::errors::soul_error::Result;
use crate::steps::step_interfaces::i_tokenizer::TokenizeResonse;
use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::parser::get_statments::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::NodeRef;
use crate::steps::parser::forward_type_stack::get_type_stack::forward_declarde_type_stack;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::{AbstractSyntacTree, StatmentBuilder};

pub fn parse_tokens(tokens: TokenizeResonse) -> Result<ParserResponse> {
    let mut tree = AbstractSyntacTree{root: Vec::new()};
    let mut stream = tokens.stream;


    let type_stack = forward_declarde_type_stack(&mut stream)?;
    stream.reset();
    #[cfg(feature="dev_mode")]
    {
        use itertools::Itertools;

        println!( // this print is to be able to see what token is at what index because rust debugger suckss
            "\ntokenizer:\n{:?}\n", 
            stream
                .iter()
                .map(|token| token.text.as_str())
                .enumerate()
                .collect::<Vec<(usize, &str)>>()
        );

        // println!(
        //     "\nforward_declarde_type_stack\n{}\n", 
        //     type_stack.scopes
        //         .iter()
        //         .map(|scope| format!("{}.{:#?}", scope.self_index, scope.symbols))
        //         .join("\n-------------\n")
        // );
    }

    let mut scopes = ScopeBuilder::new(type_stack);
    let mut scope_ref =  StatmentBuilder::Global(NodeRef::new(tree.root));    
    loop {

        if let Some(statment) = get_statment(&mut scope_ref, &mut stream, &mut scopes)? {
            scope_ref.try_push(statment)?;
        } 

        if stream.peek().is_none() {
            break;
        }
    }

    if let StatmentBuilder::Global(global) = scope_ref {
        tree.root = global.consume();
    }
    else { unreachable!() }

    Ok(ParserResponse{tree, scopes})
}








































