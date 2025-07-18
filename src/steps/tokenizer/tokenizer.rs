use std::collections::HashMap;

use once_cell::sync::Lazy;
use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan};
use crate::soul_names::SOUL_NAMES;
use crate::steps::step_interfaces::i_source_reader::{FileLine, SourceFileResponse};
use crate::steps::step_interfaces::i_tokenizer::{Token, TokenStream, TokenizeResonse};
use crate::utils::split_on::SplitOn;

static SPLIT_VEC: Lazy<Vec<&'static str>> = Lazy::new(|| {
    SOUL_NAMES
        .parse_tokens
        .iter()
        .map(|s| *s)
        .filter(|&s| s != ".")
        .collect()
});

pub fn tokenize(mut source_result: SourceFileResponse) -> Result<TokenizeResonse> {
    if source_result.source_file.is_empty() {
        return Ok(TokenizeResonse{stream: TokenStream::new(Vec::new())});
    }

    let mut tokens = Vec::with_capacity(source_result.estimated_token_count);

    let source_file = std::mem::take(&mut source_result.source_file);
    for file_line in source_file.into_iter().filter(is_not_empty_line) {
        get_tokens(file_line, &mut tokens, &mut source_result)?;
    }

    Ok(TokenizeResonse{stream: TokenStream::new(tokens)})
}

fn get_tokens(file_line: FileLine, tokens: &mut Vec<Token>, source_result: &mut SourceFileResponse) -> Result<()> {
    
    fn add_offset_with_gap(offset: usize, gap: i64) -> usize {
        if gap >= 0 {
            offset.saturating_add(gap as usize)
        } else {
            offset.saturating_sub((-gap) as usize)
        }
    }

    fn add_offset_range(offset: usize, gaps: &mut HashMap<usize, i64>, start: usize) -> usize {
        let mut sum_gap = 0i64;
        
        let to_remove: Vec<_> = gaps
            .keys()
            .filter(|&&k| k >= start && k <= offset)
            .cloned()
            .collect();

        for k in to_remove {
            sum_gap += gaps.remove(&k).unwrap();
        }

        add_offset_with_gap(offset, sum_gap)
    }


    let splits = file_line.line.split_on(&SPLIT_VEC);
    let mut line_offset = 0;
    let mut last_is_forward_slash = false;

    let mut gaps = source_result.gaps.remove(&file_line.line_number)
        .unwrap_or(HashMap::new());

    let mut in_string = false;
    let mut string_token = String::new();
    let mut string_span = SoulSpan::new(0,0,0);
    for (i, text) in splits.iter().enumerate() {
        if *text == "\"" {
            string_token.push_str(*text);
            if in_string {
                string_span.len = string_token.len();
                tokens.push(Token{text: std::mem::take(&mut string_token), span: string_span});
            }
            else {
                string_span = SoulSpan::new(file_line.line_number, line_offset, 0);
            }

            in_string = !in_string;
            continue;
        }

        if in_string {
            string_token.push_str(text);
            continue;
        }
        
        if text.is_empty() || *text == " " || *text == "\t" {
            line_offset = add_offset_range(line_offset + text.len(), &mut gaps, line_offset);
            continue;
        }

        if let Some(gap) = gaps.remove(&line_offset) {
            line_offset = add_offset_with_gap(line_offset, gap);
        } 

        if *text == ";" &&
           !splits.get(i+1).is_some_and(|token| token != &"\n") 
        {
            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                SoulSpan::new(file_line.line_number, line_offset, text.len()), 
                "can not end a line on ';'"
            ));
        }

        if !needs_to_dot_tokenize(text) {
            if *text != "\\" {
                tokens.push(Token::new(text.to_string(), SoulSpan::new(file_line.line_number, line_offset, text.len())));
                
                line_offset = add_offset_range(line_offset + text.len(), &mut gaps, line_offset);
                continue;
            }

            if i != splits.len() -1 {
                return Err(new_soul_error(
                    SoulErrorKind::UnexpectedToken, 
                    SoulSpan::new(file_line.line_number, line_offset, 1), 
                    "'\\' can only be placed at the end of a line"
                ));
            }

            last_is_forward_slash = true;
            break;
        }

        let dot_splits = text.split('.').collect::<Vec<_>>();
        let last_index = dot_splits.len() - 1;
        for (j, split) in dot_splits.into_iter().enumerate() {
            if split.is_empty() || split == " " {
                continue;
            }

            tokens.push(Token::new(
                split.to_string(),
                SoulSpan::new(file_line.line_number, line_offset, split.len())
            ));

            line_offset = add_offset_range(line_offset + text.len(), &mut gaps, line_offset);

            if j != last_index {
                tokens.push(Token::new(
                    ".".to_string(), 
                    SoulSpan::new(file_line.line_number, line_offset, split.len())
                ));
                line_offset = add_offset_range(line_offset + 1, &mut gaps, line_offset);
            }
        }
    }

    if !tokens.is_empty() && !last_is_forward_slash {
        tokens.push(Token::new(
            "\n".to_string(), 
            SoulSpan::new(file_line.line_number, line_offset, 1)
        ));
    }

    Ok(())
}

fn needs_to_dot_tokenize(text: &str) -> bool {
    text != ".." && text.contains(".") && !token_is_number(text)
}

fn token_is_number(s: &str) -> bool {
    s.parse::<f64>().is_ok()
}

fn is_not_empty_line(file_line: &FileLine) -> bool {
    !file_line.line.trim().is_empty()
}









































