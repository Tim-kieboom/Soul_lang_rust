use std::collections::HashMap;

use crate::errors::soul_error::{SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
use crate::steps::step_interfaces::i_parser::scope::{ScopeBuilder, TypeScopeStack};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::literal::{Literal, LiteralType};

fn token<T: Into<String>>(text: T) -> Token {
    Token::new(text.into(), SoulSpan::new(0, 0))
}

fn stream_from_strs(tokens: &[&str]) -> TokenStream {
    TokenStream::new(tokens.iter().map(|&s| token(s)).collect())
}

fn dummy_scopes() -> ScopeBuilder {
    ScopeBuilder::new(TypeScopeStack::new())
}

#[test]
fn test_parse_positive_integer() {
    let mut stream = stream_from_strs(&["42"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    assert_eq!(result, Literal::Int(42));
}

#[test]
fn test_parse_negative_integer() {
    let mut stream = stream_from_strs(&["-42"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    assert_eq!(result, Literal::Int(-42));
}

#[test]
fn test_parse_binary_integer() {
    let mut stream = stream_from_strs(&["0b10101010"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    assert_eq!(result, Literal::Uint(170));
}

#[test]
fn test_parse_hexidecimal_integer() {
    let mut stream = stream_from_strs(&["0xffa12"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    assert_eq!(result, Literal::Uint(1047058));
}

#[test]
fn test_invalid_hex_fails() {
    let mut stream = stream_from_strs(&["0xZZZ"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_try_from_stream_none_on_invalid() {
    let mut stream = stream_from_strs(&["not_a_literal"]);
    let mut scopes = dummy_scopes();
    let literal = Literal::try_from_stream(&mut stream, &mut scopes);
    assert!(literal.is_none());
}

#[test]
fn test_parse_float() {
    let mut stream = stream_from_strs(&["3.14"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    assert_eq!(result, Literal::Float(3.14));
}

#[test]
fn test_parse_bool_true_false() {
    let mut stream = stream_from_strs(&["true"]);
    let mut scopes = dummy_scopes();

    let lit_true = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(lit_true, Literal::Bool(true));

    stream = stream_from_strs(&["false"]);
    let lit_false = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(lit_false, Literal::Bool(false));
}

#[test]
fn test_parse_char() {
    let mut stream = stream_from_strs(&["'x'"]);
    let mut scopes = dummy_scopes();
    let lit_char = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(lit_char, Literal::Char('x'));

    stream = stream_from_strs(&["'xx'"]);
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_parse_array_same_type() {
    let mut stream = stream_from_strs(&["[", "1", ",", "2", ",", "3", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    assert_eq!(
        lit,
        Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)] }
    );
}

#[test]
fn test_parse_2d_array_same_types() {
    let mut stream = stream_from_strs(&["[", "[", "1", ",", "2", ",", "3", "]", ",", "[", "1", "]", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    
    let should_be = Literal::Array { 
        ty: LiteralType::Array(Box::new(LiteralType::Int)), 
        values: vec![
            Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)] },
            Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1)] }
        ] 
    };

    assert_eq!(
        lit,
        should_be
    );
}

#[test]
fn test_parse_2d_array_diffrent_numeric_types() {
    let mut stream = stream_from_strs(&["[", "[", "1", ",", "0xff", ",", "3", "]", ",", "[", "1.0", "]", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();
    
    let should_be = Literal::Array { 
        ty: LiteralType::Array(Box::new(LiteralType::Float)), 
        values: vec![
            Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Uint(255), Literal::Int(3)] },
            Literal::Array { ty: LiteralType::Float, values: vec![Literal::Float(1.0)] }
        ] 
    };

    assert_eq!(
        lit,
        should_be
    );
}

#[test]
fn test_parse_2d_array_mismatched_type() {
    let mut stream = stream_from_strs(&["[", "[", "1", ",", "0xff", ",", "3", "]", ",", "[", "true", "]", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes);
    
    assert!(lit.is_err());
    assert_eq!(lit.unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_parse_array_diffrent_numeric_types() {
    let mut stream = stream_from_strs(&["[", "-1", ",", "0b1", ",", "3.0", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(
        lit,
        Literal::Array{ty: LiteralType::Float, values: vec![Literal::Int(-1), Literal::Uint(1), Literal::Float(3.0)]}
    );
}

#[test]
fn test_array_type_mismatch() {
    let mut stream = stream_from_strs(&["[", "1", ",", "true", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    
    assert!(result.is_err(), "{:?}", result.unwrap());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
    

    stream = stream_from_strs(&["[", "'c'", ",", "true", "]"]);
    let result = Literal::from_stream(&mut stream, &mut scopes);

    assert!(result.is_err(), "{:?}", result.unwrap());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_empty_array() {
    let mut stream = stream_from_strs(&["[]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(
        result,
        Literal::Array{ty: LiteralType::Int, values: vec![]},
    );

    stream = stream_from_strs(&["[", "]"]);
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(
        result,
        Literal::Array{ty: LiteralType::Int, values: vec![]},
    );
}

#[test]
fn test_tuple_mixed() {
    let mut stream = stream_from_strs(&["(", "1", ",", "true", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(
        result,
        Literal::new_tuple(vec![Literal::Int(1), Literal::Bool(true)])
    );
}

#[test]
fn test_tuple_with_array() {
    let mut stream = stream_from_strs(&["(", "[", "1", ",", "2.0", "]", ",", "true", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message())).unwrap();

    assert_eq!(
        result,
        Literal::new_tuple(vec![Literal::new_array(vec![Literal::Int(1), Literal::Float(2.0)], &SoulSpan::new(0,0)).unwrap(), Literal::Bool(true)])
    );
}

#[test]
fn test_empty_tuple() {
    let mut stream = stream_from_strs(&["()"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes).unwrap();
    assert_eq!(lit, Literal::new_tuple(vec![]));

    stream = stream_from_strs(&["(", ")"]);
    let lit = Literal::from_stream(&mut stream, &mut scopes).unwrap();
    assert_eq!(lit, Literal::new_tuple(vec![]));
}

#[test]
fn test_named_tuple() {
    let mut stream = stream_from_strs(&["(", "name", ":", "true", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes).unwrap();

    let mut expected = HashMap::new();
    expected.insert(Ident("name".to_string()), Literal::Bool(true));

    assert_eq!(result, Literal::new_named_tuple(expected));
}

#[test]
fn test_mixed_named_unnamed_tuple_fails() {
    let mut stream = stream_from_strs(&["(", "1", ",", "x", ":", "2", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::InvalidType);
}




























