use itertools::Itertools;

use crate::{meta_data::meta_data::MetaData, tokenizer::file_line::FileLine};

use super::string_mapper::{rawstr_to_litstr_file, rawstr_to_litstr_line};

const TEST_FILE: &str = r#"
main() i32
{
    println("hello world")
    
    str hello = "hello"
    str world = "world"
    i32 result = sum(1, 2)
    println("hello")
    println("")
}"#;
    
const SHOULD_BE_FILE: &str = r#"
main() i32
{
    println(__Soul_c_str_0__)
    
    str hello = __Soul_c_str_1__
    str world = __Soul_c_str_2__
    i32 result = sum(1, 2)
    println(__Soul_c_str_1__)
    println(__Soul_c_str_3__)
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
fn does_rawstr_to_litstr_file_work() {
    let mut meta_data = MetaData::new();
    let result = rawstr_to_litstr_file(str_to_file_lines(TEST_FILE), &mut meta_data);
    assert!(result.is_ok(), "err: {}", result.unwrap_err().to_err_message());

    let file_result = to_string(result.unwrap());
    assert!(
        file_result == SHOULD_BE_FILE, 
        "--------expected--------\n{}\n--------got--------\n {}", SHOULD_BE_FILE, file_result
    );
}

#[test]
fn does_rawstr_to_litstr_line_work() {
    let mut meta_data = MetaData::new();
    let source_file = str_to_file_lines(TEST_FILE);

    let mut results = Vec::new();
    for line in source_file {
        let result = rawstr_to_litstr_line(line, &mut meta_data);
        assert!(result.is_ok(), "err: {}", result.unwrap_err().to_err_message());
        
        results.push(result.unwrap());
    }

    let file_result = to_string(results);
    assert!(
        file_result == SHOULD_BE_FILE, 
        "--------expected--------\n{}\n--------got--------\n {}", SHOULD_BE_FILE, file_result
    )
}