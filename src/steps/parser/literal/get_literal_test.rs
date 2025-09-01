use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::assert_eq_show_diff;
use crate::errors::soul_error::{SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Ident;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::literal::{Literal, LiteralType};

fn token<T: Into<String>>(text: T) -> Token {
    let str = text.into();
    let len = str.len();
    Token::new(str, SoulSpan::new(0, 0,len))
}

fn stream_from_strs(tokens: &[&str]) -> TokenStream {
    TokenStream::new(tokens.iter().map(|&s| token(s)).collect())
}

fn dummy_scopes() -> ScopeBuilder {
    ScopeBuilder::new()
}

#[test]
fn test_parse_positive_integer() {
    let mut stream = stream_from_strs(&["42"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    assert_eq_show_diff!(result, Literal::Int(42));
}

#[test]
fn test_parse_negative_integer() {
    let mut stream = stream_from_strs(&["-42"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    assert_eq_show_diff!(result, Literal::Int(-42));
}

#[test]
fn test_parse_binary_integer() {
    let mut stream = stream_from_strs(&["0b10101010"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    assert_eq_show_diff!(result, Literal::Uint(170));
}

#[test]
fn test_parse_hexidecimal_integer() {
    let mut stream = stream_from_strs(&["0xffa12"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    assert_eq_show_diff!(result, Literal::Uint(1047058));
}

#[test]
fn test_invalid_hex_fails() {
    let mut stream = stream_from_strs(&["0xZZZ"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq_show_diff!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_invalid_literal() {
    let mut stream = stream_from_strs(&["not_a_literal"]);
    let mut scopes = dummy_scopes();
    let literal = Literal::try_from_stream(&mut stream, &mut scopes);
    assert!(literal.is_ok());
    assert!(literal.unwrap().is_none());
}

#[test]
fn test_parse_float() {
    let mut stream = stream_from_strs(&["3.14"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    assert_eq_show_diff!(result, Literal::Float(OrderedFloat(3.14)));
}

#[test]
fn test_parse_bool_true_false() {
    let mut stream = stream_from_strs(&["true"]);
    let mut scopes = dummy_scopes();

    let lit_true = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(lit_true, Literal::Bool(true));

    stream = stream_from_strs(&["false"]);
    let lit_false = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(lit_false, Literal::Bool(false));
}

#[test]
fn test_parse_char() {
    let mut stream = stream_from_strs(&["'x'"]);
    let mut scopes = dummy_scopes();
    let lit_char = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(lit_char, Literal::Char('x'));

    stream = stream_from_strs(&["'xx'"]);
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq_show_diff!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_parse_str() {
    let mut stream = stream_from_strs(&["\"hello world\""]);
    let mut scopes = dummy_scopes();
    let lit_char = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq!(lit_char, Literal::Str("hello world".into()));

    stream = stream_from_strs(&["\"yutfuy"]);
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq_show_diff!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_parse_array_same_type() {
    let mut stream = stream_from_strs(&["[", "1", ",", "2", ",", "3", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    assert_eq_show_diff!(
        lit,
        Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)] }
    );
}

#[test]
fn test_parse_2d_array_same_types() {
    let mut stream = stream_from_strs(&["[", "[", "1", ",", "2", ",", "3", "]", ",", "[", "1", "]", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    
    let should_be = Literal::Array { 
        ty: LiteralType::Array(Box::new(LiteralType::Int)), 
        values: vec![
            Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)] },
            Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1)] }
        ] 
    };

    assert_eq_show_diff!(
        lit,
        should_be
    );
}

#[test]
fn test_parse_2d_array_diffrent_numeric_types() {
    let mut stream = stream_from_strs(&["[", "[", "1", ",", "0xff", ",", "3", "]", ",", "[", "1.0", "]", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    
    let should_be = Literal::Array { 
        ty: LiteralType::Array(Box::new(LiteralType::Float)), 
        values: vec![
            Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Uint(255), Literal::Int(3)] },
            Literal::Array { ty: LiteralType::Float, values: vec![Literal::Float(ordered_float::OrderedFloat(1.0))] }
        ] 
    };

    assert_eq_show_diff!(
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
    assert_eq_show_diff!(lit.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_parse_array_diffrent_numeric_types() {
    let mut stream = stream_from_strs(&["[", "-1", ",", "0b1", ",", "3.0", "]"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(
        lit,
        Literal::Array{ty: LiteralType::Float, values: vec![Literal::Int(-1), Literal::Uint(1), Literal::Float(ordered_float::OrderedFloat(3.0))]}
    );
}

#[test]
fn test_array_type_mismatch() {
    let mut stream = stream_from_strs(&["[", "1", ",", "true", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    
    assert!(result.is_err(), "{:?}", result.unwrap());
    assert_eq_show_diff!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
    

    stream = stream_from_strs(&["[", "'c'", ",", "true", "]"]);
    let result = Literal::from_stream(&mut stream, &mut scopes);

    assert!(result.is_err(), "{:?}", result.unwrap());
    assert_eq_show_diff!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_empty_array() {
    let mut stream = stream_from_strs(&["[]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(
        result,
        Literal::Array{ty: LiteralType::Int, values: vec![]}
    );

    stream = stream_from_strs(&["[", "]"]);
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(
        result,
        Literal::Array{ty: LiteralType::Int, values: vec![]}
    );
}

#[test]
fn test_tuple_mixed() {
    let mut stream = stream_from_strs(&["(", "1", ",", "true", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(
        result,
        Literal::new_tuple(vec![Literal::Int(1), Literal::Bool(true)])
    );

    stream = stream_from_strs(&[
        "(",
            "1", ",",
            "0b1", ",",
            "2.0", ",",
            "true", ","
            ,"\"string\"", ",",
            "[",
                "1", ",",
                "2", ",",
                "3"
            ,"]",
        ")", "\n"
    ]);

    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();
    
    assert_eq_show_diff!(
        result,
        Literal::Tuple{
            values: vec![
                Literal::Int(1), 
                Literal::Uint(0b1), 
                Literal::Float(OrderedFloat(2.0)), 
                Literal::Bool(true),
                Literal::Str("string".into()),
                Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)] }
            ]
        }
    );
}

#[test]
fn test_tuple_with_array() {
    let mut stream = stream_from_strs(&["(", "[", "1", ",", "2.0", "]", ",", "true", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(
        result,
        Literal::new_tuple(vec![Literal::new_array(vec![Literal::Int(1), Literal::Float(ordered_float::OrderedFloat(2.0))], &SoulSpan::new(0,0,7)).unwrap(), Literal::Bool(true)])
    );
}

#[test]
fn test_empty_tuple() {
    let mut stream = stream_from_strs(&["()"]);
    let mut scopes = dummy_scopes();
    let lit = Literal::from_stream(&mut stream, &mut scopes).unwrap();
    assert_eq_show_diff!(lit, Literal::new_tuple(vec![]));

    stream = stream_from_strs(&["(", ")"]);
    let lit = Literal::from_stream(&mut stream, &mut scopes).unwrap();
    assert_eq_show_diff!(lit, Literal::new_tuple(vec![]));
}

#[test]
fn test_named_tuple() {
    let mut stream = stream_from_strs(&["{", "name", ":", "true", ",", "name2", ":", "1",  "}"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes).unwrap();

    let mut expected = BTreeMap::new();
    expected.insert(Ident("name".to_string()), Literal::Bool(true));
    expected.insert(Ident("name2".to_string()), Literal::Int(1));
    assert_eq!(result, Literal::new_named_tuple(expected, false));

    stream = stream_from_strs(&["{", "name1", ":", "1", ",", "name2", ":", "1.0", ",", "}", "\n"]);

    let result = Literal::from_stream(&mut stream, &mut scopes).unwrap();
    
    let should_be = Literal::NamedTuple{
        values: BTreeMap::from([
            (Ident("name1".into()), Literal::Int(1)), 
            (Ident("name2".into()), Literal::Float(OrderedFloat(1.0))), 
        ]),
        insert_defaults: false,
    };

    assert_eq_show_diff!(should_be, result);

    stream = stream_from_strs(&["{", "name1", ":", "1", ",", "..", "}", "\n"]);

    let result = Literal::from_stream(&mut stream, &mut scopes).unwrap();
    
    let should_be = Literal::NamedTuple{
        values: BTreeMap::from([
            (Ident("name1".into()), Literal::Int(1)), 
        ]),
        insert_defaults: true,
    };

    assert_eq_show_diff!(should_be, result);
}

#[test]
fn test_mixed_named_unnamed_tuple_fails() {
    let mut stream = stream_from_strs(&["(", "1", ",", "x", ":", "2", ")"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq_show_diff!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType);
}

#[test]
fn test_literal_array_filler() {
    let mut stream = stream_from_strs(&["[", "for", "12", "=>", "1", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(result, Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1); 12] });

    stream = stream_from_strs(&["[", "for", "12", "=>", "\"foo\"", "]"]);
    let result = Literal::from_stream(&mut stream, &mut scopes)
        .inspect_err(|err| panic!("{}", err.to_err_message().join("\n"))).unwrap();

    assert_eq_show_diff!(result, Literal::Array { ty: LiteralType::Str, values: vec![Literal::Str("foo".into()); 12] });

    let mut stream = stream_from_strs(&["[", "for", "i", "in", "12", "=>", "1", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType, "{}", result.unwrap_err().to_err_message().join("\n"));

    let mut stream = stream_from_strs(&["[", "for", "-12", "=>", "1", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType, "{}", result.unwrap_err().to_err_message().join("\n"));

    let mut stream = stream_from_strs(&["[", "for", "1.2", "=>", "1", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType, "{}", result.unwrap_err().to_err_message().join("\n"));

    let mut stream = stream_from_strs(&["[", "for", "12", "=>", "funcNotLiteral", "(", ")", "]"]);
    let mut scopes = dummy_scopes();
    let result = Literal::from_stream(&mut stream, &mut scopes);
    assert!(result.is_err());
    assert_eq!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::WrongType, "{}", result.unwrap_err().to_err_message().join("\n"));
}



























