extern crate soul_lang_rust;

use itertools::Itertools;
use std::{fs::{write, File}, io::{BufReader, Read}, time::Instant};
use soul_lang_rust::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{run_options::RunOptions, show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::parse::parse_tokens, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::{abstract_syntax_tree::pretty_format::PrettyFormat, parser_response::ParserResponse}, i_source_reader::SourceFileResponse, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}};


fn main() {

    let run_option = match RunOptions::new(std::env::args()) {
        Ok(val) => val,
        Err(msg) => {eprintln!("!!invalid compiler argument!!\n{msg}"); return;},
    };

    if let Err(err) = compiler(&run_option) {
        let reader = get_file_reader(&run_option).main_err_map("while trying to get file reader")
            .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

        eprintln!("at char:line; !!error!! message\n\n{}\n", err.to_err_message());        
        eprintln!("{}", err.to_highlighed_message(reader));        
    }
}

fn compiler(run_option: &RunOptions) -> Result<()> {
    let start = Instant::now(); 

    let reader = get_file_reader(&run_option).main_err_map("while trying to get file reader")?;
    let source_response = source_reader(reader, &run_option).main_err_map("in source_reader")?;
    let token_response = tokenizer(source_response, &run_option).main_err_map("in tokenizer")?;
    let parser_reponse = parser(token_response, run_option).main_err_map("in parser")?;
    let _ = parser_reponse;

    if run_option.show_times.contains(ShowTimes::SHOW_TOTAL) {
        println!("Total time: {:.2?}", start.elapsed());
    }

    Ok(())
}

fn source_reader<R: Read>(reader: BufReader<R>, run_option: &RunOptions) -> Result<SourceFileResponse> {
    let tab_as_spaces = " ".repeat(run_option.tab_char_len as usize);
    
    let start = Instant::now(); 
    let source_file = read_source_file(reader, &tab_as_spaces)?;
    if run_option.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
        println!("source_reader time: {:.2?}", start.elapsed());
    }

    if run_option.show_outputs.contains(ShowOutputs::SHOW_SOURCE) {
        let file_path = format!("{}/steps/source.soulc", &run_option.output_dir);
        let contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
    }

    Ok(source_file)
}

fn tokenizer(source_file: SourceFileResponse, run_option: &RunOptions) -> Result<TokenizeResonse> {
    
    let start = Instant::now(); 
    let token_stream = tokenize(source_file)?;
    if run_option.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
        println!("tokenizers time: {:.2?}", start.elapsed());
    }

    if run_option.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {
        let file_path = format!("{}/steps/tokenStream.soulc", &run_option.output_dir);
        let contents = token_stream.stream
            .ref_tokens()
            .iter()
            .map(|token| &token.text)
            .join(" ");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
    }

    Ok(token_stream)
}

fn parser(token_response: TokenizeResonse, run_option: &RunOptions) -> Result<ParserResponse> {
    
    let start = Instant::now(); 
    let parse_response = parse_tokens(token_response)?;
    if run_option.show_times.contains(ShowTimes::SHOW_PARSER) {
        println!("parser time: {:.2?}", start.elapsed());
    }
    
    if run_option.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {
        let file_path = format!("{}/steps/parserAST.soulc", &run_option.output_dir);
        let scopes_file_path = format!("{}/steps/parserScopes.soulc", &run_option.output_dir);

        write(file_path, format!("{}", parse_response.tree.to_pretty_string()))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        write(scopes_file_path, format!("{}", parse_response.scopes.to_pretty_string()))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
    }

    Ok(parse_response)
}

fn get_file_reader(run_option: &RunOptions) -> Result<BufReader<File>> {
    std::fs::create_dir_all(format!("{}/steps", &run_option.output_dir))
        .map_err(|err| new_soul_error(SoulErrorKind::ArgError, SoulSpan::new(0,0,0), &err.to_string()))?;

    let file = File::open(&run_option.file_path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open file path: '{}'\n{}", &run_option.file_path, err.to_string())))?;

    Ok(BufReader::new(file))
}

trait MainErrMap<T>{fn main_err_map(self, msg: &str) -> Result<T>;}
impl<T> MainErrMap<T> for Result<T> {
    fn main_err_map(self, msg: &str) -> Result<T> {
        self.map_err(|child| pass_soul_error(SoulErrorKind::NoKind, SoulSpan::new(0, 0, 0), msg, child))
    }
}













































































