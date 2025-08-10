use threadpool::ThreadPool;
use hsoul::subfile_tree::SubFileTree;
use crate::{run_steps::{parser, source_reader, tokenizer}, MainErrMap};
use std::{fs::File, io::BufReader, path::{Path, PathBuf}, process::exit, sync::{mpsc::channel, Arc}, time::SystemTime};
use crate::{errors::soul_error::Result, steps::step_interfaces::i_parser::abstract_syntax_tree::soul_header_cache::ModifiedDate};
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, run_options::run_options::RunOptions, steps::{parser::get_header::get_header, step_interfaces::i_parser::{abstract_syntax_tree::soul_header_cache::SoulHeaderCache, parser_response::ParserResponse}}, utils::logger::Logger};

pub fn cache_files(run_option: &Arc<RunOptions>, logger: &Arc<Logger>) {

    let sub_files = if !run_option.sub_tree_path.as_os_str().is_empty() {
        
        let files = match get_sub_files(run_option) {
            Ok(val) => val,
            Err(err) => {
                for line in err.to_err_message() {
                    logger.error(line);
                } 

                exit(1);
            },
        };

        cache_all_subfiles(run_option.clone(), files.clone(), logger);
        Some(files)
    }
    else {
        None
    };

    if let Err(err) = cache_file(run_option.clone(), sub_files, Path::new(&run_option.file_path), logger.clone()) {
        let (reader, _) = get_file_reader(Path::new(&run_option.file_path)).main_err_map("while trying to get file reader")
            .inspect_err(|err| {
                for line in err.to_err_message() {
                    logger.error(line);
                } 
                exit(1);
            }).unwrap();

        logger.error("---------------------------------------------");
        for line in err.to_err_message() {
            logger.error(line);
        } 
        
        logger.error(format!("\n{}", err.to_highlighed_message(reader)));
    }

}

fn get_sub_files(run_option: &Arc<RunOptions>) -> Result<Arc<[PathBuf]>> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_option.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    Ok(sub_tree.get_all_file_paths())
}

fn cache_all_subfiles(run_option: Arc<RunOptions>, files: Arc<[PathBuf]>, logger: &Arc<Logger>) {

    let num_threads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(num_threads);
    let (sender, reciever) = channel();


    let sub_files = Some(files.clone());
    for file in files.iter() {
        let file = format!("{}.soul", file.to_string_lossy());
        let sender = sender.clone();
        let run_option = run_option.clone();
        
        let sfiles = sub_files.clone();
        let log = logger.clone();
        pool.execute(move || {
            let result = cache_file(run_option, sfiles, Path::new(&file), log);
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
        logger.error("at line:col; !!error!! message\n");        
        for (err, file) in errors {
            let (reader, _) = get_file_reader(Path::new(&file)).main_err_map("while trying to get file reader")
                .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
            
            logger.error("---------------------------------------------");  
            logger.error(format!("at subfile '{}':", file));
            for line in err.to_err_message() {
                logger.error(line);
            }
            logger.error(format!("\n{}", err.to_highlighed_message(reader)));              
        }
        exit(1)
    }
}

fn cache_file(run_option: Arc<RunOptions>, sub_files: Option<Arc<[PathBuf]>>, file_path: &Path, logger: Arc<Logger>) -> Result<()> {

    fn _is_cache_up_to_date(cache: Option<ModifiedDate>, date: SystemTime) -> bool {
        cache.is_some_and(|cache| cache.source_date == date)
    }

    let (reader, last_modified_date) = get_file_reader(file_path)
        .main_err_map("while trying to get file reader")?;

    
    if let Some(_date) = last_modified_date {
        let _cache_date = ModifiedDate::from_bin_file(&get_cache_date_path(&run_option, file_path)).ok();
        
        #[cfg(not(debug_assertions))]
        if _is_cache_up_to_date(_cache_date, _date) {
            logger.info(format!("using cache for file: {}", file_path.to_str().unwrap()));
            return Ok(());
        }
    }

    let source_response = source_reader(reader, &run_option, &logger).main_err_map("in source_reader")?;
    let token_response = tokenizer(source_response, &run_option, &logger).main_err_map("in tokenizer")?;
    let parser_response = parser(token_response, sub_files, &run_option, &logger).main_err_map("in parser")?;

    cache_parser(parser_response, &run_option, file_path)
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to cache parsed file\n{}", msg.to_string())))?;

    Ok(())
}

fn get_cache_path(run_option: &RunOptions, file_path: &Path) -> String {
    format!("{}/parsedIncremental/{}", run_option.output_dir.to_string_lossy(), file_path.to_str().unwrap())
}

fn get_cache_date_path(run_option: &RunOptions, file_path: &Path) -> String {
    format!("{}/parsedIncremental/{}.date", run_option.output_dir.to_string_lossy(), file_path.to_str().unwrap())
}

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>;
fn cache_parser(parser_response: ParserResponse, run_option: &RunOptions, file_path: &Path) -> ResErr<()> {
    let header = get_header(&parser_response.scopes);
    let cache = SoulHeaderCache::new(file_path, header, parser_response)?;
    cache.save_to_bin_file(&get_cache_path(run_option, file_path))?;
    Ok(())
}


fn get_file_reader(path: &Path) -> Result<(BufReader<File>, Option<SystemTime>)> {
    let file = File::open(&path)
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;
    
    let meta_data = file.metadata()
        .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(0,0,0), format!("while trying to open metadate of file path: '{}'\n{}", path.to_str().unwrap(), err.to_string())))?;

    Ok((BufReader::new(file), meta_data.modified().ok()))
}
























