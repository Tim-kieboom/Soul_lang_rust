use itertools::Itertools;

use crate::tokenizer::file_line::FileLine;

use super::format_stringer::{format_str_file, format_str_line};

const TEST_FILE: &str = r#"
main() i32
{
    println("hello world")
    
    str hello = "hello"
    str world = "world"
    println(f"{hello} {world}")
    str format = f"{1}, string"
    i32 result = sum(1, 2)
}"#;
    
const SHOULD_BE_FILE: &str = r#"
main() i32
{
    println("hello world")
    
    str hello = "hello"
    str world = "world"
    println(__soul_format_string__("", hello, " ", world, ""))
    str format = __soul_format_string__("", 1, ", string")
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
fn does_format_str_file_work() {
    let result = format_str_file(str_to_file_lines(TEST_FILE));
    assert!(result.is_ok(), "err: {}", result.unwrap_err());

    let file_result = to_string(result.unwrap());
    assert!(
        file_result == SHOULD_BE_FILE, 
        "--------expected--------\n{}\n--------got--------\n {}", SHOULD_BE_FILE, file_result
    );
}

#[test]
fn does_format_str_line_work() {
    let source_file = str_to_file_lines(TEST_FILE);
    let mut results = Vec::new();
    
    for (i, line) in source_file.iter().enumerate() {
        let result = format_str_line(line.clone(), i);
        assert!(result.is_ok(), "err: {}", result.unwrap_err());
        results.push(result.unwrap());
    }

    let file_result = to_string(results);
    assert!(
        file_result == SHOULD_BE_FILE, 
        "--------expected--------\n{}\n--------got--------\n {}", SHOULD_BE_FILE, file_result
    )
}

#[test] 
fn formatter_does_not_allow_string_literals_as_format_argument() {
    const FILE: &str = "main int() { println(f\"{\"hello\"} world\") }";

    let result = format_str_file(str_to_file_lines(FILE));
    
    assert!(result.is_err(), "test should have thrown error");
}