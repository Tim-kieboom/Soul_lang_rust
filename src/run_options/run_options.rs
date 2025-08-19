use std::io::Write;
use std::num::ParseIntError;
use std::result;
use std::env::Args;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use super::show_times::ShowTimes;
use super::show_output::ShowOutputs;

pub struct RunOptions {
    pub file_path: String, 
    pub is_file_path_raw_file_str: bool, 
    pub show_times: ShowTimes,
    pub show_outputs: ShowOutputs,
    pub output_dir: String,
    pub pretty_cpp_code: bool,
    pub tab_char_len: u32,
    pub command: String,
} 

type ArgFunc = Box<dyn Fn(&String, &mut RunOptions) -> Result<(), String> + Send + Sync + 'static>;

static OPTIONS: Lazy<HashMap<&'static str, ArgFunc>> = Lazy::new(|| {
    HashMap::from([
        (
            "-showOutput",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.show_outputs = ShowOutputs::from_str(input)?;
                Ok(())
            }) as ArgFunc
        ),
        (
            "-outputDir",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.output_dir = input.to_string();
                Ok(())
            }) as ArgFunc
        ),
        (
            "-tabCharLen",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.tab_char_len = input.parse()
                    .map_err(|err: ParseIntError| format!("input of argument '-tabCharLen' could not be parsed into u32 interger parserError:\n{}", err.to_string()))?;
                
                const MAX_TAB_LEN: u32 = 128;
                if options.tab_char_len > MAX_TAB_LEN {
                    return Err(format!("-tabCharLen can not be larger then {}", MAX_TAB_LEN));
                }
                Ok(())
            }) as ArgFunc
        ),
        (
            "-showTime",
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
    ])
});

const ALLOWED_COMMANDS: &[&str] = &["run", "build", "help"];

impl RunOptions {
    pub fn new(_args: Args) -> result::Result<Self, String> {
        let mut options = Self {
            file_path: String::new(),
            is_file_path_raw_file_str: false,
            show_outputs: ShowOutputs::SHOW_NONE,
            show_times: ShowTimes::SHOW_TOTAL,
            pretty_cpp_code: false,
            output_dir: "output".to_string(),
            tab_char_len: 4,
            command: String::new(),
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
        else if options.file_path.is_empty() {
            Err("Missing file path argument (type 'soul help' for more info).".to_string())
        } 
        else {
            Ok(options)
        }
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
        options.file_path = arg.clone();
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
        Run             info: Compile and Run selected file
        help            info: prints this list you are reading

    Options:
        -showOutput     info: select which steps in the compiler gets show to use in output folder (e.g. tokenizer, AST, ect..)
                        args: (Default)SHOW_NONE, SHOW_SOURCE, SHOW_TOKENIZER, SHOW_ABSTRACT_SYNTAX_TREE, SHOW_CPP_CONVERTION, SHOW_ALL 

        -prettyCppCode  info: make c++ output human readable
                        args: <none>

        -showTime       info: select which steps in the compiler gets timed and this time printed on screan
                        args: SHOW_NONE, (Default)SHOW_TOTAL, SHOW_SOURCE_READER, SHOW_TOKENIZER, SHOW_PARSER, SHOW_CODE_GENERATOR, SHOW_ALL 
        
        -tabCharLen     info: the amount of spaces in your ide for a tab this is if this amount is wrong your errors will display the wrong char
                        args: <any positive interger> 

        -outputDir      info: the path of the output folder
                        args: <any path>
";

    println!("{}", HELP_ARGS_LIST);

    // inshore this gets printed
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





