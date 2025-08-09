use itertools::Itertools;
use std::{fs::write, io::{BufReader, Read}, path::PathBuf, sync::Arc, time::Instant};
use crate::{run_options::run_options::RunOptions, steps::step_interfaces::{i_parser::parser_response::ParserResponse, i_source_reader::SourceFileResponse}, utils::logger::Logger};
use crate::{errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::parse::parse_tokens, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::abstract_syntax_tree::pretty_format::PrettyFormat, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}};

pub fn source_reader<R: Read>(reader: BufReader<R>, run_option: &RunOptions, logger: &Arc<Logger>) -> Result<SourceFileResponse> {
    let tab_as_spaces = " ".repeat(run_option.tab_char_len as usize);
    
    let start = Instant::now(); 
    let source_file = read_source_file(reader, &tab_as_spaces)?;
    if run_option.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
        logger.debug(format!("source_reader time: {:.2?}", start.elapsed()));
    }

    if run_option.show_outputs.contains(ShowOutputs::SHOW_SOURCE) {
        let start = Instant::now(); 
        let file_path = format!("{}/steps/source.soulc", &run_option.output_dir);
        let contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        if run_option.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
            logger.debug(format!("source_reader showOutput time: {:.2?}", start.elapsed()));
        }
    }

    Ok(source_file)
}

pub fn tokenizer(source_file: SourceFileResponse, run_option: &RunOptions, logger: &Logger) -> Result<TokenizeResonse> {
    
    let start = Instant::now(); 
    let token_stream = tokenize(source_file)?;
    if run_option.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
        logger.debug(format!("tokenizers time: {:.2?}", start.elapsed()));
    }

    if run_option.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {
        let start = Instant::now(); 
        let file_path = format!("{}/steps/tokenStream.soulc", &run_option.output_dir);
        let contents = token_stream.stream
            .ref_tokens()
            .iter()
            .map(|token| &token.text)
            .join(" ");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        if run_option.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
            logger.debug(format!("tokenizers showOutput time: {:.2?}", start.elapsed()));
        }
    }

    Ok(token_stream)
}

pub fn parser(token_response: TokenizeResonse, sub_files: Option<Arc<[PathBuf]>>, run_option: &RunOptions, logger: &Arc<Logger>) -> Result<ParserResponse> {
    
    let start = Instant::now(); 
    let parse_response = parse_tokens(token_response, sub_files)?;
    if run_option.show_times.contains(ShowTimes::SHOW_PARSER) {
        logger.debug(format!("parser time: {:.2?}", start.elapsed()));
    }

    if run_option.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {
        let start = Instant::now(); 
        let file_path = format!("{}/steps/parserAST.soulc", &run_option.output_dir);
        let scopes_file_path = format!("{}/steps/parserScopes.soulc", &run_option.output_dir);

        write(file_path, format!("{}", parse_response.tree.to_pretty_string()))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        write(scopes_file_path, format!("{}", parse_response.scopes.to_pretty_string()))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
        
        if run_option.show_times.contains(ShowTimes::SHOW_PARSER) {
            logger.debug(format!("parser showOutput time: {:.2?}", start.elapsed()));
        }
    }
    

    Ok(parse_response)
}



































