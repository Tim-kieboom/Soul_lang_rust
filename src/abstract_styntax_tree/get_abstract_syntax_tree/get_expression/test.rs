use std::io::Result;
use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, meta_data::soul_type::soul_type::{SoulType, TypeModifiers}};

fn simple_get_expression(line: &str, should_be_type: Option<SoulType>) -> Result<IExpression> {
    todo!();
}

#[test]
fn test_get_expression_lit_int() {
    let soul_type = SoulType::new_modifiers("name".to_string(), TypeModifiers::LITERAL);

    const LIT_INT: &str = "1;";

}