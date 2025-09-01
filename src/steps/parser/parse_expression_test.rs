use std::collections::{BTreeMap, HashMap};

use ordered_float::OrderedFloat;

use crate:: {assert_eq_show_diff, errors::soul_error::{SoulErrorKind, SoulSpan}, soul_tuple, steps::{parser::expression::parse_expression::get_expression, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{AccessField, Array, ArrayFiller, Binary, BinaryOperator, BinaryOperatorKind, Expression, ExpressionGroup, ExpressionKind, Ident, Index, NamedTuple, Tuple, Unary, UnaryOperator, UnaryOperatorKind, VariableName}, function::{StructConstructor, FunctionCall}, literal::{Literal, LiteralType}, soul_type::{soul_type::SoulType, type_kind::TypeKind}}, scope_builder::{ProgramMemmory, ProgramMemmoryId, ScopeBuilder}}, i_tokenizer::{Token, TokenStream}}}};

// ---------- helpers ----------


fn stream_from_strs(text_tokens: &[&str]) -> TokenStream {
let mut line_number = 0;
let mut line_offset = 0;
let mut tokens = vec![];
for text in text_tokens {
tokens.push(Token {
text: text.to_string(),
span: SoulSpan::new(line_number, line_offset, text.len()),
});
line_offset += text.len();
if *text == "\n" {
line_number += 1;
line_offset = 0;
}
}


TokenStream::new(tokens)
}


fn empty_scope() -> ScopeBuilder {
ScopeBuilder::new()
}


fn soul_mem_name(id: usize) -> Ident {
ProgramMemmory::to_program_memory_name(&ProgramMemmoryId(id))
}

fn int_lit(v: i64) -> ExpressionKind {
    ExpressionKind::Literal(Literal::Int(v))
}

fn uint_lit(v: u64) -> ExpressionKind {
    ExpressionKind::Literal(Literal::Uint(v))
}

fn float_lit(v: f64) -> ExpressionKind {
    ExpressionKind::Literal(Literal::Float(OrderedFloat(v)))
}

fn bool_lit(v: bool) -> ExpressionKind {
    ExpressionKind::Literal(Literal::Bool(v))
}

fn str_lit(s: &str) -> ExpressionKind {
    ExpressionKind::Literal(Literal::Str(s.into()))
}

fn char_lit(c: char) -> ExpressionKind {
    ExpressionKind::Literal(Literal::Char(c))
}

fn var(name: &str) -> ExpressionKind {
    ExpressionKind::Variable(VariableName::new(name))
}

fn const_ref(kind: ExpressionKind, span: SoulSpan) -> ExpressionKind {
    ExpressionKind::ConstRef(Box::new(Expression::new(kind, span)))
}

fn mut_ref(kind: ExpressionKind, span: SoulSpan) -> ExpressionKind {
    ExpressionKind::MutRef(Box::new(Expression::new(kind, span)))
}

fn deref(kind: ExpressionKind, span: SoulSpan) -> ExpressionKind {
    ExpressionKind::Deref(Box::new(Expression::new(kind, span)))
}

// # Literal

#[test]
fn test_literal_integers_and_floats() {
    let mut scope = empty_scope();

    let mut stream = stream_from_strs(&["42", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]).unwrap();
    assert_eq_show_diff!(result.node, int_lit(42));


    let mut stream = stream_from_strs(&["-42", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]).unwrap();
    assert_eq_show_diff!(result.node, int_lit(-42));


    let mut stream = stream_from_strs(&["4.2", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]).unwrap();
    assert_eq_show_diff!(result.node, float_lit(4.2));
}

