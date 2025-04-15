use std::io::Result;
use crate::abstract_styntax_tree::abstract_styntax_tree::IVariable;
use crate::abstract_styntax_tree::get_abstract_syntax_tree::multi_stament_result::MultiStamentResult;
use crate::abstract_styntax_tree::operator_type::{self, OperatorType};
use crate::meta_data::convert_soul_error::convert_soul_error::new_soul_error;
use crate::meta_data::soul_names::{self, InternalType, SoulNameEnum, SoulNames, TypeModifiers};
use crate::meta_data::meta_data::MetaData;
use crate::meta_data::soul_type::primitive_types::PrimitiveType;
use crate::meta_data::soul_type::soul_type::{SoulType, TypeModifiers};
use crate::tokenizer::token::Token;
use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, meta_data::{self, current_context::current_context::CurrentContext}, tokenizer::token::TokenIterator};



#[allow(dead_code)]
pub fn get_expression(
    iterator: &mut TokenIterator, 
    meta_data: &mut MetaData, 
    context: &CurrentContext,
    should_be_type: &SoulType,
    end_tokens: &Vec<&str>,
) -> Result<GetExpressionResult> {
    let mut result = MultiStamentResult::<IExpression>::new(IExpression::EmptyExpression());

    let begin_i = iterator.current_index();
    let mut stacks = ExpressionStacks::new();

    convert_expression(
        iterator, meta_data, context, &mut stacks, 
        &mut result, should_be_type, end_tokens,
    )?;

    todo!();
}

fn convert_expression(
    iterator: &mut TokenIterator, 
    meta_data: &mut MetaData, 
    context: &CurrentContext,
    
    stacks: &mut ExpressionStacks,
    result: &mut MultiStamentResult<IExpression>,

    should_be_type: &SoulType,
    end_tokens: &Vec<&str>,
) -> Result<()> {

    let mut open_bracket_stack = 0i64;
    let mut to_ref = None;
    
    iterator.next_multiple(-1);
    while let Some(token) = iterator.next() {
		// for catching ')' as endToken, 
        // (YES there are 2 isEndToken() this is because of checkBrackets() mutates the iterator DONT REMOVE PLZ)
        if is_end_token(token, end_tokens, open_bracket_stack) {
            return Ok(());
        }

        let should_get_bin_expression = check_brackets(iterator, stacks, &mut open_bracket_stack);
        if should_get_bin_expression {

        }

    }

    todo!();
}

fn get_bracket_binairy_expression(
    token: &Token,
    meta_data: &mut MetaData,
    stacks: &mut ExpressionStacks,
) -> Result<()> {

    if stacks.node_stack.len() == 1 {
        stacks.symbool_stack.pop();
        stacks.symbool_stack.pop();
        return Ok(());
    }

    if stacks.symbool_stack.pop().is_none_or(|symbool| symbool != ")") {
        return Err(new_soul_error(token, "int getBracketBinairyExpression(): symoboolStack top is not ')'"));
    }

    while let Some(symbool) = stacks.symbool_stack.last() {
        if symbool == "(" {
            break;
        }

        let op_type = OperatorType::from_str(symbool);
        stacks.symbool_stack.pop();

    }

    ()
}

fn get_binairy_expression(
    token: &Token,
    meta_data: &mut MetaData,
    stacks: &mut ExpressionStacks,
    operator_type: &OperatorType
) -> Result<IExpression> {
    
    let right = stacks.node_stack.pop().unwrap();
    let right_type = stacks.type_stack.pop().unwrap();

    if operator_type == &OperatorType::Not {
        
        let array_or_pointer = right_type.is_pointer() || right_type.is_array();
        if array_or_pointer || right_type.to_primitive_type(meta_data) != PrimitiveType::Bool {
            return Err(new_soul_error(token, format!("left_type: '{}' has to be type 'bool' to use '!'").as_str()));
        }

        if let IExpression::EmptyExpression() = right {
            return Err(new_soul_error(token, "right side of BinairyExpression is can not be empty"));
        }

        let bool_type = SoulType::new(meta_data.get_soul_name(InternalType::Boolean).to_string());
        if is_expression_literal(right, meta_data) {
            bool_type.add_modifier(TypeModifiers::LITERAL);
        }
    }

    Ok(())
}

fn is_expression_literal(expression: &IExpression, meta_data: &mut MetaData) -> bool {
    
    match expression {
        IExpression::IVariable { this } => is_ivariable_literal(this, meta_data),
        IExpression::BinairyExpression { left, operator_type, right, type_name } => todo!(),
        IExpression::Literal { value, type_name } => is_type_name_literal(type_name, meta_data),
        IExpression::ConstRef { expression } => is_expression_literal(expression, meta_data),
        IExpression::MutRef { expression } => is_expression_literal(expression, meta_data),
        IExpression::DeRef { expression } => is_expression_literal(expression, meta_data),
        IExpression::Increment { variable, is_before, amount } => is_ivariable_literal(variable, meta_data),
        IExpression::EmptyExpression() => true,
    }
}

fn is_ivariable_literal(expression: &IVariable, meta_data: &mut MetaData) -> bool {
    match expression {
        IVariable::Variable { name, type_name } => is_type_name_literal(type_name, meta_data),
        IVariable::MemberExpression { parent, expression } => is_ivariable_literal(parent.as_ref(), meta_data),
    }
}

fn is_type_name_literal(type_name: String, meta_data: &mut MetaData) -> bool {
    let soul_literal = meta_data.get_soul_name(soul_names::TypeModifiers::Literal);

    
}

fn check_brackets(
    iterator: &mut TokenIterator, 
    stacks: &mut ExpressionStacks, 
    open_bracket_stack: &mut i64,
) -> bool {
    let token = &iterator.current().text;
    if token == "(" {
        stacks.symbool_stack.push("(".to_string());
        iterator.next();
        
        *open_bracket_stack += 1;
    } 
    else if token == ")" {
        stacks.symbool_stack.push(")".to_string());
        iterator.next();

        *open_bracket_stack -= 1;
        if *open_bracket_stack >= 0 {
            return true;
        }
    }

    false
}

fn is_end_token(token: &Token, end_tokens: &Vec<&str>, open_bracket_stack: i64) -> bool {
    end_tokens.iter().any(|str| str == &token.text) && is_valid_end_token(token, open_bracket_stack)
}

fn is_valid_end_token(token: &Token, open_bracket_stack: i64) -> bool {
    token.text != ")" || (token.text == ")" && open_bracket_stack == 0)
}

struct ExpressionStacks {
    symbool_stack: Vec<String>,
    type_stack: Vec<SoulType>,
    node_stack: Vec<IExpression>,
}

impl ExpressionStacks {
    fn new() -> Self {
        ExpressionStacks { symbool_stack: Vec::new(), type_stack: Vec::new(), node_stack: Vec::new() }
    }
}

pub struct GetExpressionResult {
    result: MultiStamentResult<IExpression>,
    is_type: SoulType,
}










