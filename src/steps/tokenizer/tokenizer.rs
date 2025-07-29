use regex::Regex;
use once_cell::sync::Lazy;
use std::collections::{BTreeMap};
use crate::soul_names::SOUL_NAMES;
use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_source_reader::{FileLine, SourceFileResponse};
use crate::steps::step_interfaces::i_tokenizer::{Token, TokenStream, TokenizeResonse};

static TOKEN_SPLIT_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(&SOUL_NAMES
        .parse_tokens
        .iter()
        .map(|s| regex::escape(s))
        .collect::<Vec<_>>()
        .join("|")
    ).unwrap()
});

pub fn tokenize(mut source_response: SourceFileResponse) -> Result<TokenizeResonse> {
    if source_response.source_file.is_empty() {
        return Ok(TokenizeResonse{stream: TokenStream::new(Vec::new())});
    }

    let mut tokens = Vec::with_capacity(source_response.estimated_token_count);

    let source_file = std::mem::take(&mut source_response.source_file);
    for file_line in source_file.into_iter().filter(is_not_empty_line) {
        get_tokens(file_line, &mut tokens, &mut source_response)?;
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

    fn add_offset_range(offset: usize, gaps: &mut BTreeMap<usize, i64>, start: usize) -> usize {
        let mut sum_gap = 0i64;

        let to_remove: Vec<usize> = gaps
            .range(start..=offset)
            .map(|(&k, _)| k)
            .collect();

        for key in to_remove {
            if let Some(gap) = gaps.remove(&key) {
                sum_gap += gap;
            }
        }

        add_offset_with_gap(offset, sum_gap)
    }


    let splits = split_tokens(&file_line.line);
    let mut line_offset = 0;
    let mut last_is_forward_slash = false;

    let mut gaps = source_result.gaps.remove(&file_line.line_number)
        .unwrap_or(BTreeMap::new());

    let mut in_string = false;
    let mut string_token = String::new();
    let mut string_span = SoulSpan::new(0,0,0);
    
    for (i, mut text) in splits.iter().enumerate() {

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
            
            let possible_lifetime = if text.len() > 2 {&text[text.len()-2..]} else {&text};
            if text.chars().nth_back(1) == Some('\'') && text.len() > 2 {
                let first = &text[..text.len()-2];
                tokens.push(Token::new(first.to_string(), SoulSpan::new(file_line.line_number, line_offset, first.len())));
                text = &possible_lifetime;
            }
            
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

        if *text == "." {
            tokens.push(Token::new(".".to_string(), SoulSpan::new(file_line.line_number, line_offset, 1)));
            line_offset = add_offset_range(line_offset + 1, &mut gaps, line_offset);
            continue;
        }

        let dot_splits = text.split('.').collect::<Vec<_>>();
        let last_index = dot_splits.len() - 1;
        for (j, mut split) in dot_splits.into_iter().enumerate() {
            if split.is_empty() || split == " " {
                continue;
            }

            let possible_lifetime = if split.len() > 2 {&split[split.len()-2..]} else {&split};
            if split.chars().nth_back(1) == Some('\'') && split.len() > 2 {
                let first = &split[..split.len()-2];
                tokens.push(Token::new(first.to_string(), SoulSpan::new(file_line.line_number, line_offset, first.len())));
                split = &possible_lifetime;
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

fn split_tokens(this: &str) -> Vec<&str> { 
    let regex = &*TOKEN_SPLIT_REGEX;
    let mut result = Vec::with_capacity(this.len() / 4);
    let mut last_end = 0;

    for find in regex.find_iter(this) {
        if find.start() > last_end {
            result.push(&this[last_end..find.start()]);
        }

        result.push(find.as_str());
        last_end = find.end();
    }

    if last_end < this.len() {
        result.push(&this[last_end..]);
    }

    result
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









































