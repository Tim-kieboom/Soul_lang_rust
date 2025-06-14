use crate::{meta_data::soul_error::soul_error::Result, tokenizer::token::Token};
use super::{function_declaration::FunctionDeclaration, get_function_declaration::add_function_declaration};
use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, meta_data::{class_info::access_level::AccesLevel, current_context::current_context::CurrentContext, function::{argument_info::argument_info::ArgumentInfo, internal_functions::FIRST_FUNCTION_ID}, meta_data::MetaData, scope_and_var::scope::{Scope}, soul_names::{NamesInternalType, NamesTypeModifiers, NamesTypeWrapper, SOUL_NAMES}}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};


fn try_data_simple_get_function(line: &str, meta_data: &mut MetaData) -> Result<(Result<FunctionDeclaration>, (TokenIterator, CurrentContext))> {

    let file_line = FileLine{text:line.to_string(), line_number: 0 };
    let mut in_multi_line_commned = false;
    let tokens = tokenize_line(file_line, 0, &mut in_multi_line_commned, meta_data)?;

    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    let mut iter = TokenIterator::new(tokens); 

    Ok((add_function_declaration(&mut iter, meta_data, &mut context, false), (iter, context)))
}

fn try_simple_get_function(line: &str) -> Result<FunctionDeclaration> {
    let mut meta_data = MetaData::new();
    try_data_simple_get_function(line, &mut meta_data)?.0
}

fn simple_get_function(line: &str) -> FunctionDeclaration {
    try_simple_get_function(line)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap()
}

fn global_scope(meta_data: &MetaData) -> &Scope {
    meta_data.scope_store.get(&MetaData::GLOBAL_SCOPE_ID).expect("Internal Error: scope_id could not be found")
}

#[test]
fn test_get_function_main() {
    let array = SOUL_NAMES.get_name(NamesTypeWrapper::Array);
    let str = SOUL_NAMES.get_name(NamesInternalType::String);
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);

    let literal = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);


    const MAIN_1: &str = "main() {}";

    let mut function = simple_get_function(MAIN_1);
    let mut should_be = FunctionDeclaration::new(
        "main".to_string(), 
        None, 
        vec![], 
        true, 
        *FIRST_FUNCTION_ID,
        MetaData::GLOBAL_SCOPE_ID,
    );

    assert!(function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be
    );

//----------------------------

    let main_int = format!("main() {} {}", int,"{}"); // main() int {}

    function = simple_get_function(&main_int);
    should_be = FunctionDeclaration::new(
        "main".to_string(), 
        Some(int.to_string()), 
        vec![], 
        true, 
        *FIRST_FUNCTION_ID,
        MetaData::GLOBAL_SCOPE_ID,
    );

    assert!(function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be
    );

//----------------------------

    let lit_str_array = format!("{} {}{}", literal, str, array);
    let main_args = format!("main({} args) {}", lit_str_array, "{}"); // main(str[] args) {}

    function = simple_get_function(&main_args);
    should_be = FunctionDeclaration::new(
        "main".to_string(), 
        None, 
        vec![
            ArgumentInfo::new_argument(
                "args".to_string(), 
                lit_str_array.clone(), 
                false, 0
            )
        ], 
        true, 
        *FIRST_FUNCTION_ID,
        MetaData::GLOBAL_SCOPE_ID,
    );

    assert!(function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be
    );

//----------------------------

    let main_str = format!("main() {} {}", str,"{}"); // main() str {}

    let res = try_simple_get_function(&main_str);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_err_message(), format!("at 0:7; !!error!! function: 'main' can only be on type or type: '{}'", int));

//----------------------------

    let str_array = format!("{}{}",str, array);
    let mut_args_main = format!("main(mut {} args) {}", str_array, "{}"); // main(mut str[] args) {}

    let res = try_simple_get_function(&mut_args_main);
    assert!(res.is_err());
    assert_eq!(res.unwrap_err().to_err_message(), format!("at 0:21; !!error!! function 'main' only allows 'main()' and 'main({})' as arguments", lit_str_array));


}

