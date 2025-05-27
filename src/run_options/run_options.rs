use std::collections::HashMap;
use std::result;
use std::env::Args;
use itertools::Itertools;
use once_cell::sync::Lazy;

use crate::bitflags;

bitflags! {
    pub struct ShowOutputs: u8 {
        const SHOW_NONE = 0x0;
        const SHOW_TOKENIZER = 0b0000_0001;
        const SHOW_ABSTRACT_SYNTAX_TREE = 0b0000_0010;
        const SHOW_CPP_CONVERTION = 0b0000_0100;
        const SHOW_ALL = 0b1111_1111;
    }
}

pub struct RunOptions {
    pub file_path: String,
    pub is_compiled: bool,
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
                options.is_garbage_collected = true;
                Ok(())
            }) as ArgFunc
        ),
    ])
});

impl RunOptions {
    pub fn new(_args: Args) -> result::Result<Self, String> {
        let options = Self{ file_path: String::new(), is_compiled: false, show_outputs: ShowOutputs::SHOW_NONE, is_garbage_collected: false};
        
        let args = _args.collect::<Vec<_>>();
        if args.len() == 1 {
            return Ok(options);
        }

        
        
        Ok(options)
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








