extern crate soul_lang_rust;

use colored::Colorize;
use std::{io::stderr, path::PathBuf, process::exit, result, sync::{Arc, Mutex}, time::Instant};
use soul_lang_rust::{code_generate::generate_code, errors::soul_error::pass_soul_error, increments::{get_file_reader, parse_increment}, run_options::{run_options::RunOptions, show_times::ShowTimes}, steps::step_interfaces::i_sementic::soul_fault::{SoulFault, SoulFaultKind}, utils::{logger::{default_log_options, LogLevel, LogOptions, Logger, DEFAULT_LOG_OPTIONS}, time_logs::{format_duration, TimeLogs}}};


fn main() {
    let (run_options, logger, time_logs) = init();
 
    let timer = Instant::now();

    if let Err(msg) = parse_increment(&run_options, &logger, &time_logs) {
        logger.error(msg, &default_log_options());
        return
    }

    let errors = match generate_code(&run_options, &logger, &time_logs) {
        Ok(val) => val,
        Err(err) => {
            logger.error(err, &default_log_options());
            return
        },
    };
    

    let error_len = log_faults(errors, &logger);
    log_time_table(time_logs, &run_options, &logger);

    if run_options.show_times.contains(ShowTimes::SHOW_TOTAL) {
        logger.info(format!("Total time: {}", format_duration(timer.elapsed())), &default_log_options());    
    }

    if error_len > 0 {
        logger.error(format!("build failed because of {} error{}", error_len, if error_len > 1 {"s"} else {""}), &default_log_options());
        return
    }
}

fn log_faults(errors: Vec<(PathBuf, Vec<SoulFault>)>, logger: &Logger) -> usize {
    
    let mut options = default_log_options().clone();
    let mut errors_len = 0;
    for (file, errors) in errors {
        let (mut reader, _) = get_file_reader(&file)
            .map_err(|err| pass_soul_error(err.get_last_kind(), None, "while trying to get file reading", err))
            .inspect_err(|err| logger.panic_error(err, &default_log_options()))
            .unwrap();

        options = options.apply(|mut options| {
            options.log_file_path = Some(file.clone());
            options
        });

        for error in errors {
            let level = match error.kind {
                SoulFaultKind::Note => LogLevel::Info,
                SoulFaultKind::Error => {
                    errors_len += 1;
                    LogLevel::Error
                },
                SoulFaultKind::Warning => LogLevel::Warning,
            };

            logger._log_soul_error(level, &error.msg, &mut reader, &options);
        }
    }

    errors_len
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

    *DEFAULT_LOG_OPTIONS.write().unwrap() = LogOptions{colored: run_options.log_colored, ..Default::default()};

    let logger = match get_logger(&run_options) {
        Ok(val) => Arc::new(val),
        Err(err) => {
            let first = err;
            let second = "build interrupted because of 1 error";
            
            if default_log_options().colored {
                eprintln!("{}", first.red()); 
                eprintln!("{}", second.red());
            }
            else {
                eprintln!("{}", first); 
                eprintln!("{}", second);
            }
            exit(1)
        },
    };

    if let Err(err) = create_output_dir(&run_options) {
        logger.error(err.to_string(), &default_log_options());
        logger.error("build interrupted because of 1 error", &default_log_options());
        exit(1)
    }

    (run_options, logger, Arc::new(Mutex::new(TimeLogs::new())))
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

fn log_time_table(times: Arc<Mutex<TimeLogs>>, run_option: &RunOptions, logger: &Logger) {
    if run_option.show_times == ShowTimes::SHOW_NONE {
        return
    }

    const MAX_TABLE_LEN: usize = 200;
    let table = if run_option.show_times.contains(ShowTimes::SHOW_ALL) {
        times.lock()
            .unwrap()
            .to_table_string(MAX_TABLE_LEN)
    }
    else {
        times.lock()
            .unwrap()
            .to_total_only_table_string()
    };

    for line in table.lines() {
        logger.info(line, &default_log_options());
    }
}




