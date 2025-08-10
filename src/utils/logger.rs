use std::{fmt::Display, fs::OpenOptions, io::{self, Write}, path::PathBuf, sync::{Arc, Mutex}};
use bitflags::bitflags;
use chrono::Local;
use colored::Colorize;

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

pub struct Options{pub colored: bool}
impl Default for Options {
    fn default() -> Self {Self{colored: true}}
}

impl Logger {
    pub fn new<T: Write + Send + 'static>(output: T, mode: LogMode, level: LogLevel) -> Self {
        Self{ level, mode, output: Arc::new(Mutex::new(Box::new(output))) }
    }

    pub fn with_file_path(path: &PathBuf, mode: LogMode, level: LogLevel) -> io::Result<Self> {
        let file = OpenOptions::new().append(true).create(true).open(path)?;
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

    fn log<S: Display>(&self, level: LogLevel, message: S, options: Options) {
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

    pub fn error<S: Display>(&self, msg: S) { self.log(LogLevel::Error, &msg, Options::default()); }
    pub fn warn<S: Display>(&self, msg: S) { self.log(LogLevel::Warning, &msg, Options::default()); }
    pub fn info<S: Display>(&self, msg: S) { self.log(LogLevel::Info, &msg, Options::default()); }
    pub fn debug<S: Display>(&self, msg: S) { self.log(LogLevel::Debug, &msg, Options::default()); }

    pub fn error_options<S: Display>(&self, msg: S, options: Options) { self.log(LogLevel::Error, &msg, options); }
    pub fn warn_options<S: Display>(&self, msg: S, options: Options) { self.log(LogLevel::Warning, &msg, options); }
    pub fn info_options<S: Display>(&self, msg: S, options: Options) { self.log(LogLevel::Info, &msg, options); }
    pub fn debug_options<S: Display>(&self, msg: S, options: Options) { self.log(LogLevel::Debug, &msg, options); }
}




