#[test]
fn test_literal_misc() {
    let mut scope = empty_scope();

    let mut stream = stream_from_strs(&["0b11010", "\n"]);
    assert_eq_show_diff!(get_expression(&mut stream, &mut scope, &["\n"]).unwrap().node, uint_lit(0b11010));


    let mut stream = stream_from_strs(&["0xfa13", "\n"]);
    assert_eq_show_diff!(get_expression(&mut stream, &mut scope, &["\n"]).unwrap().node, uint_lit(0xfa13));


    let mut stream = stream_from_strs(&["'c'", "\n"]);
    assert_eq_show_diff!(get_expression(&mut stream, &mut scope, &["\n"]).unwrap().node, char_lit('c'));


    let mut stream = stream_from_strs(&["true", "\n"]);
    assert_eq_show_diff!(get_expression(&mut stream, &mut scope, &["\n"]).unwrap().node, bool_lit(true));


    // invalid char literal
    let mut stream = stream_from_strs(&["'cc'", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_err());
    assert!(
        result.as_ref().unwrap_err().get_last_kind() == SoulErrorKind::UnexpectedToken,
        "{}", result.unwrap_err().to_err_message().join("\n")
    );
}

#[test]
fn complex_literals() {
    let mut scope = empty_scope();


    // string literal
    let mut stream = stream_from_strs(&["\"string\"", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(result.clone().unwrap().node, str_lit("string"));


    // array literal [1, 2, 3]
    let mut stream = stream_from_strs(&["[", "1", ",", "2", ",", "3", "]", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(
            soul_mem_name(0),
            LiteralType::Array(Box::new(LiteralType::Int))
        ))
    );


    // array comprehension [for 3 => 12]
    let mut stream = stream_from_strs(&["[", "for", "3", "=>", "12", "]", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(
            soul_mem_name(1),
            LiteralType::Array(Box::new(LiteralType::Int))
        ))
    );


    // tuple literal (1, 0b1, 2.0, true, "string", [1,2,3])
    let mut stream = stream_from_strs(&[
        "(",
        "1", ",",
        "0b1", ",",
        "2.0", ",",
        "true", ",",
        "\"string\"", ",",
        "[", "1", ",", "2", ",", "3", "]",
        ")", "\n"
    ]);
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
        assert_eq_show_diff!(
        result.clone().unwrap().node,
            ExpressionKind::Literal(Literal::ProgramMemmory(
            soul_mem_name(0),
            LiteralType::Tuple(vec![
                LiteralType::Int,
                LiteralType::Uint,
                LiteralType::Float,
                LiteralType::Bool,
                LiteralType::Str,
                LiteralType::Array(Box::new(LiteralType::Int))
            ])
        ))
    );


    // named tuple literal {name1: 1, name2: 1.0}
    let mut stream = stream_from_strs(&["{", "name1", ":", "1", ",", "name2", ":", "1.0", ",", "}", "\n"]);
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(
            soul_mem_name(0),
            LiteralType::NamedTuple(BTreeMap::from([
                (Ident("name1".into()), LiteralType::Int),
                (Ident("name2".into()), LiteralType::Float),
            ]))
        ))
    );
}

// # Binary
#[test]
fn test_simple_binary() {
    let mut stream = stream_from_strs(&["1", "+", "2", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let res = result.unwrap();

    let should_be = Expression::new(
        ExpressionKind::Binary(Binary::new(
            Expression::new(int_lit(1), SoulSpan::new(0, 0, 1)), 
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,1,1)), 
            Expression::new(int_lit(2), SoulSpan::new(0, 2, 1)), 
        )),
        SoulSpan::new(0,0,3)
    );

    assert_eq_show_diff!(res, should_be);

    stream = stream_from_strs(&["1", "==", "2", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let res = result.unwrap();

    let should_be = Expression::new(
        ExpressionKind::Binary(Binary::new(
            Expression::new(int_lit(1), SoulSpan::new(0, 0, 1)), 
            BinaryOperator::new(BinaryOperatorKind::Eq, SoulSpan::new(0, 1, 2)), 
            Expression::new(int_lit(2), SoulSpan::new(0, 3, 1)), 
        )),
        SoulSpan::new(0,0,4)
    );

    assert_eq_show_diff!(res, should_be);

    stream = stream_from_strs(&["\"hello \"", "+", "\"world\"", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let res = result.unwrap();

    let should_be = Expression::new(
        ExpressionKind::Binary(Binary::new(
            Expression::new(str_lit("hello "), SoulSpan::new(0, 0, 8)), 
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0, 8, 1)), 
            Expression::new(str_lit("world"), SoulSpan::new(0, 9, 7)), 
        )),
        SoulSpan::new(0,0,16)
    );

    assert_eq_show_diff!(res, should_be);
}

#[test]
fn test_multiple_binary() {
    let mut stream = stream_from_strs(&["1", "+", "2", "*", "3", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    // 1 + (2 * 3)
    let should_be = Expression::new(ExpressionKind::Binary(Binary::new(
            Expression::new(int_lit(1), SoulSpan::new(0,0,1)),
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,1,1)),
            Expression::new(
                ExpressionKind::Binary(Binary::new(
                    Expression::new(int_lit(2), SoulSpan::new(0,2,1)),
                    BinaryOperator::new(BinaryOperatorKind::Mul, SoulSpan::new(0,3,1)),
                    Expression::new(int_lit(3), SoulSpan::new(0,4,1)),
                )),
                SoulSpan::new(0,2,3)
            )
        )),
        SoulSpan::new(0,0,5)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );

    let mut scope = empty_scope();
    let mut stream = stream_from_strs(&["(","1", "+", "2", ")", "*", "4", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    // (1 + 2) * 3
    let should_be = Expression::new(ExpressionKind::Binary(Binary::new(
        Expression::new(ExpressionKind::Binary(Binary::new(
                    Expression::new(int_lit(1), SoulSpan::new(0,1,1)), 
                    BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,2,1)), 
                    Expression::new(int_lit(2), SoulSpan::new(0,3,1)),
                )), 
                SoulSpan::new(0,1,3),
            ),
            BinaryOperator::new(BinaryOperatorKind::Mul, SoulSpan::new(0,5,1)),
            Expression::new(ExpressionKind::Literal(Literal::Int(4)), SoulSpan::new(0,6,1)),
        )),
        SoulSpan::new(0,1,6),
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be,
        "2"
    );


    stream = stream_from_strs(&["(","1", "+", "2", "*", "5", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    // (1 + (2 * 3))
    let should_be = Expression::new(ExpressionKind::Binary(Binary::new(
        Expression::new(int_lit(1), SoulSpan::new(0,1,1)),
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,2,1)),
            Expression::new(
                ExpressionKind::Binary(Binary::new(
                    Expression::new(int_lit(2), SoulSpan::new(0,3,1)),
                    BinaryOperator::new(BinaryOperatorKind::Mul, SoulSpan::new(0,4,1)),
                    Expression::new(int_lit(5), SoulSpan::new(0,5,1)),
                )),
                SoulSpan::new(0,3,3)
            )
        )),
        SoulSpan::new(0,1,5)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );
}

// # Unary

#[test]
fn test_simple_unary() {
    let mut stream = stream_from_strs(&["!", "true", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be = Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Not, SoulSpan::new(0,0,1)), 
            expression: Box::new(Expression::new(bool_lit(true), SoulSpan::new(0,1,4))),
        }),
        SoulSpan::new(0,0,5)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );


    stream = stream_from_strs(&["-", "1", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be = Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Neg, SoulSpan::new(0,0,1)), 
            expression: Box::new(Expression::new(int_lit(1), SoulSpan::new(0,1,1))),
        }),
        SoulSpan::new(0,0,2)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );

    stream = stream_from_strs(&["(","-", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be = ExpressionKind::Unary(Unary{
        operator: UnaryOperator::new(UnaryOperatorKind::Neg, SoulSpan::new(0,1,1)), 
        expression: Box::new(Expression::new(int_lit(2), SoulSpan::new(0,2,1))),
    });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );

    stream = stream_from_strs(&["++", "var", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be =  Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Increment{before_var: true}, SoulSpan::new(0,0,2)), 
            expression: Box::new(Expression::new(var("var"), SoulSpan::new(0,2,3))),
        }),
        SoulSpan::new(0,0,5)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );

    stream = stream_from_strs(&["var", "++", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be =  Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Increment{before_var: false}, SoulSpan::new(0,3,2)), 
            expression: Box::new(Expression::new(var("var"), SoulSpan::new(0,0,3))),
        }),
        SoulSpan::new(0,0,5)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );
}

