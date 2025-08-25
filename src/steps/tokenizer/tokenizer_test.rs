use itertools::Itertools;

use crate::{steps::{source_reader::source_reader_tests, tokenizer::tokenizer::tokenize}, utils::show_diff::show_str_diff};

const SHOULD_BE: &str = r#"sum ( i32 one , i32 two ) i32 
 { 
 return one + two 
 } 
 main ( ) 
 { 
 "str" . Hash ( ) 
 func ( ) . Hash ( ) 
 print ( "hello world\n" ) 
 string := [ "1" , "2" , "3" , "4" , "5" , "6" ] 
 i32 result := sum ( 1 , 2.0 ) 
 result += 1 
 result -= - 1 
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



























