use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

use crate::steps::step_interfaces::i_tokenizer::Token;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::literal::Literal, parser_response::FromTokenStream, scope::ScopeBuilder}, i_tokenizer::TokenStream};

impl FromTokenStream<Literal> for Literal {
    fn try_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Self>> {
        let begin_i = stream.current_index();

        let possible_result = inner_from_stream(stream, scopes);
        if possible_result.is_none() {
            stream.go_to_index(begin_i);
        }

        possible_result
    }
    
    fn from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal> {
        
        let begin_i = stream.current_index();

        let res = match inner_from_stream(stream, scopes) {
            Some(val) => val,
            None => Err(new_soul_error(SoulErrorKind::WrongType, stream.current_span(), "could not get literal")),
        };

        if res.is_err() {
            stream.go_to_index(begin_i);
        }

        res
    }
}

fn inner_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Literal>> {

    if stream.current_text() == "[" || stream.current_text() == "[]" {
        let array = get_array(stream, scopes);
        if array.is_some() {

            return array;
        }

        return None;
    }
    else if stream.current_text() == "(" || stream.current_text() == "()" {
        return pick_tuple(stream, scopes);
    }
    if stream.current_text().starts_with("\"") {
        let string = get_string(stream.current_text());
        if string.is_some() {
            return string;
        }
    }

    let number = get_number(stream.current());
    if number.is_some() {
        return number;
    }

    let boolean = get_bool(stream.current_text());
    if boolean.is_some() {
        return boolean;
    }

    let char = get_char(stream.current_text());
    if char.is_some() {
        return char;
    }

    None
}

#[inline]
fn pick_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Literal>> {
    
    if stream.peek_multiple(2).is_some_and(|token| token.text == ":") {
        
        let named_tuple = get_named_tuple(stream, scopes);
        if named_tuple.is_some() {
            return named_tuple;
        }
    }

    let tuple = get_tuple(stream, scopes);
    if tuple.is_some() {
        return tuple;
    }

    return None;
}

fn get_string(text: &str) -> Option<Result<Literal>> {
    if text.len() >= 2 && text.starts_with('\"') && text.ends_with('\"') {
        Some(Ok(Literal::Str(text[1..text.len()-1].into())))
    }
    else {
        None
    }
}

fn get_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Literal>> {
    const TUPLE_START: &str = "(";
    const TUPLE_END: &str = ")";
    const TUPLE_EMPTY: &str = "()";

    if stream.current_text() == TUPLE_EMPTY {
        return Some(Ok(Literal::new_tuple(Vec::new())));
    }
    else if stream.current_text() != TUPLE_START {
        return None;
    }
    else if stream.peek().is_some_and(|token| token.text == TUPLE_END) {
        return Some(Ok(Literal::new_tuple(Vec::new())));
    }

    let mut tuples = Vec::new();
    while stream.next().is_some() {

        if stream.current_text() == TUPLE_END {
            return Some(Ok(Literal::new_tuple(tuples)));
        }

        if stream.peek().is_some_and(|token| token.text == ":") {

            if tuples.is_empty() {
                return None;
            }
            
            return Some(Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
            )));
        }

        let literal = match Literal::try_from_stream(stream, scopes) {
            Some(val) => val.map_err(|err| return Some(Err::<Literal, SoulError>(err))).unwrap(),
            None => return None,
        };

        tuples.push(literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == TUPLE_END {
            return Some(Ok(Literal::new_tuple(tuples)));
        }
        else if stream.current_text() != "," {
            return None;
        }

    }

    None
}

fn get_named_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Literal>> {
    const TUPLE_START: &str = "(";
    const TUPLE_END: &str = ")";

    if stream.current_text() != TUPLE_START {
        return None;
    }


    let mut tuples = BTreeMap::new();
    while stream.next().is_some() {

        if stream.current_text() == TUPLE_END {
            return Some(Ok(Literal::new_named_tuple(tuples)));
        }

        if stream.peek().is_some_and(|token| token.text != ":") {

            if tuples.is_empty() {
                return None;
            }
            
            return Some(Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
            )));
        }

        let name_index = stream.current_index();

        if stream.next_multiple(2).is_none() {
            break;
        }

        let literal = match Literal::try_from_stream(stream, scopes) {
            Some(val) => val.map_err(|err| return Some(Err::<Literal, SoulError>(err))).unwrap(),
            None => return None,
        };

        tuples.insert(Ident(stream[name_index].text.clone()), literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == TUPLE_END {
            return Some(Ok(Literal::new_named_tuple(tuples)));
        }
        else if stream.current_text() != "," {
            return None;
        }

    }

    None
}

