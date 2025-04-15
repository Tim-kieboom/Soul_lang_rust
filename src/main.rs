use std::fs::write;
use std::io::Result;
use meta_data::meta_data::MetaData;
use tokenizer::tokenizer::{read_as_file_lines, tokenize_file};

mod tokenizer;
mod meta_data;
mod utils;
mod abstract_styntax_tree;

fn run_compiler(input_path: &str) -> Result<()> {
    let mut meta_data = MetaData::new();
    let file = read_as_file_lines(input_path, &meta_data)?;

    let tokens = tokenize_file(file.source_file, file.estimated_token_count, &mut meta_data)?;
    write("output/tokenizer.txt", tokens.iter().map(|token| token.text.clone()).collect::<Vec<_>>().join(" "))?;

    Ok(())
}

fn main() {
    
    if let Err(err) = run_compiler("test.soul") {
        eprintln!("{err}");
    }
}
