use std::{collections::{BTreeMap, HashMap}};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::IExpression, get_abstract_syntax_tree::multi_stament_result::MultiStamentResult}, meta_data::{current_context::current_context::CurrentContext, function::{function_declaration::{function_declaration::FunctionDeclaration, get_function_declaration::add_function_declaration}, internal_functions::INTERNAL_FUNCTIONS}, meta_data::{CloseScopeResult, MetaData}, soul_names::{NamesInternalType, NamesTypeModifiers, SOUL_NAMES}}, tokenizer::{file_line::FileLine, token::{Token, TokenIterator}, tokenizer::tokenize_line}};
use crate::meta_data::soul_error::soul_error::Result;
use super::get_function_call::get_function_call;

fn try_store_function(line: &str, meta_data: &mut MetaData, context: &mut CurrentContext) -> Result<FunctionDeclaration> {
    let mut dummy = false;
    let tokens = tokenize_line(FileLine{text: line.to_string(), line_number: 0}, 0, &mut dummy, meta_data)?;
    let mut iter = TokenIterator::new(tokens);

    let function = add_function_declaration(&mut iter, meta_data, context, false)?;

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

    println!("{:?}", iter.get_tokens_text().iter().enumerate().map(|(i, el)| (i, el)).collect::<Vec<_>>());

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

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    const FUNC_CALL1: &str = "empty();";
    let mut function = simple_get_function_call(FUNC_CALL1, &mut meta_data, &mut context);
    let mut should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            empty_func, 
            vec![], 
            BTreeMap::new(),
            &DUMMY_TOKEN,
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
                IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN),
                IExpression::new_literal("2", &lit_untyped_int, &DUMMY_TOKEN)
            ], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        ),
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
            &DUMMY_TOKEN
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

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    const CALL1: &str = "over(1);";

    let function = simple_get_function_call(CALL1, &mut meta_data, &mut context);
    let should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            over_func1, 
            vec![
                IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN)            
            ], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );
}

#[test]
fn test_get_function_call_function_in_function() {
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);

    let lit_untyped_int = format!("{} {}", literal, untyped_int);

    let mut internal_functions: HashMap<&String, Vec<&FunctionDeclaration>> = HashMap::new();
    for func in INTERNAL_FUNCTIONS.iter() {
        internal_functions.entry(&func.name).or_default().push(func);
    }

    let int_int_func = (**internal_functions
        .get(&"int".to_string())
        .expect("int not found").iter()
        .filter(|func| func.args.first().is_some_and(|arg| arg.value_type == int))
        .collect::<Vec<_>>()
        .first()
        .expect("int(int) not found")).clone();

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    let func_declr = format!("ParseIntToString({} num) {}", int, "{}");
    let parse_int_func = store_function(&func_declr, &mut meta_data, &mut context);

    let func_call = "ParseIntToString(int(1))";
    let function = simple_get_function_call(func_call, &mut meta_data, &mut context);

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    let should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            parse_int_func, 
            vec![
                IExpression::new_funtion_call(
                    int_int_func, 
                    vec![IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN)], 
                    BTreeMap::new(),
                    &DUMMY_TOKEN
                ),
            ], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );
}

#[test]
fn test_get_function_call_internal_function() {
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);
    let untyped_float = SOUL_NAMES.get_name(NamesInternalType::UntypedFloat);
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);
    let f32 = SOUL_NAMES.get_name(NamesInternalType::Float32);
    
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    
    let lit_untyped_int = format!("{} {}", literal, untyped_int);
    let lit_untyped_float = format!("{} {}", literal, untyped_float);

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    

    let mut internal_functions: HashMap<&String, Vec<&FunctionDeclaration>> = HashMap::new();
    for func in INTERNAL_FUNCTIONS.iter() {
        internal_functions.entry(&func.name).or_default().push(func);
    }

    let u8_int_func = (**internal_functions
        .get(&"u8".to_string())
        .expect("u8 not found").iter()
        .filter(|func| func.args.first().is_some_and(|arg| arg.value_type == int))
        .collect::<Vec<_>>()
        .first()
        .expect("u8(int) not found")).clone();

    let u8_f32_func = (**internal_functions
        .get(&"u8".to_string())
        .expect("u8 not found").iter()
        .filter(|func| func.args.first().is_some_and(|arg| arg.value_type == f32))
        .collect::<Vec<_>>()
        .first()
        .expect("u8(f32) not found")).clone();

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    const CALL1: &str = "u8(1);";

    let mut function = simple_get_function_call(CALL1, &mut meta_data, &mut context);
    let mut should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            u8_int_func, 
            vec![
                IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN)            
            ], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

    const CALL2: &str = "u8(1.0);";

    function = simple_get_function_call(CALL2, &mut meta_data, &mut context);
    should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            u8_f32_func, 
            vec![
                IExpression::new_literal("1.0", &lit_untyped_float, &DUMMY_TOKEN)            
            ], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

}

