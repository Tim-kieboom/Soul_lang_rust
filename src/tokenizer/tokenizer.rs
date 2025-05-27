use super::token::Token;
use std::io::{BufRead, Result};
use super::file_line::FileLine;
use crate::meta_data::meta_data::MetaData;
use crate::meta_data::soul_names::{SoulNames, SOUL_NAMES};
use crate::meta_data::soul_type::primitive_types::PrimitiveType;
use super::comment_remover::comment_remover::remove_comment_file;
use crate::tokenizer::comment_remover::comment_remover::remove_comment_line;
use crate::meta_data::convert_soul_error::convert_soul_error::new_soul_error;
use crate::tokenizer::string_tokenizer::format_stringer::{format_str_file, format_str_line};
use crate::meta_data::soul_type::type_checker::type_checker::get_primitive_type_from_literal;
use crate::tokenizer::string_tokenizer::string_mapper::{rawstr_to_litstr_file, rawstr_to_litstr_line};
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref SPLIT_REGEX: Regex = SoulNames::str_vec_to_regex(&SOUL_NAMES.parse_tokens);
}

pub struct FileLineResult {
    pub source_file: Vec<FileLine>, 
    pub estimated_token_count: u64
}

pub fn read_as_file_lines(path: &str) -> Result<FileLineResult> {
    
    use std::fs::File;
    use std::io::BufReader;
    let parse_tokens = &SOUL_NAMES.parse_tokens;

    let file = File::open(path)?;
    let reader = BufReader::new(file);
    
    let mut line_number = 1;
    let mut result = FileLineResult{source_file: Vec::new(), estimated_token_count: 0};
    for res in reader.lines() {
        let line = res?;
        result.estimated_token_count += get_token_count(line.as_str(), parse_tokens);
        result.source_file.push(FileLine{text: line, line_number});
        line_number += 1;
    }

    Ok(result)
}

pub fn tokenize_file(source_file: Vec<FileLine>, estimated_token_count: u64, meta_data: &mut MetaData) -> Result<Vec<Token>> {
    if source_file.is_empty() {
        return Ok(Vec::new());
    }

    assert!(estimated_token_count < usize::MAX as u64);
    let mut tokens = Vec::with_capacity(estimated_token_count as usize);

    let comment_less_file = remove_comment_file(source_file);
    let formated_str_file = format_str_file(comment_less_file)?; 
    let new_source_file = rawstr_to_litstr_file(formated_str_file, meta_data)?;

    for line in new_source_file {
        get_tokens(line, &mut tokens)?;
    }

    Ok(tokens)
}

#[allow(dead_code)]
pub fn tokenize_line(line: FileLine, line_index: usize, in_multi_line_commend: &mut bool, meta_data: &mut MetaData) -> Result<Vec<Token>> {
    if line.text.is_empty() {
        return Ok(Vec::new());
    }
    
    let num_spaces = line.text.matches(" ").count();
    let mut tokens = Vec::with_capacity(num_spaces+1);

    let comment_less_line = remove_comment_line(line, in_multi_line_commend);
    let formated_str_line = format_str_line(comment_less_line, line_index)?; 
    let new_line = rawstr_to_litstr_line(formated_str_line, meta_data)?;

    get_tokens(new_line, &mut tokens)?;

    Ok(tokens)
}

fn get_token_count(str: &str, strings: &Vec<&str>) -> u64 {
    // Estimate based on string length and average token size
    let whitespace_count = str.chars().filter(|c| c.is_whitespace()).count();
    let non_whitespace = str.len() - whitespace_count;
    // Assume average token length of 4 characters plus some overhead for special tokens
    (non_whitespace / 4 + whitespace_count / 2 + strings.len() / 10) as u64
}

fn get_tokens(line: FileLine, tokens: &mut Vec<Token>) -> Result<()> {
    let splits = line.text.split_on(&SOUL_NAMES.parse_tokens);
    let mut line_offset = 0;
    let mut last_is_forward_slash = false;

    for (i, text) in splits.iter().enumerate() {
        if text.is_empty() || *text == " " || *text == "\t" {
            line_offset += text.len();
            continue;
        }

        if !needs_to_dot_tokenize(text) {
            if *text == "\\" {
                if i != splits.len() -1 {
                    return Err(new_soul_error(&Token{
                        text: text.to_string(), 
                        line_number: line.line_number as usize, 
                        line_offset,
                    }, "'\\' can only be placed at the end of a line"));
                }

                last_is_forward_slash = true;
                break;
            }

            tokens.push(Token{
                text: (*text).to_string(), 
                line_number: line.line_number as usize, 
                line_offset,
            });
            line_offset += text.len();
            continue;
        }

        let dot_splits: Vec<&str> = text.split('.').collect();
        let num_splits = dot_splits.len();
        for (j, split) in dot_splits.iter().enumerate() {
            if split.is_empty() || *split == " " {
                continue;
            }

            tokens.push(Token{
                text: (*split).to_string(), 
                line_number: line.line_number as usize, 
                line_offset,
            });
            line_offset += split.len();

            if j != num_splits - 1 {
                tokens.push(Token {
                    text: ".".to_string(),
                    line_number: line.line_number as usize,
                    line_offset,
                });
                line_offset += 1;
            }
        }
    } 

    if !tokens.is_empty() && !last_is_forward_slash {
        tokens.push(Token{
            text: "\n".to_string(), 
            line_number: line.line_number as usize, 
            line_offset,
        });
    }

    Ok(())
}

pub trait SplitOn {fn split_on(&self, delims: &Vec<&str>) -> Vec<&str>;}
impl SplitOn for &str {
    fn split_on(&self, _delims: &Vec<&str>) -> Vec<&str> {
        let mut result = Vec::with_capacity(self.len() / 4); // Estimate average token length of 4
        let mut last_end = 0;

        for find in SPLIT_REGEX.find_iter(self) {
            if find.start() > last_end {
                result.push(&self[last_end..find.start()]);
            }

            result.push(find.as_str());
            last_end = find.end();
        }

        if last_end < self.len() {
            result.push(&self[last_end..]);
        }

        result
    }
}

impl SplitOn for String {
    fn split_on(&self, _delims: &Vec<&str>) -> Vec<&str> {
        let mut result = Vec::with_capacity(self.len() / 4);
        let mut last_end = 0;

        for find in SPLIT_REGEX.find_iter(self) {
            if find.start() > last_end {
                result.push(&self[last_end..find.start()]);
            }

            result.push(find.as_str());
            last_end = find.end();
        }

        if last_end < self.len() {
            result.push(&self[last_end..]);
        }

        result
    }
}

fn needs_to_dot_tokenize(text: &str) -> bool {
    text.contains(".") && get_primitive_type_from_literal(text) == PrimitiveType::Invalid
}




