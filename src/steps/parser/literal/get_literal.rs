use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use crate::steps::step_interfaces::i_tokenizer::Token;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::errors::soul_error::{pass_soul_error, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::parser_response::{new_from_stream_error, FromStreamError, FromStreamErrorKind};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::literal::Literal, parser_response::FromTokenStream}, i_tokenizer::TokenStream};

impl FromTokenStream<Literal> for Literal {
    fn try_from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Self>, SoulError> {
        let begin_i = stream.current_index();

        match inner_from_stream(stream, scopes) {
            Ok(value) => Ok(Some(value)),
            Err(err) => match err.kind {
                FromStreamErrorKind::IsOfType => Err(err.err),
                FromStreamErrorKind::IsNotOfType => {
                    stream.go_to_index(begin_i);
                    Ok(None)
                }
            }
        }
    }
    
    fn from_stream(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Literal, SoulError> {
        
        let begin_i = stream.current_index();

        let res = match inner_from_stream(stream, scopes) {
            Ok(val) => Ok(val),
            Err(err) => Err(pass_soul_error(SoulErrorKind::WrongType, stream.current_span(), "could not get literal", err.err)),
        };

        if res.is_err() {
            stream.go_to_index(begin_i);
        }

        res
    }
}

fn inner_from_stream(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal, FromStreamError> {

    if stream.current_text() == "[" || stream.current_text() == "[]" {
        return get_array(stream, scopes);
    }
    else if stream.current_text() == "(" || stream.current_text() == "()" {
        return pick_tuple(stream, scopes);
    }   
    else if stream.current_text().starts_with("\"") {
        let string = get_string(stream.current_text());
        if string.is_some() {
            return string.unwrap();
        }

        return Err(new_from_stream_error(
            SoulErrorKind::InvalidStringFormat, 
            stream.current_span(), 
            "token starts with \" but is not string", 
            FromStreamErrorKind::IsNotOfType
        ))
    }

    let number = get_number(stream.current());
    if number.is_ok() {
        return number;
    }

    let boolean = get_bool(stream.current());
    if boolean.is_ok() {
        return boolean;
    }

    let char = get_char(stream.current());
    if char.is_ok() {
        return char;
    }

    Err(return_best_error(stream.current(), number, char))
}

fn return_best_error(token: &Token, number: Result<Literal, FromStreamError>, char: Result<Literal, FromStreamError>) -> FromStreamError {
    if token.text.starts_with("'") {
        char.unwrap_err()
    }
    else {
        number.unwrap_err()
    }
}

#[inline]
fn pick_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal, FromStreamError> {
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

fn get_string(text: &str) -> Option<Result<Literal, FromStreamError>> {
    if text.len() >= 2 && text.starts_with('\"') && text.ends_with('\"') {
        Some(Ok(Literal::Str(text[1..text.len()-1].into())))
    }
    else {
        None
    }
}

fn get_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal, FromStreamError> {
    const TUPLE_START: &str = "(";
    const TUPLE_END: &str = ")";
    const TUPLE_EMPTY: &str = "()";

    if stream.current_text() == TUPLE_EMPTY {
        return Ok(Literal::new_tuple(Vec::new()));
    }
    else if stream.current_text() != TUPLE_START {
        return Err(new_from_stream_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "tuple does not start with '('", FromStreamErrorKind::IsNotOfType));
    }
    else if stream.peek().is_some_and(|token| token.text == TUPLE_END) {
        return Ok(Literal::new_tuple(Vec::new()));
    }

    let mut tuples = Vec::new();
    while stream.next().is_some() {

        if stream.current_text() == TUPLE_END {
            return Ok(Literal::new_tuple(tuples));
        }

        if stream.peek().is_some_and(|token| token.text == ":") {

            if tuples.is_empty() {
                return Err(new_from_stream_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "internal THIS SHOULD NOT BE THROWN get_tuple() condition: tuples.is_empty()", FromStreamErrorKind::IsNotOfType));
            }
            
            return Err(new_from_stream_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
                FromStreamErrorKind::IsOfType
            ));
        }

        let begin_i = stream.current_index();
        let literal = match inner_from_stream(stream, scopes) {
            Ok(val) => val,
            Err(err) => {
                if err.kind == FromStreamErrorKind::IsNotOfType {
                    stream.go_to_index(begin_i);
                }
                
                return Err(err);
            }
        };

        tuples.push(literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == TUPLE_END {
            return Ok(Literal::new_tuple(tuples));
        }
        else if stream.current_text() != "," {
            return Err(new_from_stream_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("tuple '{}', should be ','", stream.current_text()), FromStreamErrorKind::IsNotOfType));
        }

    }

