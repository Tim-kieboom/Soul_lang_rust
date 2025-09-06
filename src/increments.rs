use itertools::Itertools;
use threadpool::ThreadPool;
use hsoul::subfile_tree::SubFileTree;
use std::{fs::{self, write, File}, result, time::SystemTime};
use std::{io::{BufReader, Read}, path::Path, sync::{mpsc::channel, Arc, Mutex}, time::Instant};
use crate::{errors::soul_error::{SoulError, SoulSpan}, file_cache::FileCache, run_options::run_options::RunOptions, steps::step_interfaces::i_parser::header::Header, utils::{logger::Logger, time_logs::TimeLogs}};
use crate::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulErrorKind}, run_options::{show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::parser::{parse_ast}, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::{abstract_syntax_tree::pretty_format::PrettyFormat, parser_response::ParserResponse}, i_source_reader::SourceFileResponse, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}, utils::logger::DEFAULT_LOG_OPTIONS};


pub fn parse_increment(run_options: &Arc<RunOptions>, logger: &Arc<Logger>, time_logs: &Arc<Mutex<TimeLogs>>) -> result::Result<(), String> {

    let has_sub_tree = !run_options.sub_tree_path.as_os_str().is_empty();

    let mut errors = vec![];
    if has_sub_tree {
        
        let files = get_sub_files(run_options)
            .map_err(|err| logger.panic_error(&err, DEFAULT_LOG_OPTIONS))
            .unwrap();

        errors = Vec::with_capacity(files.files_amount+1);
        parse_sub_files(run_options.clone(), files.clone(), logger, time_logs, &mut errors);
    }

    let main_file_path = Path::new(&run_options.file_path);
    if let Err(error) = parse_file(main_file_path, logger.clone(), run_options.clone(), time_logs.clone()) {
        let main_file = run_options.file_path.to_string_lossy().to_string();
        errors.push((error, main_file));
    }
    
    log_errors(errors, logger)
}

fn parse_sub_files(
    run_options: Arc<RunOptions>, 
    subfiles_tree: Arc<SubFileTree>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
    errors: &mut Vec<(SoulError, String)>,
) {

    let num_threads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(num_threads);
    let (sender, reciever) = channel();

    let subfiles = subfiles_tree.get_all_file_paths();
    for file in subfiles.iter() {
        let file = format!("{}.soul", file.to_string_lossy());
        let sender = sender.clone();
        let run_option = run_options.clone();
        
        let log = logger.clone();
        let t_log = time_logs.clone();
        pool.execute(move || {
            let result = parse_file(Path::new(&file), log, run_option, t_log);
            sender.send((result, file)).expect("channel receiver should be alive");
        });
    }

    drop(sender);

    for (result, file) in reciever {
        if let Err(err) = result {
            errors.push((err, file));
        }
    }

}

fn parse_file(
    file_path: &Path, 
    logger: Arc<Logger>, 
    run_options: Arc<RunOptions>, 
    time_logs: Arc<Mutex<TimeLogs>>
) -> Result<()> {

    let (reader, file_date) = get_file_reader(file_path)
        .map_err(|err| pass_soul_error(err.get_last_kind(), None, "while trying to get file reading", err))?;

    #[cfg(not(feature="dev_mode"))]
    if let Some(date) = file_date {
        

        let last_modified_date = FileCache::read_date(&run_options, file_path);
        if last_modified_date.ok() == Some(date) {
            logger.debug(format!("using cache for file: {}", file_path.to_str().unwrap()), DEFAULT_LOG_OPTIONS);
            return Ok(())
        }
    }

    let path_string = file_path.to_string_lossy().to_string();
    let info = RunStepsInfo{
        logger: &logger, 
        time_logs: &time_logs,
        run_options: &run_options, 
        current_path: &path_string, 
    };
    
    let source_response = source_reader(reader, &info)
        .map_err(|err| pass_soul_error(err.get_last_kind(), None, "while reading source file", err))?;
    
    let tokenize_response = tokenizer(source_response, &info)
        .map_err(|err| pass_soul_error(err.get_last_kind(), None, "while tokenizing file", err))?;

    let parser_reponse = parser(tokenize_response, &info)
        .map_err(|err| pass_soul_error(err.get_last_kind(), None, "while parsing file", err))?;

    cache_file(parser_reponse, &run_options, file_path)
        .map_err(|msg| new_soul_error(
            SoulErrorKind::InternalError, 
            None, 
            format!("error while trying to cache parsed file\n{}", msg.to_string())),
        )
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn cache_file(response: ParserResponse, run_options: &RunOptions, file_path: &Path) -> DynResult<()> {

    let header = Header::from_scope(&response.scopes);
    let cache = FileCache::new(file_path, header, response)?;
    cache.write_to_disk(run_options, file_path)   
}

fn log_errors(errors: Vec<(SoulError, String)>, logger: &Arc<Logger>) -> result::Result<(), String> {
    if errors.is_empty() {
        return Ok(())
    }

    let amount_errors = errors.len();
    for (mut error, file_name) in errors {
        let (mut reader, _) = get_file_reader(Path::new(&file_name))
                .map_err(|err| pass_soul_error(err.get_last_kind(), None, "while trying to get file reading", err))
                .inspect_err(|err| logger.panic_error(err, DEFAULT_LOG_OPTIONS))
                .unwrap();

        error = pass_soul_error(error.get_last_kind(), None, format!("at file: '{}'", file_name), error);
        logger.soul_error(&error, &mut reader, DEFAULT_LOG_OPTIONS);
    }

    Err(format!("build interrupted because of '{}' error{}", amount_errors, if amount_errors > 1 {"s"} else {""}))
} 

fn get_sub_files(run_options: &Arc<RunOptions>) -> Result<Arc<SubFileTree>> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_options.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, None, format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    Ok(Arc::new(sub_tree))
}

