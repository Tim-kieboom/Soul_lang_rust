extern crate soul_lang_rust;

use std::path::Path;
use itertools::Itertools;
use std::fs::{self, write};
use std::io::{self, Result, Write};
use std::{env::args, time::Instant};
use soul_lang_rust::tokenizer::file_line::FileLine;
use soul_lang_rust::meta_data::meta_data::MetaData;
use soul_lang_rust::tokenizer::token::TokenIterator;
use soul_lang_rust::run_options::show_times::ShowTimes;
use soul_lang_rust::run_options::run_options::RunOptions;
use soul_lang_rust::run_options::show_output::ShowOutputs;
use soul_lang_rust::meta_data::current_context::current_context::CurrentContext;
use soul_lang_rust::abstract_styntax_tree::abstract_styntax_tree::AbstractSyntaxTree;
use soul_lang_rust::tokenizer::tokenizer::{read_as_file_lines, tokenize_file, tokenize_line};
use soul_lang_rust::abstract_styntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree::{get_abstract_syntax_tree_file, get_abstract_syntax_tree_line};

fn main() {
    let start = Instant::now();

    let run_option = match RunOptions::new(args()) {
        Ok(val) => val,
        Err(err) => {eprintln!("{err}"); return;},
    };

    let is_compiled = run_option.is_compiled;
    let result = if is_compiled {
        run_compiler(run_option)
    } else {
        run_interpreter(run_option)
    };
    
    let duration = start.elapsed();

    if let Err(err) = result {
        eprintln!("{err}");
    }

    if is_compiled {
        println!("Elapsed time: {:.2?}", duration);
    }
}

fn run_compiler(run_options: RunOptions) -> Result<()> {
    let mut meta_data = MetaData::new();

    let start = Instant::now();
    let file = read_as_file_lines(&run_options.file_path)?;
    let tokens = tokenize_file(file.source_file, file.estimated_token_count, &mut meta_data)?;
    let duration = start.elapsed();

    if run_options.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
        println!("tokenizer time: {:.2?}", duration);
    }

    if run_options.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {       
        let tokens_string = tokens
            .iter()
            .map(|token| token.text.clone())
            .collect::<Vec<_>>()
            .join(" ")
            .replace("\n ", "\n");
        
        let file_path = "output/tokenizer.soul";
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        write(file_path, tokens_string)?;
    }


    let start = Instant::now();
    let mut iter = TokenIterator::new(tokens);
    let tree = get_abstract_syntax_tree_file(&mut iter, &mut meta_data)?;
    let duration = start.elapsed();

    if run_options.show_times.contains(ShowTimes::SHOW_ABSTRACT_SYNTAX_TREE) {
        println!("abstractSyntaxTree parser time: {:.2?}", duration);
    }

    if run_options.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {
        let tree_string = tree.main_nodes
            .iter()
            .map(|node| node.to_string(true))
            .join("\n");

        let file_path = "output/abstractSyntaxTree.soul";
        if let Some(parent) = Path::new(file_path).parent() {
            fs::create_dir_all(parent)?;
        }

        write(file_path, tree_string)?;
    }

    Ok(())
}

fn run_interpreter(run_options: RunOptions) -> Result<()> {
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

        get_abstract_syntax_tree_line(&mut tree, &mut iter, &mut context, &mut meta_data, &mut open_bracket_stack)?;

        if run_options.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {       
            println!("{}", tree.main_nodes.iter().map(|node| node.to_string(true)).join("\n"));
        }
    }
}





