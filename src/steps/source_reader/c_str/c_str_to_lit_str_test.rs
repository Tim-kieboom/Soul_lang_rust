use itertools::Itertools;

use crate::{steps::{source_reader::c_str::c_str_to_lit_str::c_str_to_lit_str, step_interfaces::i_source_reader::{FileLine, SourceFileResult}}, utils::show_diff::show_str_diff};


const TEST_FILE: &str = r#"
main() int
{
    println("hello world")
    
    str hello = "hello"
    str world = "world"
    i32 result = sum(1, 2)
    println("hello")
    println("")
}"#;
    
const SHOULD_BE_FILE: &str = r#"
main() int
{
    println(__cstr_0__)
    
    str hello = __cstr_1__
    str world = __cstr_2__
    i32 result = sum(1, 2)
    println(__cstr_1__)
    println(__cstr_3__)
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
fn does_c_str_to_lit_str_work() {
    let source_file = str_to_file_lines(TEST_FILE);

    let mut source_result = SourceFileResult::new(); 

    let mut results = Vec::new();
    for line in source_file {
        let result = c_str_to_lit_str(line, &mut source_result);
        assert!(result.is_ok(), "err: {}", result.unwrap_err().to_err_message());
        
        results.push(result.unwrap());
    }

    let file_result = to_string(results);
    assert!(
        file_result == SHOULD_BE_FILE, 
        "{}", show_str_diff(SHOULD_BE_FILE, file_result.as_str())
    )
}






















