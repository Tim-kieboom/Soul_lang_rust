use crate::tokenizer::file_line::FileLine;
use itertools::Itertools;

use super::comment_remover::{remove_comment_file, remove_comment_line};

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
        .map(|(i, line)| FileLine{text: line.to_string(), line_number: i as u64+1})
        .collect()
}

fn to_string(file_lines: Vec<FileLine>) -> String {
    file_lines.iter()
              .map(|file_line| &file_line.text)
              .join("\n")
}

#[test]
fn does_remove_comment_line_work() {
    let source_file = str_to_file_lines(TEST_FILE);

    let mut in_multi_line = false;
    let mut result = Vec::new();

    for line in &source_file {
        let new_line = remove_comment_line(line.clone(), &mut in_multi_line);
        if in_multi_line || new_line.text.is_empty() {
            continue;
        } 

        result.push(new_line);
    }

    let file_result = to_string(result);

    assert!(
        file_result == SHOULD_BE_RESULT, 
        "--------expected--------\n{}\n--------got--------\n {}", SHOULD_BE_RESULT, file_result
    );
}

#[test]
fn does_remove_comment_file_work() {
    let source_file = str_to_file_lines(TEST_FILE);
    let result = remove_comment_file(source_file);

    let file_result = to_string(result);

    assert!(
        file_result == SHOULD_BE_RESULT,
        "--------expected--------\n{}\n--------got--------\n {}", SHOULD_BE_RESULT, file_result
    );
}