fn get_array(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Literal>> {
    const ARRAY_START: &str = "[";
    const ARRAY_END: &str = "]";
    const ARRAY_EMPTY: &str = "[]";

    if stream.current_text() == ARRAY_EMPTY {
        return Some(Literal::new_array(Vec::new(), &stream.current_span()));
    }
    else if stream.current_text() != ARRAY_START {
        return None;
    }
    else if stream.peek().is_some_and(|token| token.text == ARRAY_END) {
        return Some(Literal::new_array(Vec::new(), &stream.current_span()));
    }

    let mut literals = Vec::new();
    while stream.next().is_some() {

        let literal = match Literal::try_from_stream(stream, scopes) {
            Some(val) => val.map_err(|err| return Some(Err::<Literal, SoulError>(err))).unwrap(),
            None => return None,
        };

        if literals.last().is_some_and(|lit| !literal.are_compatible(lit)) {
            return Some(Err(new_soul_error(
                SoulErrorKind::WrongType, 
                stream.current_span(), 
                format!("in literal array element '{}' and '{}' are not compatible (should ',')", literals.last().unwrap().value_to_string(), literal.value_to_string()))
            ))
        }

        literals.push(literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == ARRAY_END {
            return Some(Literal::new_array(literals, &stream.current_span()));
        }
        else if stream.current_text() != "," {
            return Some(Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token '{}' is not allowed in literal array", stream.current_text())
            )));
        }
    }

    None
}

fn get_char(text: &str) -> Option<Result<Literal>> {
    if text.len() == 3 && text.starts_with('\'') && text.ends_with('\'') {
        Some(Ok(Literal::Char(text.chars().nth(1).unwrap())))
    }
    else {
        None
    }
}

fn get_bool(text: &str) -> Option<Result<Literal>> {
    if text == "true" || text == "false" {
        Some(Ok(Literal::Bool(text=="true")))
    }
    else {
        None
    }
}

fn get_number(token: &Token) -> Option<Result<Literal>> {
    const BINARY: u32 = 2;
    const HEXIDECIMAL: u32 = 16;

    if token.text.is_empty() {
        return None;
    }

    // handle negative for hex/bin values (for decimal not needed)
    let is_neg = token.text.chars().nth(0).unwrap() == '-';

    if token.text.starts_with("0x") || token.text.starts_with("0x") {
        let hex_digits = &token.text[2..];
        
        return Some(
            u64::from_str_radix(hex_digits, HEXIDECIMAL)
                .map(|val| if is_neg {Literal::Int((val as i64)*-1)} else {Literal::Uint(val)})
                .map_err(|child| new_soul_error(
                    SoulErrorKind::WrongType, 
                    token.span, 
                    format!("while trying to parse hexidecimal number\n{}", child.to_string())
                ))
        );
    }
    else if token.text.starts_with("0b") {
        let bits = &token.text[2..];
        
        return Some(
            u64::from_str_radix(bits, BINARY)
                .map(|val| if is_neg {Literal::Int((val as i64)*-1)} else {Literal::Uint(val)})
                .map_err(|child| new_soul_error(
                    SoulErrorKind::WrongType, 
                    token.span, 
                    format!("while trying to parse binary number\n{}", child.to_string())
                ))
        );
    }

    let int_res = token.text.parse::<i64>()
        .map(|val| Literal::Int(val));

    let float_res = token.text.parse::<f64>()
        .map(|val| Literal::Float(OrderedFloat(val)));
    
    if let Ok(int) = int_res {
        return Some(Ok(int));
    }
    else if let Ok(float) = float_res {
        return Some(Ok(float));
    }

    None
}































