use std::io::{self, Result, Write};
use itertools::Itertools;

use crate::{abstract_styntax_tree::{abstract_styntax_tree::AbstractSyntaxTree, get_abstract_syntax_tree::{get_abstract_syntax_tree::get_abstract_syntax_tree_line, get_stament::statment_type::statment_type::StatmentIterator}}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData}, run_options::{run_options::RunOptions, show_output::ShowOutputs}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};

pub fn run_interpreter(run_options: RunOptions) -> Result<()> {
    let mut line_index = 1;
    let mut in_multi_line_commend = false;
    let mut open_bracket_stack = 0;
    
    let mut meta_data = MetaData::new();
    let mut tree = AbstractSyntaxTree::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    loop {
        let mut input = String::new();
        
        for _ in 0..open_bracket_stack+1 {
            print!(">> ");
        }

        io::stdout().flush().unwrap();
        match io::stdin().read_line(&mut input) {
            Ok(_) => (),
            Err(err) => {println!("Could not get Input, error: {}", err.to_string()); continue;},
        }

        //remove "\r\n"
        input.pop(); 
        input.pop();
        
        if input == "exit()" {
            break Ok(());
        }

        let line = FileLine{text: input.clone(), line_number: line_index};
        let tokens = tokenize_line(line, line_index as usize, &mut in_multi_line_commend, &mut meta_data)?;
        line_index += 1;

        if run_options.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {       
            println!("{:?}", tokens.iter().map(|token| &token.text).collect::<Vec<_>>());
        }

        if in_multi_line_commend {
            continue;
        }

        let mut iter = TokenIterator::new(tokens);
        let mut statment_iter = StatmentIterator::new(Vec::new());

        get_abstract_syntax_tree_line(&mut tree, &mut iter, &mut statment_iter, &mut context, &mut meta_data, &mut open_bracket_stack)?;

        if run_options.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {       
            println!("{}", tree.main_nodes.iter().map(|node| node.to_string(true)).join("\n"));
        }
    }
}



