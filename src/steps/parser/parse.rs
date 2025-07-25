use crate::errors::soul_error::{Result, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExprKind, Expression};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::{VariableDecl, VariableRef};
use crate::steps::step_interfaces::i_parser::external_header::ExternalHeader;
use crate::steps::step_interfaces::i_tokenizer::TokenizeResonse;
use crate::steps::step_interfaces::i_parser::scope::{ProgramMemmory, ScopeBuilder, ScopeKind};
use crate::steps::parser::get_statments::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::steps::parser::forward_type_stack::get_type_stack::forward_declarde_type_stack;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::{AbstractSyntacTree, GlobalKind, StatmentBuilder};
use crate::utils::node_ref::NodeRef;

pub fn parse_tokens(tokens: TokenizeResonse) -> Result<ParserResponse> {
    let mut tree = AbstractSyntacTree{root: Vec::new()};
    let mut stream = tokens.stream;


    let type_stack = forward_declarde_type_stack(&mut stream)?;
    stream.reset();
    #[cfg(feature="dev_mode")]
    println!( // this print is to be able to see what token is at what index because rust debugger suckss
        "\ntokenizer:\n{:?}\n", 
        stream
            .iter()
            .map(|token| token.text.as_str())
            .enumerate()
            .collect::<Vec<(usize, &str)>>()
    );

    let mut scopes = ScopeBuilder::new(type_stack, ExternalHeader::new());
    let mut scope_ref = StatmentBuilder::Global(NodeRef::new(tree.root));    
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

    let first_span = SoulSpan::new(0, 0, 0);
    for (literal, id) in std::mem::take(&mut scopes.global_literal.store) {
        let name = ProgramMemmory::to_program_memory_name(&id);
        
        let var_ref = VariableRef::new(
            VariableDecl{
                name: name.clone(), 
                ty: literal.to_soul_type(), 
                initializer: Some(Box::new(Expression::new(ExprKind::Literal(literal), first_span))),
                lit_retention: None,
            },
        );
        let var = ScopeKind::Variable(var_ref.clone());

        scopes.insert_global(name.0, var);
        tree.root.push(Spanned::new(GlobalKind::VarDecl(var_ref), first_span));
    }

    Ok(ParserResponse{tree, scopes})
}








































