extern crate hsoul;

use std::path::PathBuf;
use hsoul::read_sub_tree::read_sub_tree;

fn main() {
    let mut args = std::env::args();
    args.next();
    let first = match args.next() {
        Some(val) => val,
        None => {
            eprintln!("program should at least have 1 argument (first arg is output file_path and second is root_folder of soul project if empty root is current)"); 
            return;
        },
    };

    let possible_second = args.next();
    if args.next().is_some() {
        eprintln!("!!error!! program can only have 2 arguments (first arg is output file_path and second is root_folder of soul project if empty root is current)");
        return;
    }

    let tree = match read_sub_tree(possible_second) {
        Ok(val) => val,
        Err(err) => {
            eprintln!("while trying to read tree: {}", err.to_string()); 
            return;
        },
    };
    println!("{}", tree.to_tree_string());

    let mut out_path = PathBuf::from(first);
    out_path.push("soul_subfiles.tree.bin");
    match tree.save_to_bin_file(&out_path) {
        Ok(()) => (),
        Err(err) => {
            eprintln!("while trying to save subfile_tree to disk: {}", err.to_string()); 
            return;
        },
    }
}





















