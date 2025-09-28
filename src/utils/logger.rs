use std::{fmt::Display, fs::OpenOptions, io::{self, BufReader, Read, Seek, Write}, path::PathBuf, process::exit, sync::{Arc, Mutex, RwLock, RwLockReadGuard}};
use bitflags::bitflags;
use chrono::Local;
use colored::Colorize;

use crate::errors::soul_error::SoulError;

pub static DEFAULT_LOG_OPTIONS: RwLock<LogOptions> = RwLock::new(LogOptions::const_default());

pub fn default_log_options<'a>() -> RwLockReadGuard<'a, LogOptions> {
    DEFAULT_LOG_OPTIONS.read().unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error,
    Warning,
    Info,
    Debug,
    Any
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct LogMode: u8 {
        const ShowDate = 0b00000001;
        const ShowLevel = 0b00000010;
        const ShowAll = 0b11111111;
    }
}

impl LogMode {
    pub fn from_str(text: &str) -> Result<Self, String> {
        let tokens = text.split("+");
        let mut this = Self::empty();
        for token in tokens {
            match token {
                "SHOW_DATE" => this |= Self::ShowDate,
                "SHOW_LEVEL" => this |= Self::ShowLevel,
                "SHOW_ALL" => return Ok(Self::ShowAll), 
                "SHOW_NONE" => return Ok(Self::empty()),
                _ => return Err(format!("token: '{}' is not a valid option", token)),               
            }
        }

        Ok(this)
    }
}

impl LogLevel {

    pub fn from_str(text: &str) -> Self {
        match text {
            "ERROR" => LogLevel::Error,
            "WARNING" => LogLevel::Warning,
            "INFO" => LogLevel::Info,
            "DEBUG" => LogLevel::Debug,
            _ => LogLevel::Any,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Error => "ERROR",
            LogLevel::Warning  => "WARNING",
            LogLevel::Info  => "INFO",
            LogLevel::Debug => "DEBUG",
            LogLevel::Any => "ANY",
        }
    }
}

pub struct Logger {
    level: LogLevel,
    mode: LogMode,
    output: Arc<Mutex<Box<dyn Write + Send>>>,
}

#[derive(Debug, Clone)]
pub struct LogOptions {
    pub colored: bool,
    pub highlight_soul: bool,
    pub log_file_path: Option<PathBuf>,
}
impl LogOptions {
    pub fn new(colored: bool, highlight_soul: bool, log_file_path: Option<PathBuf>) -> Self {
        Self{colored, highlight_soul, log_file_path}
    }
    pub const fn const_default() -> Self {Self{colored: true, highlight_soul: false, log_file_path: None}}

    pub fn apply<F: FnOnce(Self) -> Self>(self, apply: F) -> Self {
        apply(self)
    }
}
impl Default for LogOptions {
    fn default() -> Self {Self{colored: true, highlight_soul: false, log_file_path: None}}
}

impl Logger {
    pub fn new<T: Write + Send + 'static>(output: T, mode: LogMode, level: LogLevel) -> Self {
        Self{ level, mode, output: Arc::new(Mutex::new(Box::new(output))) }
    }

    pub fn with_file_path(path: &PathBuf, mode: LogMode, level: LogLevel) -> io::Result<Self> {
        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(path)?;
        
        Ok(Self {
            level,
            mode,
            output: Arc::new(Mutex::new(Box::new(file))),
        })
    }

    fn current_time_string() -> String {
        let now = Local::now();
        now.format("%Y-%m-%d %H:%M:%S%.3f").to_string()
    }

    pub fn _log<S: Display>(&self, level: LogLevel, message: S, options: &LogOptions) {
        if level <= self.level {
            let mut log_msg = String::new();

            if self.mode.contains(LogMode::ShowDate) {
                log_msg.push('[');
                log_msg.push_str(&Self::current_time_string());
                log_msg.push_str("] ");
            }

            if self.mode.contains(LogMode::ShowLevel) {
                log_msg.push('[');
                log_msg.push_str(level.as_str());
                log_msg.push_str("] ");
            }

            log_msg.push_str(&message.to_string());
            log_msg.push('\n');

            let mut output = self.output.lock().unwrap();
            let color_msg = if options.colored {
                match level {
                    LogLevel::Error => log_msg.red(),
                    LogLevel::Warning => log_msg.yellow(),
                    LogLevel::Info => log_msg.blue(),
                    LogLevel::Debug => log_msg.purple(),
                    LogLevel::Any => log_msg.purple(),
                }.to_string()
            }
            else {
                log_msg
            };

            let msg = color_msg.as_bytes();
            
            let _ = output.write_all(msg);
            let _ = output.flush();
        }
    }

    pub fn _log_soul_error<R: Read + Seek>(&self, level: LogLevel, soul_error: &SoulError, reader: &mut BufReader<R>, options: &LogOptions) {
        self._log(level, "---------------------------------------------", options);
        if let Some(path) = &options.log_file_path {
            self._log(level, format!("in file: {}", path.to_string_lossy()), options);
        }

        for line in soul_error.to_err_message() {
            self._log(level, line, options);
        }
        self._log(level, format!("\n{}", soul_error.to_highlighed_message(reader)), options);
    }

    pub fn error<S: Display>(&self, msg: S, options: &LogOptions) { self._log(LogLevel::Error, &msg, options); }
    pub fn warn<S: Display>(&self, msg: S, options: &LogOptions) { self._log(LogLevel::Warning, &msg, options); }
    pub fn info<S: Display>(&self, msg: S, options: &LogOptions) { self._log(LogLevel::Info, &msg, options); }
    pub fn debug<S: Display>(&self, msg: S, options: &LogOptions) { self._log(LogLevel::Debug, &msg, options); }
    
    pub fn soul_error<R: Read + Seek>(&self, soul_error: &SoulError, reader: &mut BufReader<R>, options: &LogOptions) { self._log_soul_error(LogLevel::Error, soul_error, reader, options); }
    pub fn soul_warn<R: Read + Seek>(&self, soul_error: &SoulError, reader: &mut BufReader<R>, options: &LogOptions) { self._log_soul_error(LogLevel::Warning, soul_error, reader, options); }
    pub fn soul_info<R: Read + Seek>(&self, soul_error: &SoulError, reader: &mut BufReader<R>, options: &LogOptions) { self._log_soul_error(LogLevel::Info, soul_error, reader, options); }
    pub fn soul_debug<R: Read + Seek>(&self, soul_error: &SoulError, reader: &mut BufReader<R>, options: &LogOptions) { self._log_soul_error(LogLevel::Debug, soul_error, reader, options); }

    pub fn panic_error(&self, err: &SoulError, options: &LogOptions) {
        for line in err.to_err_message() {
            self.error(line, options);
        } 
        self.error("build interrupted because of 1 error", options);
        exit(1);
    }
}




























