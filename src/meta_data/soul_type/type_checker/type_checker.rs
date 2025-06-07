use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulError};
use crate::{abstract_styntax_tree::abstract_styntax_tree::{IExpression, IVariable}, meta_data::{current_context::current_context::CurrentGenerics, function::function_modifiers::FunctionModifiers, meta_data::MetaData, soul_names::{NamesTypeWrapper, SOUL_NAMES}, soul_type::{primitive_types::PrimitiveType, soul_type::SoulType, type_wrappers::TypeWrappers}, type_meta_data::TypeMetaData}, tokenizer::token::{Token, TokenIterator}};

pub fn get_primitive_type_from_literal(literal: &str) -> PrimitiveType {
    if literal.is_empty() {
        return PrimitiveType::Invalid;
    }

    let number_type = get_number_from_literal(literal);

    let is_bool = literal == "true" || literal == "false";
    let is_char = literal.len() > 2 && literal.chars().nth(0).unwrap() == '\'' && literal.chars().nth_back(1).unwrap() == '\'';
    let is_number = number_type != PrimitiveType::Invalid;

    if is_bool {
        PrimitiveType::Bool
    } else if is_char {
        PrimitiveType::Char
    } else if is_number {
        number_type
    } else {
        PrimitiveType::Invalid
    }
}



pub fn check_convert_to_ref(
    iter: &TokenIterator, 
    first_expression: &IExpression, 
    ref_wrap: &TypeWrappers, 
    token: &Token,
    meta_data: &MetaData, 
    generics: &mut CurrentGenerics,
) -> Result<()> {
    let mut expression_stack = Vec::with_capacity(2);
    expression_stack.push(first_expression);

    assert!(ref_wrap.is_any_ref(), "Internal error: in check_convert_to_ref() ref_wrap is not ref");

    while let Some(expression) = expression_stack.pop() {

        match expression {
            IExpression::BinairyExpression { type_name, .. } => {
                    if ref_wrap == &TypeWrappers::ConstRef {
                        continue;
                    }

                    if is_type_name_literal(type_name, token, meta_data, generics)? {
                        return Err(err_literal_mut_refs(iter, format!("{:?}", expression).as_str()))
                    }
                },
            IExpression::Literal { value, type_name: _ } => {
                    if ref_wrap == &TypeWrappers::ConstRef {
                        continue;
                    }
            
                    return Err(err_literal_mut_refs(&iter, &value));
                },
            IExpression::IVariable { this } => {
                    if ref_wrap == &TypeWrappers::ConstRef {
                        continue;
                    }

                    if is_ivariable_literal(this, token, meta_data, generics)? {
                        return Err(err_literal_mut_refs(iter, format!("{:?}", this).as_str()))
                    }
                },
            IExpression::Increment { .. } => {
                    return Err(new_soul_error(token, "you can not refrence an increment expression"));
                },
            IExpression::FunctionCall { function_info, .. } => {
                    if let Some(return_type) = &function_info.return_type {
                        if ref_wrap == &TypeWrappers::ConstRef {
                            continue;
                        }

                        if is_type_name_literal(&return_type, token, meta_data, generics)? {
                            return Err(err_literal_mut_refs(iter, format!("{:?}", expression).as_str()))
                        }
                    }
                    else {
                        return Err(new_soul_error(iter.current(), "can not ref functionCall with no return type"));
                    }                    
                },
            IExpression::ConstRef { expression } => expression_stack.push(&expression),
            IExpression::MutRef { expression } => expression_stack.push(&expression),
            IExpression::DeRef { expression } => expression_stack.push(&expression),
            IExpression::EmptyExpression() => (),
        };
    }

    Ok(())
}

pub fn is_expression_literal(
    first_expression: &IExpression, 
    token: &Token, 
    meta_data: &MetaData, 
    generics: &mut CurrentGenerics,
) -> Result<bool> {
    let mut expression_stack = Vec::with_capacity(2);
    expression_stack.push(first_expression);

    while let Some(expression) = expression_stack.pop() {

        match expression {
            IExpression::BinairyExpression { type_name, .. } => {
                    return is_type_name_literal(type_name, token, meta_data, generics);
                },
            IExpression::Literal { type_name, .. } => {
                    return is_type_name_literal(type_name, token, meta_data, generics);
                },
            IExpression::EmptyExpression() => {
                    return Ok(true);
                },
            IExpression::IVariable { this } => {
                    return is_ivariable_literal(this, token, meta_data, generics);
                },
            IExpression::Increment { variable, .. } => {
                    return is_ivariable_literal(variable, token, meta_data, generics);
                },
            IExpression::FunctionCall { function_info, .. } => {
                    return Ok(function_info.modifiers.contains(FunctionModifiers::Literal))
                },
            IExpression::ConstRef { expression } => expression_stack.push(&expression),
            IExpression::MutRef { expression } => expression_stack.push(&expression),
            IExpression::DeRef { expression } => expression_stack.push(&expression),
        };
    }

    Err(new_soul_error(token, "Internal error: could not find type from expression in is_expression_literal()"))
}