#[test]
fn test_unary_in_binary() {
    let mut stream = stream_from_strs(&["-", "1", "+", "8", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be = ExpressionKind::Binary(Binary::new(
        Expression::new(ExpressionKind::Unary(Unary{
                operator: UnaryOperator::new(UnaryOperatorKind::Neg, SoulSpan::new(0,0,1)), 
                expression: Box::new(Expression::new(int_lit(1), SoulSpan::new(0,1,1))),
            }), 
            SoulSpan::new(0,0,2),
        ), 
        BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,2,1)), 
        Expression::new(
            int_lit(8), 
            SoulSpan::new(0,3,1),
        ), 
    ));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );

    stream = stream_from_strs(&["(", "-", "1", ")", "+", "8", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be = ExpressionKind::Binary(Binary::new(
        Expression::new(ExpressionKind::Unary(Unary{
                operator: UnaryOperator::new(UnaryOperatorKind::Neg, SoulSpan::new(0,1,1)), 
                expression: Box::new(Expression::new(int_lit(1), SoulSpan::new(0,2,1))),
            }), 
            SoulSpan::new(0,1,2),
        ), 
        BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,4,1)), 
        Expression::new(
            int_lit(8), 
            SoulSpan::new(0,5,1),
        ), 
    ));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );
}

