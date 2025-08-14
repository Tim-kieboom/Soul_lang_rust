extern crate hsoul;

use std::path::PathBuf;
use hsoul::read_sub_tree::read_sub_tree;

fn main() {
    let mut args = std::env::args();
    args.next();
    let first = match args.next() {
        Some(val) => val,
        None => {
            eprintln!("program should at least have 1 argument (first arg is output file_path and second is root_folder of soul project if empty root is current and optional '-v' to enable verbose)"); 
            return;
        },
    };

    let mut verbose = false;
    let possible_second = args.next();
    if let Some(arg) = args.next() {
        if arg == "-v" {
            verbose = true;
        }
        else {
            eprintln!("!!error!! 3de argument '{}' is invalid", arg);
            return;
        }

        if args.next().is_some() {
            eprintln!("!!error!! program can only have 3 arguments (first arg is output file_path and second is root_folder of soul project if empty root is current and optional '-v' to enable verbose)");
            return;
        }
    }

    if let Err(msg) = read_save_sub_tree(first, possible_second, verbose) {
        eprintln("{}", msg);
    }
}





















