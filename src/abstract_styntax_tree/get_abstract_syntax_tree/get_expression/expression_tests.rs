use std::io::Result;
use itertools::assert_equal;

use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IExpression, IVariable}, get_abstract_syntax_tree::multi_stament_result::MultiStamentResult, operator_type::{self, OperatorType}}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, scope_and_var::var_info::{VarFlags, VarInfo}, soul_names::{NamesInternalType, SOUL_NAMES}, soul_type::{soul_type::SoulType, type_modifiers::TypeModifiers, type_wrappers::TypeWrappers}}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};

use super::get_expression::{get_expression, GetExpressionResult};

fn get_iter(line: &str, meta_data: &mut MetaData) -> Result<TokenIterator> {
    let mut in_multi_line_commned = false;
    let file_line = FileLine{text: line.to_string(), line_number: 1};
    let tokens = tokenize_line(file_line, 0, &mut in_multi_line_commned, meta_data)?;
    
    Ok(TokenIterator::new(tokens))
}

fn simple_get_expression_metadata(line: &str, should_be_type: Option<&SoulType>, meta_data: &mut MetaData, context: &CurrentContext) -> GetExpressionResult {
    let mut iter = get_iter(line, meta_data).unwrap();

    let end_tokens = vec![";"];
    get_expression(&mut iter, meta_data, &context, &should_be_type, &end_tokens)
        .inspect_err(|msg| panic!("{:#?}", msg))
        .unwrap()
}
fn try_simple_get_expression_metadata(line: &str, should_be_type: Option<&SoulType>, meta_data: &mut MetaData, context: &CurrentContext) -> Result<GetExpressionResult> {
    let mut iter = get_iter(line, meta_data).unwrap();

    let end_tokens = vec![";"];
    get_expression(&mut iter, meta_data, &context, &should_be_type, &end_tokens)
}

fn simple_get_expression(line: &str, should_be_type: Option<&SoulType>) -> GetExpressionResult {
    let mut meta_data = MetaData::new();
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    try_simple_get_expression_metadata(line, should_be_type, &mut meta_data, &context)
        .inspect_err(|msg| panic!("{:#?}", msg))
        .unwrap()
}
fn try_simple_get_expression(line: &str, should_be_type: Option<&SoulType>) -> Result<GetExpressionResult> {
    let mut meta_data = MetaData::new();
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    try_simple_get_expression_metadata(line, should_be_type, &mut meta_data, &context)
}

fn check_literal_expression(expr_result: GetExpressionResult, lit_value: &str, is_type: &SoulType) {
    if let IExpression::Literal{value, type_name} = expr_result.result.value {
        assert_eq!(value, lit_value);
        assert_eq!(type_name, is_type.to_string());
        assert_eq!(expr_result.is_type.to_string(), is_type.to_string());
    }
    else {
        assert!(false, "expr_result.result.valu should return 'literal': {:#?}", expr_result)
    }
}

fn check_variable_expression(expr_result: GetExpressionResult, variable_name: &str, is_type: &str) {
    if let IExpression::IVariable{this} = expr_result.result.value {
        if let IVariable::Variable {name, type_name} = this {
            assert_eq!(name, variable_name);
            assert_eq!(type_name, is_type.to_string());
            assert_eq!(expr_result.is_type.to_string(), is_type.to_string());
        }
        else {
            assert!(false, "expr_result.result.value.this should return 'Variable': {:#?}", this)
        }
    }
    else {
        assert!(false, "expr_result.result.value should return 'IVariable': {:#?}", expr_result)
    }
}

fn check_lit_binary_single(expr_result: GetExpressionResult, left_value: &str, operator: OperatorType, right_value: &str, should_be_type: &SoulType) {
    check_lit_binary_single_multi_type(expr_result, left_value, operator, right_value, should_be_type, should_be_type, should_be_type)
}

