use std::fs;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};

use crate::subfile_tree::{SubFileTree, SubFileTreeBuilder};

pub fn read_sub_tree(possible_root: Option<String>) -> io::Result<SubFileTree> {
    let root_path = match possible_root {
        Some(p) => PathBuf::from(p),
        None => std::env::current_dir()?,
    };

    let root_name = root_path
        .file_name()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_else(|| "".to_string());

    let mut builder = SubFileTreeBuilder::new(root_name);

    fn recurse(builder: &mut SubFileTreeBuilder, path: &Path) -> io::Result<()> {
        let book_file_path = path.join("book.hsoul");
        if !book_file_path.exists() {
            return Ok(());
        }

        let file = fs::File::open(&book_file_path)?;
        let reader = io::BufReader::new(file);

        for (line_num, line_result) in reader.lines().enumerate() {
            let line = line_result?;
            let line = line.trim();

            if line.is_empty() || line.starts_with("//") {
                continue;
            }

            let (kind, name) = match line.split_once(' ') {
                Some((k, n)) => (k.trim(), n.trim()),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Syntax error in '{}', line {}: expected '<kind> <name>'",
                            book_file_path.display(),
                            line_num + 1
                        ),
                    ));
                }
            };

            if name.is_empty() {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Syntax error in '{}', line {}: missing name after '{}'",
                        book_file_path.display(),
                        line_num + 1,
                        kind
                    ),
                ));
            }

            let is_public = match kind {
                "book" | "page" => false,
                "Book" | "Page" => true,
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        format!(
                            "Syntax error in '{}', line {}: unknown keyword '{}'",
                            book_file_path.display(),
                            line_num + 1,
                            kind
                        ),
                    ));
                }
            };

            match kind {
                "book" | "Book" => {
                    let subfolder_path = path.join(name);
                    builder.push_folder(name, is_public);
                    recurse(builder, &subfolder_path)?;
                    builder.pop_folder();
                }
                "page" | "Page" => {
                    builder.add_subfile(name, is_public);
                }
                _ => unreachable!(),
            }
        }

        Ok(())
    }

    recurse(&mut builder, &root_path)?;
    Ok(builder.consume_to_sub_file_tree())
}