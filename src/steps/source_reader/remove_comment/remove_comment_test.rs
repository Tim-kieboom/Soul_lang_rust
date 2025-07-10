use std::collections::HashMap;

use itertools::Itertools;

use crate::{steps::{source_reader::remove_comment::remove_comment::remove_comment, step_interfaces::i_source_reader::{FileLine, LineNumber, LineOffset, SourceFileResponse}}, utils::show_diff::show_str_diff};

const TEST_FILE: &str = r#"
// test
sum(i32 one, i32 two) i32
{
    return one + /*comment*/ two
}
/*

aowieufrygh

*/
main() i32
{
	println("hello // test /*test*/ world")
	i32 result = sum(1, 2)
}"#;

const SHOULD_BE_RESULT: &str = r#"sum(i32 one, i32 two) i32
{
    return one +  two
}
main() i32
{
	println("hello // test /*test*/ world")
	i32 result = sum(1, 2)
}"#;

fn str_to_file_lines(text: &str) -> Vec<FileLine> {
    text.lines()
        .enumerate()
        .map(|(i, line)| FileLine{line: line.to_string(), line_number: i+1})
        .collect()
}

fn to_string(file_lines: Vec<FileLine>) -> String {
    file_lines.iter()
        .map(|file_line| &file_line.line)
        .join("\n")
}

#[test]
fn does_remove_comment_work() {
    let source_file = str_to_file_lines(TEST_FILE);

    let mut source_file_result = SourceFileResponse::new();

    let mut in_multi_line = false;
    let mut result = Vec::new();

    for line in &source_file {
        let new_line = remove_comment(line.clone(), &mut in_multi_line, &mut source_file_result);
        let is_empty_line = new_line.line.is_empty();
        
        if in_multi_line || is_empty_line {
            continue;
        } 

        result.push(new_line);
    }

    let file_result = to_string(result);

    assert!(
        file_result == SHOULD_BE_RESULT, 
        "{}", show_str_diff(file_result.as_str(), SHOULD_BE_RESULT)
    );
}

#[test]
fn does_remove_comment_give_correct_gaps() {
    let source_file = str_to_file_lines(TEST_FILE);
    let mut source_file_result = SourceFileResponse::new();

    let mut in_multi_line = false;
    let mut result = Vec::new();

    for line in &source_file {
        let new_line = remove_comment(line.clone(), &mut in_multi_line, &mut source_file_result);
        let is_empty_line = new_line.line.is_empty();
        
        if in_multi_line || is_empty_line {
            continue;
        } 
        
        result.push(new_line);
    }

    let should_be_gaps: HashMap<LineNumber, HashMap<LineOffset, i64>> = HashMap::from([
        (5, HashMap::from([(18, 11)])),
        (11, HashMap::from([(0, 2)])), 
    ]);

    assert!(
        source_file_result.gaps == should_be_gaps, 
        "--------expected--------\n{:#?}\n--------got--------\n{:#?}\n----------------\n", should_be_gaps, source_file_result.gaps
    );
}





