fn check_lit_binary_single_multi_type(expr_result: GetExpressionResult, left_value: &str, operator: OperatorType, right_value: &str, left_type: &SoulType, right_type: &SoulType, bin_type: &SoulType) {
    if let IExpression::BinairyExpression{left, operator_type, right, type_name} = expr_result.result.value {
        
        if let IExpression::Literal{value, type_name} = *left {
            assert_eq!(&value, left_value, "left value");
            assert_eq!(type_name, left_type.to_string(), "left type");
        }

        assert_eq!(operator_type, operator);

        if let IExpression::Literal{value, type_name} = *right {
            assert_eq!(&value, right_value, "right value");
            assert_eq!(type_name, right_type.to_string(), "right type");
        }

        assert_eq!(type_name, bin_type.to_string(), "binary type");

    }
    else {
        assert!(false, "expr_result.result.value should return 'BinaryExpression': {:#?}", expr_result) 
    }
}

fn assert_eq_iexpression(expr_result: GetExpressionResult, binary: IExpression) {
    if expr_result.result.value != binary {
        panic!("----------------\n{:#?}\n--------should be:--------\n{:#?}\n----------------", expr_result.result.value, binary);
    }
}

#[test]
fn test_get_expression_lit_int() {
    let lit_int_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Int).to_string(), TypeModifiers::Literal);
    let lit_untyped_int_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(), TypeModifiers::Literal);

    const LIT_INT: &str = "1;";

    let mut expr_result = simple_get_expression(LIT_INT, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "1", &lit_untyped_int_type);


    expr_result = simple_get_expression(LIT_INT, Some(&lit_int_type));
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "1", &lit_int_type);



    const LIT_MINUS_INT: &str = "-1;";

    expr_result = simple_get_expression(LIT_MINUS_INT, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "-1", &lit_untyped_int_type);


    expr_result = simple_get_expression(LIT_MINUS_INT, Some(&lit_int_type));
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "-1", &lit_int_type);
}

#[test]
fn test_get_expression_lit_float() {
    let lit_f32_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Float32).to_string(), TypeModifiers::Literal); 
    let lit_untyped_float_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(), TypeModifiers::Literal); 

    const LIT_FLOAT: &str = "1.0;";

    let mut expr_result = simple_get_expression(LIT_FLOAT, None);

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    if let IExpression::Literal{value, type_name} = expr_result.result.value {
        assert_eq!(value, "1.0");
        assert_eq!(type_name, lit_untyped_float_type.to_string());
        assert_eq!(expr_result.is_type.to_string(), lit_untyped_float_type.to_string());
    }
    else {
        assert!(false, "expr_result should return 'literal': {:#?}", expr_result)
    }

    expr_result = simple_get_expression(LIT_FLOAT, Some(&lit_f32_type));

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    if let IExpression::Literal{value, type_name} = expr_result.result.value {
        assert_eq!(value, "1.0");
        assert_eq!(type_name, lit_f32_type.to_string());
        assert_eq!(expr_result.is_type.to_string(), lit_f32_type.to_string());
    }
    else {
        assert!(false, "expr_result should return 'literal': {:#?}", expr_result)
    }

    const LIT_MINUS_FLOAT: &str = "-1.0;";

    expr_result = simple_get_expression(LIT_MINUS_FLOAT, None);

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    if let IExpression::Literal{value, type_name} = expr_result.result.value {
        assert_eq!(value, "-1.0");
        assert_eq!(type_name, lit_untyped_float_type.to_string());
        assert_eq!(expr_result.is_type.to_string(), lit_untyped_float_type.to_string());
    }
    else {
        assert!(false, "expr_result should return 'literal': {:#?}", expr_result)
    }

    expr_result = simple_get_expression(LIT_MINUS_FLOAT, Some(&lit_f32_type));

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    if let IExpression::Literal{value, type_name} = expr_result.result.value {
        assert_eq!(value, "-1.0");
        assert_eq!(type_name, lit_f32_type.to_string());
        assert_eq!(expr_result.is_type.to_string(), lit_f32_type.to_string());
    }
    else {
        assert!(false, "expr_result should return 'literal': {:#?}", expr_result)
    }
}