// # variable

#[test]
fn test_simple_variable() {
    let var_name = "var";
    let mut stream = stream_from_strs(&[var_name, "\n"]);
    let mut scope = empty_scope();
    
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        var(var_name)
    );

    stream = stream_from_strs(&["!", var_name, "\n"]);

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Not, SoulSpan::new(0,0,1)),
            expression: Box::new(Expression::new(var(var_name), SoulSpan::new(0,1,var_name.len()))),
        })
    );
}

// # group (tuple, array and namedTuple that contain non literal expressions)

#[test]
fn test_group_expressions() {
// 
    let mut stream = stream_from_strs(&[
        "[",
            "var", ",",
            "2", ",",
            "3",
        "]", "\n"
    ]);
    let mut scope = empty_scope();

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(var("var"), SoulSpan::new(0,1,3)),
        Expression::new(int_lit(2), SoulSpan::new(0,5,1)),
        Expression::new(int_lit(3), SoulSpan::new(0,7,1)),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type: None, element_type: None, values: values.clone()}))
    );

    let mut stream = stream_from_strs(&[
        "[",
            "f32", ":",
            "var", ",",
            "2", ",",
            "3",
        "]", "\n"
    ]);
    let mut scope = empty_scope();

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(var("var"), SoulSpan::new(0,5,3)),
        Expression::new(int_lit(2), SoulSpan::new(0,9,1)),
        Expression::new(int_lit(3), SoulSpan::new(0,11,1)),
    ];

    let ty_unkown_f32 = SoulType::from_type_kind(TypeKind::Unknown("f32".into())); 

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type: None, element_type: Some(ty_unkown_f32.clone()), values: values.clone()}))
    );

    let mut stream = stream_from_strs(&[
        "[",
            "func", "(", ")",  ",",
            "2", ",",
            "3",
        "]", "\n"
    ]);
    let mut scope = empty_scope();

    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    let values = vec![
        Expression::new(ExpressionKind::FunctionCall(FunctionCall{name: "func".into(), callee: None, generics: vec![], arguments: soul_tuple![]}), SoulSpan::new(0,1,6)),
        Expression::new(int_lit(2), SoulSpan::new(0,8,1)),
        Expression::new(int_lit(3), SoulSpan::new(0,10,1)),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type: None, element_type: None, values: values.clone()}))
    );

    let mut stream = stream_from_strs(&[
        "[",
            "[", "var", ",", "1", "]", ",",
            "[", "2", "]", ",",
            "[", "3", "]",
        "]", "\n"
    ]);
    let mut scope = empty_scope();

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(
            ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{
                values: vec![
                    Expression::new(var("var"), SoulSpan::new(0,2,3)),
                    Expression::new(int_lit(1), SoulSpan::new(0,6,1)),
                ], 
                collection_type: None, 
                element_type: None,
            })), 
            SoulSpan::new(0,1,7),
        ),
        Expression::new(
            ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(0), LiteralType::Array(Box::new(LiteralType::Int)))),
            SoulSpan::new(0,9,3),
        ),
        Expression::new(
            ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(1), LiteralType::Array(Box::new(LiteralType::Int)))),
            SoulSpan::new(0,13,3),
        ),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type: None, element_type: None, values: values.clone()}))
    );

//

    stream = stream_from_strs(&[
        "(",
            "var", ",",
            "2", ",",
            "3",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(var("var"), SoulSpan::new(0,1,3)),
        Expression::new(int_lit(2), SoulSpan::new(0,5,1)),
        Expression::new(int_lit(3), SoulSpan::new(0,7,1)),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values: values.clone()}))
    );

    stream = stream_from_strs(&[
        "(",
            "var",",",
            "2", ",",
            "(", "3", ",", "true", ")",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(var("var"), SoulSpan::new(0,1,3)),
        Expression::new(int_lit(2), SoulSpan::new(0,5,1)),
        Expression::new(
            ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(0), LiteralType::Tuple(vec![LiteralType::Int, LiteralType::Bool]))),
            SoulSpan::new(0,7,8),
        ),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values: values.clone()}))
    );