pub fn get_file_reader(path: &Path) -> Result<(BufReader<File>, Option<SystemTime>)> {
    let file = File::open(&path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, None, format!("while trying to open file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;
    
    let meta_data = file.metadata()
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, None, format!("while trying to open metadate of file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;

    Ok((BufReader::new(file), meta_data.modified().ok()))
}

pub struct RunStepsInfo<'a> {
    pub logger: &'a Arc<Logger>, 
    pub current_path: &'a String,
    pub run_options: &'a RunOptions, 
    pub time_logs: &'a Arc<Mutex<TimeLogs>>,
}

fn source_reader<'a, R: Read>(reader: BufReader<R>, info: &RunStepsInfo<'a>) -> Result<SourceFileResponse> {
    let tab_as_spaces = " ".repeat(info.run_options.tab_char_len as usize);
    
    let start = Instant::now(); 
    let source_file = read_source_file(reader, &tab_as_spaces)?;
    if info.run_options.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
        info.time_logs
            .lock().unwrap()
            .push(&info.current_path, "source_reader time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_SOURCE) {
        let start = Instant::now(); 

        let print_path = format!("{}/steps/{}", info.run_options.output_dir.to_string_lossy(), info.current_path);
        let file_path = format!("{}/source.soulc", print_path);
        let contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        fs::create_dir_all(&print_path).unwrap();
        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, None, err.to_string()))?;
        if info.run_options.show_times.contains(ShowTimes::SHOW_SOURCE_READER) {
            info.time_logs
                .lock().unwrap()
                .push(&info.current_path, "source_reader showOutput time", start.elapsed());
        }
    }

    Ok(source_file)
}

pub fn tokenizer<'a>(source_file: SourceFileResponse, info: &RunStepsInfo<'a>) -> Result<TokenizeResonse> {
    
    let start = Instant::now(); 
    let token_stream = tokenize(source_file)?;
    if info.run_options.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
        info.time_logs
            .lock().unwrap()
            .push(&info.current_path, "tokenizers time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_TOKENIZER) {
        let start = Instant::now(); 
        let print_path = format!("{}/steps/{}", info.run_options.output_dir.to_string_lossy(), info.current_path);
        let file_path = format!("{}/tokenStream.soulc", print_path);
        let contents = token_stream.stream
            .ref_tokens()
            .iter()
            .map(|token| &token.text)
            .join(" ");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, None, err.to_string()))?;

        if info.run_options.show_times.contains(ShowTimes::SHOW_TOKENIZER) {
            info.time_logs
                .lock().unwrap()
                .push(&info.current_path, "tokenizers showOutput time", start.elapsed());
        }
    }

    Ok(token_stream)
}

pub fn parser<'a>(token_response: TokenizeResonse, info: &RunStepsInfo<'a>) -> Result<ParserResponse> {
    
    let start = Instant::now(); 

    let parse_response = parse_ast(token_response)?;
    if info.run_options.show_times.contains(ShowTimes::SHOW_PARSER) {
        info.time_logs
            .lock().unwrap()
            .push(&info.current_path, "parser time", start.elapsed());
    }

    if info.run_options.show_outputs.contains(ShowOutputs::SHOW_ABSTRACT_SYNTAX_TREE) {
        let start = Instant::now(); 
        let print_path = format!("{}/steps/{}", info.run_options.output_dir.to_string_lossy(), info.current_path);
        let file_path = format!("{}/parserAST.soulc", print_path);
        let scopes_file_path = format!("{}/parserScopes.soulc", print_path);

        write(file_path, parse_response.tree.to_pretty_string())
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, None, err.to_string()))?;

        write(scopes_file_path, parse_response.scopes.to_pretty_string())
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, None, err.to_string()))?;

        if info.run_options.show_times.contains(ShowTimes::SHOW_PARSER) {
            info.time_logs
                .lock().unwrap()
                .push(&info.current_path, "parser showOutput time", start.elapsed());
        }
    }
    
    Ok(parse_response)
}


























