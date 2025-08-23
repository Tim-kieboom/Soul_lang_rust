use itertools::Itertools;
use threadpool::ThreadPool;
use hsoul::subfile_tree::SubFileTree;
use std::{fs::{write, File}, process::exit, time::SystemTime};
use std::{io::{BufReader, Read}, path::Path, sync::{mpsc::channel, Arc, Mutex}, time::Instant};
use crate::{run_options::run_options::RunOptions, utils::{logger::Logger, time_logs::TimeLogs}};
use crate::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulErrorKind, SoulSpan}, run_options::{show_output::ShowOutputs, show_times::ShowTimes}, steps::{parser::parser::{parse_ast}, source_reader::source_reader::read_source_file, step_interfaces::{i_parser::{abstract_syntax_tree::pretty_format::PrettyFormat, parser_response::ParserResponse}, i_source_reader::SourceFileResponse, i_tokenizer::TokenizeResonse}, tokenizer::tokenizer::tokenize}, utils::logger::DEFAULT_LOG_OPTIONS};


pub fn parse_increment(run_options: &Arc<RunOptions>, logger: &Arc<Logger>, time_logs: &Arc<Mutex<TimeLogs>>) {

    let has_sub_tree = !run_options.sub_tree_path.as_os_str().is_empty();

    if has_sub_tree {
        
        let files = get_sub_files(run_options)
            .map_err(|err| logger.exit_error(&err, DEFAULT_LOG_OPTIONS))
            .unwrap();

        parse_sub_files(run_options.clone(), files.clone(), logger, time_logs);
    }

    let main_file_path = Path::new(&run_options.file_path);
    let result = parse_file(run_options.clone(), main_file_path, logger.clone(), time_logs.clone());
    if let Err(err) = result {

        let (mut reader, _) = get_file_reader(Path::new(&run_options.file_path))
            .map_err(|err| pass_soul_error(err.get_last_kind(), SoulSpan::new(0,0,0), "while trying to get file reading", err))
            .inspect_err(|err| logger.exit_error(err, DEFAULT_LOG_OPTIONS))
            .unwrap();

        logger.soul_error(&err, &mut reader, DEFAULT_LOG_OPTIONS);
        logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
        exit(1);
    }
}

fn parse_sub_files(
    run_options: Arc<RunOptions>, 
    subfiles_tree: Arc<SubFileTree>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>
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
            let result = parse_file(run_option, Path::new(&file), log, t_log);
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
        logger.error("at line:col; !!error!! message\n", DEFAULT_LOG_OPTIONS);        
        for (err, file) in errors {
            let (mut reader, _) = get_file_reader(Path::new(&file))
                .map_err(|err| pass_soul_error(err.get_last_kind(), SoulSpan::new(0,0,0), "while trying to get file reading", err))
                .inspect_err(|err| logger.exit_error(err, DEFAULT_LOG_OPTIONS))
                .unwrap();
            
            logger.error("---------------------------------------------", DEFAULT_LOG_OPTIONS);  
            logger.error(format!("at subfile '{}':", file), DEFAULT_LOG_OPTIONS);
            for line in err.to_err_message() {
                logger.error(line, DEFAULT_LOG_OPTIONS);
            }
            logger.error(format!("\n{}", err.to_highlighed_message(&mut reader)), DEFAULT_LOG_OPTIONS);              
        }
        logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
        exit(1)
    }
}

fn parse_file(
    run_options: Arc<RunOptions>, 
    file_path: &Path, 
    logger: Arc<Logger>, 
    time_logs: Arc<Mutex<TimeLogs>>
) -> Result<()> {

    let empty_span = SoulSpan::new(0,0,0);

    let (reader, _) = get_file_reader(file_path)
        .map_err(|err| pass_soul_error(err.get_last_kind(), empty_span, "while trying to get file reading", err))?;

    let path_string = file_path.to_string_lossy().to_string();
    let info = RunStepsInfo{current_path: &path_string, logger: &logger, run_options: &run_options, time_logs: &time_logs};
    
    let source_response = source_reader(reader, &info)
        .map_err(|err| pass_soul_error(err.get_last_kind(), empty_span, "while reading source file", err))?;
    
    let tokenize_response = tokenizer(source_response, &info)
        .map_err(|err| pass_soul_error(err.get_last_kind(), empty_span, "while tokenizing file", err))?;

    let parser_reponse = parser(tokenize_response, &info)
        .map_err(|err| pass_soul_error(err.get_last_kind(), empty_span, "while parsersing file", err))?;

    Ok(())
}

fn get_sub_files(run_options: &Arc<RunOptions>) -> Result<Arc<SubFileTree>> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_options.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    Ok(Arc::new(sub_tree))
}

pub fn get_file_reader(path: &Path) -> Result<(BufReader<File>, Option<SystemTime>)> {
    let file = File::open(&path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;
    
    let meta_data = file.metadata()
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open metadate of file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;

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
        let file_path = format!("{}/steps/source.soulc", info.run_options.output_dir.to_string_lossy());
        let contents = source_file.source_file
            .iter()
            .map(|line| &line.line)
            .join("\n");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
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
        let file_path = format!("{}/steps/tokenStream.soulc", info.run_options.output_dir.to_string_lossy());
        let contents = token_stream.stream
            .ref_tokens()
            .iter()
            .map(|token| &token.text)
            .join(" ");

        write(file_path, contents)
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

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
        let file_path = format!("{}/steps/parserAST.soulc", info.run_options.output_dir.to_string_lossy());
        let scopes_file_path = format!("{}/steps/parserScopes.soulc", info.run_options.output_dir.to_string_lossy());

        write(file_path, parse_response.tree.to_pretty_string())
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;

        write(scopes_file_path, parse_response.scopes.to_pretty_string())
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), err.to_string()))?;
        
        if info.run_options.show_times.contains(ShowTimes::SHOW_PARSER) {
            info.time_logs
                .lock().unwrap()
                .push(&info.current_path, "parser showOutput time", start.elapsed());
        }
    }
    
    Ok(parse_response)
}


























