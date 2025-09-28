use std::io::Write;
use std::num::ParseIntError;
use std::path::{Path, PathBuf};
use std::result;
use std::env::Args;
use std::str::ParseBoolError;
use hsoul::subfile_tree::SubFileTree;
use once_cell::sync::Lazy;
use std::collections::HashMap;

use crate::errors::soul_error::{new_soul_error, SoulErrorKind, Result};
use crate::utils::logger::{LogLevel, LogMode};

use super::show_times::ShowTimes;
use super::show_output::ShowOutputs;

pub struct RunOptions {
    pub file_path: PathBuf, 
    pub project_name: String,
    pub is_file_path_raw_file_str: bool, 
    pub show_times: ShowTimes,
    pub show_outputs: ShowOutputs,
    pub output_dir: PathBuf,
    pub pretty_cpp_code: bool,
    pub tab_char_len: u32,
    pub command: String,
    pub sub_tree_path: PathBuf,

    pub log_path: Option<PathBuf>,
    pub log_level: LogLevel,
    pub log_mode: LogMode,
    pub log_colored: bool,

    pub max_thread_count: Option<usize>,
} 

type ArgFunc = Box<dyn Fn(&String, &mut RunOptions) -> std::result::Result<(), String> + Send + Sync + 'static>;

static OPTIONS: Lazy<HashMap<&'static str, ArgFunc>> = Lazy::new(|| {
    HashMap::from([
        (
            "--showOutput",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.show_outputs = ShowOutputs::from_str(input)?;
                Ok(())
            }) as ArgFunc
        ),
        (
            "--outputDir",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.output_dir = PathBuf::from(input);
                Ok(())
            }) as ArgFunc
        ),
        (
            "--tabCharLen",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.tab_char_len = input.parse()
                    .map_err(|err: ParseIntError| format!("input of argument '--tabCharLen' could not be parsed into u32 interger parserError:\n{}", err.to_string()))?;
                
                const MAX_TAB_LEN: u32 = 128;
                if options.tab_char_len > MAX_TAB_LEN {
                    return Err(format!("--tabCharLen can not be larger then {}", MAX_TAB_LEN));
                }
                Ok(())
            }) as ArgFunc
        ),
        (
            "--showTime",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.show_times = ShowTimes::from_str(input)?;
                Ok(())
            }) as ArgFunc
        ),
        (
            "-prettyCppCode",
            Box::new(|arg: &String, options: &mut RunOptions| {
                should_not_have_input(arg)?;
                options.pretty_cpp_code = true;
                Ok(())
            }) as ArgFunc
        ),
        (
            "--subtreePath",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.sub_tree_path = input.into();
                Ok(())
            }) as ArgFunc
        ),
        (
            "--logPath",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.log_path = Some(input.into());
                Ok(())
            }) as ArgFunc
        ),
        (
            "--logLevel",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.log_level = LogLevel::from_str(input);
                Ok(())
            }) as ArgFunc
        ),
        (
            "--logMode",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.log_mode = LogMode::from_str(input)?;
                Ok(())
            }) as ArgFunc
        ),
        (
            "--logColored",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.log_colored = input.parse()
                    .map_err(|err: ParseBoolError| err.to_string())?;
                Ok(())
            }) as ArgFunc
        ),
        (
            "--projectName",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.project_name = input.to_string();
                Ok(())
            }) as ArgFunc
        ),
        (
            "--maxThreads",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.max_thread_count = Some(input.parse()
                    .map_err(|err: ParseIntError| format!("input of argument '--maxThreads' could not be parsed into usize interger parserError:\n{}", err.to_string()))?);
                Ok(())
            }) as ArgFunc
        ),
    ])
});

const ALLOWED_COMMANDS: &[&str] = &["build", "help"];

impl RunOptions {
    pub fn new(_args: Args) -> result::Result<Self, String> {
        let mut options = Self {
            file_path: PathBuf::new(),
            is_file_path_raw_file_str: false,
            show_outputs: ShowOutputs::SHOW_NONE,
            show_times: ShowTimes::SHOW_TOTAL,
            pretty_cpp_code: false,
            output_dir: PathBuf::from("output"),
            tab_char_len: 4,
            command: "".into(),
            sub_tree_path: "".into(),
            log_level: LogLevel::Any,
            log_mode: LogMode::ShowAll,
            log_path: None,
            log_colored: true,

            project_name: std::env::current_dir()
                .expect("can not get current dir")
                .file_name()
                .expect("could not get name of dir")
                .to_str()
                .expect("current dir name is not UTF8 valid")
                .to_string(),
            max_thread_count: None,
        };

        let mut args = _args.collect::<Vec<_>>();
        if args.len() == 1 {
            return Ok(options);
        }
        else if args.len() < 2 {
            return Err(format!("Missing command (commands: {:?}).", ALLOWED_COMMANDS));
        }

        let run_command = &args[1];
        if !ALLOWED_COMMANDS.contains(&run_command.as_str()) {
            return Err(format!("Unknown command: '{}'. Allowed commands: {:?}", run_command, ALLOWED_COMMANDS));
        }
        
        if run_command == "help" {
            print_help_list();

            std::process::exit(0);
        }

        options.command = std::mem::take(&mut args[1]);

        let mut errors = Vec::new();
        let mut file_path_set = false;

        for arg in &args[2..] {
            pocess_arg(arg, &mut options, &mut file_path_set, &mut errors);
        }

        if !errors.is_empty() {
            Err(errors.join("\n"))
        } 
        else if options.file_path.as_os_str().is_empty() {
            Err("Missing file path argument (type 'soul help' for more info).".to_string())
        } 
        else {
            Ok(options)
        }
    }

