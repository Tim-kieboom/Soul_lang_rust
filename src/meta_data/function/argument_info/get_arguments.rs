use std::io::{Error, Result};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::IExpression, get_abstract_syntax_tree::get_expression::get_expression::get_expression}, meta_data::{convert_soul_error::convert_soul_error::new_soul_error, current_context::current_context::CurrentContext, function::function_modifiers::FunctionModifiers, meta_data::MetaData, soul_names::{check_name, NamesInternalType, SOUL_NAMES}, soul_type::{soul_type::SoulType, type_modifiers::TypeModifiers}}, tokenizer::token::TokenIterator};
use super::argument_info::ArgumentInfo;

pub struct FunctionArguments {
    pub args: Vec<ArgumentInfo>,
    pub options: Vec<ArgumentInfo>,
} 
impl FunctionArguments{
    pub fn new_empty() -> Self {
        FunctionArguments { args: Vec::new(), options: Vec::new() }
    }

    pub fn push(&mut self, arg: ArgumentInfo) {
        if arg.default_value.is_none() {
            self.args.push(arg);
        } 
        else {
            self.options.push(arg);
        }
    }
}

struct ArgInfo {
    name: String,
    is_mutable: bool,

    soul_type: SoulType,
    default_value: Option<IExpression>,
}
impl ArgInfo {
    pub fn new_empty() -> Self {
        ArgInfo { name: String::new(), is_mutable: false, soul_type: SoulType::new_empty(), default_value: None }
    }

    pub fn is_optional(&self) -> bool{
        self.default_value.is_some()
    }
}

struct StoreArgInfo {
    open_bracked_counter: i32, 
    argument_position: i32, 
    arg_info: ArgInfo, 
}
impl StoreArgInfo {
    pub fn reset(&mut self) {
        if !self.arg_info.is_optional() {
            self.argument_position += 1;
        }

        self.arg_info.name.clear();
        self.arg_info.is_mutable = false;
        self.arg_info.default_value = None;
        self.arg_info.soul_type.clear();
    }
    
    pub fn get_argument_info(&mut self, iter: &TokenIterator, function_modifiers: &Option<FunctionModifiers>) -> Result<ArgumentInfo> {
        
        if let Some(modifiers) = function_modifiers {
            
            if modifiers.contains(FunctionModifiers::Literal | FunctionModifiers::Const) {
                check_for_mutable_arg(iter, &self.arg_info)?;
            }
        }

        if self.arg_info.name.is_empty() {
            return Err(
                new_soul_error(
                    iter.current(), 
                    "no name given to argument"
                )
            );
        }

        if !self.arg_info.is_mutable {
            self.arg_info.soul_type.add_modifier(TypeModifiers::Const)
                .map_err(|msg| new_soul_error(iter.current(), format!("while trying to get argument from function\n{}", msg).as_str()))?;
        }

        Ok(ArgumentInfo {
            name: self.arg_info.name.clone(),
            arg_position: self.argument_position as u32,
            value_type: self.arg_info.soul_type.to_string(),
            default_value: self.arg_info.default_value.clone(),
            is_mutable: self.arg_info.is_mutable,
            can_be_multiple: false,
        })
    }
}