#[test]
fn test_get_expression_lit_binary() {
    let lit_u8_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint8).to_string(), TypeModifiers::Literal); 
    let lit_u16_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint16).to_string(), TypeModifiers::Literal); 
    let lit_u32_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint32).to_string(), TypeModifiers::Literal); 
    let lit_u64_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint64).to_string(), TypeModifiers::Literal); 
    
    let lit_i8_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Int8).to_string(), TypeModifiers::Literal); 
    
    const LIT_BINAIRY8: &str = "0b00000001;"; // is 1

    let mut expr_result = simple_get_expression(LIT_BINAIRY8, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0b00000001", &lit_u8_type);

    expr_result = simple_get_expression(LIT_BINAIRY8, Some(&lit_u8_type));
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0b00000001", &lit_u8_type);

    expr_result = simple_get_expression(LIT_BINAIRY8, Some(&lit_u16_type));
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0b00000001", &lit_u16_type);
    
    
    
    const LIT_MINUS_BINAIRY8: &str = "-0b00000001;"; // is -1

    expr_result = simple_get_expression(LIT_MINUS_BINAIRY8, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "-0b00000001", &lit_i8_type);


    
    const LIT_BINAIRY16: &str = "0b0000000100000001;"; // is 257

    expr_result = simple_get_expression(LIT_BINAIRY16, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0b0000000100000001", &lit_u16_type);
    
    
    
    const LIT_BINAIRY32: &str = "0b00000001000000010000000100000001;"; // is 16843009

    expr_result = simple_get_expression(LIT_BINAIRY32, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0b00000001000000010000000100000001", &lit_u32_type);



    const LIT_BINAIRY64: &str = "0b0000000100000001000000010000000100000001000000010000000100000001;"; // is 72340172838076673

    expr_result = simple_get_expression(LIT_BINAIRY64, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0b0000000100000001000000010000000100000001000000010000000100000001", &lit_u64_type);
    
    const LIT_TO_MANY_BITS: &str = "0b00000001000000010000000100000001000000010000000100000001000000011";
    assert!(try_simple_get_expression(LIT_TO_MANY_BITS, None).is_err());

}

#[test]
fn test_get_expression_lit_hexdeciaml() {
    let lit_u8_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint8).to_string(), TypeModifiers::Literal); 
    let lit_u16_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint16).to_string(), TypeModifiers::Literal); 
    let lit_u32_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint32).to_string(), TypeModifiers::Literal); 
    let lit_u64_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Uint64).to_string(), TypeModifiers::Literal); 
    
    let lit_i8_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Int8).to_string(), TypeModifiers::Literal); 

    const LIT_HEX_8: &str = "0xf;";

    let mut expr_result = simple_get_expression(LIT_HEX_8, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0xf", &lit_u8_type);

    expr_result = simple_get_expression(LIT_HEX_8, Some(&lit_u8_type));
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0xf", &lit_u8_type);

    const LIT_MINUS_HEX_8: &str = "-0xf;";

    expr_result = simple_get_expression(LIT_MINUS_HEX_8, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "-0xf", &lit_i8_type);



    const LIT_HEX_16: &str = "0x1f;";

    expr_result = simple_get_expression(LIT_HEX_16, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0x1f", &lit_u16_type);



    const LIT_HEX_32: &str = "0xf1fa;";

    expr_result = simple_get_expression(LIT_HEX_32, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0xf1fa", &lit_u32_type);



    const LIT_HEX_64: &str = "0xead9cb0f;";

    expr_result = simple_get_expression(LIT_HEX_64, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "0xead9cb0f", &lit_u64_type);



    const LIT_HEX_TO_BIG: &str = "0xfffffffff;";
    assert!(try_simple_get_expression(LIT_HEX_TO_BIG, None).is_err());

}

#[test]
fn test_get_expression_lit_string() {
    let lit_str_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::String).to_string(), TypeModifiers::Literal);

    const LIT_STRING: &str = "\"string\";";
    let mut expr_result = simple_get_expression(LIT_STRING, None);

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_variable_expression(expr_result, "__Soul_c_str_0__", &lit_str_type.to_string());

    
    const LIT_STRING_2: &str = "\"string\\\"\";";

    expr_result = simple_get_expression(LIT_STRING_2, None);

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_variable_expression(expr_result, "__Soul_c_str_0__", &lit_str_type.to_string());

    
    const LIT_EMPTY_STRING: &str = "\"\";";

    expr_result = simple_get_expression(LIT_EMPTY_STRING, None);

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_variable_expression(expr_result, "__Soul_c_str_0__", &lit_str_type.to_string());

}