#[test]
fn test_get_function_default() {
    let i32 = SOUL_NAMES.get_name(NamesInternalType::Int32);
    
    let const_ = SOUL_NAMES.get_name(NamesTypeModifiers::Constent);
    
    let mut_ref = SOUL_NAMES.get_name(NamesTypeWrapper::MutRef);
    let const_ref = SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef);

    let const_i32 = format!("{} {}", const_, i32);

    let mut should_be = FunctionDeclaration::new(
        "sum".to_string(), 
        Some(i32.to_string()), 
        vec![
            ArgumentInfo::new_argument(
                "one".to_string(), 
                const_i32.to_string(), 
                false, 
                0,
            ),
            ArgumentInfo::new_argument(
                "two".to_string(), 
                const_i32.to_string(), 
                false, 
                1,
            ),
        ], 
        true, 
        *FIRST_FUNCTION_ID,
        MetaData::GLOBAL_SCOPE_ID,
    );

    // sum(i32 one, i32 two) i32 {}
    let func1 = format!("sum({} one, {} two) {} {}", i32, i32, i32, "{}");
    let mut function = simple_get_function(&func1);
    assert!(function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be
    );

//----------------------------

    let i32_mut_ref = format!("{}{}",i32, mut_ref);
    let i32_const_ref = format!("{}{}",i32, const_ref);
    // sum(mut i32 one, i32& two) i32@ {}
    let func1 = format!("sum(mut {} one, {} two) {} {}", i32, i32_mut_ref, i32_const_ref, "{}");

    should_be = FunctionDeclaration::new(
        "sum".to_string(), 
        Some(i32_const_ref), 
        vec![
            ArgumentInfo::new_argument(
                "one".to_string(), 
                i32.to_string(), 
                true, 
                0,
            ),
            ArgumentInfo::new_argument(
                "two".to_string(), 
                format!("{} {}", const_, i32_mut_ref), 
                false, 
                1,
            ),
        ], 
        true, 
        *FIRST_FUNCTION_ID,
        MetaData::GLOBAL_SCOPE_ID,
    );

    function = simple_get_function(&func1);
    assert!(function == should_be,
        "{:#?}\n!=\n{:#?}", function, should_be
    );
}

#[test]
fn test_get_function_optional() {
    let int = SOUL_NAMES.get_name(NamesInternalType::Int);
    let const_ = SOUL_NAMES.get_name(NamesTypeModifiers::Constent);

    let const_int = format!("{} {}", const_, int);

    let func_1 = format!("random({} seed = 2) {} {}", int, int, "{}");

    let should_be = FunctionDeclaration::from_optional(
        "random".to_string(), 
        Some(int.to_string()), 
        vec![],
        true, 
        *FIRST_FUNCTION_ID,
        vec![
            ArgumentInfo::new_optional(
                "seed".to_string(), 
                const_int.clone(), 
                Some(IExpression::new_literal("2", int, &Token{line_number: 0, line_offset: 0, text: String::new()})), 
                false, 
                0,
            )
        ], 
        MetaData::GLOBAL_SCOPE_ID,
    );


    let function = simple_get_function(&func_1);
    assert!(
        function == should_be, 
        "{:#?}\n!=\n{:#?}", function, should_be
    );
}

