use std::collections::HashMap;
use std::result;
use std::env::Args;
use once_cell::sync::Lazy;

use super::show_output::ShowOutputs;
use super::show_times::ShowTimes;

pub struct RunOptions {
    pub file_path: String,
    pub is_compiled: bool,
    pub show_times: ShowTimes,
    pub show_outputs: ShowOutputs,
    pub is_garbage_collected: bool,
} 

type ArgFunc = Box<dyn Fn(&String, &mut RunOptions) -> Result<(), String> + Send + Sync + 'static>;

static OPTIONS: Lazy<HashMap<&'static str, ArgFunc>> = Lazy::new(|| {
    HashMap::from([
        (
            "-isCompiled",
            Box::new(|arg: &String, options: &mut RunOptions| {
                should_not_have_input(arg)?;
                options.is_compiled = true;
                Ok(())
            }) as ArgFunc
        ),
        (
            "-isInterpreted",
            Box::new(|arg: &String, options: &mut RunOptions| {
                should_not_have_input(arg)?;
                options.is_compiled = false;
                Ok(())
            }) as ArgFunc
        ),
        (
            "-isGarbageCollected",
            Box::new(|arg: &String, options: &mut RunOptions| {
                should_not_have_input(arg)?;
                options.is_garbage_collected = true;
                Ok(())
            }) as ArgFunc
        ),
        (
            "-showOutput",
            Box::new(|arg: &String, options: &mut RunOptions| {
                let input = get_input(arg)?;
                options.show_outputs = ShowOutputs::from_str(input)?;
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
    ])
});

impl RunOptions {
    pub fn new(_args: Args) -> result::Result<Self, String> {
        let mut options = Self {
                file_path: String::new(),
                is_compiled: true,
                show_outputs: ShowOutputs::SHOW_NONE,
                show_times: ShowTimes::SHOW_TOTAL,
                is_garbage_collected: false,
            };

            let args = _args.collect::<Vec<_>>();
            if args.len() < 2 {
                return Err("Missing command (e.g., 'run' or 'build').".to_string());
            }

            let run_command = &args[1];
            let allowed_commands = ["run", "build"]; // Add more as needed
            if !allowed_commands.contains(&run_command.as_str()) {
                return Err(format!("Unknown command: '{}'. Allowed commands: {:?}", run_command, allowed_commands));
            }

            let mut errors = Vec::new();
            let mut file_path_set = false;

            for arg in &args[2..] {
                if arg.starts_with('-') {
                    let key = if let Some(idx) = arg.find('=') {
                        &arg[..idx]
                    } 
                    else {
                        arg.as_str()
                    };

                    if let Some(func) = OPTIONS.get(key) {
                        if let Err(e) = func(arg, &mut options) {
                            errors.push(e);
                        }
                    } 
                    else {
                        errors.push(format!("Unknown option: '{}'", key));
                    }
                } 
                else if !file_path_set {
                    options.file_path = arg.clone();
                    file_path_set = true;
                } 
                else {
                    errors.push(format!("Unexpected positional argument: '{}'", arg));
                }
            }

            if !errors.is_empty() {
                Err(errors.join("\n"))
            } 
            else if options.file_path.is_empty() {
                Err("Missing file path argument.".to_string())
            } 
            else {
                Ok(options)
            }
    }
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