#[test]
fn test_get_expression_lit_bracked_as_end_token() {
    let lit_untyped_int_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(), TypeModifiers::Literal);

    const LIT_INT: &str = "1)";
    let mut meta_data = MetaData::new();
    
    let mut iter = get_iter(LIT_INT, &mut meta_data).unwrap();
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    let expr_result = get_expression(&mut iter, &mut meta_data, &context, &None, &vec![")"]).unwrap();

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "1", &lit_untyped_int_type);

}

#[test]
fn test_get_expression_variable() {
    let mut global_var = VarInfo::new("global1".to_string(), SOUL_NAMES.get_name(NamesInternalType::Int).to_string());
    global_var.add_var_flag(VarFlags::IsAssigned);
    let mut scope_var = VarInfo::new("scope1".to_string(), SOUL_NAMES.get_name(NamesInternalType::Int).to_string());
    scope_var.add_var_flag(VarFlags::IsAssigned);

    let mut meta_data = MetaData::new();
    let mut context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    
    meta_data.add_to_global_scope(global_var.clone());

    
    const GLOBAL_VAR: &str = "global1;";

    let mut iter = get_iter(GLOBAL_VAR, &mut meta_data).unwrap();

    let expr_result = get_expression(&mut iter, &mut meta_data, &context, &None, &vec![";"]).unwrap();
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_variable_expression(expr_result, &global_var.name, &global_var.type_name);


    context.current_scope_id = meta_data.new_scope(context.current_scope_id).unwrap();
    meta_data.add_to_scope(scope_var.clone(), &context.current_scope_id);
    
    const SCOPE_VAR: &str = "scope1;";
    iter = get_iter(SCOPE_VAR, &mut meta_data).unwrap();

    let expr_result = get_expression(&mut iter, &mut meta_data, &context, &None, &vec![";"]).unwrap();
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_variable_expression(expr_result, &scope_var.name, &scope_var.type_name);
}

#[test]
fn test_get_expression_binary_expression_single_number() {
    let lit_untyped_int_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(), TypeModifiers::Literal);
    let lit_bool_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(), TypeModifiers::Literal);
    
    const BINARY_ADD: &str = "1 + 2;";
    
    let mut expr_result = simple_get_expression(BINARY_ADD, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "1", OperatorType::Add, "2", &lit_untyped_int_type);
    


    const BINARY_POWER: &str = "1 ** 2;";

    expr_result = simple_get_expression(BINARY_POWER, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "1", OperatorType::Pow, "2", &lit_untyped_int_type);



    const BINARY_LOG: &str = "1 log 2;";

    expr_result = simple_get_expression(BINARY_LOG, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "1", OperatorType::Log, "2", &lit_untyped_int_type);



    const BINARY_ROOT: &str = "1 </ 2;";

    expr_result = simple_get_expression(BINARY_ROOT, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "1", OperatorType::Root, "2", &lit_untyped_int_type);



    const BINARY_EQ: &str = "1 == 2;";

    expr_result = simple_get_expression(BINARY_EQ, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression( 
        IExpression::new_literal("1", &lit_untyped_int_type.to_string()), 
        OperatorType::Equals, 
        IExpression::new_literal("2", &lit_untyped_int_type.to_string()), 
        &lit_bool_type.to_string(),
    ));

    const BINARY_LOGICAL_AND: &str = "true && true;";

    expr_result = simple_get_expression(BINARY_LOGICAL_AND, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression( 
        IExpression::new_literal("true", &lit_bool_type.to_string()), 
        OperatorType::LogicalAnd, 
        IExpression::new_literal("true", &lit_bool_type.to_string()), 
        &lit_bool_type.to_string(),
    ));

    const BINARY_BIT_AND: &str = "1 & 2;";

    let expr_result = simple_get_expression(BINARY_BIT_AND, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression( 
        IExpression::new_literal("1", &lit_untyped_int_type.to_string()), 
        OperatorType::BitWiseAnd, 
        IExpression::new_literal("2", &lit_untyped_int_type.to_string()), 
        &lit_untyped_int_type.to_string(),
    ));
}

