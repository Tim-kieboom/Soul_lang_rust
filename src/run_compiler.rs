use crate::cpp_transpiller::transpiller::transpiller_to_cpp;
use crate::meta_data::function::internal_functions::INTERNAL_FUNCTIONS;
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

    fs::create_dir_all("output")
        .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;

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
        write(file_path, tree_string)
            .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;

        let mut scope_string = String::new();
        let sorted_scopes = meta_data.scope_store.clone().into_iter().sorted_by_key(|(id, _)| *id);
        for (id, scope) in sorted_scopes {
            scope_string.push_str(format!("id: {} (\n\tfunctions:(\n\t\t", id.0.to_string()).as_str());
            scope_string.push_str(&scope.function_store.from_id.iter().filter(|(id, _)| !INTERNAL_FUNCTIONS.iter().any(|func| func.id == **id)).map(|(_, func)| func.to_string()).join(",\n\t\t"));
            scope_string.push_str("\n\t),\n\tvars: (\n\t\t");
            scope_string.push_str(&scope.vars.iter().map(|(_, var)| var.to_string()).join(",\n\t\t"));
            scope_string.push_str("\n\t),\n),\n");
        }

        let file_path = "output/metaData_scopes.soul";
        write(file_path, scope_string)
            .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;
    }

    let cpp_file = transpiller_to_cpp(&meta_data, &tree)?;
    let file_path = "output/out.cpp";
    write(file_path, cpp_file)
        .map_err(|err| new_soul_error(&new_token(), &err.to_string()))?;

    Ok(())
}

fn new_token() -> Token {
    Token{text: "".to_string(), line_number: 0, line_offset: 0}
}







