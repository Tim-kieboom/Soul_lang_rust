use std::cmp::min;
use std::path::{PathBuf};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use threadpool::ThreadPool;

use crate::file_cache::FileCache;
use crate::run_options::show_times::ShowTimes;
use crate::utils::logger::DEFAULT_LOG_OPTIONS;
use crate::steps::step_interfaces::i_parser::header::ExternalHeaders;
use crate::steps::step_interfaces::i_sementic::soul_fault::SoulFault;
use crate::steps::step_interfaces::i_sementic::scope_vistitor::ScopeVisitor;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::steps::step_interfaces::i_sementic::sementic_response::SementicResponse;
use crate::{run_options::run_options::RunOptions, utils::{logger::Logger, time_logs::TimeLogs}};
use crate::steps::step_interfaces::i_sementic::ast_visitor::{AstAnalyser, ExternalHeaderAnalyser, NameResolutionAnalyser};

/// Runs semantic analysis and code generation preparation for all parsed source files.
///
/// This function performs the following steps:
/// 1. Collects all source file paths from the provided [`RunOptions`].
/// 2. For each file, retrieves the cached [`ParserResponse`] from disk (produced by [`parse_increment`]).
/// 3. Performs semantic analysis on the parsed AST using multiple analysers:
///    - [`NameResolutionAnalyser`]
///    - [`ExternalHeaderAnalyser`]
///    - [`ScopeVisitor`]
/// 4. Aggregates semantic faults and warnings from all files.
/// 5. Records timing information for analysis if `ShowTimes::SHOW_CODE_GENERATOR` is enabled.
///
/// # Parameters
/// - `run_options`: Shared configuration for the current compilation run, including 
///   file paths, cache options, and display settings.
/// - `logger`: Shared logging facility used for reporting information, warnings, and errors.
/// - `time_logs`: Shared timing table for recording performance metrics across compilation stages.
///
/// # Returns
/// - `Ok(Vec<(PathBuf, Vec<SoulFault>)>)` containing a list of files and the faults detected in each.
/// - `Err(String)` if a failure prevents code generation from proceeding (e.g., missing cached parse data).
///
/// # Notes
/// - Runs files in parallel using a thread pool (limited by [`RunOptions::max_thread_count`] or CPU count).
/// - Panics if a thread sends back an unexpected error instead of a valid semantic response.
/// - This step depends on [`parse_increment`] having successfully cached parser results for all files.
///
/// # Example
/// ```ignore
/// let (run_options, logger, time_logs) = init();
/// match generate_code(&run_options, &logger, &time_logs) {
///     Ok(errors) => {
///         for (path, faults) in errors {
///             println!("{} had {} issues", path.display(), faults.len());
///         }
///     }
///     Err(err) => logger.error(err, &default_log_options()),
/// }
/// ```
pub fn generate_code(
    run_options: &Arc<RunOptions>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
) -> Result<Vec<(PathBuf, Vec<SoulFault>)>, String> {
    
    let source_files = run_options.get_file_paths()
        .map_err(|msg| msg.to_err_message().join(" "))?;
    
    generate_all_codes(run_options, logger, time_logs, source_files)
}

fn generate_all_codes(
    run_options: &Arc<RunOptions>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
    subfiles: Vec<PathBuf>,
) -> Result<Vec<(PathBuf, Vec<SoulFault>)>, String> {
    
    let mut errors = vec![];
    let available_threads = std::thread::available_parallelism().unwrap().get();
    let num_threads = if let Some(max_threads) = run_options.max_thread_count {
        min(available_threads, max_threads)
    }
    else {
        available_threads
    };

    let pool = ThreadPool::new(num_threads.min(subfiles.len()));
    let (sender, reciever) = channel();
    
    for file in subfiles {
        let sender = sender.clone();
        let run_option = run_options.clone();
        
        let log = logger.clone();
        let t_log = time_logs.clone();
        pool.execute(move || {
            
            match FileCache::read_parse(&run_option, &file) {
                Ok(parser_response) => {
                    let response = sementic_analyse(parser_response, &run_option, &t_log, file);
                    sender.send(response).expect("channel receiver should be alive");
                },
                Err(err) => log.error(err, &DEFAULT_LOG_OPTIONS.read().unwrap()),
            }
        });
    }

    drop(sender);

    for result in reciever {
        
        match result {
            Ok(response) => errors.push((response.path, response.faults)),
            Err(err) => panic!("build interupted code generation failed, error: {}", err),
        }
    }

    Ok(errors)
}

fn sementic_analyse(
    parser: ParserResponse, 
    run_options: &Arc<RunOptions>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
    file_path: PathBuf,
) -> Result<SementicResponse, String> {
    let ParserResponse{mut tree, scopes} = parser;
    let scope_vistitor = ScopeVisitor::new(scopes, ExternalHeaders::new(run_options)?);

    const SHOULD_RESET_SCOPE: bool = true;

    let start = Instant::now();

    let mut analyser = NameResolutionAnalyser::new(scope_vistitor, SHOULD_RESET_SCOPE);
    analyser.analyse_ast(&mut tree);

    let mut analyser = ExternalHeaderAnalyser::new(analyser, SHOULD_RESET_SCOPE);
    analyser.analyse_ast(&mut tree);

    let (scopes, faults, has_error) = analyser.consume_to_tuple();

    if run_options.show_times.contains(ShowTimes::SHOW_CODE_GENERATOR) {
        time_logs
            .lock().unwrap()
            .push(&file_path.to_string_lossy().to_string(), "semeticAnalyser", start.elapsed());
    }

    Ok(SementicResponse{tree, scopes, faults, has_error, path: file_path})
}