#[test]
fn test_get_expression_binary_expression_single_str() {
    let lit_str_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::String).to_string(), TypeModifiers::Literal);
    let lit_bool_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(), TypeModifiers::Literal);
    
    let lit_str_string = lit_str_type.to_string();

    const BINARY_ADD_STRING: &str = "\"hello \" + \"world\";";

    let mut expr_result = simple_get_expression(BINARY_ADD_STRING, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "__Soul_c_str_0__", OperatorType::Add, "__Soul_c_str_1__", &lit_str_type);



    const BINARY_EQ_STRING: &str = "\"hello \" == \"world\";";

    expr_result = simple_get_expression(BINARY_EQ_STRING, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "__Soul_c_str_0__", OperatorType::Equals, "__Soul_c_str_1__", &lit_bool_type);


    
    const BINARY_NOT_EQ_STRING: &str = "\"hello \" != \"world\";";

    expr_result = simple_get_expression(BINARY_NOT_EQ_STRING, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_lit_binary_single(expr_result, "__Soul_c_str_0__", OperatorType::NotEquals, "__Soul_c_str_1__", &lit_bool_type);



    const BINARY_ERROR_SMALLER_STRING: &str = "\"hello \" < \"world\";";

    let result = try_simple_get_expression(BINARY_ERROR_SMALLER_STRING, None);
    if let Err(err) = result {
        assert_eq!(
            &err.to_string(), 
            format!("at 1:17; !!error!! while trying to parse binary expression: operator: '<' is not allowed for type: '{}' allows: '[==, !=, +]'", lit_str_string).as_str(),
        );
    }
    else {
        assert!(false, "gert_expression of: '{}' should return error", BINARY_ERROR_SMALLER_STRING);
    }



    const BINARY_ERROR_MUL_STRING: &str = "\"hello \" * \"world\";";

    let result = try_simple_get_expression(BINARY_ERROR_MUL_STRING, None);
    if let Err(err) = result {
        assert_eq!(
            &err.to_string(), 
            format!("at 1:17; !!error!! while trying to parse binary expression: operator: '*' is not allowed for type: '{}' allows: '[==, !=, +]'", lit_str_string).as_str(),
        );
    }
    else {
        assert!(false, "gert_expression of: '{}' should return error", BINARY_ERROR_MUL_STRING);
    }



    const BINARY_ERROR_SUB_STRING: &str = "\"hello \" - \"world\";";

    let result = try_simple_get_expression(BINARY_ERROR_SUB_STRING, None);
    if let Err(err) = result {
        assert_eq!(
            &err.to_string(), 
            format!("at 1:17; !!error!! while trying to parse binary expression: operator: '-' is not allowed for type: '{}' allows: '[==, !=, +]'", lit_str_string).as_str(),
        );
    }
    else {
        assert!(false, "gert_expression of: '{}' should return error", BINARY_ERROR_SUB_STRING);
    }

}   

