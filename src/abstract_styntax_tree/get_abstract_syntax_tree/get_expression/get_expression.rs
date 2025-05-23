use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::io::Result;
use std::result;

use crate::tokenizer::token::Token;
use crate::meta_data::meta_data::MetaData;
use crate::meta_data::type_store::ImplOperators;
use crate::meta_data::type_meta_data::TypeMetaData;
use crate::meta_data::soul_type::soul_type::SoulType;
use crate::meta_data::scope_and_var::var_info::VarInfo;
use crate::meta_data::soul_type::type_wrappers::TypeWrappers;
use crate::meta_data::soul_type::type_modifiers::TypeModifiers;
use crate::meta_data::soul_type::primitive_types::PrimitiveType;
use super::get_function_call::get_function_call::get_function_call;
use crate::meta_data::convert_soul_error::convert_soul_error::new_soul_error;
use crate::abstract_styntax_tree::operator_type::{OperatorType, ALL_OPERATORS};
use crate::meta_data::soul_names::{NamesInternalType, NamesTypeWrapper, SOUL_NAMES};
use crate::meta_data::current_context::current_context::{CurrentContext, CurrentGenerics};
use crate::abstract_styntax_tree::get_abstract_syntax_tree::multi_stament_result::MultiStamentResult;
use crate::{abstract_styntax_tree::abstract_styntax_tree::IExpression, tokenizer::token::TokenIterator};
use crate::meta_data::soul_type::type_checker::type_checker::{check_convert_to_ref, duck_type_equals, is_expression_literal};

/// literal value 'true'
static TRUE_LITERAL: Lazy<IExpression> = Lazy::new(|| {
    IExpression::new_literal(
        "true", 
        &SoulType::from_modifiers(
            SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string(),
            TypeModifiers::Literal
        ).to_string()
    )
});

/// literal value '-1'
static NEGATIVE_ONE_LITERAL: Lazy<IExpression> = Lazy::new(|| {
    IExpression::new_literal(
        "-1", 
        &SoulType::from_modifiers(
            SOUL_NAMES.get_name(NamesInternalType::UntypedInt).to_string(),
            TypeModifiers::Literal
        ).to_string(),
    )
});

