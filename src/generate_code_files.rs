use std::path::PathBuf;
use std::process::exit;
use std::sync::mpsc::channel;
use std::{path::Path, sync::Arc};
use hsoul::subfile_tree::SubFileTree;
use threadpool::ThreadPool;
use crate::cache_file::get_cache_path_ast;
use crate::errors::soul_error::{Result, SoulError};
use crate::run_steps::{sementic_analyse, RunStepsInfo};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_header_cache::SoulHeaderCache;
use crate::utils::logger::LogOptions;
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, run_options::run_options::RunOptions, steps::step_interfaces::i_sementic::fault::SoulFault, utils::{logger::Logger, node_ref::MultiRef, time_logs::TimeLogs}};

const DEFAULT_LOG_OPTIONS: &'static LogOptions = &LogOptions::const_default();

pub struct FileFaults {
    pub file_path: PathBuf,
    pub faults: Vec<SoulFault>,
}

pub fn generate_code_files(run_options: &Arc<RunOptions>, logger: &Arc<Logger>, time_log: &MultiRef<TimeLogs>) -> Vec<FileFaults> {
    
    let mut faults = if !run_options.sub_tree_path.as_os_str().is_empty() {
        
        let files = match get_sub_files(run_options) {
            Ok(val) => val,
            Err(err) => {
                for line in err.to_err_message() {
                    logger.error(line, DEFAULT_LOG_OPTIONS);
                } 
                logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
                exit(1);
            },
        };

        generate_code_all_subfiles(run_options.clone(), files.clone(), logger, time_log)
    }
    else {
        vec![]
    };

    let file_fault = FileFaults{ file_path: run_options.file_path.clone(), faults: generate_code(run_options.clone(), run_options.file_path.clone(), logger.clone(), time_log.clone()) };
    faults.push(file_fault);

    faults
}

fn generate_code_all_subfiles(run_options: Arc<RunOptions>, subfiles_tree: Arc<SubFileTree>, logger: &Arc<Logger>, time_log: &MultiRef<TimeLogs>) -> Vec<FileFaults> {
    
    let num_threads = std::thread::available_parallelism().unwrap().get();
    let pool = ThreadPool::new(num_threads);
    let (sender, reciever) = channel();

    let subfiles = subfiles_tree.get_all_file_paths();
    for file in subfiles.iter() {
        let file = format!("{}.soul", file.to_string_lossy());
        let sender = sender.clone();
        let run_option = run_options.clone();
        
        let log = logger.clone();
        let t_log = time_log.clone();
        pool.execute(move || {
            let result = generate_code(run_option, PathBuf::from(&file), log, t_log);
            sender.send((result, file)).expect("channel receiver should be alive");
        });
    }

    drop(sender);

    let mut faults = vec![];
    for (results, file) in reciever {
        faults.push(FileFaults{ file_path: PathBuf::from(file), faults: results });
    }

    faults
}

fn generate_code(run_options: Arc<RunOptions>, file: PathBuf, logger: Arc<Logger>, time_log: MultiRef<TimeLogs>) -> Vec<SoulFault> {
    let path = get_cache_path_ast(&run_options, &file);
    let parser_responese = match SoulHeaderCache::ast_from_bin_file(&path) {
        Ok(val) => val,
        Err(err) => {
            let err = new_soul_error(
                SoulErrorKind::ReaderError, 
                SoulSpan::new(0,0,0), 
                format!("while trying to open main files: '{}' cache: {}", path.to_string_lossy(), err.to_string())
            );
            exit_error(&err, &logger);
            unreachable!()
        },
    };

    let path_string = file.to_string_lossy().to_string();
    let info = RunStepsInfo{current_path: &path_string, logger: &logger, run_options: &run_options, time_log: &time_log};

    let sementic_response = match sementic_analyse(parser_responese, &info) {
        Ok(val) => val,
        Err(err) => {
            exit_error(&err, &logger);
            unreachable!()
        },
    };

    //add code_generator

    sementic_response.faults
}

fn get_sub_files(run_option: &Arc<RunOptions>) -> Result<Arc<SubFileTree>> {
    let sub_tree = SubFileTree::from_bin_file(Path::new(&run_option.sub_tree_path))
        .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
    
    Ok(Arc::new(sub_tree))
}

fn exit_error(err: &SoulError, logger: &Arc<Logger>) {
    for line in err.to_err_message() {
        logger.error(line, DEFAULT_LOG_OPTIONS);
    } 
    logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
    exit(1);
}
















