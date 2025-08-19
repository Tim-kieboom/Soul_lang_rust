use itertools::Itertools;

use crate::{errors::soul_error::SoulErrorKind, steps::{source_reader::c_str::format_stringer::format_string, step_interfaces::i_source_reader::FileLine}, utils::show_diff::show_str_diff};

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
    println(std.fmt.FormatArgs("",std.fmt.Arg(hello)," ",std.fmt.Arg(world),""))
    str format = std.fmt.FormatArgs("",std.fmt.Arg(1),", string")
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
fn does_format_str_line_work() {
    let source_file = str_to_file_lines(TEST_FILE);
    let mut results = Vec::new();
    
    for line in &source_file {
        let result = format_string(line.clone());
        assert!(result.is_ok(), "err: {}", result.unwrap_err().to_err_message().join("\n"));
        results.push(result.unwrap());
    }

    let file_result = to_string(results);
    assert!(
        file_result == SHOULD_BE_FILE, 
        "{}", show_str_diff(SHOULD_BE_FILE, file_result.as_str())
    )
}

#[test] 
fn formatter_with_hashtag_arg_makes_format_func() {
    const FILE: &str = "main int() { println(f\"{0} {#1} {##2} world\") }";
    let result = format_string(FileLine{line: FILE.to_string(), line_number: 1});
    
    assert!(result.is_err(), "test should have thrown error (so allowes c_strings as arg in f_string) ok: {:#?}", result.unwrap());
    assert_eq!(result.unwrap_err().get_kinds().last(), Some(&SoulErrorKind::InvalidStringFormat));
}

#[test] 
fn formatter_does_not_allow_string_literals_as_format_argument() {
    const FILE: &str = "main int() { println(f\"{\"hello\"} world\") }";
    let result = format_string(FileLine{line: FILE.to_string(), line_number: 1});
    
    assert!(result.is_err(), "test should have thrown error (so allowes c_strings as arg in f_string) ok: {:#?}", result.unwrap());
    assert_eq!(result.unwrap_err().get_kinds().last(), Some(&SoulErrorKind::InvalidStringFormat));
}