#[test]
fn test_get_expression_binary_expression_multiple_operators() {
    let lit_untyped_int_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(), TypeModifiers::Literal);
    let int_type = SoulType::new(SOUL_NAMES.get_name(NamesInternalType::Int).to_string());
    let lit_untyped_float_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(), TypeModifiers::Literal);
    let lit_bool_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(), TypeModifiers::Literal);
    
    let lit_untyped_float_string = lit_untyped_float_type.to_string();
    let lit_untyped_int_string = lit_untyped_int_type.to_string();
    let lit_bool_string = lit_bool_type.to_string();
    let int_string = int_type.to_string();

    const BINARY_ADD: &str = "1 + 2 + 3;";
    let mut expr_result = simple_get_expression(BINARY_ADD, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_literal("1", &lit_untyped_int_string), 
            OperatorType::Add, 
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &lit_untyped_int_string,
        ),
        OperatorType::Add,
        IExpression::new_literal("3", &lit_untyped_int_string),
        &lit_untyped_int_string
    ));

    const BINARY_ADD_FLOAT: &str = "1 + 2 + 3.0;";
    expr_result = simple_get_expression(BINARY_ADD_FLOAT, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_literal("1", &lit_untyped_int_string), 
            OperatorType::Add, 
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &lit_untyped_int_string,
        ),
        OperatorType::Add,
        IExpression::new_literal("3.0", &lit_untyped_float_string),
        &lit_untyped_float_string
    ));

    const BINARY_ADD_FLOAT_2: &str = "1.0 + 2 + 3;";
    expr_result = simple_get_expression(BINARY_ADD_FLOAT_2, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_literal("1.0", &lit_untyped_float_string), 
            OperatorType::Add, 
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &lit_untyped_float_string,
        ),
        OperatorType::Add,
        IExpression::new_literal("3", &lit_untyped_int_string),
        &lit_untyped_float_string
    ));


    const BINARY_MUL_ADD: &str = "1 * 2 + 3;";
    expr_result = simple_get_expression(BINARY_MUL_ADD, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_literal("1", &lit_untyped_int_string), 
            OperatorType::Mul, 
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &lit_untyped_int_string,
        ),
        OperatorType::Add,
        IExpression::new_literal("3", &lit_untyped_int_string),
        &lit_untyped_int_string
    ));

    const BINARY_ADD_MULL: &str = "1 + 2 * 3;";
    expr_result = simple_get_expression(BINARY_ADD_MULL, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_literal("1", &lit_untyped_int_string),
        OperatorType::Add,
        IExpression::new_binary_expression(
            IExpression::new_literal("2", &lit_untyped_int_string), 
            OperatorType::Mul, 
            IExpression::new_literal("3", &lit_untyped_int_string), 
            &lit_untyped_int_string,
        ),
        &lit_untyped_int_string
    ));


    const BINARY_BRACKETS: &str = "(1 + 2) * 3;";
    expr_result = simple_get_expression(BINARY_BRACKETS, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_literal("1", &lit_untyped_int_string), 
            OperatorType::Add,
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &lit_untyped_int_string,
        ),
        OperatorType::Mul, 
        IExpression::new_literal("3", &lit_untyped_int_string),
        &lit_untyped_int_string
    ));


    let mut meta_data = MetaData::new();
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    let mut global_var = VarInfo::new("var1".to_string(), SOUL_NAMES.get_name(NamesInternalType::Int).to_string());
    global_var.add_var_flag(VarFlags::IsAssigned);

    meta_data.add_to_global_scope(global_var);

    const BINARY_BRACKETS_VAR: &str = "(var1 + 2) * 3;";
    expr_result = simple_get_expression_metadata(BINARY_BRACKETS_VAR, None, &mut meta_data, &context);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_variable("var1", &int_string), 
            OperatorType::Add,
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &int_string,
        ),
        OperatorType::Mul, 
        IExpression::new_literal("3", &lit_untyped_int_string),
        &int_string
    ));

    const BINARY_BRACKETS_VAR_INCR: &str = "(var1++ + 2) * 3;";

    expr_result = simple_get_expression_metadata(BINARY_BRACKETS_VAR_INCR, None, &mut meta_data, &context);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_increment(IVariable::new_variable("var1", &int_string), false, 1), 
            OperatorType::Add,
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &int_string,
        ),
        OperatorType::Mul, 
        IExpression::new_literal("3", &lit_untyped_int_string),
        &int_string
    ));

    const BINARY_BRACKETS_VAR_INCR2: &str = "(++var1 + 2) * 3;";

    expr_result = simple_get_expression_metadata(BINARY_BRACKETS_VAR_INCR2, None, &mut meta_data, &context);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_increment(IVariable::new_variable("var1", &int_string), true, 1), 
            OperatorType::Add,
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &int_string,
        ),
        OperatorType::Mul, 
        IExpression::new_literal("3", &lit_untyped_int_string),
        &int_string
    ));

    const BINARY_BRACKETS_BOOL: &str = "1 < 2 != true;";

    expr_result = simple_get_expression_metadata(BINARY_BRACKETS_BOOL, None, &mut meta_data, &context);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_binary_expression(
        IExpression::new_binary_expression(
            IExpression::new_literal("1", &lit_untyped_int_string), 
            OperatorType::IsSmaller,
            IExpression::new_literal("2", &lit_untyped_int_string), 
            &lit_bool_string,
        ),
        OperatorType::NotEquals, 
        IExpression::new_literal("true", &lit_bool_string),
        &lit_bool_string
    ));
}