    fn has_sub_tree(&self) -> bool {
        !self.sub_tree_path.as_os_str().is_empty()
    }

    pub fn get_file_paths(&self) -> Result<Vec<PathBuf>> {

        let main_file_path = self.file_path.clone();

        let mut source_files = vec![main_file_path];
        if self.has_sub_tree() {
            
            let subfiles_tree = self.get_sub_files()?;

            source_files.extend(
                subfiles_tree.get_all_file_paths()
                    .into_iter()
                    .map(|mut path| {path.set_extension("soul"); path})
            );

        }

        Ok(source_files)
    } 

    fn get_sub_files(&self) -> Result<SubFileTree> {
        let sub_tree = SubFileTree::from_bin_file(Path::new(&self.sub_tree_path))
            .map_err(|msg| new_soul_error(SoulErrorKind::InternalError, None, format!("!!internal error!! while trying to get subfilesTree\n{}", msg.to_string())))?;
        
        Ok(sub_tree)
    }
}

fn pocess_arg(arg: &String, options: &mut RunOptions, file_path_set: &mut bool, errors: &mut Vec<String>) {
    if arg.starts_with('-') {
        let key = if let Some(idx) = arg.find('=') {
            &arg[..idx]
        } 
        else {
            arg.as_str()
        };

        if let Some(func) = OPTIONS.get(key) {
            if let Err(e) = func(arg, options) {
                errors.push(e);
            }
        } 
        else {
            errors.push(format!("Unknown option: '{}'", key));
        }
    } 
    else if !*file_path_set {
        options.file_path = PathBuf::from(arg);
        *file_path_set = true;
    } 
    else {
        errors.push(format!("Unexpected positional argument: '{}'", arg));
    }
}

fn print_help_list() {
    const HELP_ARGS_LIST: &str = 
"
Tim Kieboom Presentsssss,
The Soul Compiler,
have fun :).

    Usage:
        soul [Commands] [FilePathOfMain] [Options]

    Commands:
        build           info: Compile the selected file
        help            info: prints this list you are reading
    
    Options:
        to call flag you do '-flag'
        to call arg you do '--option=arg1'
        to chain args together you do '--option=arg1+arg2'

        --showOutput    info: select which steps in the compiler gets show to use in output folder (e.g. tokenizer, AST, ect..)
                        args(chainable): (Default)SHOW_NONE, SHOW_SOURCE, SHOW_TOKENIZER, SHOW_ABSTRACT_SYNTAX_TREE, SHOW_CPP_CONVERTION, SHOW_ALL 

        -prettyCppCode  info: make c++ output human readable (no arguments its just a flag)

        --showTime      info: select which steps in the compiler gets timed and this time printed on screan
                        args(chainable): SHOW_NONE, (Default)SHOW_TOTAL, SHOW_SOURCE_READER, SHOW_TOKENIZER, SHOW_PARSER, SHOW_CODE_GENERATOR, SHOW_ALL 
        
        --tabCharLen    info: the amount of spaces in your ide for a tab this is if this amount is wrong your errors will highlight the wrong char
                        args: (Default)4, <any positive interger> 

        --outputDir     info: the path of the output folder
                        args: (Default)<empty>, <any path>
        
        --subtreePath   info: .bin file describing the subfile structure of the project if empty no subfiles
                        args: (Default)<empty>, <any path>

        --logPath       info: if not empty logs to file of given filePath instead of terminal
                        args: (Default)<empty>, <any path>
        
        --logLevel      info: the lowest level that will be show
                        args: (Default)ANY, ERROR, WARNING, INFO, DEBUG  

        --logMode       info: what info will be show when a message in printed 
                        args(chainable): (Default)SHOW_ALL, SHOW_DATE, SHOW_LEVEL  
        
        --logColored    info: if `true` will terminal print colored text else will print default color
                        args: (Default)true, false
        
        --maxThreads    info: the max amout of threads allowed to be used in compiler
                        args: (Default)<max amount available>, <any positive interger>
";

    println!("{}", HELP_ARGS_LIST);

    // insure this gets printed
    std::io::stdout().flush().expect("could not flush");
}

fn get_input<'a>(arg: &'a String) -> result::Result<&'a str, String> {
    if !arg.contains("=") {
        Err(format!("arg: '{}' should have an input (add '=<input>')", arg))
    } 
    else if arg.matches("=").count() != 1 {
        Err(format!("arg: '{}' should only have 1 '='", arg))
    }
    else {
        Ok(arg.split("=").nth(1).unwrap())
    }
}

fn should_not_have_input(arg: &String) -> result::Result<(), String> {
    if arg.contains("=") {
        Err(format!("arg: '{}' does not have an input (remove '=<input>')", arg))
    }
    else {
        Ok(())
    }
}





