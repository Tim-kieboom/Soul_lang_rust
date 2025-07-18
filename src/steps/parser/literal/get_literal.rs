use std::collections::BTreeMap;
use ordered_float::OrderedFloat;

use crate::steps::step_interfaces::i_tokenizer::Token;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::literal::Literal, parser_response::FromTokenStream, scope::ScopeBuilder}, i_tokenizer::TokenStream};

impl FromTokenStream<Literal> for Literal {
    fn try_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Option<Result<Self>> {
        let begin_i = stream.current_index();

        let possible_result = inner_from_stream(stream, scopes);
        
        
        if possible_result.is_err() {
            stream.go_to_index(begin_i);
            None
        }
        else {
            Some(possible_result.unwrap())
        }
    }
    
    fn from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal> {
        
        let begin_i = stream.current_index();

        let res = match inner_from_stream(stream, scopes) {
            Ok(val) => val,
            Err(err) => Err(pass_soul_error(SoulErrorKind::WrongType, stream.current_span(), "could not get literal", err)),
        };

        if res.is_err() {
            stream.go_to_index(begin_i);
        }

        res
    }
}

fn inner_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Result<Literal>> {

    if stream.current_text() == "[" || stream.current_text() == "[]" {
        return get_array(stream, scopes);
    }
    else if stream.current_text() == "(" || stream.current_text() == "()" {
        return pick_tuple(stream, scopes);
    }   
    else if stream.current_text().starts_with("\"") {
        let string = get_string(stream.current_text());
        if string.is_some() {
            return Ok(string.unwrap());
        }

        return Err(new_soul_error(SoulErrorKind::InvalidStringFormat, stream.current_span(), "token starts with \" but is not string"))
    }

    let number = get_number(stream.current());
    if number.is_ok() {
        return number;
    }

    let boolean = get_bool(stream.current_text());
    if boolean.is_ok() {
        return boolean;
    }

    let char = get_char(stream.current_text());
    if char.is_ok() {
        return char;
    }

    Err(return_best_error(stream.current(), number, char))
}

fn return_best_error(token: &Token, number: Result<Result<Literal>>, char: Result<Result<Literal>>) -> SoulError {
    if token.text.starts_with("'") {
        char.unwrap_err()
    }
    else {
        number.unwrap_err()
    }
}

#[inline]
fn pick_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Result<Literal>> {
    let mut named_tuple = None;
    if stream.peek_multiple(2).is_some_and(|token| token.text == ":") {
        
        named_tuple = Some(get_named_tuple(stream, scopes));
        if named_tuple.as_ref().unwrap().is_ok() {
            return named_tuple.unwrap();
        }
    }

    let tuple = get_tuple(stream, scopes);
    if tuple.is_ok() {
        return tuple;
    }

    if let Some(tu) = named_tuple {
        return Err(tu.unwrap_err());
    }
    else {
        return Err(tuple.unwrap_err())
    }
}

fn get_string(text: &str) -> Option<Result<Literal>> {
    if text.len() >= 2 && text.starts_with('\"') && text.ends_with('\"') {
        Some(Ok(Literal::Str(text[1..text.len()-1].into())))
    }
    else {
        None
    }
}

fn get_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Result<Literal>> {
    const TUPLE_START: &str = "(";
    const TUPLE_END: &str = ")";
    const TUPLE_EMPTY: &str = "()";

    if stream.current_text() == TUPLE_EMPTY {
        return Ok(Ok(Literal::new_tuple(Vec::new())));
    }
    else if stream.current_text() != TUPLE_START {
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "tuple does not start with '('"));
    }
    else if stream.peek().is_some_and(|token| token.text == TUPLE_END) {
        return Ok(Ok(Literal::new_tuple(Vec::new())));
    }

    let mut tuples = Vec::new();
    while stream.next().is_some() {

        if stream.current_text() == TUPLE_END {
            return Ok(Ok(Literal::new_tuple(tuples)));
        }

        if stream.peek().is_some_and(|token| token.text == ":") {

            if tuples.is_empty() {
                return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "internal THIS SHOULD NOT BE THROWN get_tuple() condition: tuples.is_empty()"));
            }
            
            return Ok(Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
            )));
        }

        let begin_i = stream.current_index();
        let literal = match inner_from_stream(stream, scopes) {
            Ok(val) => val.map_err(|err| return Some(Err::<Literal, SoulError>(err))).unwrap(),
            Err(err) => {
                stream.go_to_index(begin_i);
                return Err(pass_soul_error(SoulErrorKind::InvalidInContext, stream[begin_i].span, "while getting element in tuple", err));
            }
        };

        tuples.push(literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == TUPLE_END {
            return Ok(Ok(Literal::new_tuple(tuples)));
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("tuple '{}', should be ','", stream.current_text())));
        }

    }

    return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing tuple"));

}

