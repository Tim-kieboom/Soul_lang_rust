use std::time::Instant;
use std::fs::write;
use std::io::Result;
use bitflags::bitflags;
use itertools::Itertools;
use meta_data::meta_data::MetaData;
use tokenizer::token::TokenIterator;
use run_options::run_options::{RunOptions, ShowOutputs};
use tokenizer::tokenizer::{read_as_file_lines, tokenize_file};
use abstract_styntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree_file;

mod utils;
mod tokenizer;
mod meta_data;
mod run_options;
mod abstract_styntax_tree;

#[cfg(test)]
mod compiler_tests;

fn main() {

    let start = Instant::now();

    //soul run test.soul -showOutputs=SHOW_ALL
    let run_option = 
    // match RunOptions::new(args()) {
    //     Ok(val) => val,
    //     Err(err) => {eprintln!("{err}"); std::process::exit(1);},
    // };
    RunOptions { 
        is_compiled: true,
        file_path: "test.soul".to_string(),
        is_garbage_collected: false, 
        show_outputs: ShowOutputs::SHOW_ALL, 
    };

    if run_option.is_compiled {
        if let Err(err) = run_compiler(run_option) {
            eprintln!("{err}");
        }
    }
    else {
        todo!("run interpreter");
    }

    let duration = start.elapsed();
    println!("Elapsed time: {:.2?}", duration);
}

fn run_compiler(run_options: RunOptions) -> Result<()> {
    let mut meta_data = MetaData::new();
    let file = read_as_file_lines(&run_options.file_path)?;
    let tokens = tokenize_file(file.source_file, file.estimated_token_count, &mut meta_data)?;
   
    if run_options.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {       
        let tokens_string = tokens
            .iter()
            .map(|token| token.text.clone())
            .collect::<Vec<_>>()
            .join(" ")
            .replace("\n ", "\n");
        
        write("output/tokenizer.soul", tokens_string)?;
    }


    let mut iter = TokenIterator::new(tokens);
    let tree = get_abstract_syntax_tree_file(&mut iter, &mut meta_data)?;

    if run_options.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {
        let tree_string = tree.main_nodes
            .iter()
            .map(|node| node.to_string(true))
            .join("\n");
        
        write("output/abstractSyntaxTree.soul", tree_string)?;
    }

    Ok(())
}




























