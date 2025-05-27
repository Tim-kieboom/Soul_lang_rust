use std::{io::{Error, Result}, slice::Iter};
use crate::{meta_data::convert_soul_error::convert_soul_error::new_soul_error, tokenizer::{file_line::FileLine, token::Token}};

const QUOTE: &str = "\"";
const IN_STR_QUOTE: &str = "\\\"";

struct FormatSpan {
    pub line_index: usize,
    pub start: usize,
    pub end: usize,
}

#[allow(dead_code)]
pub fn format_str_file<I>(_source_file: I) -> Result<Vec<FileLine>> 
where
    I: Iterator<Item = FileLine>
{
    let mut source_file = _source_file.collect::<Vec<_>>();
    let num_qouts = numberof_qoutes(&source_file);
    let mut raw_format_strs = Vec::with_capacity(num_qouts/2);
    
    let qoute_iter = indexesof_qoutes(&source_file);
    
    for (i, (qoute_indexes, line)) in qoute_iter.enumerate() {
        raw_format_strs = get_format_str(&line, i, &qoute_indexes, raw_format_strs)?;
    }

    for span in &raw_format_strs {
        let line = &mut source_file[span.line_index];
        line.text = f_str_to_soul_formatter(&line, &span)?; 
    }

    Ok(source_file)
}

#[allow(dead_code)]
pub fn format_str_line(line: FileLine, line_index: usize) -> Result<FileLine> {
    let num_qouts = numberof_qoutes_line(&line);
    let mut raw_format_strs = Vec::with_capacity(num_qouts/2);

    let qoute_iter = indexesof_qoutes_line(&line);
    raw_format_strs = get_format_str(&line, line_index, &qoute_iter, raw_format_strs)?;

    let mut new_line = line.clone();
    for span in &raw_format_strs {
        new_line.text = f_str_to_soul_formatter(&line, &span)?; 
    }

    Ok(new_line)
}

fn f_str_to_soul_formatter(line: &FileLine, span: &FormatSpan) -> Result<String> {
    const SOUL_FORMATTER_FUNCTION_NAME: &str = "__soul_format_string__(";

    let mut buffer = String::with_capacity(line.text.len() + SOUL_FORMATTER_FUNCTION_NAME.len());

	// Append everything before the 'f' character in 'f"..."'
    buffer.insert_str(0, &line.text[..span.start-1]);
    buffer.insert_str(buffer.len(), SOUL_FORMATTER_FUNCTION_NAME);

    
    let mut i = span.start;
    let mut start_bracket = 0;
    let mut current_open_bracket = false;
    for ch in line.text[span.start..span.end+1].chars() {
        match ch {
            '{' => {
                buffer.push_str("\", ");
                start_bracket = i;
                current_open_bracket = true;
            },
            '}' => {
                buffer.push_str(", \"");
                validate_format_argument(line, start_bracket, i)?;
                current_open_bracket = false;
            },
            _ => buffer.push(ch),
        }

        i += 1;
    }
    buffer.push(')');
    buffer.push_str(&line.text[span.end+1..]);

    if current_open_bracket {
        return Err(new_soul_error(&Token{text: buffer, line_number: line.line_number.max(0) as usize, line_offset: i}, "string formatter opens a bracket with out cloding it (add '}' somewhere in f\"...\")"))
    } 

    Ok(buffer)
} 

fn get_format_str(line: &FileLine, line_index: usize, _qoute_indexes: &Vec<usize>, mut raw_format_strs: Vec<FormatSpan>) -> Result<Vec<FormatSpan>> {
    let mut qoute_indexes = _qoute_indexes.iter();
    let mut chars = line.text.chars();

    while let Some(index) = qoute_indexes.next() {
        if *index == 0 {
            continue;
        }

        if chars.nth(index - 1) != Some('f') {
            continue;
        }

        let end = get_end_of_string(&line, *index, &mut qoute_indexes)?;
        raw_format_strs.push(FormatSpan{line_index, start: *index, end});
    }

    Ok(raw_format_strs)
}

fn get_end_of_string(line: &FileLine, index: usize, qoute_indexes: &mut Iter<usize>) -> Result<usize> {
    if let Some(_index) = qoute_indexes.next() {
        Ok(*_index)
    }
    else {
        Err(new_soul_error( 
            &Token{text: String::new(), line_number: line.line_number as usize, line_offset: index}, 
            format!("string has no end (probably missing a '{}')", QUOTE).as_str()
        ))
    }
}

fn validate_format_argument(line: &FileLine, start_bracket: usize, i: usize) -> Result<()> {
    let format_arg = &line.text[start_bracket+1..i].trim();
    match format_arg.chars().nth(0) {
        Some(char) => {if char == '"' {return Err(err_string_in_format_arg(line, start_bracket))}},
        None => return Err(err_emty_format_arg(line, start_bracket)),
    }

    Ok(())
}

fn err_emty_format_arg(line: &FileLine, offset: usize) -> Error {
    let err_token = Token{text: String::new(), line_number: line.line_number as usize, line_offset: offset};
    new_soul_error(&err_token, "in format_string (so 'f\"...\"') format argument is empty")
}


fn err_string_in_format_arg(line: &FileLine, offset: usize) -> Error {
    let err_token = Token{text: String::new(), line_number: line.line_number as usize, line_offset: offset};
    new_soul_error(&err_token, "in format_string (so 'f\"...\"') can not contain a string literal as an argument")
}

pub fn indexesof_qoutes_line<'a>(line: &'a FileLine) -> Vec<usize> {
    line.text.replace(IN_STR_QUOTE, "##")
             .match_indices(QUOTE)
             .map(|(start, _)| start)
             .collect()
}

pub fn indexesof_qoutes(source_file: &Vec<FileLine>) -> impl Iterator<Item = (Vec<usize>, FileLine)>  {
    source_file.
        iter()
        .map(|line| {
            let new_text = line.text.replace(IN_STR_QUOTE, "##");
            let new_line = FileLine {
                text: new_text.clone(),
                line_number: line.line_number,
            };
            let indices = new_text.match_indices(QUOTE).map(|(start, _)| start).collect::<Vec<usize>>();
            (indices, new_line)
        })
}

pub fn numberof_qoutes_line(line: &FileLine) -> usize {
    let num_all_qoutes: usize = line.text.matches(QUOTE).count();
    let num_in_str_qoutes: usize = line.text.matches(IN_STR_QUOTE).count();

    num_all_qoutes - num_in_str_qoutes
}

pub fn numberof_qoutes(source_file: &Vec<FileLine>) -> usize 
{
    source_file
        .iter()
        .map(|line| {
            let all_quotes = line.text.matches(QUOTE).count();
            let in_str_quotes = line.text.matches(IN_STR_QUOTE).count();
            all_quotes - in_str_quotes
        })
        .sum()
}