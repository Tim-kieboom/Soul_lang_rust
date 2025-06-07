use crate::meta_data::soul_error::soul_error::{new_soul_error, Result};
use std::fs::{self, write};
use std::path::Path;
use std::time::Instant;
use itertools::Itertools;

use crate::abstract_styntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree::get_abstract_syntax_tree_file;
use crate::meta_data::meta_data::MetaData;
use crate::run_options::run_options::RunOptions;
use crate::run_options::show_output::ShowOutputs;
use crate::run_options::show_times::ShowTimes;
use crate::tokenizer::token::{Token, TokenIterator};
use crate::tokenizer::tokenizer::{raw_file_as_file_lines, read_as_file_lines, tokenize_file};

pub fn run_compiler(run_options: RunOptions) -> Result<()> {
    let mut meta_data = MetaData::new();

    let start = Instant::now();
    let file = if run_options.is_file_raw_str {
        raw_file_as_file_lines(run_options.file_path)?
    }
    else {
        read_as_file_lines(&run_options.file_path)?
    };

    let tokens;
    match tokenize_file(file.source_file, file.estimated_token_count, &mut meta_data) {
        Ok(val) => tokens = val,
        Err(err) => {
            if run_options.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
                println!("tokenizer time: {:.2?}", start.elapsed());
            }
            return Err(err);
        },
    }
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
            fs::create_dir_all(parent)
                .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;
        }

        write(file_path, tokens_string)
            .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;

    }


    let start = Instant::now();
    let iter = TokenIterator::new(tokens);
    let tree;
    match get_abstract_syntax_tree_file(iter, &mut meta_data) {
        Ok(val) => tree = val,
        Err(err) => {
            if run_options.show_times.contains(ShowTimes::SHOW_ABSTRACT_SYNTAX_TREE) {
                println!("abstractSyntaxTree parser time: {:.2?}", start.elapsed());
            }
            return Err(err); 
        },
    }
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
            fs::create_dir_all(parent)
                .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;
        }

        write(file_path, tree_string)
            .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;

    }

    Ok(())
}

fn new_token() -> Token {
    Token{text: "".to_string(), line_number: 0, line_offset: 0}
}