    return Err(new_from_stream_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing tuple", FromStreamErrorKind::IsNotOfType));

}

fn get_named_tuple(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal, FromStreamError> {
    const TUPLE_START: &str = "(";
    const TUPLE_END: &str = ")";

    if stream.current_text() != TUPLE_START {
        return Err(new_from_stream_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "named tuple does not start with '('", FromStreamErrorKind::IsNotOfType));
    }


    let mut tuples = BTreeMap::new();
    while stream.next().is_some() {

        if stream.current_text() == TUPLE_END {
            return Ok(Literal::new_named_tuple(tuples));
        }

        if stream.peek().is_some_and(|token| token.text != ":") {

            if tuples.is_empty() {
                return Err(new_from_stream_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "internal THIS SHOULD NOT BE THROWN get_named_tuple() condition: tuples.is_empty()", FromStreamErrorKind::IsNotOfType));
            }
            
            return Err(new_from_stream_error(
                SoulErrorKind::InvalidType, 
                stream.current_span(), 
                "can not have a named tuple element (e.g. (field: 1, fiedl2: 1)) and unnamed tuple element (e.g. (1, 2)) in the same tuple",
                FromStreamErrorKind::IsOfType,
            ));
        }

        let name_index = stream.current_index();

        if stream.next_multiple(2).is_none() {
            break;
        }

        let begin_i = stream.current_index();
        let literal = match inner_from_stream(stream, scopes) {
            Ok(val) => val,
            Err(err) => {
                if err.kind == FromStreamErrorKind::IsNotOfType {
                    stream.go_to_index(begin_i);
                }
                
                return Err(err);
            }
        };

        tuples.insert(Ident(stream[name_index].text.clone()), literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == TUPLE_END {
            return Ok(Literal::new_named_tuple(tuples));
        }
        else if stream.current_text() != "," {
            return Err(new_from_stream_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), format!("tuple '{}', should be ','", stream.current_text()), FromStreamErrorKind::IsNotOfType));
        }

    }

    return Err(new_from_stream_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing tuple", FromStreamErrorKind::IsNotOfType));
}

fn get_array(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Literal, FromStreamError> {
    const ARRAY_START: &str = "[";
    const ARRAY_END: &str = "]";
    const ARRAY_EMPTY: &str = "[]";
    
    fn err_out_of_bounds(stream: &TokenStream) -> FromStreamError {
        new_from_stream_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing array", FromStreamErrorKind::IsNotOfType)
    }

    if stream.current_text() == ARRAY_EMPTY || stream.peek().is_some_and(|token| token.text == ARRAY_END) {
        return Literal::new_array(Vec::new(), &stream.current_span())
            .map_err(|err| FromStreamError{ err, kind: FromStreamErrorKind::IsOfType});
    }

    if stream.current_text() != ARRAY_START {
        return Err(new_from_stream_error(SoulErrorKind::UnexpectedToken, stream.current_span(), "array should start with '['", FromStreamErrorKind::IsNotOfType));
    }

    if stream.peek_is("for") {

        return Literal::new_array(get_array_filler(stream, scopes)?, &stream.current_span())
            .map_err(|err| FromStreamError{err, kind: FromStreamErrorKind::IsOfType});
    }

    let mut literals = Vec::new();
    while stream.next().is_some() {

        let literal = inner_from_stream(stream, scopes)?;

        if literals.last().is_some_and(|lit| !literal.are_compatible(lit)) {
            return Err(new_from_stream_error(
                SoulErrorKind::WrongType, 
                stream.current_span(), 
                format!("in literal array element '{}' and '{}' are not compatible (should ',')", literals.last().unwrap().value_to_string(), literal.value_to_string()),
                FromStreamErrorKind::IsOfType,
            ))
        }

        literals.push(literal);

        if stream.next().is_none() {
            break;
        }

        if stream.current_text() == ARRAY_END {
            return Literal::new_array(literals, &stream.current_span())
                .map_err(|err| FromStreamError{err, kind: FromStreamErrorKind::IsOfType});
        }
        else if stream.current_text() != "," {
            return Err(new_from_stream_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("token '{}' is not allowed in literal array", stream.current_text()),
                FromStreamErrorKind::IsOfType,
            ));
        }
    }

    return Err(err_out_of_bounds(stream));
}

