use itertools::Itertools;

use crate::{errors::soul_error::SoulError, steps::{source_reader::source_reader_tests, step_interfaces::i_tokenizer::TokenizeResonse, tokenizer::tokenizer::tokenize}, utils::show_diff::show_str_diff};

const SHOULD_BE: &str = r#"sum ( i32 one , i32 two ) i32 
 { 
 return one + two 
 } 
 main ( ) 
 { 
 print ( __cstr_0__ ) 
 string := [ __cstr_6__ , __cstr_5__ , __cstr_4__ , __cstr_3__ , __cstr_2__ , __cstr_1__ ] 
 i32 result := sum ( 1 , 2 ) 
 result += 1 ; result -= - 1 
 result = 2 
 println ( result ) 
 if true { 
 return 0 
 } 
 else { 
 return 1 
 } 
 } 
"#;

pub fn get_test_tokenizer(file: &str) -> Result<TokenizeResonse, SoulError> {
	tokenize(source_reader_tests::get_test_source_reader(file)?)
}

pub fn get_test_result() -> Result<TokenizeResonse, SoulError> {
	tokenize(source_reader_tests::get_test_result()?)
}

#[test]
fn tokenizer_should_work() {

    let source_file = source_reader_tests::get_test_result()
        .inspect_err(|err| panic!("while trying to get prev test result{:?}", err))
        .unwrap();

    let token_stream = tokenize(source_file)        
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap().stream;

    let tokens_string = token_stream
        .iter()
        .map(|token| &token.text)
        .join(" ");

    assert!(
        SHOULD_BE == tokens_string.as_str(),
        "{}", show_str_diff(SHOULD_BE, tokens_string.as_str())
    );
}



























