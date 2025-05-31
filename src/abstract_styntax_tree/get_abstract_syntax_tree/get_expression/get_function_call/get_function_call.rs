use crate::{abstract_styntax_tree::{abstract_styntax_tree::IExpression, get_abstract_syntax_tree::{get_expression::get_expression::get_expression, multi_stament_result::MultiStamentResult}}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, function::{argument_info::argument_info::ArgumentInfo, function_declaration::function_declaration::FunctionDeclaration}, meta_data::MetaData, soul_type::soul_type::SoulType}, tokenizer::token::TokenIterator};
use std::{collections::BTreeMap, io::{Error, Result}};

struct Arguments {
    pub args: Vec<ArgumentInfo>,
    pub arg_expressions: Vec<IExpression>,
    
    pub optionals: Vec<ArgumentInfo>,
    pub optional_expressions: Vec<IExpression>,
} 
impl Arguments {
    pub fn new() -> Self {
        Arguments { 
            args: Vec::new(), 
            arg_expressions: Vec::new(), 
            optionals: Vec::new(), 
            optional_expressions: Vec::new(),
        }
    }
}

pub fn get_function_call(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
) -> Result<MultiStamentResult<IExpression>> {
    fn pass_err(err: Error, function_name: &str, iter: &TokenIterator) -> Error {
        new_soul_error(iter.current(), format!("while trying to get functionCall of: '{}'\n{}", function_name, err.to_string()).as_str())
    }
    
    let mut statment_result = MultiStamentResult::new(IExpression::EmptyExpression());
    if iter.current().text == "main" {
        return Err(new_soul_error(iter.current(), "can not call 'main' function"));
    }

    let function_name_index = iter.current_index();

    let is_func_result = meta_data.is_function(&iter[function_name_index].text, context);
    if !is_func_result.is_function() {
        return Err(new_soul_error(iter.current(), format!("function: '{}' does not exist", &iter[function_name_index].text).as_str()));
    }

    if let None = iter.next() {
        return Err(new_soul_error(iter.current(), "unexpected end while parsing FunctionCall"));
    }

    let generic_defines = get_generics(iter, meta_data, context)
        .map_err(|err| pass_err(err, &iter[function_name_index].text, iter))?;

    let arguments = get_arguments(iter, meta_data, context)
        .map_err(|err| pass_err(err, &iter[function_name_index].text, iter))?;
    
    let function_id = meta_data.try_get_function(&iter[function_name_index].text, iter, context, &arguments.args, &arguments.optionals, generic_defines)
        .map_err(|err| pass_err(err, &iter[function_name_index].text, iter))?;

    let scope = meta_data.scope_store.get(&function_id.0)
        .expect("Internal Error: scope_id could not be found");

    let function = scope.function_store.from_id.get(&function_id.1)
        .expect("Internal Error function id is not in function_store");

    let expressions = get_argument_expression(arguments, function);
    statment_result.value = IExpression::new_funtion_call(function.clone(), expressions, BTreeMap::new());


    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while parsing FunctionCall"));
    }

    Ok(statment_result)   
}

fn get_generics(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
) -> Result<Vec<String>> {

    let mut generic_defines = Vec::new();
    if iter.current().text == "<" {  
        if iter.next().is_none() {
            return Err(new_soul_error(iter.current(), "unexpected end while parsing FunctionCall"));
        }

        loop {
            let begin_i = iter.current_index();
            SoulType::from_iterator(iter, &meta_data.type_meta_data, &context.current_generics)
                .map_err(|err| new_soul_error(&iter[begin_i], format!("while trying to get generics\n{}", err.to_string()).as_str()))?;

            generic_defines.push(iter.current().text.clone());
            
            if iter.next().is_none() {
                return Err(new_soul_error(iter.current(), "unexpected end while parsing FunctionCall"));
            }

            if iter.current().text != "," {
                break;
            } 

            if iter.next().is_none() {
                return Err(new_soul_error(iter.current(), "unexpected end while parsing FunctionCall"));
            }
        }


        if iter.current().text != ">" {
            return Err(new_soul_error(
                iter.current(), 
                format!("while trying to get generics, generics should en with '>' but ends on '{}'", iter.current().text).as_str())
            );
        }
    
        if iter.next().is_none() {
            return Err(new_soul_error(iter.current(), "unexpected end while parsing FunctionCall"));
        }
    }

    Ok(generic_defines)
}

fn get_arguments(
    iter: &mut TokenIterator, 
    meta_data: &mut MetaData, 
    context: &mut CurrentContext, 
) -> Result<Arguments> {
    
    if iter.current().text != "(" {
        return Err(new_soul_error(iter.current(), "function call should start with ')'"));
    }

    let mut arguments = Arguments::new();

    let mut open_bracket_stack = 1;
    let mut arg = ArgumentInfo::new_empty();
    while iter.next().is_some() {
        arg.default_value = None;

        if iter.current().text == ")" {
            open_bracket_stack -= 1;

            if open_bracket_stack <= 0 {
                return Ok(arguments)
            }
        }
        else if iter.current().text == "(" {
            open_bracket_stack += 1;
        }

        arg.name.clear();

        let next_token = iter.peek()
            .ok_or(err_func_call_out_of_bounds(iter))?;
        
        if next_token.text == ":" {

            arg.name = iter.current().text.clone();
            if let None = iter.next_multiple(2) {
                return Err(err_func_call_out_of_bounds(iter));
            }
        }

        let begin_i = iter.current_index();
        let expr_result = get_expression(iter, meta_data, context, &None, &vec![",", ")"])
            .map_err(|err| new_soul_error(&iter[begin_i], format!("at argument number: {}\n{}", arg.arg_position+1, err.to_string()).as_str()))?;

        let (is_type, expression) = (expr_result.is_type, expr_result.result);
        if is_type.is_empty() {
            return Err(new_soul_error(iter.current(), format!("argument number: {}, '{}' is of type 'none', you can not have a 'none' type in an argument", arg.arg_position+1, expression.value.to_string()).as_str()));
        }

        let is_optional = !arg.name.is_empty();

        arg.value_type = is_type.to_string();

        if is_optional {
            arg.default_value = Some(expression.value.clone());
            arguments.optionals.push(arg.clone());
            arguments.optional_expressions.push(expression.value);
        }
        else {
            arguments.args.push(arg.clone());
            arguments.arg_expressions.push(expression.value);
        }

        arg.arg_position += 1;

        if iter.current().text == ")" { 
            open_bracket_stack -= 1;

            if open_bracket_stack <= 0 {
                return Ok(arguments);
            }
        }
    }

    Err(err_func_call_out_of_bounds(iter))
}

fn err_func_call_out_of_bounds(iter: &TokenIterator) -> Error {
    new_soul_error(iter.current(), "unexpeced end while trying to get arguments from function call")
}

fn get_argument_expression(arguments: Arguments, function: &FunctionDeclaration) -> Vec<IExpression> {
    let mut expressions = vec![IExpression::EmptyExpression(); arguments.args.len() + function.optionals.len()];

    for (_, arg) in &function.optionals {
        expressions[arg.arg_position as usize] = arg.default_value.clone().unwrap();
    }

    let mut arg_i = 0; 
    let mut optional_i = 0;

    for arg_expression in arguments.arg_expressions {
        expressions[arg_i] = arg_expression;
        arg_i += 1;
    } 

    for optional_expression in arguments.optional_expressions {
        let arg = &arguments.optionals[optional_i];
        let arg_opsition = function.optionals.get(&arg.name).unwrap().arg_position as usize;
        expressions[arg_opsition] = optional_expression;

        optional_i += 1;
    }

    expressions
}






