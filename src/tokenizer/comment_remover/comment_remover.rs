use super::super::file_line::FileLine;

pub fn remove_comment_file(source_file: Vec<FileLine>) -> Vec<FileLine> {
    let mut is_multi_line = false;
    let mut in_string = false;
    let mut string_delimiter = char::from_u32(0).unwrap();

    let mut new_source_file = Vec::with_capacity(source_file.len());

    for line in source_file {
        let str = remove_comment(&line, &mut string_delimiter, &mut in_string, &mut is_multi_line);
        if str.len() == 0 {
            continue;
        }

        new_source_file.push(FileLine {text: str, line_number: line.line_number});
    }

    new_source_file
}

pub fn remove_comment_line(mut line: FileLine, in_multi_line_comment: &mut bool) -> FileLine {
    let mut in_string = false;
    let mut string_delimiter = char::from_u32(0).unwrap();
    line.text = remove_comment(&line, &mut string_delimiter, &mut in_string, in_multi_line_comment);
    line
}

fn remove_comment(file_line: &FileLine, string_delimiter: &mut char, in_string: &mut bool, in_multi_line: &mut bool) -> String {
    let line = &file_line.text;
    let mut result = String::with_capacity(line.len());
    let mut in_single_line_comment = false;

    let mut skip_index = 0;
    let mut prev_ch = ' ';
    let mut chars = line.chars().peekable();

    while let Some(ch) = chars.next() {
        if skip_index > 0 {
            skip_index -= 1;
            continue;
        }

        if *in_string {
            result.push(ch);
            if ch == *string_delimiter && prev_ch != '\\' {
                *in_string = false;
            }
            prev_ch = ch;
            continue;
        }

        if !*in_multi_line && !in_single_line_comment {
            if ch == '"' {
                *in_string = true;
                *string_delimiter = ch;
                result.push(ch);

                prev_ch = ch;
                continue;
            }

            if ch == '/' {
                if chars.peek() == Some(&'/') {
                    in_single_line_comment = true;
                    
                    prev_ch = ch;
                    continue;
                }
                
                if chars.peek() == Some(&'*') {
                    *in_multi_line = true;
                    
                    prev_ch = ch;
                    continue;
                }
            }
        }

        if in_single_line_comment {
            if ch == '\n' {
                in_single_line_comment = false;
                result.push(ch);
            }

            prev_ch = ch;
            continue;
        }

        if *in_multi_line {
            if ch == '*' && chars.peek() == Some(&'/') {
                *in_multi_line = false;
                skip_index = 1;
            }

            prev_ch = ch;
            continue;
        }

        result.push(ch);
    }

    result
}