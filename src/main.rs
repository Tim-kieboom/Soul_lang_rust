extern crate soul_lang_rust;

use itertools::Itertools;
use threadpool::ThreadPool;
use std::{fs::{write, File}, io::{BufReader, Read}, path::Path, process::exit, sync::{mpsc::channel, Arc}, time::Instant};
use soul_lang_rust::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{run_options::RunOptions, show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::{get_header::get_header, parse::parse_tokens}, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::{abstract_syntax_tree::{pretty_format::PrettyFormat, soul_header_cache::SoulHeaderCache}, parser_response::ParserResponse}, i_source_reader::SourceFileResponse, i_subfile_tree::SubFileTree, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}};


fn main() {

    let run_option = match RunOptions::new(std::env::args()) {
        Ok(val) => Arc::new(val),
        Err(msg) => {eprintln!("!!invalid compiler argument!!\n{msg}"); return;},
    };

    let start = Instant::now();

    if !run_option.sub_tree_path.is_empty() {

        match compile_all_subfiles(run_option.clone()) {
            Ok(()) => (),
            Err(msg) => {eprintln!("{}", msg.to_err_message()); return;},
        }
    }

    if let Err(err) = parse_and_cache_file(run_option.clone(), Path::new(&run_option.file_path), ) {
        let reader = get_file_reader(&run_option, Path::new(&run_option.file_path)).main_err_map("while trying to get file reader")
            .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

        eprintln!("at char:line; !!error!! message\n\n{}\n", err.to_err_message());        
        eprintln!("{}", err.to_highlighed_message(reader));        
    }

    if run_option.show_times.contains(ShowTimes::SHOW_TOTAL) {
        println!("Total time: {:.2?}", start.elapsed());
    }
}

fn compile_all_subfiles(run_option: Arc<RunOptions>) -> Result<()> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_option.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    let files = sub_tree.get_all_file_paths();
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(num_threads);
    let (sender, reciever) = channel();

    for file in files {
        let sender = sender.clone();
        let run_option = run_option.clone();

        pool.execute(move || {
            let result = parse_and_cache_file(run_option, Path::new(&file));
            sender.send((result, file)).expect("channel receiver should be alive");
        });
    }

    drop(sender);

    let mut errors = vec![];

    for (result, file) in reciever {
        if let Err(err) = result {
            errors.push((err, file));
        }
    }

    if !errors.is_empty() {
        eprintln!("at char:line; !!error!! message\n");        
        for (err, file) in errors {
            let reader = get_file_reader(&run_option, Path::new(&file)).main_err_map("while trying to get file reader")
                .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
            
            eprintln!("\n{}\n", err.to_err_message());                
            eprintln!("at subfile '{}':\n{}", file, err.to_highlighed_message(reader));  
        }
        exit(1)
    }

    Ok(())
}

fn parse_and_cache_file(run_option: Arc<RunOptions>, file_path: &Path) -> Result<()> {

    let reader = get_file_reader(&run_option, file_path).main_err_map("while trying to get file reader")?;
    let source_response = source_reader(reader, &run_option).main_err_map("in source_reader")?;
    let token_response = tokenizer(source_response, &run_option).main_err_map("in tokenizer")?;
    let parser_response = parser(token_response, &run_option).main_err_map("in parser")?;

    cache_parser(parser_response, &run_option, file_path)
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to cache parsed file\n{}", msg.to_string())))?;

    Ok(())
}

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn cache_parser(parser_response: ParserResponse, run_option: &RunOptions, file_path: &Path) -> ResErr<()> {
    let header = get_header(&parser_response.scopes);
    let cache = SoulHeaderCache::new(file_path, header, parser_response)?;

    cache.save_to_bin_file(&format!("{}/parsedIncremental/{}.bin", &run_option.output_dir, file_path.to_str().unwrap()))?;
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
        let start = Instant::now(); 
        let file_path = format!("{}/steps/source.soulc", &run_option.output_dir);
        let contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        if run_option.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
            println!("source_reader showOutput time: {:.2?}", start.elapsed());
        }
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
            println!("tokenizers showOutput time: {:.2?}", start.elapsed());
        }
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
        let start = Instant::now(); 
        let file_path = format!("{}/steps/parserAST.soulc", &run_option.output_dir);
        let scopes_file_path = format!("{}/steps/parserScopes.soulc", &run_option.output_dir);

        write(file_path, format!("{}", parse_response.tree.to_pretty_string()))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        write(scopes_file_path, format!("{}", parse_response.scopes.to_pretty_string()))
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
        
        if run_option.show_times.contains(ShowTimes::SHOW_PARSER) {
            println!("parser showOutput time: {:.2?}", start.elapsed());
        }
    }
    

    Ok(parse_response)
}

fn get_file_reader(run_option: &RunOptions, path: &Path) -> Result<BufReader<File>> {
    std::fs::create_dir_all(format!("{}/steps", &run_option.output_dir))
        .map_err(|err| new_soul_error(SoulErrorKind::ArgError, SoulSpan::new(0,0,0), &err.to_string()))?;
    std::fs::create_dir(format!("{}/parsedIncremental", &run_option.output_dir))
        .map_err(|err| new_soul_error(SoulErrorKind::ArgError, SoulSpan::new(0,0,0), &err.to_string()))?;

    let file = File::open(&path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;

    Ok(BufReader::new(file))
}

trait MainErrMap<T>{fn main_err_map(self, msg: &str) -> Result<T>;}
impl<T> MainErrMap<T> for Result<T> {
    fn main_err_map(self, msg: &str) -> Result<T> {
        self.map_err(|child| pass_soul_error(SoulErrorKind::NoKind, SoulSpan::new(0, 0, 0), msg, child))
    }
}






































