#[test]
fn test_get_function_store_in_meta_data() {
    let mut meta_data = MetaData::new();

    const FUNC: &str = "func() {}";

    let old_funcs_len = global_scope(&meta_data).function_store.from_id.len();
    let old_funcs_ids_len = global_scope(&meta_data).function_store.to_id.len();
    let (result, (mut iter, mut context)) = try_data_simple_get_function(FUNC, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    assert_eq!(global_scope(&meta_data).function_store.from_id.len(), old_funcs_len+1);
    assert_eq!(global_scope(&meta_data).function_store.to_id.len(), old_funcs_ids_len+1);

    let function = result
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    let optionals = function.optionals.values().cloned().collect::<Vec<_>>();
    meta_data.try_get_function(&function.name, &mut iter, &mut context, &function.args, &optionals, Vec::new())
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

//----------------------------

    const FUNC_SUM: &str = "sum(int a, int b) {}";
    let (result, (mut iter, mut context)) = try_data_simple_get_function(FUNC_SUM, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    assert_eq!(global_scope(&meta_data).function_store.from_id.len(), old_funcs_len+2);
    assert_eq!(global_scope(&meta_data).function_store.to_id.len(), old_funcs_ids_len+2);

    let function = result
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    let optionals = function.optionals.values().cloned().collect::<Vec<_>>();
    meta_data.try_get_function(&function.name, &mut iter, &mut context, &function.args, &optionals, Vec::new())
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

//----------------------------

    const FUNC_ARG: &str = "func(int a) {}";
    let (result, (mut iter, mut context)) = try_data_simple_get_function(FUNC_ARG, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    assert_eq!(global_scope(&meta_data).function_store.from_id.len(), old_funcs_len+3);
    assert_eq!(global_scope(&meta_data).function_store.to_id.len(), old_funcs_ids_len+2);

    let function = result
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    let optionals = function.optionals.values().cloned().collect::<Vec<_>>();
    meta_data.try_get_function(&function.name, &mut iter, &mut context, &function.args, &optionals, Vec::new())
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

//----------------------------

    const FUNC_OPTIONAL: &str = "func(int b = 1) {}";
    let (result, (mut iter, mut context)) = try_data_simple_get_function(FUNC_OPTIONAL, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    assert_eq!(global_scope(&meta_data).function_store.from_id.len(), old_funcs_len+4);
    assert_eq!(global_scope(&meta_data).function_store.to_id.len(), old_funcs_ids_len+2);

    let function = result
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    let optionals = function.optionals.values().cloned().collect::<Vec<_>>();
    meta_data.try_get_function(&function.name, &mut iter, &mut context, &function.args, &optionals, Vec::new())
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();
}

#[test]
fn test_get_function_forward_declaring() {
    let mut meta_data = MetaData::new();

    const FUNC: &str = "func() {}";
   
    let old_funcs_len = global_scope(&meta_data).function_store.from_id.len();
    let old_funcs_ids_len = global_scope(&meta_data).function_store.to_id.len();
    let (result, (mut iter, mut context)) = try_data_simple_get_function(FUNC, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    assert_eq!(global_scope(&meta_data).function_store.from_id.len(), old_funcs_len+1);
    assert_eq!(global_scope(&meta_data).function_store.to_id.len(), old_funcs_ids_len+1);

    let function = result
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    let optionals = function.optionals.values().cloned().collect::<Vec<_>>();
    meta_data.try_get_function(&function.name, &mut iter, &mut context, &function.args, &optionals, Vec::new())
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

//----------------------------

    let (result, (mut iter, mut context)) = try_data_simple_get_function(FUNC, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    assert_eq!(global_scope(&meta_data).function_store.from_id.len(), old_funcs_len+1);
    assert_eq!(global_scope(&meta_data).function_store.to_id.len(), old_funcs_ids_len+1);

    let function = result
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

    let optionals = function.optionals.values().cloned().collect::<Vec<_>>();
    meta_data.try_get_function(&function.name, &mut iter, &mut context, &function.args, &optionals, Vec::new())
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();

//----------------------------

    let (result, _) = try_data_simple_get_function(FUNC, &mut meta_data)
        .inspect_err(|err| panic!("{}", err.to_err_message()))
        .unwrap();
    
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_err_message(), "at 0:7; !!error!! function with these arguments already exists, name 'func', args: '<empty>'\n");
}

#[test]
fn test_get_function_public_and_private() {
    const PRIVATE: &str = "private() {}";

    let mut function = simple_get_function(PRIVATE);
    assert_eq!(function.access_level, AccesLevel::Private);
    
    const PUBLIC: &str = "Public() {}";

    function = simple_get_function(PUBLIC);
    assert_eq!(function.access_level, AccesLevel::Public);
}




