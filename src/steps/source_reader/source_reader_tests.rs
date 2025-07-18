use itertools::Itertools;
use std::io::{BufReader, Cursor};
use crate::{errors::soul_error::SoulError, steps::{source_reader::source_reader::read_source_file, step_interfaces::i_source_reader::SourceFileResponse}, utils::show_diff::show_str_diff};

pub const TEST_FILE: &str = r#"
sum(i32 one, i32 two) i32
{
	return one + two
// text etxt
}

// sdfrghsdf

main() 
{

/*
ertghserth
uytrduy
poujyitd
adsrfg
*/

	print("hello world\n")
	string := ["1", "2", "3", "4", "5", "6"]
	i32 result := sum(1, /*comment*/2)
	result += 1; result -= -1
	result = \
		2

	println(result)

	if true {
		return 0
    }
    else {
		return 1
    }
}"#;

const SHOULD_BE: &str = r#"sum(i32 one, i32 two) i32
{
    return one + two
}
main() 
{
    print("hello world\n")
    string := ["1", "2", "3", "4", "5", "6"]
    i32 result := sum(1, 2)
    result += 1; result -= -1
    result = \
        2
    println(result)
    if true {
        return 0
    }
    else {
        return 1
    }
}"#;

pub fn get_test_source_reader(file: &str) -> Result<SourceFileResponse, SoulError> {
	let amount_of_spaces_in_tab = 4;
	let reader = BufReader::new(Cursor::new(file.as_bytes()));
	
	read_source_file(reader, &" ".repeat(amount_of_spaces_in_tab))
}

pub fn get_test_result() -> Result<SourceFileResponse, SoulError> {
	get_test_source_reader(TEST_FILE)
}

#[test]
fn should_source_reader_work() {

	let amount_of_spaces_in_tab = 4;
	let reader = BufReader::new(Cursor::new(TEST_FILE.as_bytes()));
	let source_file = read_source_file(reader, &" ".repeat(amount_of_spaces_in_tab))
		.inspect_err(|err| panic!("{}", err.to_err_message()))
		.unwrap();

	let file = source_file.source_file.iter().map(|file_line| &file_line.line).join("\n");

	assert!(
		SHOULD_BE == file.as_str(),
		"{}", show_str_diff(SHOULD_BE, file.as_str())
	);
}


