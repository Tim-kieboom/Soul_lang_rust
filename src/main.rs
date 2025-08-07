extern crate soul_lang_rust;

use itertools::Itertools;
use threadpool::ThreadPool;
use hsoul::subfile_tree::SubFileTree;  
use std::{fs::{write, File}, io::{BufReader, Read}, path::Path, process::exit, sync::{mpsc::channel, Arc}, time::{Instant, SystemTime}};
use soul_lang_rust::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{run_options::RunOptions, show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::{get_header::get_header, parse::parse_tokens}, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::{abstract_syntax_tree::{pretty_format::PrettyFormat, soul_header_cache::{ModifiedDate, SoulHeaderCache}}, parser_response::ParserResponse}, i_source_reader::SourceFileResponse, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}};

fn main() {

    let run_option = match RunOptions::new(std::env::args()) {
        Ok(val) => Arc::new(val),
        Err(msg) => {eprintln!("!!invalid compiler argument!!\n{msg}"); return;},
    };

    let start = Instant::now();

    if let Err(err) = create_output_dir(&run_option) {
        eprintln!("{}", err.to_string());
        return;
    }
    
    if !parse_and_cache_files(&run_option) {
        return;
    }

    if run_option.show_times.contains(ShowTimes::SHOW_TOTAL) {
        println!("Total time: {:.2?}", start.elapsed());
    }
}

fn create_output_dir(run_option: &RunOptions) -> std::io::Result<()> {
    std::fs::create_dir_all(format!("{}/steps", &run_option.output_dir))?;
    std::fs::create_dir_all(format!("{}/parsedIncremental", &run_option.output_dir))
}

fn parse_and_cache_files(run_option: &Arc<RunOptions>) -> bool {
    let mut no_errors = true;
    if !run_option.sub_tree_path.is_empty() {

        if let Err(err) = parse_and_cache_all_subfiles(run_option.clone()) {
            eprintln!("{}", err.to_err_message()); 
            no_errors = false;
        }
    }

    if let Err(err) = parse_and_cache_file(run_option.clone(), Path::new(&run_option.file_path)) {
        let (reader, _) = get_file_reader(Path::new(&run_option.file_path)).main_err_map("while trying to get file reader")
            .inspect_err(|err| {eprintln!("{}", err.to_err_message()); exit(1);}).unwrap();

        eprintln!("---------------------------------------------");  
        eprintln!("at line:col; !!error!! message\n\n{}\n", err.to_err_message());        
        eprintln!("{}", err.to_highlighed_message(reader));        
        no_errors = false;
    }

    return no_errors;
}

fn parse_and_cache_all_subfiles(run_option: Arc<RunOptions>) -> Result<()> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_option.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    let files = sub_tree.get_all_file_paths();
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(num_threads);
    let (sender, reciever) = channel();

    for file in files {
        let file = format!("{file}.soul");
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
        eprintln!("at line:col; !!error!! message\n");        
        for (err, file) in errors {
            let (reader, _) = get_file_reader(Path::new(&file)).main_err_map("while trying to get file reader")
                .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
            
            eprintln!("---------------------------------------------");  
            eprintln!("at subfile '{}':\n{}", file, err.to_err_message());  
            eprintln!("\n{}\n", err.to_highlighed_message(reader));                
        }
        exit(1)
    }

    Ok(())
}

fn parse_and_cache_file(run_option: Arc<RunOptions>, file_path: &Path) -> Result<()> {

    fn _is_cache_up_to_date(cache: Option<ModifiedDate>, date: SystemTime) -> bool {
        cache.is_some_and(|cache| cache.source_date == date)
    }

    let (reader, last_modified_date) = get_file_reader(file_path)
        .main_err_map("while trying to get file reader")?;

    
    if let Some(_date) = last_modified_date {
        let _cache_date = ModifiedDate::from_bin_file(&get_cache_date_path(&run_option, file_path)).ok();
        
        #[cfg(not(debug_assertions))]
        if _is_cache_up_to_date(_cache_date, _date) {
            println!("using cache for file: {}", file_path.to_str().unwrap());
            return Ok(());
        }
    }

    let source_response = source_reader(reader, &run_option).main_err_map("in source_reader")?;
    let token_response = tokenizer(source_response, &run_option).main_err_map("in tokenizer")?;
    let parser_response = parser(token_response, &run_option).main_err_map("in parser")?;

    cache_parser(parser_response, &run_option, file_path)
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to cache parsed file\n{}", msg.to_string())))?;

    Ok(())
}

fn get_cache_path(run_option: &RunOptions, file_path: &Path) -> String {
    format!("{}/parsedIncremental/{}", &run_option.output_dir, file_path.to_str().unwrap())
}

fn get_cache_date_path(run_option: &RunOptions, file_path: &Path) -> String {
    format!("{}/parsedIncremental/{}.date", &run_option.output_dir, file_path.to_str().unwrap())
}

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn cache_parser(parser_response: ParserResponse, run_option: &RunOptions, file_path: &Path) -> ResErr<()> {
    let header = get_header(&parser_response.scopes);
    let cache = SoulHeaderCache::new(file_path, header, parser_response)?;

    cache.save_to_bin_file(&get_cache_path(run_option, file_path))?;
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

fn get_file_reader(path: &Path) -> Result<(BufReader<File>, Option<SystemTime>)> {
    let file = File::open(&path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;
    
    let meta_data = file.metadata()
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open metadate of file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;

    Ok((BufReader::new(file), meta_data.modified().ok()))
}

trait MainErrMap<T>{fn main_err_map(self, msg: &str) -> Result<T>;}
impl<T> MainErrMap<T> for Result<T> {
    fn main_err_map(self, msg: &str) -> Result<T> {
        self.map_err(|child| pass_soul_error(SoulErrorKind::NoKind, SoulSpan::new(0, 0, 0), msg, child))
    }
}


































































