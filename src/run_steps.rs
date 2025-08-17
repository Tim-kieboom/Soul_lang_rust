use hsoul::subfile_tree::SubFileTree;
use itertools::Itertools;
use std::{env, fs::{self, write}, io::{BufReader, Read}, path::PathBuf, sync::{Arc, Mutex}, time::Instant};
use crate::{run_options::run_options::RunOptions, steps::{sementic_analyser::sementic::sementic_analyse_ast, step_interfaces::{i_parser::parser_response::ParserResponse, i_sementic::sementic_respone::SementicAnalyserResponse, i_source_reader::SourceFileResponse}}, utils::{logger::Logger, node_ref::{MultiRefPool}, time_logs::TimeLogs}};
use crate::{errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::parse::parse_tokens, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::abstract_syntax_tree::pretty_format::PrettyFormat, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}};

pub struct RunStepsInfo<'a> {
    pub logger: &'a Arc<Logger>, 
    pub current_path: &'a String,
    pub run_options: &'a RunOptions, 
    pub time_log: &'a Arc<Mutex<TimeLogs>>,
}

pub fn source_reader<'a, R: Read>(reader: BufReader<R>, info: &RunStepsInfo<'a>, path: &PathBuf, file_name: &String) -> Result<SourceFileResponse> {
    let tab_as_spaces = " ".repeat(info.run_options.tab_char_len as usize);
    
    let start = Instant::now(); 
    let source_file = read_source_file(reader, &tab_as_spaces)?;
    if info.run_options.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
        info.time_log
            .lock().unwrap()
            .push(&info.current_path, "source_reader time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_SOURCE) {
        let start = Instant::now(); 
        let mut file_path = info.run_options.output_dir.clone();
        file_path.push("steps");
        file_path.push(path);
        fs::create_dir_all(&file_path).map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("for path: '{}', {}", path.to_string_lossy(), err.to_string())))?;

        let file_path = get_out_path("sourceReader.soulc", info, path, file_name)?;
        let contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        if info.run_options.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
            info.time_log
                .lock().unwrap()
                .push(&info.current_path, "source_reader showOutput time", start.elapsed());
        }
    }

    Ok(source_file)
}

pub fn tokenizer<'a>(source_file: SourceFileResponse, info: &RunStepsInfo<'a>, path: &PathBuf, file_name: &String) -> Result<TokenizeResonse> {
    
    let start = Instant::now(); 
    let token_stream = tokenize(source_file)?;
    if info.run_options.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
        info.time_log
            .lock().unwrap()
            .push(&info.current_path, "tokenizers time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {
        let start = Instant::now(); 

        let file_path = get_out_path("tokenStream.soulc", info, path, file_name)?;
        let contents = token_stream.stream
            .ref_tokens()
            .iter()
            .map(|token| &token.text)
            .join(" ");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        if info.run_options.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
            info.time_log
                .lock().unwrap()
                .push(&info.current_path, "tokenizers showOutput time", start.elapsed());
        }
    }

    Ok(token_stream)
}

pub fn parser<'a>(token_response: TokenizeResonse, ref_pool: MultiRefPool, sub_files: Option<Arc<SubFileTree>>, info: &RunStepsInfo<'a>, path: &PathBuf, file_name: &String) -> Result<ParserResponse> {
    
    let start = Instant::now(); 

    #[cfg(feature="dev_mode")]
    println!(
        "currenly parsing: {}", &info.current_path
    );

    let absulute_path = env::current_dir().unwrap().join(info.run_options.file_path.clone());
    let project_name = absulute_path
        .parent()
        .expect("file_path should have parent (if not maybe file is in root of pc)")
        .file_name()
        .expect("file_path should have parent (if not maybe file is in root of pc)")
        .to_string_lossy()
        .to_string();

    let parse_response = parse_tokens(token_response, ref_pool, sub_files, project_name)?;
    if info.run_options.show_times.contains(ShowTimes::SHOW_PARSER) {
        info.time_log
            .lock().unwrap()
            .push(&info.current_path, "parser time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {
        let start = Instant::now(); 

        let file_path = get_out_path("parserAST.soulc", info, path, file_name)?;
        let scopes_file_path = get_out_path("parserScopes.soulc", info, path, file_name)?;

        write(file_path, format!("{}", parse_response.tree.to_pretty_string(&parse_response.scopes.ref_pool)))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        write(scopes_file_path, format!("{}", parse_response.scopes.to_pretty_string(&parse_response.scopes.ref_pool)))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
        
        if info.run_options.show_times.contains(ShowTimes::SHOW_PARSER) {
            info.time_log
                .lock().unwrap()
                .push(&info.current_path, "parser showOutput time", start.elapsed());
        }
    }
    

    Ok(parse_response)
}

pub fn sementic_analyse<'a>(parser_response: ParserResponse, ref_pool: &MultiRefPool, info: &RunStepsInfo<'a>, path: &PathBuf, file_name: &String) -> Result<SementicAnalyserResponse> {
    let start = Instant::now(); 

    let response = sementic_analyse_ast(parser_response, info.run_options)?;

    if info.run_options.show_times.contains(ShowTimes::SHOW_SEMENTIC_ANALYSER) {
        info.time_log
            .lock().unwrap()
            .push(&info.current_path, "sementic_analyser time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_SEMENTIC_ANALYSER) {
        let start = Instant::now(); 

        let file_path = get_out_path("sementicAST.soulc", info, path, file_name)?;
        let scopes_file_path = get_out_path("sementicScopes.soulc", info, path, file_name)?;

        write(file_path, format!("{}", response.tree.to_pretty_string(ref_pool)))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        write(scopes_file_path, format!("{}", response.scope.to_pretty_string(ref_pool)))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
        
        if info.run_options.show_times.contains(ShowTimes::SHOW_PARSER) {
            info.time_log
                .lock().unwrap()
                .push(&info.current_path, "sementic_analyser showOutput time", start.elapsed());
        }
    }

    Ok(response)
}


fn get_out_path<'a>(name: &str, info: &RunStepsInfo<'a>, path: &PathBuf, file_name: &String) -> Result<String> {
    let mut file_path = info.run_options.output_dir.clone();
    file_path.push("steps");
    file_path.push(path);
    file_path.push(file_name);

    fs::create_dir_all(&file_path).map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("for path: '{}', {}", path.to_string_lossy(), err.to_string())))?;


    Ok(format!("{}\\{}", file_path.to_string_lossy(), name))
}




























