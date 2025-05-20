use std::{fs::write, ops::Add};
use std::io::Result;
use bitflags::bitflags;
use meta_data::meta_data::MetaData;
use tokenizer::tokenizer::{read_as_file_lines, tokenize_file};

mod tokenizer;
mod meta_data;
mod utils;
mod abstract_styntax_tree;


bitflags! {
    pub struct ShowOutputs: u8 {
        const SHOW_NONE = 0x0;
        const SHOW_TOKENIZER = 0b0000_0001;
        const SHOW_ABSTRACT_SYNTAX_TREE = 0b0000_0010;
        const SHOW_CPP_CONVETION = 0b0000_0100;
        const SHOW_ALL = 0b1111_1111;
    }
}

pub struct RunOptions {
    pub show_outputs: ShowOutputs,
    pub is_garbage_collected: bool,
    pub is_compiled: bool
} 

fn run_compiler(input_path: &str, run_options: RunOptions) -> Result<()> {
    let mut meta_data = MetaData::new();
    let file = read_as_file_lines(input_path)?;
    let tokens = tokenize_file(file.source_file, file.estimated_token_count, &mut meta_data)?;
    if run_options.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {
        let tokens_string = tokens.iter().map(|token| token.text.clone()).collect::<Vec<_>>().join(" ");
        write("output/tokenizer.soul", tokens_string)?;
    }

    Ok(())
}

fn main() {

    let run_option = RunOptions { show_outputs: ShowOutputs::SHOW_ALL, is_garbage_collected: false, is_compiled: true };

    if run_option.is_compiled {
        if let Err(err) = run_compiler("test.soul", run_option) {
            eprintln!("{err}");
        }
    }
    else {
        todo!("run interpreter");
    }
}




