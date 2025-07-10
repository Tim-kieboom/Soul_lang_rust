extern crate soul_lang_rust;

use itertools::Itertools;
use std::{fs::{write, File}, io::{BufReader, Read}, time::Instant};
use soul_lang_rust::{errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{run_options::RunOptions, show_output::ShowOutputs, show_times::ShowTimes}, steps::{source_reader::source_reader::read_source_file, step_interfaces::{i_source_reader::SourceFileResponse, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}};

fn main() {

    let run_option = match RunOptions::new(std::env::args()) {
        Ok(val) => val,
        Err(msg) => {eprintln!("!!invalid compiler argument!!\n{msg}"); return;},
    };

    if let Err(err) = compiler(run_option) {
        eprintln!("{}", err.to_err_message());
    }
}

fn compiler(run_option: RunOptions) -> Result<()> {
    let start = Instant::now(); 
    
    let reader = get_file_reader(&run_option)?;
    let source_file = source_reader(reader, &run_option)?;
    let token_stream = tokenizer(source_file, &run_option)?;
    
    let duration = start.elapsed();

    if run_option.show_times.contains(ShowTimes::SHOW_TOTAL) {
        println!("Total time: {:.2?}", duration);
    }

    Ok(())
}

fn source_reader<R: Read>(reader: BufReader<R>, run_option: &RunOptions) -> Result<SourceFileResponse> {
    let tab_as_spaces = " ".repeat(run_option.tab_char_len as usize);
    let source_file = read_source_file(reader, &tab_as_spaces)?;
    
    if run_option.show_outputs.contains(ShowOutputs::SHOW_SOURCE) {
        let file_path = format!("{}/steps/source.soul", &run_option.output_dir);
        let mut contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        contents.push_str("\n/*\n");
        contents.push_str(&source_file.c_str_store.iter().map(|(c_str, var_str)| format!("{var_str} = {c_str}")).join("\n"));
        contents.push_str("\n*/\n");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0), err.to_string()))?;
    }

    Ok(source_file)
}

fn tokenizer(source_file: SourceFileResponse, run_option: &RunOptions) -> Result<TokenizeResonse> {
    let token_stream = tokenize(source_file)?;

    if run_option.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {
        let file_path = format!("{}/steps/tokenStream.soul", &run_option.output_dir);
        let contents = token_stream.stream
            .ref_tokens()
            .iter()
            .map(|token| &token.text)
            .join(" ");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0), err.to_string()))?;
    }   

    Ok(token_stream)
}

fn get_file_reader(run_option: &RunOptions) -> Result<BufReader<File>> {
    std::fs::create_dir_all(format!("{}/steps", &run_option.output_dir))
        .map_err(|err| new_soul_error(SoulErrorKind::ArgError, SoulSpan::new(0,0), &err.to_string()))?;

    let file = File::open(&run_option.file_path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0), format!("while trying to open file path: '{}'\n{}", &run_option.file_path, err.to_string())))?;

    Ok(BufReader::new(file))
}























