use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan};
use crate::steps::source_reader::c_str::format_stringer::indexesof_qoutes;
use crate::steps::step_interfaces::i_source_reader::{FileLine, SourceFileResponse};

pub fn c_str_to_lit_str(mut file_line: FileLine, source_result: &mut SourceFileResponse) -> Result<FileLine> {
    let indexes = indexesof_qoutes(&file_line);
    
    if indexes.len() % 2 != 0 {
        return Err(new_soul_error(SoulErrorKind::UnterminatedStringLiteral, SoulSpan::new(file_line.line_number, 0), "opening qoute (so '\"') without a closing qoute every str NEEDS TO BE CLOSED ON THE SAME LINE"))
    }

    for i in (0..indexes.len()).rev().step_by(2) {
        let begin = indexes[i-1];
        let end = indexes[i];

        let c_str = &file_line.line[begin..=end];

        if !source_result.c_str_store.contains_key(c_str) {
            let c_str = c_str.to_string();
            let str_name = format!("__cstr_{}__", source_result.c_str_store.len());
            source_result.c_str_store.insert(c_str, str_name);
        }

        let c_pair = source_result.c_str_store.get(c_str).unwrap();
        file_line.line.replace_range(begin..=end, c_pair.as_str());
    }

    Ok(file_line)
}




















