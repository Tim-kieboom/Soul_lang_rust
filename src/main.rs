extern crate soul_lang_rust;

use colored::Colorize;
use std::{io::stderr, process::exit, result, sync::{Arc, Mutex}, time::Instant};
use soul_lang_rust::{cache_file::{cache_files, get_file_reader}, errors::soul_error::SoulError, generate_code_files::{generate_code_files, FileFaults}, meta_data::internal_functions_headers::load_std_headers, run_options::{run_options::RunOptions, show_times::ShowTimes}, steps::step_interfaces::i_sementic::fault::{SoulFaultKind}, utils::{logger::{LogOptions, Logger}, time_logs::{format_duration, TimeLogs}}, MainErrMap};

const DEFAULT_LOG_OPTIONS: &'static LogOptions = &LogOptions::const_default();

fn main() {
    let (run_options, logger, time_log) = init();
    
    #[cfg(debug_assertions)]
    if let Err(err) = load_std_headers() {
        eprintln!("{}", err.to_string());
        eprintln!("build interupted because of 1 error");
    }

    let start = Instant::now();
    
    cache_files(&run_options, &logger, &time_log);

    let faults = generate_code_files(&run_options, &logger, &time_log);

    let error_count = log_faults(faults, &logger);
    log_times(time_log, &run_options, &logger);

    if run_options.show_times.contains(ShowTimes::SHOW_TOTAL) {
        logger.info(format!("Total time: {}", format_duration(start.elapsed())), DEFAULT_LOG_OPTIONS);    
    }

    if error_count > 0 {
        logger.error(format!("build failed because of {} error{}", error_count, if error_count > 1 {"s"} else {""}), DEFAULT_LOG_OPTIONS);
        return
    }
}

fn init() -> (Arc<RunOptions>, Arc<Logger>, Arc<Mutex<TimeLogs>>) {
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
            eprintln!("build interrupted because of 1 error");
            exit(1)
        },
    };

    if let Err(err) = create_output_dir(&run_options) {
        logger.error(err.to_string(), DEFAULT_LOG_OPTIONS);
        logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
        exit(1)
    }

    (run_options, logger, Arc::new(Mutex::new(TimeLogs::new())))
}

fn log_times(times: Arc<Mutex<TimeLogs>>, run_option: &RunOptions, logger: &Arc<Logger>) {
    if run_option.show_times == ShowTimes::SHOW_NONE {
        return
    }

    const MAX_LEN: usize = 200;
    let table = if run_option.show_times.contains(ShowTimes::SHOW_ALL) {
        times.lock().unwrap().to_files_table_string(MAX_LEN)
    }
    else {
        times.lock().unwrap().to_total_table_string()
    };

    for line in table.lines() {
        logger.info(line, DEFAULT_LOG_OPTIONS);
    }
}

fn log_faults(faults: Vec<FileFaults>, logger: &Arc<Logger>) -> usize {
    
    let error_options = LogOptions::const_default();
    let warning_options = LogOptions{colored: true, highlight_soul: false};
    let note_options = LogOptions{colored: true, highlight_soul: false};

    let mut error_count = 0; 
    for FileFaults{file_path, faults} in faults {
        let (mut reader, _) = get_file_reader(std::path::Path::new(&file_path))
            .main_err_map("while trying to get file reader")
            .inspect_err(|err| exit_error(err, &logger))
            .unwrap();

        for fault in faults {
             match fault.kind {
                SoulFaultKind::Error => {
                    error_count += 1;
                    logger.soul_error(&fault.err, &mut reader, &error_options)
                },
                SoulFaultKind::Warning => logger.soul_warn(&fault.err, &mut reader, &warning_options),
                SoulFaultKind::Note => logger.soul_info(&fault.err, &mut reader, &note_options),
            }
        }
    }

    error_count
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
    std::fs::create_dir_all(format!("{}/steps", run_option.output_dir.to_string_lossy()))?;
    std::fs::create_dir_all(format!("{}/parsedIncremental", run_option.output_dir.to_string_lossy()))
}

fn exit_error(err: &SoulError, logger: &Arc<Logger>) {
    for line in err.to_err_message() {
        logger.error(line, DEFAULT_LOG_OPTIONS);
    } 
    logger.error("build interrupted because of 1 error", DEFAULT_LOG_OPTIONS);
    exit(1);
}










