#[allow(dead_code)]
pub fn get_expression(
    iter: &mut TokenIterator, 
    meta_data: &mut MetaData, 
    context: &mut CurrentContext,
    should_be_type: &Option<&SoulType>,
    end_tokens: &Vec<&str>,
) -> Result<GetExpressionResult> {
    let mut result = MultiStamentResult::<IExpression>::new(IExpression::EmptyExpression());
    

    let begin_i = iter.current_index();
    let mut stacks = ExpressionStacks::new();

    convert_expression(
        iter, meta_data, context, &mut stacks, 
        &mut result, should_be_type, end_tokens,
    )?;

    while let Some(symbool) = stacks.symbool_stack.pop() {
        let operator = OperatorType::from_str(&symbool);
        let binary = get_binairy_expression(iter, meta_data, context, &mut stacks, &operator)?;

        stacks.node_stack.push(binary);
    }

    if stacks.node_stack.is_empty() {
        if !stacks.symbool_stack.is_empty() {
            panic!("Internal error: in getExpression nodeStack.IsEmpty() but typeStack is not");
        }

        result.value = IExpression::EmptyExpression();
        let none_type = SoulType::new(SOUL_NAMES.get_name(NamesInternalType::None).to_string());
        return Ok(GetExpressionResult{result, is_type: none_type});
    }

    if stacks.node_stack.len() > 1 {
        let mut string_builder = String::new();
        let last_index = iter.current_index()-1;
        for i in begin_i..last_index {
            string_builder.push_str(&iter[i].text);
            string_builder.push(' ');
        }

        string_builder.push_str(&iter[last_index].text);
        return Err(new_soul_error(&iter[begin_i], format!("expression: '{}' is invalid (node_stack.len() > 1)", string_builder).as_str()));
    }

    let is_type: SoulType;
    if stacks.type_stack.is_empty() {
        if stacks.node_stack.is_empty() {
            panic!("Internal error: in getExpression typeStack.IsEmpty() but nodeStack is not");
        }

        let none_type = SoulType::new(SOUL_NAMES.get_name(NamesInternalType::None).to_string());
        is_type = none_type;
    }
    else {
        is_type = stacks.type_stack.pop().unwrap(); 
    }

    result.value = stacks.node_stack.pop().unwrap();
    Ok(GetExpressionResult{result, is_type})
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct GetExpressionResult {
    pub result: MultiStamentResult<IExpression>,
    pub is_type: SoulType,
}

fn convert_expression(
    iter: &mut TokenIterator, 
    meta_data: &mut MetaData, 
    context: &mut CurrentContext,
    
    stacks: &mut ExpressionStacks,
    result: &mut MultiStamentResult<IExpression>,

    should_be_type: &Option<&SoulType>,
    end_tokens: &Vec<&str>,
) -> Result<()> {

    let mut open_bracket_stack = 0i64;
    let mut ref_stack = Vec::new();
    
    iter.next_multiple(-1);
    let mut prev_token = Token{text: String::new(), line_number: 0, line_offset: 0};
    while iter.next().is_some() {
		// for catching ')' as endToken, 
        // (YES there are 2 isEndToken() this is because of checkBrackets() mutates the iterator DONT REMOVE PLZ)
        if is_end_token(iter.current(), end_tokens, open_bracket_stack) {
            return Ok(());
        }

        if check_brackets(iter, stacks, &mut open_bracket_stack) {
            get_bracket_binairy_expression(iter, meta_data, context, stacks)?;
        }

        let mut is_literal = false;
        let possible_literal = SoulType::from_literal(iter, &meta_data.type_meta_data, &mut context.current_generics, *should_be_type, &mut is_literal);
        if is_literal && matches!(possible_literal, Err(_)) {
            return Err(possible_literal.unwrap_err());
        }

        let possible_variable = meta_data.try_get_variable(&iter.current().text, &context.current_scope_id);
        
        if is_end_token(iter.current(), end_tokens, open_bracket_stack) {
            return Ok(());
        }
        else if is_ref(iter, &stacks) {
            ref_stack.push(iter.current().text.clone());
        }
        else if is_token_operator(iter.current()) {
            let operator_type = OperatorType::from_str(&iter.current().text);
            
            if operator_type == OperatorType::Increment || 
               operator_type == OperatorType::Decrement 
            {
                let is_before = !meta_data.is_variable(&prev_token.text, &context.current_scope_id);
                stacks.increment_info_stack.push(is_before);
            }

            convert_operator(iter, stacks, meta_data, context, result, should_be_type)?;
        }
        else if let Some(variable) = possible_variable {
            convert_function(iter, stacks, meta_data, context, variable)?;
        }
        else if possible_literal.is_ok() {
            convert_literal(iter, stacks, meta_data, context, possible_literal)?;
        }
        else if meta_data.is_function(&iter.current().text, context).is_none() {
            convert_function_call(iter, stacks, meta_data, context, result)?;
        }
        else if iter.current().text == "\n" {
            continue;
        }
        else {
            return Err(new_soul_error(iter.current(), format!("token: '{}' is not valid espression", iter.current().text).as_str()));
        }
        prev_token = iter.current().clone();

        if should_convert_to_ref(&ref_stack, stacks) {
            convert_to_ref(iter, &mut ref_stack, stacks, &meta_data, &mut context.current_generics)?;
        }

    }

    Err(new_soul_error(iter.current(), "unexpected end while parsing expression"))
}

fn is_ref(iter: &TokenIterator, stacks: &ExpressionStacks) -> bool {
    is_token_any_ref(iter.current()) && (stacks.node_stack.is_empty() || !stacks.symbool_stack.is_empty())
}

fn convert_function(
    iter: &mut TokenIterator, 
    stacks: &mut ExpressionStacks, 
    meta_data: &MetaData,
    context: &mut CurrentContext,
    variable: &VarInfo,
) -> Result<()> {
    if !variable.is_assigned() {
        return Err(new_soul_error(iter.current(), format!("'{}' can not be used before it is assigned", variable.name).as_str()));
    }

    let var_type;
    match SoulType::from_stringed_type(&variable.type_name, iter.current(), &meta_data.type_meta_data, &mut context.current_generics) {
        Ok(val) => var_type = val,
        Err(err) => return Err(new_soul_error(iter.current(), format!("while trying to get type of variable '{}'\n'{}'", variable.name, err.to_string()).as_str())),
    }

    stacks.type_stack.push(var_type);
    stacks.node_stack.push(IExpression::new_variable(&variable.name, &variable.type_name));
    Ok(())
}

fn convert_literal(
    iter: &mut TokenIterator, 
    stacks: &mut ExpressionStacks, 
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    possible_literal: Result<(SoulType, String)>
) -> Result<()> {
    let (literal_type, literal_value) = possible_literal.unwrap();

    let possible_original_type = literal_type.convert_typedef_to_original(
        iter.current(), 
        &meta_data.type_meta_data, 
        &mut context.current_generics,
    );

    let original_type;
    match possible_original_type {
        Some(val) => original_type = val,
        None => original_type = literal_type.clone(),
    };

    if original_type.to_primitive_type(&meta_data.type_meta_data) == PrimitiveType::Invalid {
        return Err(new_soul_error(iter.current(), "Literal has to be one of the primitiveTypes"));
    }

    let literal_type_string = literal_type.to_string();
    stacks.type_stack.push(literal_type);
    stacks.node_stack.push(IExpression::Literal{value: literal_value, type_name: literal_type_string});
    Ok(())
}

fn convert_function_call(
    iter: &mut TokenIterator, 
    stacks: &mut ExpressionStacks, 
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    result: &mut MultiStamentResult<IExpression>,
) -> Result<()> {
    let function_result = get_function_call(iter, meta_data, context)?;
    result.add_result(&function_result);
    let function_call = function_result.value;

    if let IExpression::FunctionCall { function_info, .. } = &function_call {
        let return_type;
        if let Some(type_string) = &function_info.return_type {
            let begin_i = iter.current_index();
            return_type = SoulType::from_stringed_type(type_string, iter.current(), &meta_data.type_meta_data, &mut context.current_generics)
                .map_err(|err| new_soul_error(&iter[begin_i], format!("while trying to get return type of function call: '{}'\n{}", function_info.name, err.to_string()).as_str()))?;
        } 
        else {
            return_type = SoulType::new_empty();
        }

        stacks.type_stack.push(return_type);
    }
    else {
        panic!("Internal Error iexpression from get_function_call is not function_call: {:#?}", function_call);
    }

    stacks.node_stack.push(function_call);
    Ok(())
}

fn convert_operator(
    iter: &mut TokenIterator, 
    stacks: &mut ExpressionStacks, 
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    result: &mut MultiStamentResult<IExpression>,
    should_be_type: &Option<&SoulType>,
) -> Result<()> {
    let operator_type = OperatorType::from_str(&iter.current().text);

    let current_precedence = operator_type.get_precedence();
    while !stacks.symbool_stack.is_empty() && 
          OperatorType::get_precedence_str(stacks.symbool_stack.last().unwrap()) >= current_precedence 
    {
        let operator = OperatorType::from_str(&stacks.symbool_stack.pop().unwrap());
        let binary_expression = get_binairy_expression(iter, meta_data, context, stacks, &operator)?;
        stacks.node_stack.push(binary_expression);
    }

    if is_negative_number(iter.current(), stacks) {
        let (negative_result, negative_type) = get_negative_expression(iter, meta_data, context, should_be_type)?;

        result.add_result(&negative_result);
        stacks.type_stack.push(negative_type);
        stacks.node_stack.push(negative_result.value);
        return Ok(());
    }

    stacks.symbool_stack.push(iter.current().text.clone());
    Ok(())
}

fn convert_to_ref(
    iter: &mut TokenIterator, 
    ref_stack: &mut Vec<String>, 
    stacks: &mut ExpressionStacks, 
    meta_data: &MetaData, 
    generics: &mut CurrentGenerics,
) -> Result<()> {
    debug_assert!(!ref_stack.is_empty());

    while let Some(to_ref) = ref_stack.pop() {
        let expression = stacks.node_stack.pop().unwrap();
        
        let is_double;
        let ref_wrap;

        if &to_ref == DOUBLE_CONST_REF.as_str() {
            is_double = true;
            ref_wrap = TypeWrappers::ConstRef;
        }
        else if &to_ref == DOUBLE_MUT_REF.as_str() {
            is_double = true;
            ref_wrap = TypeWrappers::MutRef;
        }
        else {
            is_double = false;
            ref_wrap = TypeWrappers::from_str(&to_ref);
        }


        check_convert_to_ref(iter, &expression, &ref_wrap, iter.current(), meta_data, generics)?;

        let mut refrence;
        if ref_wrap == TypeWrappers::MutRef {
            refrence = IExpression::new_mutref(expression);
            if is_double {
                refrence = IExpression::new_mutref(refrence);
            }
        }
        else {
            refrence = IExpression::new_constref(expression);
            if is_double {
                refrence = IExpression::new_constref(refrence);
            }
        }

        stacks.node_stack.push(refrence);
        let mut expression_type = stacks.type_stack.pop().unwrap();
        if let Err(msg) = expression_type.add_wrapper(ref_wrap) {
            return Err(new_soul_error(iter.current(), format!("while trying to add refrence to type: '{}'\n{}", expression_type.to_string(), msg).as_str()))
        }

        stacks.type_stack.push(expression_type);
    } 

    Ok(())
}

fn get_bracket_binairy_expression(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    stacks: &mut ExpressionStacks,
) -> Result<()> {

    if stacks.node_stack.len() == 1 {
        stacks.symbool_stack.pop();
        stacks.symbool_stack.pop();
        return Ok(());
    }

    if stacks.symbool_stack.pop().is_none_or(|symbool| symbool != ")") {
        return Err(new_soul_error(iter.peek_multiple(-1).unwrap_or(iter.current()), "int getBracketBinairyExpression(): symoboolStack top is not ')'"));
    }

    while let Some(symbool) = stacks.symbool_stack.last() {
        if symbool == "(" {
            break;
        }

        let op_type = OperatorType::from_str(symbool);
        stacks.symbool_stack.pop();
        let expression = get_binairy_expression(iter, meta_data, context, stacks, &op_type)?;

        stacks.node_stack.push(expression);
    }

    if stacks.symbool_stack.last().is_some_and(|symbool| symbool == &"(") {
        stacks.symbool_stack.pop();
    }

    Ok(())
}

fn get_binairy_expression(
    iter: &TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    stacks: &mut ExpressionStacks,
    operator_type: &OperatorType
) -> Result<IExpression> {
    assert!(!stacks.node_stack.is_empty(), "at: {}, Internal error while trying to get binaryExpression node_stack is empty", new_soul_error(iter.current(), ""));
    let right = stacks.node_stack.pop().unwrap();
    let right_type = stacks.type_stack.pop().unwrap();
    if right_type.is_empty() {
        return Err(new_soul_error(
            iter.current(), 
            format!(
                "binairy expression: '{}' rights type is 'none' which is not a valid type for binairy expressions", 
                format!("{} {} ..", right.to_string(), operator_type.to_str())
            ).as_str()
        ));
    }

    if let Err(msg) = is_valid_oparator(&right_type, operator_type, &meta_data.type_meta_data) {
        let current = iter.peek_multiple(-1).unwrap_or(iter.current());
        return Err(new_soul_error(current, format!("while trying to parse binary expression: {}", msg).as_str()));
    }

    if operator_type == &OperatorType::Increment || operator_type == &OperatorType::Decrement {
        let incr_variable;
        if let IExpression::IVariable{this} = right {
            incr_variable = this.clone();
        }
        else {
            return Err(new_soul_error(iter.current(), format!("you can not increment or decrement '{}' because you can only dso this for variables or members", right.to_string()).as_str()));
        }

        let is_before = stacks.increment_info_stack.pop().unwrap();
        
        let amount = if operator_type == &OperatorType::Increment {1}
                     else {-1};

        stacks.type_stack.push(right_type);
        return Ok(IExpression::new_increment(incr_variable, is_before, amount));         
    }
    else if operator_type == &OperatorType::Not {
        if let IExpression::EmptyExpression() = right {
            return Err(new_soul_error(iter.current(), "right side of BinairyExpression is can not be empty"));
        }

        let mut bool_type = SoulType::new(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string());
        if is_expression_literal(&right, iter.current(), meta_data, &mut context.current_generics)? {
            bool_type.add_modifier(TypeModifiers::Literal);
        }

        let type_name = bool_type.to_string();
        stacks.type_stack.push(bool_type);
        return Ok(IExpression::new_binary_expression(
            right,
            OperatorType::NotEquals, 
            TRUE_LITERAL.clone(), 
            &type_name,
        ));
    }

    if stacks.node_stack.is_empty() {
        return Err(new_soul_error(iter.current(), format!("BinairyExpression invalid at: '{}'", right.to_string()).as_str()));
    }

    let left = stacks.node_stack.pop().unwrap();
    let left_type = stacks.type_stack.pop().unwrap(); 
    if left_type.is_empty() {
        return Err(new_soul_error(
            iter.current(), 
            format!(
                "binairy expression: '{}' lefts type is 'none' which is not a valid type for binairy expressions", 
                format!("{} {} {}", right.to_string(), operator_type.to_str(), left.to_string())
            ).as_str()
        ));
    }
    if matches!(left, IExpression::EmptyExpression()) || 
       matches!(right, IExpression::EmptyExpression()) 
    {
        return Err(new_soul_error(iter.current(), "one of the expressions in binairyExpression is empty"));
    }

    check_if_types_in_binary_compatible(iter.current(), meta_data, operator_type, &right_type, &left_type)?;

    let binary_type = get_binary_type(left_type, operator_type, right_type, &meta_data);
    let binary_type_string = binary_type.to_string();
    stacks.type_stack.push(binary_type);

    Ok(IExpression::new_binary_expression( 
        left, 
        operator_type.clone(), 
        right, 
        &binary_type_string
    ))
}

fn get_binary_type(left_type: SoulType, operator_type: &OperatorType, right_type: SoulType, meta_data: &MetaData) -> SoulType {
    let is_left_literal = left_type.is_literal();
    let is_right_literal = right_type.is_literal();

    let is_left_untyped = left_type.is_untyped_type(&meta_data.type_meta_data);
    let is_right_untyped = right_type.is_untyped_type(&meta_data.type_meta_data);

    let mut binary_type;
    if operator_type.is_boolean_operator() {
        binary_type = SoulType::new(SOUL_NAMES.get_name(NamesInternalType::Boolean).to_string());
        if right_type.is_literal() && left_type.is_literal() {
            binary_type.add_modifier(TypeModifiers::Literal);
        }
    }
    else if is_left_untyped || is_right_untyped {
        binary_type = get_highest_priority_untyped_type(left_type, right_type, meta_data);
    } 
    else {
        binary_type = left_type;
    }
    
    if is_right_literal && is_left_literal {
        binary_type.add_modifier(TypeModifiers::Literal);
    }
    else {
        binary_type.remove_modifier(TypeModifiers::Literal);
    }

    binary_type
}

fn get_highest_priority_untyped_type(left_type: SoulType, right_type: SoulType, meta_data: &MetaData) -> SoulType {
    let binary_type;
    let is_left_untyped = left_type.is_untyped_type(&meta_data.type_meta_data);
    let is_right_untyped = right_type.is_untyped_type(&meta_data.type_meta_data);

    if is_left_untyped && is_right_untyped {
        let left_prim = left_type.to_primitive_type(&meta_data.type_meta_data);
        let right_prim = right_type.to_primitive_type(&meta_data.type_meta_data);
        if left_prim == PrimitiveType::UntypedFloat{
            binary_type = left_type;
        }
        else if right_prim == PrimitiveType::UntypedFloat {
            binary_type = right_type;
        }
        else if left_prim == PrimitiveType::UntypedInt {
            binary_type = left_type;
        }
        else if right_prim == PrimitiveType::UntypedInt {
            binary_type = right_type;
        }
        else {
            binary_type = left_type;
        }
    }
    else if is_left_untyped {
        binary_type = right_type;
    }
    else {
        binary_type = left_type
    }

    binary_type
}

fn check_if_types_in_binary_compatible(
    token: &Token,
    meta_data: &mut MetaData,
    operator_type: &OperatorType,
    right_type: &SoulType,
    left_type: &SoulType,
) -> Result<()> {
    let is_right_class = right_type.is_class(&meta_data.type_meta_data.class_store);
    let is_left_class = left_type.is_class(&meta_data.type_meta_data.class_store);
    if is_right_class || is_left_class {
        todo!();
        return Ok(());
    }

    if right_type.is_untyped_type(&meta_data.type_meta_data) || 
        left_type.is_untyped_type(&meta_data.type_meta_data) 
    {
        if !duck_type_equals(&left_type, &right_type, &meta_data.type_meta_data) {
            return Err(new_soul_error(token, format!("types: '{} {} {}' is invalid", left_type.to_string(), operator_type.to_str(), right_type.to_string()).as_str()));
        }
    }
    else if right_type != left_type {
        return Err(new_soul_error(token, format!("types: '{} {} {}' is invalid", left_type.to_string(), operator_type.to_str(), right_type.to_string()).as_str()));
    }

    Ok(())
}

fn get_negative_expression(
    iter: &mut TokenIterator, 
    meta_data: &mut MetaData, 
    context: &mut CurrentContext, 
    should_be_type: &Option<&SoulType>,
) -> Result<(MultiStamentResult<IExpression>, SoulType)> {
    if let None = iter.next() {
        return Err(new_soul_error(iter.current(), "unexpected end while trying to parse expression"));
    }

    let mut dummy = false;
    let possible_literal = SoulType::from_literal(iter, &meta_data.type_meta_data, &mut context.current_generics, *should_be_type, &mut dummy);
    let possible_variable = meta_data.try_get_variable(&iter.current().text, &context.current_scope_id);
    let mut result = MultiStamentResult::<IExpression>::new(IExpression::EmptyExpression());

    if let Some(variable) = possible_variable {
        let var_type = SoulType::from_stringed_type(
            &variable.type_name, 
            iter.current(), 
            &meta_data.type_meta_data, 
            &mut context.current_generics,
        )?;

        let var_expression = IExpression::new_variable(&variable.name, &variable.type_name);

        result.value = IExpression::new_binary_expression(
            var_expression, 
            OperatorType::Mul, 
            NEGATIVE_ONE_LITERAL.clone(), 
            &variable.type_name
        );
        
        return Ok((result, var_type)); 
    }
    else if let Ok((literal_type, literal_value)) = possible_literal {
        if literal_type.to_primitive_type(&meta_data.type_meta_data) == PrimitiveType::Invalid {
            return Err(new_soul_error(iter.current(), "Literal has to be one of the primitiveTypes"));
        }

        let literal_type_string = literal_type.to_string();
        let literal_expression = IExpression::new_literal(&literal_value, &literal_type_string);
        result.value = IExpression::new_binary_expression(
            literal_expression, 
            OperatorType::Mul, 
            NEGATIVE_ONE_LITERAL.clone(), 
            &literal_type_string,
        );

        return Ok((result, literal_type));
    }
    else {
        return Err(new_soul_error(
            iter.current(), 
            format!("'{}' is invalid after '-' (can only be a variable or a literal value)", iter.current().text).as_str()),
        );
    }
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
    pub symbool_stack: Vec<String>,
    pub type_stack: Vec<SoulType>,
    pub node_stack: Vec<IExpression>,
    pub increment_info_stack: Vec<bool>,
}

impl ExpressionStacks {
    fn new() -> Self {
        ExpressionStacks { symbool_stack: Vec::new(), type_stack: Vec::new(), node_stack: Vec::new(), increment_info_stack: Vec::new() }
    }
}
static DOUBLE_MUT_REF: Lazy<String> = Lazy::new(|| format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::MutRef), SOUL_NAMES.get_name(NamesTypeWrapper::MutRef)));
static DOUBLE_CONST_REF: Lazy<String> = Lazy::new(|| format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef), SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef)));

