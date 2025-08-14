mod serde_sub_tree;
pub mod subfile_tree;
pub mod read_sub_tree;

pub fn read_save_sub_tree(project_root: Option<String>, output: String, verbose: bool) -> std::result::Result<(), String>{
    let tree = match read_sub_tree::read_sub_tree(project_root) {
        Ok(val) => val,
        Err(err) => {
            return Err(format!("while trying to read tree: {}", err.to_string()));
        },
    };
    
    if verbose {
        println!("{}", tree.to_tree_string());
    }
    
    let mut out_path = std::path::PathBuf::from(output);
    out_path.push("soul_subfiles.tree.bin");
    match tree.save_to_bin_file(&out_path) {
        Ok(()) => Ok(()),
        Err(err) => {
            return Err(format!("while trying to save subfile_tree to disk: {}", err.to_string()));
        },
    }
}