#[test]
fn test_get_function_call_generic_no_validater() {
    let untyped_int = SOUL_NAMES.get_name(NamesInternalType::UntypedInt);
    let bool = SOUL_NAMES.get_name(NamesInternalType::Boolean);
    
    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    
    let lit_untyped_int = format!("{} {}", literal, untyped_int);
    let lit_bool = format!("{} {}", literal, bool);

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    

    let mut internal_functions: HashMap<&String, Vec<&FunctionDeclaration>> = HashMap::new();
    for func in INTERNAL_FUNCTIONS.iter() {
        internal_functions.entry(&func.name).or_default().push(func);
    }

    let print_func = (**internal_functions
        .get(&"Print".to_string())
        .expect("Print not found")
        .first()
        .expect("Print(any) not found")).clone();

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    const FUNC_CALL1: &str = "Print<int>(1);";
    let function = simple_get_function_call(FUNC_CALL1, &mut meta_data, &mut context);
    let should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            print_func.clone(), 
            vec![IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN)], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

    const FUNC_CALL2: &str = "Print(1);";
    let function = simple_get_function_call(FUNC_CALL2, &mut meta_data, &mut context);
    let should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            print_func.clone(), 
            vec![IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN)], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

    let str_format = (**internal_functions
        .get(&"__soul_format_string__".to_string())
        .expect("__soul_format_string__ not found")
        .first()
        .expect("__soul_format_string__(any...) not found")).clone();

    const FUNC_CALL3: &str = "__soul_format_string__(1);";
    let function = simple_get_function_call(FUNC_CALL3, &mut meta_data, &mut context);
    let should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            str_format.clone(), 
            vec![IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN)], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

    const FUNC_CALL4: &str = "__soul_format_string__(1, true);";
    let function = simple_get_function_call(FUNC_CALL4, &mut meta_data, &mut context);
    let should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            str_format.clone(), 
            vec![IExpression::new_literal("1", &lit_untyped_int, &DUMMY_TOKEN), IExpression::new_literal("true", &lit_bool, &DUMMY_TOKEN)], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );
}

#[test]
fn test_get_function_call_in_child_scope() {
    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);


    let func_declr1 = "empty();";
    let empty_func = store_function(&func_declr1, &mut meta_data, &mut context);

    let new_id = meta_data.open_scope(&context, true, false)
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap();

    context.set_current_scope_id(new_id);

    const DUMMY_TOKEN: Token = Token{line_number: 0, line_offset: 0, text: String::new()};
    const FUNC_CALL1: &str = "empty();";
    let mut function = simple_get_function_call(FUNC_CALL1, &mut meta_data, &mut context);
    let mut should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            empty_func, 
            vec![], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );

    

    /* 
        empty() {
            emptyInEmpty() {
                //do stuff
            }

            emptyInEmpty() // call
        }
    */
    let func_declr2 = "emptyInEmpty();";
    let empty_func2 = store_function(&func_declr2, &mut meta_data, &mut context);

    let new_id = meta_data.open_scope(&context, true, false)
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap();

    context.set_current_scope_id(new_id);
    const FUNC_CALL2: &str = "emptyInEmpty();";
    function = simple_get_function_call(FUNC_CALL2, &mut meta_data, &mut context);
    should_be = MultiStamentResult::new(
        IExpression::new_funtion_call(
            empty_func2, 
            vec![], 
            BTreeMap::new(),
            &DUMMY_TOKEN
        )
    );

    assert!(
        function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be 
    );


    let CloseScopeResult{delete_list:_, parent} = meta_data.close_scope(&context.get_current_scope_id(), false)
        .inspect_err(|err| panic!("{:?}", err))
        .unwrap();

    context.set_current_scope_id(parent);

}








