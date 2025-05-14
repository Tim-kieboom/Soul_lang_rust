use std::io::Result;
use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_type::{soul_type::SoulType, type_modifiers::TypeModifiers}}, tokenizer::{file_line::FileLine, token::TokenIterator, tokenizer::tokenize_line}};

use super::get_expression::{get_expression, GetExpressionResult};

fn get_iter(line: &str, meta_data: &mut MetaData) -> Result<TokenIterator> {
    let mut in_multi_line_commned = false;
    let file_line = FileLine{text: line.to_string(), line_number: 1};
    let tokens = tokenize_line(file_line, 0, &mut in_multi_line_commned, meta_data)?;
    
    Ok(TokenIterator::new(tokens))
}

fn simple_get_expression(line: &str, should_be_type: Option<&SoulType>) -> Result<GetExpressionResult> {
    let mut meta_data = MetaData::new();
    
    let mut iter = get_iter(line, &mut meta_data)?;
    let context = CurrentContext::new(MetaData::GLOBAL_SCOPE_ID);

    get_expression(&mut iter, &mut meta_data, &context, &should_be_type, &vec![";"])
}

#[test]
fn test_get_expression_lit_int() -> Result<()> {
    let soul_type = SoulType::from_modifiers("int".to_string(), TypeModifiers::LITERAL);

    const LIT_INT: &str = "1;";
    
    let expr_result = simple_get_expression(LIT_INT, Some(&soul_type))?;

    assert!(expr_result.result.after.is_none() && expr_result.result.before.is_none(), "before or after is not empty");
    if let IExpression::Literal{value, type_name} = expr_result.result.value {
        assert_eq!(value, "1");
        assert_eq!(type_name, soul_type.to_string());
        assert_eq!(expr_result.is_type.to_string(), soul_type.to_string());
    }
    else {
        assert!(false, "expr_result should return 'literal': {:#?}", expr_result)
    }

    Ok(())
}







