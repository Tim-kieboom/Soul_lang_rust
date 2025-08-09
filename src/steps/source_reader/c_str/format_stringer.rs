use std::slice::Iter;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_source_reader::FileLine;


const QUOTE: &str = "\"";
const IN_STR_QUOTE: &str = "\\\"";

struct FormatSpan {
    pub start: usize,
    pub end: usize,
}

pub fn format_string(file_line: FileLine) -> Result<FileLine> {
    let qoutes = indexesof_qoutes(&file_line);
    let mut raw_format_strs = Vec::with_capacity(qoutes.len()/2);

    let mut qoute_iter = qoutes.iter();
    let mut chars = file_line.line.chars();

    while let Some(index) = qoute_iter.next() {
        if *index == 0 {
            continue;
        }

        if chars.nth(index - 1) != Some('f') {
            continue;
        }

        let end = get_end_of_string(&file_line, *index, &mut qoute_iter)?;
        raw_format_strs.push(FormatSpan{start: *index, end});
    }

    let mut new_file_line = file_line.clone();
    for span in raw_format_strs {
        new_file_line.line = format_str_to_soul_formatter(&file_line, &span)?;
    }

    Ok(new_file_line)
}

fn format_str_to_soul_formatter(file_line: &FileLine, span: &FormatSpan) -> Result<String> {
    const SOUL_FORMATTER_FUNCTION_NAME: &str = "std::fmt::FormatArgs(";

    let mut buffer = String::with_capacity(file_line.line.len() + SOUL_FORMATTER_FUNCTION_NAME.len());

	// Append everything before the 'f' character in 'f"..."'
    buffer.insert_str(0, &file_line.line[..span.start-1]);
    buffer.insert_str(buffer.len(), SOUL_FORMATTER_FUNCTION_NAME);

    
    let mut i = span.start;
    let mut pretty_format = false;
    let mut start_bracket = 0;
    let mut current_open_bracket = false;
    for ch in file_line.line[span.start..span.end+1].chars() {
        match ch {
            '{' => {
                buffer.push_str("\",");
                buffer.push_str("std::fmt::Arg(");
                start_bracket = i;
                current_open_bracket = true;
            },
            '}' => {
                if pretty_format {
                    buffer.push(',');
                    buffer.push_str("pretty=true");
                }

                buffer.push(')');
                pretty_format = false;

                buffer.push_str(",\"");
                validate_format_argument(file_line, start_bracket, i)?;
                current_open_bracket = false;
            },
            '#' => {
                if current_open_bracket {
                    if i-1 != start_bracket {
                        return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, SoulSpan::new(file_line.line_number, i, 1), "'#' is only allowed in stringformat at as the first char in arg (so 'f\"foo {#1}\"' is oke but 'f\"foo {1#} or { #1} or {##1}\"' not)"))
                    }
                    pretty_format = true;
                }
                else {
                    buffer.push(ch);
                }
            }
            _ => buffer.push(ch),
        }



        i += 1;
    }
    buffer.push_str(")");
    buffer.push_str(&file_line.line[span.end+1..]);

    if current_open_bracket {
        return Err(new_soul_error(
            SoulErrorKind::InvalidStringFormat,
            SoulSpan::new(file_line.line_number.max(0), i, buffer.len()), 
            "string formatter opens a bracket with out closing it (add '}' somewhere in f\"...\")"
        ))
    } 

    Ok(buffer)
}

fn validate_format_argument(file_line: &FileLine, start_bracket: usize, i: usize) -> Result<()> {
    let format_arg = &file_line.line[start_bracket+1..i].trim();
    match format_arg.chars().nth(0) {
        Some(char) => {if char == '"' {return Err(err_string_in_format_arg(file_line, start_bracket))}},
        None => return Err(err_emty_format_arg(file_line, start_bracket)),
    }

    Ok(())
}

fn err_emty_format_arg(line: &FileLine, offset: usize) -> SoulError {
    let span = SoulSpan::new(line.line_number, offset, line.line.len());
    new_soul_error(
        SoulErrorKind::InvalidStringFormat,
        span, 
        "in format_string (so 'f\"...\"') format argument is empty"
    )
}


fn err_string_in_format_arg(line: &FileLine, offset: usize) -> SoulError {
    let span = SoulSpan::new(line.line_number, offset, line.line.len());
    new_soul_error(
        SoulErrorKind::InvalidStringFormat,
        span, 
        "in format_string (so 'f\"...\"') can not contain a string literal as an argument"
    )
}

fn get_end_of_string(line: &FileLine, index: usize, qoute_iter: &mut Iter<usize>) -> Result<usize> {
    if let Some(_index) = qoute_iter.next() {
        Ok(*_index)
    }
    else {
        Err(new_soul_error( 
            SoulErrorKind::UnterminatedStringLiteral,
            SoulSpan::new(line.line_number, index, line.line.len()), 
            format!("string has no end (probably missing a '{}')", QUOTE)
        ))
    }
}

pub fn indexesof_qoutes<'a>(file_line: &'a FileLine) -> Vec<usize> {
    file_line.line.replace(IN_STR_QUOTE, "##")
        .match_indices(QUOTE)
        .map(|(start, _)| start)
        .collect()
}










