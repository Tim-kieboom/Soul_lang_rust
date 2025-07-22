use std::{collections::{BTreeMap}, iter::Peekable, str::Chars};

use crate::steps::step_interfaces::i_source_reader::{FileLine, SourceFileResponse};


struct RemoveCommentInfo<'a> {
    pub chars: Peekable<Chars<'a>>,
    
    pub current: char,
    pub prev: char,

    pub line_number: usize,
    pub line_offset: usize,
    pub gap_count: usize,
    pub gap_offset: usize,

    pub in_string: bool,
    pub string_delimiter: char,
    pub in_single_line_comment: bool,
    pub in_multi_line_comment: &'a mut bool,

    pub new_line: String,
}


pub fn remove_comment(file_line: FileLine, in_multi_line_comment: &mut bool, result: &mut SourceFileResponse) -> FileLine {
    let mut chars = file_line.line.chars().peekable();

    if file_line.line_number == 5 {
        println!("");
    } 

    let current;
    if let Some(ch) = chars.next() {
        current = ch;
    }
    else {
        return file_line;
    }

    let mut info = RemoveCommentInfo {
        chars,
        current,
        prev: ' ',
        
        line_number: file_line.line_number,
        line_offset: 0,
        gap_count: 0,
        gap_offset: 0,

        in_string: false,
        string_delimiter: ' ',
        in_multi_line_comment,
        in_single_line_comment: false,
        new_line: String::with_capacity(file_line.line.len()),
    };

    loop {
        check_char(&mut info, result);
        info.line_offset += 1;
        info.prev = info.current;

        if let Some(val) = info.chars.next() {
            info.current = val;
        }
        else {
            break;
        }
    }

    FileLine { line: info.new_line, line_number: file_line.line_number }
}

fn check_char<'a>(info: &mut RemoveCommentInfo<'a>, result: &mut SourceFileResponse) {

    if info.in_string {
        info.new_line.push(info.current);

        if info.current == info.string_delimiter && 
           info.prev != '\\' 
        {
            info.in_string = false;
        }
        return;
    }

    if !*info.in_multi_line_comment && !info.in_single_line_comment {
        
        if info.current == '"' {
            info.in_string = true;
            info.string_delimiter = info.current;
            info.new_line.push(info.current);
            return;
        }

        if info.current == '/' {
            if info.chars.peek() == Some(&'/') {
                info.in_single_line_comment = true;
                return;
            }

            if info.chars.peek() == Some(&'*') {
                info.gap_offset = info.line_offset+1;
                info.gap_count += 1;
                *info.in_multi_line_comment = true;
                return;
            }
        }
    }

    if info.in_single_line_comment {
        if info.current == '\n' {
            info.in_single_line_comment = false;
            info.new_line.push(info.current);
        }

        return;
    }

    if *info.in_multi_line_comment {
        
        info.gap_count += 1;

        if info.current == '*' && 
            info.chars.peek() == Some(&'/')
        {
            *info.in_multi_line_comment = false; 
            info.chars.next();

            let gaps = result.gaps.entry(info.line_number).or_insert(BTreeMap::new());
            gaps.insert(info.gap_offset, info.gap_count as i64 +1);
        }

        return;
    }

    info.new_line.push(info.current);
}


