#[test]
fn test_get_expression_ref_literal() {
    let lit_untyped_float_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(), TypeModifiers::Literal);
    let lit_untyped_int_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(), TypeModifiers::Literal);
    let lit_bool_type = SoulType::from_modifiers(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(), TypeModifiers::Literal);
    
    let lit_untyped_float_string = lit_untyped_float_type.to_string();
    let lit_untyped_int_string = lit_untyped_int_type.to_string();
    let lit_bool_string = lit_bool_type.to_string();
    
    const LIT_MUT_REF_ERROR: &str = "&1;";
    const EXPECTED_ERROR: &str = "at 1:1; !!error!! while trying to get ref expression '&1'\nis a literal type so can not be mutRef'ed (remove '&' use '@' instead)";
    let result = try_simple_get_expression(LIT_MUT_REF_ERROR, None);
    if let Err(err) = result {
        assert_eq!(err.to_string().as_str(), EXPECTED_ERROR)
    }
    else {
        panic!("result should return error");
    }

    const INT_CONST_REF: &str = "@1;";
    let mut expr_result = simple_get_expression(INT_CONST_REF, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_literal("1", &lit_untyped_int_string)
    ));

    const FLOAT_CONST_REF: &str = "@1.0;";
    expr_result = simple_get_expression(FLOAT_CONST_REF, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_literal("1.0", &lit_untyped_float_string)
    ));

    const BOOL_CONST_REF: &str = "@true;";
    expr_result = simple_get_expression(BOOL_CONST_REF, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_literal("true", &lit_bool_string)
    ));

    const BOOL_CONST_REF_REF: &str = "@@true;";
    expr_result = simple_get_expression(BOOL_CONST_REF_REF, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_constref(
            IExpression::new_literal("true", &lit_bool_string),
        ),
    ));

    const BOOL_CONST_REF_REF_REF: &str = "@@@true;";
    expr_result = simple_get_expression(BOOL_CONST_REF_REF_REF, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_constref(
            IExpression::new_constref(
                IExpression::new_literal("true", &lit_bool_string),
            )
        ),
    ));
}

#[test]
fn test_get_expression_ref_variable() {
    let int_string = SOUL_NAMES.get_name(NamesInternalType::Int).to_string();
    let mut global_var = VarInfo::new("var".to_string(), int_string.clone());
    global_var.add_var_flag(VarFlags::IsAssigned);
    
    let mut meta_data = MetaData::new();
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);
    meta_data.add_to_global_scope(global_var.clone());

    
    const VAR_MUT_REF: &str = "&var;";
    let expr_result = simple_get_expression_metadata(VAR_MUT_REF, None, &mut meta_data, &context);
    assert_eq_iexpression(expr_result, IExpression::new_mutref(
        IExpression::new_variable("var", &int_string)
    ));

    const VAR_MUT_REF_REF: &str = "&&var;";
    let expr_result = simple_get_expression_metadata(VAR_MUT_REF_REF, None, &mut meta_data, &context);
    assert_eq_iexpression(expr_result, IExpression::new_mutref(
        IExpression::new_mutref(
            IExpression::new_variable("var", &int_string),
        ),
    ));

    const VAR_CONST_REF: &str = "@var;";
    let expr_result = simple_get_expression_metadata(VAR_CONST_REF, None, &mut meta_data, &context);
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_variable("var", &int_string)
    ));

    const VAR_CONST_REF_REF: &str = "@@var;";
    let expr_result = simple_get_expression_metadata(VAR_CONST_REF_REF, None, &mut meta_data, &context);
    assert_eq_iexpression(expr_result, IExpression::new_constref(
        IExpression::new_constref(
            IExpression::new_variable("var", &int_string)
        )
    ));
}