//

    stream = stream_from_strs(&[
        "{",
            "field", ":", "var", ",",
            "field2", ":", "2", ",",
            "field3", ":", "3",
        "}", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = HashMap::from([
        (Ident("field".into()), Expression::new(var("var"), SoulSpan::new(0,7,3))),
        (Ident("field2".into()), Expression::new(int_lit(2), SoulSpan::new(0,18,1))),
        (Ident("field3".into()), Expression::new(int_lit(3), SoulSpan::new(0,27,1))),
    ]);

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::NamedTuple(NamedTuple{values: values.clone(), insert_defaults: false}))
    );

    stream = stream_from_strs(&[
        "{",
            "field", ":", "var", ",",
            "field2", ":", "2", ",",
            "field3", ":", "{", "field", ":", "1", "}",
        "}", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = HashMap::from([
        (Ident("field".into()), Expression::new(var("var"), SoulSpan::new(0,7,3))),
        (Ident("field2".into()), Expression::new(int_lit(2), SoulSpan::new(0,18,1))),
        (Ident("field3".into()), Expression::new(ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(0), LiteralType::NamedTuple(BTreeMap::from([(Ident("field".into()), LiteralType::Int)])))), SoulSpan::new(0,27,9))),
    ]);

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::NamedTuple(NamedTuple{values: values.clone(), insert_defaults: false}))
    );

    stream = stream_from_strs(&[
        "{",
            "field", ":", "var", ",",
            "field", ":", "2", ",",
        "}", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_err(), "{:#?}", result.unwrap());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::InvalidName);

}

// # ArrayFiller
#[test]
fn test_array_filler_expressions() {
    let mut scope = empty_scope();

    let mut stream = stream_from_strs(&["[", "for", "2", "=>", "var", "]", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::ArrayFiller(
            ArrayFiller {
                collection_type: None,
                element_type: None,
                amount: Box::new(Expression::new(int_lit(2), SoulSpan::new(0,4,1))),
                index: None,
                fill_expr: Box::new(Expression::new(var("var"), SoulSpan::new(0,7,3))),
            }
        ))
    );


    let mut stream = stream_from_strs(&["[", "for", "2", "=>", "1", "+", "2", "]", "\n"]);
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::ArrayFiller(
            ArrayFiller {
                collection_type: None,
                element_type: None,
                amount: Box::new(Expression::new(int_lit(2), SoulSpan::new(0,4,1))),
                index: None,
                fill_expr: Box::new(Expression::new(
                    ExpressionKind::Binary(Binary{
                        left: Box::new(Expression::new(int_lit(1), SoulSpan::new(0,7,1))),
                        operator: BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,8,1)),
                        right: Box::new(Expression::new(int_lit(2), SoulSpan::new(0,9,1))),
                    }), 
                    SoulSpan::new(0,7,3),
                )),
            }
        ))
    );
}

// # Function

#[test]
fn test_function_call() {
    let mut scope = empty_scope();

    let mut stream = stream_from_strs(&["Println", "(", "\"hello world\"" ,")" , "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let should_be = Expression::new(
        ExpressionKind::FunctionCall(FunctionCall{
            callee: None, 
            name: Ident("Println".into()), 
            generics: vec![], 
            arguments: soul_tuple![
                Expression::new(ExpressionKind::Literal(Literal::Str("hello world".into())), SoulSpan::new(0,8,13)),
            ],
        }),
        SoulSpan::new(0,0,22)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);

    
    stream = stream_from_strs(&["sum", "(", "1", ",", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let should_be = Expression::new(
        ExpressionKind::FunctionCall(FunctionCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: soul_tuple![
                Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,4,1)),
                Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,6,1)),
            ],
        }),
        SoulSpan::new(0,0,8)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);

    stream = stream_from_strs(&["Type", "{", "field", ":", "1", ",", "field2", ":", "2", "}", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let should_be = Expression::new(
        ExpressionKind::StructConstructor(StructConstructor{
            calle: SoulType::from_type_kind(TypeKind::Unknown("Type".into())),
            arguments: NamedTuple{values: HashMap::from([
                ("field".into(), Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,11,1))),
                ("field2".into(), Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,20,1))),
            ]), insert_defaults: false},
        }),
        SoulSpan::new(0,0,22)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);

    stream = stream_from_strs(&["sum", "()", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let should_be = Expression::new(
        ExpressionKind::FunctionCall(FunctionCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: soul_tuple![],
        }),
        SoulSpan::new(0,0,5)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);

    stream = stream_from_strs(&["sum", "(", "1", ",", "2", "\n"]);
    let not_closing_fn = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(not_closing_fn.is_err());
    assert_eq!(not_closing_fn.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::UnexpectedEnd, "{}", not_closing_fn.unwrap_err().to_err_message().join("\n"));


    stream = stream_from_strs(&["sum", "(", "1", ",", "name", ":", ")", "\n"]);
    let not_closing_fn = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(not_closing_fn.is_err());
    stream = stream_from_strs(&["sum", "{", "name", ":", "1", ",", "2", "}", "\n"]);
    let not_closing_fn = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(not_closing_fn.is_err());
    assert_eq!(not_closing_fn.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::InvalidType, "{}", not_closing_fn.unwrap_err().to_err_message().join("\n"));
}