fn get_array_filler(stream: &mut TokenStream, scopes: &ScopeBuilder) -> Result<Vec<Literal>, FromStreamError> {
    fn err_out_of_bounds(stream: &TokenStream) -> FromStreamError {
        new_from_stream_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing array", FromStreamErrorKind::IsNotOfType)
    }
    
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.peek_is("in") {
        return Err(new_from_stream_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            "Literal arrayFiller can not have 'in'", 
            FromStreamErrorKind::IsNotOfType,
        ))
    }

    let literal = inner_from_stream(stream, scopes)?;
    let amount = match literal {
        Literal::Int(num) => {
            if num < 0 {
                return Err(new_from_stream_error(
                    SoulErrorKind::WrongType, 
                    stream.current_span(), 
                    "Literal ArrayFillers element length can not be negative", 
                    FromStreamErrorKind::IsOfType,
                ));
            }
            num as u64
        },
        Literal::Uint(num) => num as u64,
        _ => return Err(new_from_stream_error(
            SoulErrorKind::WrongType, 
            stream.current_span(), 
            format!("Literal ArrayFillers element length has to be interger or unsigned interger is '{}'", literal.get_literal_type().type_to_string()), 
            FromStreamErrorKind::IsOfType,
        ))
    };
    
    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream));
    }
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.current_text() != "=>" {
        return Err(new_from_stream_error(
            SoulErrorKind::WrongType, 
            stream.current_span(), 
            format!("token: '{}' should be '=>'", stream.current_text(), ), 
            FromStreamErrorKind::IsOfType,
        ))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let literal = inner_from_stream(stream, scopes)?;
    Ok(vec![literal; amount as usize])
}

fn get_char(token: &Token) -> Result<Literal, FromStreamError> {
    if token.text.len() != 3 {
        Err(new_from_stream_error(SoulErrorKind::InvalidInContext, token.span, format!("char literal can only have 1 char {} hav {} chars", token.text, token.text.len()), FromStreamErrorKind::IsNotOfType))
    }
    else if token.text.starts_with('\'') && token.text.ends_with('\'') {
        Ok(Literal::Char(token.text.chars().nth(1).unwrap()))
    }
    else {
        Err(new_from_stream_error(SoulErrorKind::InvalidInContext, token.span, format!("{} starts with ' but does not end with it", token.text), FromStreamErrorKind::IsNotOfType))
    }
}

fn get_bool(token: &Token) -> Result<Literal, FromStreamError> {
    if token.text == "true" || token.text == "false" {
        Ok(Literal::Bool(token.text=="true"))
    }
    else {
        Err(new_from_stream_error(SoulErrorKind::UnexpectedToken, token.span, format!("'{}' is not true or false", token.text), FromStreamErrorKind::IsNotOfType))
    }
}

fn get_number(token: &Token) -> Result<Literal, FromStreamError> {
    const BINARY: u32 = 2;
    const HEXIDECIMAL: u32 = 16;

    if token.text.is_empty() {
        return Err(new_from_stream_error(SoulErrorKind::UnexpectedToken, token.span, "trying to get literal number but token is empty", FromStreamErrorKind::IsNotOfType));
    }

    // handle negative for hex/bin values (for decimal not needed)
    let is_neg = token.text.chars().nth(0).unwrap() == '-';

    if token.text.starts_with("0x") || token.text.starts_with("0x") {
        let hex_digits = &token.text[2..];
        
        return u64::from_str_radix(hex_digits, HEXIDECIMAL)
            .map(|val| if is_neg {Literal::Int((val as i64)*-1)} else {Literal::Uint(val)})
            .map_err(|child| new_from_stream_error(
                SoulErrorKind::WrongType, 
                token.span, 
                format!("while trying to parse hexidecimal number\n{}", child.to_string()),
                FromStreamErrorKind::IsOfType,
            ));
    }
    else if token.text.starts_with("0b") {
        let bits = &token.text[2..];
        
        return u64::from_str_radix(bits, BINARY)
            .map(|val| if is_neg {Literal::Int((val as i64)*-1)} else {Literal::Uint(val)})
            .map_err(|child| new_from_stream_error(
                SoulErrorKind::WrongType, 
                token.span, 
                format!("while trying to parse binary number\n{}", child.to_string()),
                FromStreamErrorKind::IsOfType,
            ));
    }

    let int_res = token.text.parse::<i64>()
        .map(|val| Literal::Int(val));

    let float_res = token.text.parse::<f64>()
        .map(|val| Literal::Float(OrderedFloat(val)));
    
    if let Ok(int) = int_res {
        return Ok(int);
    }
    else if let Ok(float) = float_res {
        return Ok(float);
    }

    Err(new_from_stream_error(
        SoulErrorKind::InvalidType, 
        token.span, 
        format!("while trying to get literal number '{}' \n{}", token.text, int_res.unwrap_err()),
        FromStreamErrorKind::IsNotOfType,
    ))
}































