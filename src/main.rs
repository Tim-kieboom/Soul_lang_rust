use std::{env::args, time::Instant};

use soul_lang_rust::{run_compiler::run_compiler, run_interpreter::run_interpreter, run_options::run_options::RunOptions};

extern crate soul_lang_rust;

fn main() {

    let run_option = match RunOptions::new(args()) {
        Ok(val) => val,
        Err(err) => {eprintln!("{err}"); return;},
    };
    
    let is_compiled = run_option.is_compiled;
    
    let start = Instant::now();
    let result = if is_compiled {
        run_compiler(run_option)
    } else {
        run_interpreter(run_option)
    };
    
    let duration = start.elapsed();

    if is_compiled {
        println!("Elapsed time: {:.2?}", duration);
    }
    
    if let Err(err) = result {
        println!("{}", err.to_string());
    }
}