fn is_token_any_ref(token: &Token) -> bool {
    token.text == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) || 
    token.text == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) ||
    token.text == DOUBLE_MUT_REF.as_str() ||
    token.text == DOUBLE_CONST_REF.as_str()
}

fn is_token_operator(token: &Token) -> bool {
    OperatorType::from_str(&token.text) != OperatorType::Invalid
}

fn is_negative_number(token: &Token, stacks: &ExpressionStacks) -> bool {
    token.text == "-" && (stacks.node_stack.is_empty() || !stacks.symbool_stack.is_empty())
}

fn should_convert_to_ref(to_ref: &Vec<String>, stacks: &ExpressionStacks) -> bool {
    !to_ref.is_empty() && !stacks.node_stack.is_empty()
}

fn is_valid_oparator(current_type: &SoulType, operator: &OperatorType, type_meta_data: &TypeMetaData) -> result::Result<(), String> {

    let allowed_operators = get_allowed_operators(current_type, type_meta_data)?;
    let is_allowed = allowed_operators.operator.contains(operator);

    if !is_allowed {
        let allowed_str = allowed_operators.operator
            .iter()
            .map(|op| op.to_str())
            .collect::<Vec<_>>()
            .join(", ");

        return Err(format!("operator: '{}' is not allowed for type: '{}' allows: '[{}]'", operator.to_str(), current_type.to_string(), allowed_str))
    }

    Ok(())
}

