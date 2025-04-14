use crate::{meta_data::{self, meta_data::MetaData}, tokenizer::{file_line, tokenizer::{tokenize_file, tokenize_line}}};

use super::file_line::FileLine;

const TEST_FILE: &str = r#"

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
	string := {"1", "2", "3", "4", "5", "6"}
	i32 result := sum(1, /*comment*/2)
	result += 1; result -= -1; 
	result = \
		2

	println(result)

	if true
		return 0
	else
		return 1

}
"#;

fn str_to_file_lines(text: &str) -> Vec<FileLine> {
    text.lines()
        .enumerate()
        .map(|(i, line)| FileLine{text: line.to_string(), line_number: i as u64+1})
        .collect()
}

#[test]
fn test_tokenize_line() {
    let should_be_tokens = vec![
    	/*0*/  vec![],
		/*1*/  vec![],
		/*2*/  vec!["sum", "(", "i32", "one", ",", "i32", "two", ")", "i32", "\n"],
		/*3*/  vec!["{", "\n"],
		/*4*/  vec!["return", "one", "+", "two", "\n"],
		/*5*/  vec![],
		/*6*/  vec!["}", "\n"],
		/*7*/  vec![],
		/*8*/  vec![],
		/*9*/  vec![],
		/*10*/ vec!["main", "(", ")", "\n"],
		/*11*/ vec!["{", "\n"],
		/*12*/ vec![],
		/*13*/ vec![],
		/*14*/ vec![],
		/*15*/ vec![],
		/*16*/ vec![],
		/*17*/ vec![],
		/*18*/ vec![],
		/*19*/ vec![],
		/*20*/ vec!["print", "(", "__Soul_c_str_0__", ")", "\n"],
		/*21*/ vec!["string", ":=", "{", "__Soul_c_str_1__", ",", "__Soul_c_str_2__", ",", "__Soul_c_str_3__", ",", "__Soul_c_str_4__", ",", "__Soul_c_str_5__", ",", "__Soul_c_str_6__", "}", "\n"],
		/*22*/ vec!["i32", "result", ":=", "sum", "(", "1", ",", "2", ")", "\n"],
		/*23*/ vec!["result", "+=", "1", ";", "result", "-=", "-", "1", ";", "\n"],
		/*24*/ vec!["result", "="],
		/*25*/ vec!["2", "\n"],
		/*26*/ vec![],
		/*27*/ vec!["println", "(", "result", ")", "\n"],
		/*28*/ vec![],
		/*29*/ vec!["if", "true", "\n"],
		/*30*/ vec!["return", "0", "\n"],
		/*31*/ vec!["else", "\n"],
		/*32*/ vec!["return", "1", "\n"],
		/*33*/ vec![],
		/*34*/ vec!["}", "\n"],
		/*35*/ vec![],
    ];

    let mut meta_data = MetaData::new();

    let source_file = str_to_file_lines(TEST_FILE);

    assert_eq!(source_file.len() + 1, should_be_tokens.len(), "len(source_file)[{}] != len(should_be_tokens)[{}]", source_file.len(), should_be_tokens.len());

    let mut in_multi_line = false;
    let mut multi_tokens = Vec::new();
    for (i, line) in source_file.iter().enumerate() {

        let result = tokenize_line(line.clone(), i, &mut in_multi_line, &mut meta_data);
        assert!(result.is_ok(), "err: {}", result.unwrap_err());
        let mut tokens = result.unwrap();
        if in_multi_line {
            tokens.clear();
        } 

        multi_tokens.extend(tokens.clone());

        let should_bo_tokens_line = &should_be_tokens[i];
        assert_eq!(should_bo_tokens_line.len(), tokens.len(), "line[{}]: len(tokens)[{}] != len(should_be_tokens)[{}], tokens: {:#?}", i, tokens.len(), should_bo_tokens_line.len(), tokens);

        for (j, token) in tokens.iter().enumerate() {
            assert_eq!(should_bo_tokens_line[j], token.text, "line[{}]: token is '{}' shuould be '{}'", i, should_bo_tokens_line[j], token.text);
        }
    }
}

#[test]
fn test_tokenize_file() {
    let should_be_tokens = vec![
		"sum", "(", "i32", "one", ",", "i32", "two", ")", "i32", "\n",
		"{", "\n",
		"return", "one", "+", "two", "\n",
		"}", "\n",
		"main", "(", ")", "\n",
		"{", "\n",
		"print", "(", "__Soul_c_str_0__", ")", "\n",
		"string", ":=", "{", "__Soul_c_str_1__", ",", "__Soul_c_str_2__", ",", "__Soul_c_str_3__", ",", "__Soul_c_str_4__", ",", "__Soul_c_str_5__", ",", "__Soul_c_str_6__", "}", "\n",
		"i32", "result", ":=", "sum", "(", "1", ",", "2", ")", "\n",
		"result", "+=", "1", ";", "result", "-=", "-", "1", ";", "\n",
		"result", "=",
		"2", "\n",
		"println", "(", "result", ")", "\n",
		"if", "true", "\n",
		"return", "0", "\n",
		"else", "\n",
		"return", "1", "\n",
		"}", "\n",
    ];

    let mut meta_data = MetaData::new();

    let source_file = str_to_file_lines(TEST_FILE);
    
	let est_token_size: usize = source_file.iter()
	                                       .map(|line| line.text.matches(" ").count())
										   .sum();

	let result_tokens = tokenize_file(source_file, est_token_size as u64, &mut meta_data);
	assert!(result_tokens.is_ok(), "err: {:#?}", result_tokens.unwrap_err());
	let tokens = result_tokens.unwrap();

    assert_eq!(tokens.len(), should_be_tokens.len());

	for (i, token) in tokens.iter().enumerate() {
		assert_eq!(token.text, should_be_tokens[i]);
	}
}