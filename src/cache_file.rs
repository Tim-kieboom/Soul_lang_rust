use threadpool::ThreadPool;
use hsoul::subfile_tree::SubFileTree;
use crate::{run_steps::{parser, source_reader, tokenizer, RunStepsInfo}, utils::{logger::LogOptions, serde_multi_ref::{MultiRefPool}, time_logs::TimeLogs}, MainErrMap};
use std::{fs::File, io::BufReader, path::{Path, PathBuf}, process::exit, sync::{mpsc::channel, Arc, Mutex}, time::SystemTime};
use crate::{errors::soul_error::Result, steps::step_interfaces::i_parser::abstract_syntax_tree::soul_header_cache::ModifiedDate};
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, run_options::run_options::RunOptions, steps::{parser::get_header::get_header, step_interfaces::i_parser::{abstract_syntax_tree::soul_header_cache::SoulHeaderCache, parser_response::ParserResponse}}, utils::logger::Logger};

static DEFAULT_LOG_OPTIONS: &'static LogOptions = &LogOptions::const_default();

pub fn cache_files(run_option: &Arc<RunOptions>, logger: &Arc<Logger>, time_log: &Arc<Mutex<TimeLogs>>) {

    let sub_files = if !run_option.sub_tree_path.as_os_str().is_empty() {
        
        let files = match get_sub_files(run_option) {
            Ok(val) => val,
            Err(err) => {
                for line in err.to_err_message() {
                    logger.error(line, DEFAULT_LOG_OPTIONS);
                } 
                logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
                exit(1);
            },
        };

        cache_all_subfiles(run_option.clone(), files.clone(), logger, time_log);
        Some(files)
    }
    else {
        None
    };

    if let Err(err) = cache_file(run_option.clone(), MultiRefPool::new(), sub_files, Path::new(&run_option.file_path), logger.clone(), time_log.clone()) {
        let (mut reader, _) = get_file_reader(Path::new(&run_option.file_path)).main_err_map("while trying to get file reader")
            .inspect_err(|err| {
                for line in err.to_err_message() {
                    logger.error(line, DEFAULT_LOG_OPTIONS);
                } 
                logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
                exit(1);
            }).unwrap();

        logger.soul_error(&err, &mut reader, DEFAULT_LOG_OPTIONS);
        logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
        exit(1);
    }

}

fn get_sub_files(run_option: &Arc<RunOptions>) -> Result<Arc<SubFileTree>> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_option.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    Ok(Arc::new(sub_tree))
}

fn cache_all_subfiles(run_option: Arc<RunOptions>, subfiles_tree: Arc<SubFileTree>, logger: &Arc<Logger>, time_log: &Arc<Mutex<TimeLogs>>) {

    let num_threads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(num_threads);
    let (sender, reciever) = channel();

    let subfiles = subfiles_tree.get_all_file_paths();
    let possible_tree = Some(subfiles_tree.clone());
    for file in subfiles.iter() {
        let file = format!("{}.soul", file.to_string_lossy());
        let sender = sender.clone();
        let run_option = run_option.clone();
        
        let tree = possible_tree.clone();
        let log = logger.clone();
        let t_log = time_log.clone();
        pool.execute(move || {
            let result = cache_file(run_option, MultiRefPool::new(), tree, Path::new(&file), log, t_log);
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
            let (mut reader, _) = get_file_reader(Path::new(&file)).main_err_map("while trying to get file reader")
                .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
            
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

fn cache_file<'a>(run_option: Arc<RunOptions>, ref_pool: MultiRefPool, sub_files: Option<Arc<SubFileTree>>, file_path: &Path, logger: Arc<Logger>, time_log: Arc<Mutex<TimeLogs>>) -> Result<()> {

    fn _is_cache_up_to_date(cache: Option<ModifiedDate>, date: SystemTime) -> bool {
        cache.is_some_and(|cache| cache.source_date == date)
    }

    let (reader, last_modified_date) = get_file_reader(file_path)
        .main_err_map("while trying to get file reader")?;

    
    if let Some(_date) = last_modified_date {
        let _cache_date = ModifiedDate::from_bin_file(&get_cache_date_path(&run_option, file_path)).ok();
        
        #[cfg(not(debug_assertions))]
        if _is_cache_up_to_date(_cache_date, _date) {
            logger.info(format!("using cache for file: {}", file_path.to_str().unwrap()), DEFAULT_LOG_OPTIONS);
            return Ok(());
        }
    }

    let path_string = file_path.to_string_lossy().to_string();
    let info = RunStepsInfo{current_path: &path_string, logger: &logger, run_options: &run_option, time_log: &time_log};

    let mut path = PathBuf::from(file_path);
    let file_name = path.file_name().expect("path has filename").to_string_lossy().to_string();
    path.pop();
    let source_response = source_reader(reader, &info, &path, &file_name).main_err_map("in source_reader")?;
    let token_response = tokenizer(source_response, &info, &path, &file_name).main_err_map("in tokenizer")?;
    let parser_response = parser(token_response, ref_pool, sub_files, &info, &path, &file_name).main_err_map("in parser")?;

    cache_parser(parser_response, &run_option, file_path)
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to cache parsed file\n{}", msg.to_string())))?;

    Ok(())
}

pub fn get_cache_path(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = run_option.output_dir.clone();
    dir.push("parsedIncremental");
    dir.push(file_path);
    dir
}

pub fn get_cache_path_header(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = run_option.output_dir.clone();
    dir.push("parsedIncremental");
    dir.push(format!("{}.header", file_path.to_string_lossy()));
    dir
}

pub fn get_cache_path_ast(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = run_option.output_dir.clone();
    dir.push("parsedIncremental");
    dir.push(format!("{}.AST", file_path.to_string_lossy()));
    dir
}

fn get_cache_date_path(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = run_option.output_dir.clone();
    dir.push("parsedIncremental");
    dir.push(format!("{}.date", file_path.to_string_lossy()));
    dir
}

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn cache_parser(parser_response: ParserResponse, run_option: &RunOptions, file_path: &Path) -> ResErr<()> {
    let header = get_header(&parser_response.scopes);
    let cache = SoulHeaderCache::new(file_path, header, parser_response)?;
    cache.save_to_bin_file(&get_cache_path(run_option, file_path))?;
    Ok(())
}


pub fn get_file_reader(path: &Path) -> Result<(BufReader<File>, Option<SystemTime>)> {
    let file = File::open(&path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;
    
    let meta_data = file.metadata()
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open metadate of file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;

    Ok((BufReader::new(file), meta_data.modified().ok()))
}
