fn get_named_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Result<Literal>> {
    const TUPLE_START: &str = "(";
    const TUPLE_END: &str = ")";

    if stream.current_text() != TUPLE_START {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "named tuple does not start with '('"));
    }


    let mut tuples = BTreeMap::new();
    while stream.next().is_some() {

        if stream.current_text() == TUPLE_END {
            return Ok(Ok(Literal::new_named_tuple(tuples)));
        }

        if stream.peek().is_some_and(|token| token.text != ":") {

            if tuples.is_empty() {
                return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "internal THIS SHOULD NOT BE THROWN get_named_tuple() condition: tuples.is_empty()"));
            }
            
            return Ok(Err(new_soul_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
            )));
        }

        let name_index = stream.current_index();

        if stream.next_multiple(2).is_none() {
            break;
        }

        let begin_i = stream.current_index();
        let literal = match inner_from_stream(stream, scopes) {
            Ok(val) => val.map_err(|err| return Some(Err::<Literal, SoulError>(err))).unwrap(),
            Err(err) => {
                stream.go_to_index(begin_i);
                return Err(pass_soul_error(SoulErrorKind::InvalidInContext, stream[begin_i].span, "while getting element in named tuple", err));
            }
        };

        tuples.insert(Ident(stream[name_index].text.clone()), literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == TUPLE_END {
            return Ok(Ok(Literal::new_named_tuple(tuples)));
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("tuple '{}', should be ','", stream.current_text())));
        }

    }

    return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing tuple"));
}

fn get_array(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Result<Literal>> {
    const ARRAY_START: &str = "[";
    const ARRAY_END: &str = "]";
    const ARRAY_EMPTY: &str = "[]";

    if stream.current_text() == ARRAY_EMPTY {
        return Ok(Literal::new_array(Vec::new(), &stream.current_span()));
    }
    else if stream.current_text() != ARRAY_START {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), "array should start with '['"));
    }
    else if stream.peek().is_some_and(|token| token.text == ARRAY_END) {
        return Ok(Literal::new_array(Vec::new(), &stream.current_span()));
    }

    let mut literals = Vec::new();
    while stream.next().is_some() {

        let begin_i = stream.current_index();
        let literal = match inner_from_stream(stream, scopes) {
            Ok(val) => val.map_err(|err| return Some(Err::<Literal, SoulError>(err))).unwrap(),
            Err(err) => {
                stream.go_to_index(begin_i);
                return Err(pass_soul_error(SoulErrorKind::InvalidInContext, stream[begin_i].span, "while getting element in named array", err));
            }
        };


        if literals.last().is_some_and(|lit| !literal.are_compatible(lit)) {
            return Ok(Err(new_soul_error(
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
            return Ok(Literal::new_array(literals, &stream.current_span()));
        }
        else if stream.current_text() != "," {
            return Ok(Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token '{}' is not allowed in literal array", stream.current_text())
            )));
        }
    }

    return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing array"));
}

fn get_char(token: &Token) -> Result<Result<Literal>> {
    if token.text.len() != 3 {
        Err(new_soul_error(SoulErrorKind::InvalidInContext, token.span, format!("char literal can only have 1 char {} hav {} chars", token.text, token.text.len()) ))
    }
    else if token.text.starts_with('\'') && token.text.ends_with('\'') {
        Ok(Ok(Literal::Char(token.text.chars().nth(1).unwrap())))
    }
    else {
        Err(new_soul_error(SoulErrorKind::InvalidInContext, token.span, format!("{} starts with ' but does not end with it", token.text) ))
    }
}

fn get_bool(token: &Token) -> Result<Result<Literal>> {
    if token.text == "true" || token.text == "false" {
        Ok(Ok(Literal::Bool(token.text=="true")))
    }
    else {
        Err(new_soul_error(SoulErrorKind::UnexpectedToken, token.span, format!("'{}' is not true or false", token.text)))
    }
}

fn get_number(token: &Token) -> Result<Result<Literal>> {
    const BINARY: u32 = 2;
    const HEXIDECIMAL: u32 = 16;

    if token.text.is_empty() {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, span, msg));
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































