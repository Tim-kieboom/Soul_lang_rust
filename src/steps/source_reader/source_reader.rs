use std::io::{BufRead, BufReader, Read};
use crate::errors::soul_error::Result;
use crate::{errors::soul_error::{new_soul_error, pass_soul_error, SoulErrorKind, SoulSpan}, soul_names::SOUL_NAMES, steps::{source_reader::{c_str::{format_stringer::format_string}, remove_comment::remove_comment::remove_comment}, step_interfaces::i_source_reader::{FileLine, SourceFileResponse}}};

/// Reads a source file into memory, applies preprocessing, and produces a [`SourceFileResponse`].
///
/// This function:
/// 1. Iterates through the file line by line.
/// 2. Replaces all tab characters (`\t`) with the configured number of spaces (`tab_as_spaces`).
/// 3. Removes both single-line and multi-line comments (tracking multi-line comment state across lines).
/// 4. Formats string literals using `format_string`.
/// 5. Estimates the number of tokens on each line (used later by the tokenizer).
/// 6. Skips lines that are empty or fully contained within a multi-line comment.
/// 7. Collects cleaned [`FileLine`] entries into a [`SourceFileResponse`].
///
/// # Parameters
/// - `reader`: A buffered reader over the source file.
/// - `tab_as_spaces`: A string of spaces used to replace tab characters, based on `RunOptions::tab_char_len`.
///
/// # Returns
/// - `Ok(SourceFileResponse)` containing all processed lines of the source file and an estimated token count.
/// - `Err(SoulError)` if any I/O or formatting error occurs.
///
/// # Errors
/// - Returns a [`SoulErrorKind::ReaderError`] if the file cannot be read or if string formatting fails.
///
/// # Notes
/// - Lines inside multi-line comments are skipped entirely.
/// - The `estimated_token_count` is a heuristic based on whitespace and known language tokens
///   from [`SOUL_NAMES`], intended to help preallocate memory in later compilation stages.
///
/// # Example
/// ```ignore
/// use std::fs::File;
/// use std::io::BufReader;
/// use crate::steps::source_reader::read_source_file;
///
/// let file = File::open("example.soul").unwrap();
/// let reader = BufReader::new(file);
/// let response = read_source_file(reader, "    ")?; // replace tabs with 4 spaces
/// println!("Read {} lines", response.source_file.len());
/// ```
pub fn read_source_file<R>(reader: BufReader<R>, tab_as_spaces: &str) -> Result<SourceFileResponse> 
where 
    R: Read
{
    let mut line_number = 1;
    let mut source_result = SourceFileResponse::new();
    let mut in_multi_line_comment = false;
    for possible_line in reader.lines() {
        let line = possible_line
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, Some(SoulSpan::new(line_number, 0, 0)), format!("while trying to get line from file reader\n{}", err.to_string())))?;
    
        let mut file_line = FileLine{line, line_number};

        file_line.line = file_line.line.replace("\t", &tab_as_spaces);

        file_line = remove_comment(file_line, &mut in_multi_line_comment, &mut source_result);
        file_line = format_string(file_line)
            .map_err(|err| pass_soul_error(SoulErrorKind::ReaderError, Some(SoulSpan::new(line_number, 0, 0)), "while trying to convert string_formaters", err))?;

        source_result.estimated_token_count += get_estimated_token_count(&file_line.line);
        line_number += 1;

        if file_line.line.is_empty() || in_multi_line_comment {
            continue;
        }

        source_result.source_file.push(file_line);
    }

    Ok(source_result)
}

fn get_estimated_token_count(str: &str) -> usize {
    let mut count = str.chars().filter(|c| c.is_whitespace()).count();
    for token in &SOUL_NAMES.parse_tokens {
        count += str.matches(token).count()
    }

    count
}















