// impl after test success
// find allowed_operatoers.min() validate all operators in min
fn get_allowed_operators(current_type: &SoulType, type_meta_data: &TypeMetaData) -> result::Result<ImplOperators, String> {    
    let READ_COMMENT = 1;
    let type_id = type_meta_data
        .type_store
        .to_id
        .get(&current_type.name)
        .ok_or_else(|| format!("type: '{}' is not found", current_type.to_string()))?;

    let mut operator_counts: HashMap<OperatorType, usize> = HashMap::with_capacity(ALL_OPERATORS.len());
    
    let allowed_type_operators = type_meta_data.type_store.implemented_type_operators.get(type_id).expect(format!("Internal error: implemented_type_operators missing type: '{}'", current_type.name).as_str());
    for operator in &allowed_type_operators.operator {
        *operator_counts.entry(*operator).or_insert(0) += 1;
    }

    for wrap in &current_type.wrappers {
        let wrap_operators;
        match type_meta_data.type_store.implemented_wrapper_operators.get(wrap) {
            Some(val) => wrap_operators = val,
            None => continue,
        }
        
        for operator in &wrap_operators.operator {
            *operator_counts.entry(*operator).or_insert(0) += 1;
        }
    }

    let num_of_impls = current_type.wrappers.len() + 1;
    let allowed_operators = operator_counts
        .into_iter()
        .filter_map(|(op_type, count)| if count == num_of_impls { Some(op_type) } else { None })
        .collect();

    Ok(ImplOperators { operator: allowed_operators })
}












