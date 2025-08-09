extern crate soul_lang_rust;

use colored::Colorize;
use std::{io::stderr, process::exit, result, sync::Arc, time::Instant};
use soul_lang_rust::{cache_file::cache_files, run_options::{run_options::RunOptions, show_times::ShowTimes}, utils::logger::Logger};

fn main() {
    let (run_options, logger) = init();

    let start = Instant::now();
    
    cache_files(&run_options, &logger);

    // let faults = generate_code(&run_option);

    // for fault in faults {
    //     let is_error = fault.is_error();
    //     let inner = fault.consume(); 
    // }

    if run_options.show_times.contains(ShowTimes::SHOW_TOTAL) {
        logger.debug(format!("Total time: {:.2?}", start.elapsed()));
    }
}

fn init() -> (Arc<RunOptions>, Arc<Logger>) {
    use std::env::args;

    let run_options = match RunOptions::new(args()) {
        Ok(val) => Arc::new(val),
        Err(msg) => {
            eprintln!("{}", format!("!!invalid compiler argument!!\n{msg}").red());
            exit(1)
        },
    };

    let logger = match get_logger(&run_options) {
        Ok(val) => Arc::new(val),
        Err(err) => {
            eprintln!("{}", err.red()); 
            exit(1)
        },
    };

    if let Err(err) = create_output_dir(&run_options) {
        logger.error(err.to_string());
        exit(1)
    }

    (run_options, logger)
}

fn get_logger(run_option: &RunOptions) -> result::Result<Logger, String> {
    let logger = if let Some(path) = &run_option.log_path {
        match Logger::with_file_path(path, run_option.log_mode, run_option.log_level) {
            Ok(val) => val,
            Err(err) => return Err(format!("while trying to get file based logger: {err}")),
        }
    }
    else {
        Logger::new(stderr(), run_option.log_mode, run_option.log_level)
    };
    
    Ok(logger)
}

fn create_output_dir(run_option: &RunOptions) -> std::io::Result<()> {
    std::fs::create_dir_all(format!("{}/steps", &run_option.output_dir))?;
    std::fs::create_dir_all(format!("{}/parsedIncremental", &run_option.output_dir))
}

// fn generate_code(run_option: &Arc<RunOptions>) -> Vec<SoulFault> {

// }
































































