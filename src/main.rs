extern crate Soul_lang_rust;

use std::path::Path;
use std::io::Result;
use itertools::Itertools;
use std::fs::{self, write};
use std::{env::args, time::Instant};
use Soul_lang_rust::meta_data::meta_data::MetaData;
use Soul_lang_rust::tokenizer::token::TokenIterator;
use Soul_lang_rust::run_options::show_times::ShowTimes;
use Soul_lang_rust::run_options::run_options::RunOptions;
use Soul_lang_rust::run_options::show_output::ShowOutputs;
use Soul_lang_rust::tokenizer::tokenizer::{read_as_file_lines, tokenize_file};
use Soul_lang_rust::abstract_styntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree_file;

fn main() {

    let start = Instant::now();

    //soul run test.soul -showOutput=SHOW_ALL -showTime=SHOW_ALL
    let run_option = match RunOptions::new(args()) {
        Ok(val) => val,
        Err(err) => {eprintln!("{err}"); return;},
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