pub fn is_ivariable_literal(
    var: &IVariable, 
    token: &Token, 
    meta_data: &MetaData, 
    generics: &mut CurrentGenerics,
) -> Result<bool> {
    match var {
        IVariable::Variable{ name: _, type_name } => return is_type_name_literal(&type_name, token, meta_data, generics),
        // IVariable::MemberExpression{ parent, .. } => {

        //     if let IVariable::Variable{ name: _, type_name } = &**parent {
        //         return is_type_name_literal(&type_name, token, meta_data, generics);
        //     }
        //     else {
        //         return Err(new_soul_error(token, format!("").as_str()));
        //     }
        // },
    }
}

pub fn is_type_name_literal(
    type_name: &str, 
    token: &Token, 
    meta_data: &MetaData, 
    generics: &mut CurrentGenerics,
) -> Result<bool> {
    let soul_type = SoulType::from_stringed_type(&type_name, token, &meta_data.type_meta_data, generics)?;
    
    Ok(soul_type.is_literal())
}

pub fn duck_type_equals(
    a: &SoulType,
    b: &SoulType,
    type_meta_data: &TypeMetaData,
) -> bool {
    a.to_primitive_type(type_meta_data).to_duck_type() == b.to_primitive_type(type_meta_data).to_duck_type()
}

fn err_literal_mut_refs(iter: &TokenIterator, value: &str)-> SoulError {
    new_soul_error(
        iter.current(), 
        format!(
            "while trying to get ref expression '{}{}'\nis a literal type so can not be mutRef'ed (remove '{}' use '{}' instead)", 
            SOUL_NAMES.get_name(NamesTypeWrapper::MutRef), 
            value, 
            SOUL_NAMES.get_name(NamesTypeWrapper::MutRef), 
            SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef)
        ).as_str(),
    )
}

fn get_number_from_literal(s: &str) -> PrimitiveType {
    const BINARY: u32 = 2;
    const HEXIDECIMAL: u32 = 16;

    if s.is_empty() {
        return PrimitiveType::Invalid;
    }

    // handle negative binary/hex like -0b00000001
    let num_minus;
    if s.chars().nth(0).unwrap() == '-' {
        num_minus = 1;
    }
    else {
        num_minus = 0;
    }

    if s.len() > 2 && (&s[num_minus..2+num_minus] == "0x" || &s[num_minus..2+num_minus] == "0b") {

        let mut num_bytes = s[2+num_minus..].len();
        if &s[num_minus..2+num_minus] == "0b" {
            num_bytes /= 8;

            if u64::from_str_radix(&s[2+num_minus..], BINARY).is_err() {
                return PrimitiveType::Invalid;
            }
        }
        else {
            if u64::from_str_radix(&s[2+num_minus..], HEXIDECIMAL).is_err() {
                return PrimitiveType::Invalid;
            }
        }

        return match num_bytes {
            var if var <= 1 => {
                if num_minus == 0 {
                    PrimitiveType::U8
                }
                else {
                    PrimitiveType::I8
                }
            },
            var if var <= 2 => {
                if num_minus == 0 {
                    PrimitiveType::U16
                }
                else {
                    PrimitiveType::I16
                }
            },
            var if var <= 4 => {
                if num_minus == 0 {
                    PrimitiveType::U32
                }
                else {
                    PrimitiveType::I32
                }
            },
            var if var <= 8 => {
                if num_minus == 0 {
                    PrimitiveType::U64
                }
                else {
                    PrimitiveType::I64
                }
            },
            _ => PrimitiveType::Invalid,
        }
    }

    if s.parse::<i64>().is_ok() {
        return PrimitiveType::UntypedInt;
    }

    if s.parse::<f64>().is_ok() {
        return PrimitiveType::UntypedFloat;
    }

    PrimitiveType::Invalid
}