#[test]
fn test_get_expression_function_call() {
    todo!();
}

#[test]
fn test_get_expression_binary_expression_single_function() {
    todo!();
}

#[test]
fn test_get_expression_lit_tuple() {
    const LIT_TUPLE_STR_INT: &str = "(\"key\", 1)";
    const LIT_TUPLE_INT_FLOAT: &str = "(1, 1.1)";
    
    todo!();
}

#[test]
fn test_get_expression_lit_array() {
    let lit_int_array_type = SoulType::from(
        SOUL_NAMES.get_name(NamesInternalType::Int).to_string(), 
        vec![TypeWrappers::Array],
        TypeModifiers::Literal,
        Vec::new(),
    );
    let lit_untyped_int_array_type = SoulType::from(
        SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(), 
        vec![TypeWrappers::Array],
        TypeModifiers::Literal,
        Vec::new(),
    );

    let lit_untyped_float_array_type = SoulType::from(
        SOUL_NAMES.get_name(NamesInternalType::UntypedFloat).to_string(), 
        vec![TypeWrappers::Array],
        TypeModifiers::Literal,
        Vec::new(),
    );
    let lit_f32_array_type = SoulType::from(
        SOUL_NAMES.get_name(NamesInternalType::Float32).to_string(), 
        vec![TypeWrappers::Array],
        TypeModifiers::Literal,
        Vec::new(),
    );

    let lit_str_array_type = SoulType::from(
        SOUL_NAMES.get_name(NamesInternalType::String).to_string(), 
        vec![TypeWrappers::Array],
        TypeModifiers::Literal,
        Vec::new(),
    );

    const LIT_UNTYPED_INT_ARRAY: &str = "[1,2,3,4,5,6,7];";

    let mut expr_result = simple_get_expression(LIT_UNTYPED_INT_ARRAY, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1,2,3,4,5,6,7]", &lit_untyped_int_array_type);


    const LIT_UNTYPED_INT_ARRAY_2: &str = "[1,2,3,4,5,6,7,];";

    expr_result = simple_get_expression(LIT_UNTYPED_INT_ARRAY_2, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1,2,3,4,5,6,7,]", &lit_untyped_int_array_type);


    const LIT_INT_ARRAY: &str = "int[1,2,3,4,5];";

    expr_result = simple_get_expression(LIT_INT_ARRAY, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1,2,3,4,5]", &lit_int_array_type);
    

    const LIT_UNTYPED_FLOAT_ARRAY_1: &str = "[1.0,2.0,3.0,4.0,5.0];";

    expr_result = simple_get_expression(LIT_UNTYPED_FLOAT_ARRAY_1, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1.0,2.0,3.0,4.0,5.0]", &lit_untyped_float_array_type);
    

    const LIT_UNTYPED_FLOAT_ARRAY_2: &str = "[1.0,2,3,4,5];";

    expr_result = simple_get_expression(LIT_UNTYPED_FLOAT_ARRAY_2, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1.0,2,3,4,5]", &lit_untyped_float_array_type);


    const LIT_UNTYPED_FLOAT_ARRAY_3: &str = "[1,2.0,3,4,5];";

    expr_result = simple_get_expression(LIT_UNTYPED_FLOAT_ARRAY_3, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1,2.0,3,4,5]", &lit_untyped_float_array_type);


    const LIT_F32_ARRAY: &str = "f32[1,2,3,4,5];";

    expr_result = simple_get_expression(LIT_F32_ARRAY, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[1,2,3,4,5]", &lit_f32_array_type);


    const LIT_STR_ARRAY: &str = "[\"hello\", \"world\"];";
    
    expr_result = simple_get_expression(LIT_STR_ARRAY, None);
    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    check_literal_expression(expr_result, "[__Soul_c_str_0__,__Soul_c_str_1__]", &lit_str_array_type);
    
    todo!("[(1, 2), (1, 2)] impl tuple array\n [6 => 0] impl init arrays");
}




