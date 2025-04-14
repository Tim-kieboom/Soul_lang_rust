use std::path::PathBuf;

use meta_data::meta_data::MetaData;
use tokenizer::tokenizer::{read_as_file_lines, tokenize_file};

mod tokenizer;
mod meta_data;
mod utils;

fn main() {
    let mut meta_data = MetaData::new();
    let result_file = read_as_file_lines("test.soul", &meta_data);
    if result_file.is_err() {
        println!("{:#?}", result_file.err());
        return;
    }

    let file = result_file.unwrap();

    let result_tokens = tokenize_file(file.source_file, file.estimated_token_count, &mut meta_data);
    if result_tokens.is_err() {
        println!("{:#?}", result_tokens.err());
        return;
    }
    let tokens = result_tokens.unwrap();

    println!("{}", tokens.iter().map(|token| token.text.clone()).collect::<Vec<_>>().join(" "));
}