// # Field/Methode

#[test]
fn test_field_access_and_methods() {
    let mut scope = empty_scope();


    let mut stream = stream_from_strs(&["obj", ".", "field", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::AccessField(AccessField{ 
            object: Box::new(Expression::new(var("obj"), SoulSpan::new(0,0,3))),
            field: VariableName::new("field"),
        })
    );



    let mut stream = stream_from_strs(&["obj", ".", "methode", "(", "1", ",", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::FunctionCall(FunctionCall{ 
            name: "methode".into(), 
            callee: Some(Box::new(Expression::new(var("obj"), SoulSpan::new(0,0,3)))), 
            generics: vec![], 
            arguments: soul_tuple![
                Expression::new(int_lit(1), SoulSpan::new(0,12,1)),
                Expression::new(int_lit(2), SoulSpan::new(0,14,1)),
            ],
        })
    );



    let mut stream = stream_from_strs(&["obj", ".", "field", ".", "methode", "(", "true", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::FunctionCall(FunctionCall{ 
            name: "methode".into(), 
            callee: Some(Box::new(Expression::new(
                ExpressionKind::AccessField(AccessField{
                    object: Box::new(Expression::new(var("obj"), SoulSpan::new(0,0,3))), 
                    field: VariableName::new("field"),
                }),
                SoulSpan::new(0,0,9)
            ))), 
            generics: vec![], 
            arguments: soul_tuple![
                Expression::new(bool_lit(true), SoulSpan::new(0,18,4)),
            ],
        })
    );
}

// # Index 

#[test]
fn test_index_expressions() {
    let mut scope = empty_scope();

    let mut stream = stream_from_strs(&["arr", "[", "1", "]", "[", "2", "]", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::Index(Index {
            collection: Box::new(Expression::new(
                ExpressionKind::Index(Index{
                    collection: Box::new(Expression::new(ExpressionKind::Variable(VariableName::new("arr")), SoulSpan::new(0,0,3))), 
                    index: Box::new(Expression::new(int_lit(1), SoulSpan::new(0,4,1))),
                }),
                SoulSpan::new(0,5,1)
            )),
            index: Box::new(Expression::new(int_lit(2), SoulSpan::new(0,7,1)))
        })
    );

    let mut stream = stream_from_strs(&["obj", ".", "field", "[", "3", "]", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        ExpressionKind::Index(Index {
            collection: Box::new(Expression::new(ExpressionKind::AccessField(AccessField{
                    object: Box::new(Expression::new(var("obj"), SoulSpan::new(0,0,3))), 
                    field: VariableName::new("field"),
                }),
                SoulSpan::new(0,0,9),
            )),
            index: Box::new(Expression::new(int_lit(3), SoulSpan::new(0,10,1))),
        })
    );
}


// # Ref

#[test]
fn test_ref_and_deref_expression() {
    let mut scope = empty_scope();


    let mut stream = stream_from_strs(&["@", "1", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        const_ref(
            ExpressionKind::Literal(
                Literal::ProgramMemmory(soul_mem_name(0), 
                LiteralType::Int),
            ), 
            SoulSpan::new(0,1,1),
        )
    );


    let mut stream = stream_from_strs(&["&", "var", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        mut_ref(var("var"), SoulSpan::new(0,1,3))
    );


    let mut stream = stream_from_strs(&["@", "@", "&", "var", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        const_ref(
            const_ref(
                mut_ref(var("var"), SoulSpan::new(0,3,3)),
                SoulSpan::new(0,2,4)
            ),
            SoulSpan::new(0,1,5)
        )
    );

    
    let mut stream = stream_from_strs(&["*", "ref", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.clone().unwrap().node,
        deref(var("ref"), SoulSpan::new(0,1,3))
    );
}



























