use std::io::{BufRead, BufReader, Read};
use crate::errors::soul_error::Result;
use crate::{errors::soul_error::{new_soul_error, pass_soul_error, SoulErrorKind, SoulSpan}, soul_names::SOUL_NAMES, steps::{source_reader::{c_str::{format_stringer::format_string}, remove_comment::remove_comment::remove_comment}, step_interfaces::i_source_reader::{FileLine, SourceFileResponse}}};

pub fn read_source_file<R>(reader: BufReader<R>, tab_as_spaces: &str) -> Result<SourceFileResponse> 
where 
    R: Read
{
    let mut line_number = 1;
    let mut source_result = SourceFileResponse::new();
    let mut in_multi_line_comment = false;
    for possible_line in reader.lines() {
        let line = possible_line
            .map_err(|err| new_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(line_number, 0, 0), format!("while trying to get line from file reader\n{}", err.to_string())))?;
    
        let mut file_line = FileLine{line, line_number};

        file_line.line = file_line.line.replace("\t", &tab_as_spaces);

        file_line = remove_comment(file_line, &mut in_multi_line_comment, &mut source_result);
        file_line = format_string(file_line)
            .map_err(|err| pass_soul_error(SoulErrorKind::ReaderError, SoulSpan::new(line_number, 0, 0), "while trying to convert string_formaters", err))?;

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















































