use itertools::Itertools;

use crate::{abstract_styntax_tree::{abstract_styntax_tree::IExpression, get_abstract_syntax_tree::multi_stament_result::MultiStamentResult}, debug_println, meta_data::{current_context::current_context::CurrentContext, function::{function_declaration::{function_declaration::FunctionDeclaration, get_function_declaration::get_function_declaration}, internal_functions::INTERNAL_FUNCTIONS}, meta_data::{self, MetaData}, soul_names::{NamesInternalType, NamesTypeModifiers, SOUL_NAMES}}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};
use std::{collections::{BTreeMap, HashMap}, io::Result, ops::Deref};

use super::get_function_call::get_function_call;

fn try_store_function(line: &str, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<FunctionDeclaration> {
    let mut dummy = false;
    let tokens = tokenize_line(FileLine{text: line.to_string(), line_number: 0}, 0, &mut dummy, meta_data)?;
    let mut iter = TokenIterator::new(tokens);

    let function = get_function_declaration(&mut iter, meta_data, context)?;

    Ok(function)
}

fn store_function(line: &str, meta_data: &mut MetaData, context: &mut CurrentContext) -> FunctionDeclaration {
    try_store_function(line, meta_data, context)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap()
}

fn try_simple_get_function_call(line: &str, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<MultiStamentResult<IExpression>> {
    let mut dummy = false;
    let tokens = tokenize_line(FileLine{text: line.to_string(), line_number: 0}, 0, &mut dummy, meta_data)?;
    let mut iter = TokenIterator::new(tokens);

    get_function_call(&mut iter, meta_data, context)
}

fn simple_get_function_call(line: &str, meta_data: &mut MetaData, context: &mut CurrentContext) -> MultiStamentResult<IExpression> {
    try_simple_get_function_call(line, meta_data, context)
        .inspect_err(|err| panic!("{:#?}", err))
        .unwrap()
}

#[test]
fn test_get_function_call() {
    let i32 = SOUL_NAMES.get_name(NamesInternalType::Int32);
    let str = SOUL_NAMES.get_name(NamesInternalType::String);
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);

    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    let lit_untyped_int = format!("{} {}", literal, untyped_int);

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);


    let func_declr1 = "empty();";
    let empty_func = store_function(&func_declr1, &mut meta_data, &mut context);

    const FUNC_CALL1: &str = "empty();";
    let mut function = simple_get_function_call(FUNC_CALL1, &mut meta_data, &mut context);
    let mut should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            empty_func, 
            vec![], 
            BTreeMap::new(),
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

//------------------------

    // sum(i32 a, i32 b) i32
    let func_declr2 = format!("sum({} a, {} b) {};", i32, i32, i32);
    let sum_func = store_function(&func_declr2, &mut meta_data, &mut context);

    const FUNC_CALL2: &str = "sum(1, 2);";
    function = simple_get_function_call(FUNC_CALL2, &mut meta_data, &mut context);
    should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            sum_func, 
            vec![
                IExpression::new_literal("1", &lit_untyped_int),
                IExpression::new_literal("2", &lit_untyped_int)
            ], 
            BTreeMap::new(),
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

//-----------------------

    //bar() str;
    let func_declr3 = format!("bar() {};", str);
    let empty_func = store_function(&func_declr3, &mut meta_data, &mut context);

    const FUNC_CALL3: &str = "bar()";
    function = simple_get_function_call(FUNC_CALL3, &mut meta_data, &mut context);
    should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            empty_func, 
            vec![], 
            BTreeMap::new(),
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );
}

#[test]
fn test_get_function_call_overload() {
    let i32 = SOUL_NAMES.get_name(NamesInternalType::Int32);
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);
    
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    let lit_untyped_int = format!("{} {}", literal, untyped_int);

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    // sum(i32 a, i32 b) i32
    let func_declr1 = format!("over({} a) {};", i32, i32);
    let over_func1 = store_function(&func_declr1, &mut meta_data, &mut context);

    const CALL1: &str = "over(1);";

    let mut function = simple_get_function_call(CALL1, &mut meta_data, &mut context);
    let mut should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            over_func1, 
            vec![
                IExpression::new_literal("1", &lit_untyped_int)            
            ], 
            BTreeMap::new(),
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );
}

#[test]
fn test_get_function_call_internal() {
    let i32 = SOUL_NAMES.get_name(NamesInternalType::Int32);
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);
    let untyped_float = SOUL_NAMES.get_name(NamesInternalType::UntypedFloat);
    
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    let const_ = SOUL_NAMES.get_name(NamesTypeModifiers::Constent);
    
    let lit_untyped_int = format!("{} {}", literal, untyped_int);

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    const CALL1: &str = "u8(1);";

    let mut internal_functions: HashMap<&String, Vec<&FunctionDeclaration>> = HashMap::new();
    for func in INTERNAL_FUNCTIONS.iter() {
        internal_functions.entry(&func.name).or_default().push(func);
    }

    let u8_func = (**internal_functions
        .get(&"u8".to_string())
        .expect("u8 not found").iter()
        .filter(|func| func.args.first().is_some_and(|arg| arg.value_type == untyped_float))
        .collect::<Vec<_>>()
        .first()
        .expect("u8(untypedInt) not found")).clone();

    let mut function = simple_get_function_call(CALL1, &mut meta_data, &mut context);
    let mut should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            u8_func, 
            vec![
                IExpression::new_literal("1", &lit_untyped_int)            
            ], 
            BTreeMap::new(),
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

}





