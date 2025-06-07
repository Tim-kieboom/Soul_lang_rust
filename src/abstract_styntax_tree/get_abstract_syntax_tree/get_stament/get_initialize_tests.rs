use crate::{meta_data::soul_error::soul_error::Result, tokenizer::token::Token};
use super::get_initialize::get_initialize;
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IExpression, IStatment, IVariable}, get_abstract_syntax_tree::multi_stament_result::MultiStamentResult}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_names::{NamesInternalType, NamesTypeModifiers, SOUL_NAMES}}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};

fn try_simple_initialize(line: &str) -> Result<MultiStamentResult<IStatment>> {
    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    let mut dummy = false;
    let tokens = tokenize_line(FileLine{text: line.to_string(), line_number: 0}, 0, &mut dummy, &mut meta_data)?;
    let mut iter = TokenIterator::new(tokens);

    get_initialize(&mut iter, &mut meta_data, &mut context)
}

fn simple_initialize(line: &str) -> MultiStamentResult<IStatment> {
    try_simple_initialize(line)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap()
}

#[test]
fn test_initialize_default_typed() {
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);

    let init1 = format!("{} foo = 1", int);
    
    let mut result = simple_initialize(&init1);

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    let mut varaiable = IVariable::new_variable("foo", int, &DUMMY_TOKEN);
    let mut should_be = MultiStamentResult::new(
        IStatment::new_initialize(
            varaiable.clone(), 
            Some(IStatment::new_assignment(varaiable, IExpression::new_literal("1", int, &DUMMY_TOKEN), &DUMMY_TOKEN)),
            &DUMMY_TOKEN
        )
    );

    assert!(
        result == should_be,
        "{:#?}\n!=\n{:#?}", result, should_be
    );


    let init2 = format!("{} foo", int);
    
    result = simple_initialize(&init2);

    varaiable = IVariable::new_variable("foo", int, &DUMMY_TOKEN);
    should_be = MultiStamentResult::new(
        IStatment::new_initialize(
            varaiable.clone(), 
            None,
            &DUMMY_TOKEN
        )
    );

    assert!(
        result == should_be,
        "{:#?}\n!=\n{:#?}", result, should_be
    );

    let init3 = format!("{} foo = \"hello\"", int);

    let res = try_simple_initialize(&init3);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_err_message(), "at 0:26; !!error!! assignment type: 'Literal str' is not compatible with variable type: 'int'");
}

#[test]
fn test_initialize_default_invered() {
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);
    
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);

    let lit_untyped_int = format!("{} {}", literal, untyped_int);

    let init1 = "foo := 1";
    
    let result = simple_initialize(&init1);

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    let varaiable = IVariable::new_variable("foo", int, &DUMMY_TOKEN);
    let should_be = MultiStamentResult::new(
        IStatment::new_initialize(
            varaiable.clone(), 
            Some(IStatment::new_assignment(varaiable, IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN), &DUMMY_TOKEN)),
            &DUMMY_TOKEN
        )
    );

    assert!(
        result == should_be,
        "{:#?}\n!=\n{:#?}", result, should_be
    );

    let init2 = "foo";
    
    let res = try_simple_initialize(&init2);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_err_message(), "at 0:3; !!error!! variable: 'foo' is not assign a type (add type before variable like 'i32 var')");
}






