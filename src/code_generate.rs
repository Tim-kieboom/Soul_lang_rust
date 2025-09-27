use std::path::{PathBuf};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

use crate::file_cache::FileCache;
use crate::steps::step_interfaces::i_sementic::ast_visitor::{AstAnalyser, NameResolutionAnalyser};
use crate::steps::step_interfaces::i_sementic::scope_vistitor::ScopeVisitor;
use crate::steps::step_interfaces::i_sementic::soul_fault::SoulFault;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::steps::step_interfaces::i_sementic::sementic_response::SementicResponse;
use crate::utils::logger::DEFAULT_LOG_OPTIONS;
use crate::{run_options::run_options::RunOptions, utils::{logger::Logger, time_logs::TimeLogs}};

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
    let num_threads = std::thread::available_parallelism().unwrap().get();

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
                    let response = sementic_analyse(parser_response, &run_option, &log, &t_log, file);
                    sender.send(response).expect("channel receiver should be alive");
                },
                Err(err) => log.error(err, DEFAULT_LOG_OPTIONS),
            }
        });
    }

    drop(sender);

    for result in reciever {
        errors.push((result.scopes.file_path, result.faults))
    }

    Ok(errors)
}

fn sementic_analyse(
    parser: ParserResponse, 
    run_options: &Arc<RunOptions>, 
    logger: &Arc<Logger>, 
    time_logs: &Arc<Mutex<TimeLogs>>,
    file_path: PathBuf,
) -> SementicResponse {
    let ParserResponse{mut tree, scopes} = parser;
    let scope_vistitor = ScopeVisitor::new(scopes, file_path);

    const SHOULD_RESET_SCOPE: bool = true;

    let mut analyser = impl_ast_analyser(
        NameResolutionAnalyser::new(scope_vistitor, SHOULD_RESET_SCOPE)
    );
    analyser.analyse_ast(&mut tree);

    let (scopes, faults, has_error) = analyser.consume();

    SementicResponse{tree, scopes, faults, has_error}
}

///force analyser to impl AstAnalyser trait
fn impl_ast_analyser(val: impl AstAnalyser) -> impl AstAnalyser {
    val
}