pub fn get_arguments(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    function_modifiers: Option<FunctionModifiers>,
    function_name: &str,
) -> Result<FunctionArguments> {
    let next_token = iter.peek()
        .ok_or(new_soul_error(iter.current(), "unexpeced end while trying to get arguments"))?;

    if next_token.text == ")" {
        return Ok(FunctionArguments::new_empty());
    }

    let mut store_arg_info = StoreArgInfo {
        open_bracked_counter: 1,
        argument_position: 0,
        arg_info: ArgInfo::new_empty(),
    };

    let mut arg_result = FunctionArguments::new_empty();

    while iter.next().is_some() {

        let type_result = SoulType::from_iterator(iter, &meta_data.type_meta_data, &context.current_generics);
        
        if iter.current().text == "," {
            let argument = store_arg_info.get_argument_info(iter, &function_modifiers)
                .map_err(|err| pass_err(iter, function_name, err))?; 

            arg_result.push(argument);
            store_arg_info.reset();
        }
        else if iter.current().text == "(" {
            store_arg_info.open_bracked_counter += 1;
        }
        else if iter.current().text == ")" {

            if store_arg_info.open_bracked_counter <= 1 {
                let argument = store_arg_info.get_argument_info(iter, &function_modifiers)
                    .map_err(|err| pass_err(iter, function_name, err))?; 

                arg_result.push(argument);

                let mut argument_position = 0;
                for arg in &mut arg_result.args {
                    arg.arg_position = argument_position;
                    argument_position += 1;
                }

                for arg in &mut arg_result.options {
                    arg.arg_position = argument_position;
                    argument_position += 1;
                }

                return Ok(arg_result);
            }
        }
        else if iter.current().text == "mut" {
            store_arg_info.arg_info.is_mutable = true;
        }
        else if iter.current().text == "=" {

            if iter.next().is_none() {
                break;
            }

            let default_value_i = iter.current_index();
            let should_be_type = Some(&store_arg_info.arg_info.soul_type);
            let expression = get_expression(iter, meta_data, &context, &should_be_type, &vec![",", ")"])
                .map_err(|err| pass_err(
                    iter, 
                    function_name, 
                    new_soul_error(
                        iter.current(), 
                        format!("while trying to get optional argument: '{}'\n{}", store_arg_info.arg_info.name, err.to_string()).as_str()
                    )
                ))?;
            
            iter.next_multiple(-1);

            let arg_type = &store_arg_info.arg_info.soul_type;
            if !expression.is_type.is_convertable(arg_type, iter.current(), &meta_data.type_meta_data, &context.current_generics) {

                let mut string_builder = String::new();
                for i in default_value_i..iter.current_index() {
                    string_builder.push_str(&iter[i].text);
                    string_builder.push(' ');
                }

                return Err(err_wrong_default_type(iter, string_builder, &expression.is_type, arg_type));
            }

            store_arg_info.arg_info.default_value = Some(expression.result.value);

        }
        else if let Ok(arg_type) = type_result {
            store_arg_info.arg_info.soul_type = arg_type;
        }
        else if iter.current().text == "this" {
            return Err(new_soul_error(iter.current(), "'this.' not yet implemented in get_arguments"));
            todo!()
        }
        else {
            if store_arg_info.arg_info.soul_type.is_empty() {
                return Err(new_soul_error(iter.current(), format!("name of argument: '{}' can not go before a type", iter.current().text).as_str()));
            }

            check_name(&iter.current().text)
                .map_err(|msg| pass_err(iter, function_name, new_soul_error(iter.current(), msg.as_str())))?;

            store_arg_info.arg_info.name = iter.current().text.clone();
        }
    }

    Err(err_arguments_out_of_bounds(iter, function_name))
}

fn check_for_mutable_arg(iter: &TokenIterator, arg_info: &ArgInfo) -> Result<()> {
    if arg_info.is_mutable {
        return Err(
            new_soul_error(
                iter.current(), 
                format!(" '{}' and '{}' function don't allow 'mut' arguments", FunctionModifiers::Literal.to_str(), FunctionModifiers::Const.to_str()).as_str()
            )
        );
    }

    if arg_info.soul_type.is_mut_ref() {
        return Err(
            new_soul_error(
                iter.current(), 
                format!(" '{}' and '{}' function don't allow mut ref arguments", FunctionModifiers::Literal.to_str(), FunctionModifiers::Const.to_str()).as_str()
            )
        );
    }

    Ok(())
}

fn err_wrong_default_type(
    iter: &mut TokenIterator, 
    string_builder: String, 
    is_type: &SoulType, 
    arg_type: &SoulType,
) -> Error {
    new_soul_error(
        iter.current(), 
        format!(
            "default type: '{}', which is type: '{}' is not convertable with argType: '{}'", 
            string_builder, 
            is_type.to_string(), 
            arg_type.to_string(),
        ).as_str(),
    )
}

fn err_arguments_out_of_bounds(iter: &mut TokenIterator, function_name: &str) -> Error {
    new_soul_error(iter.current(), format!("unexpeced end while trying to get arguments of function: {}", function_name).as_str())
}

fn pass_err(iter: &TokenIterator, name: &str, err: Error) -> Error {
    new_soul_error(iter.current(), format!("while trying to get arguments of function: {}\n{}", name, err.to_string()).as_str())
}


