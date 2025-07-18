use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::{assert_eq_show_diff, errors::soul_error::SoulSpan, steps::{parser::get_expressions::parse_expression::get_expression, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{BinOp, BinOpKind, BinaryExpr, ExprKind, Expression, Ident, UnaryExpr, UnaryOp, UnaryOpKind}, literal::{Literal, LiteralType}}, scope::{ScopeBuilder, TypeScopeStack}}, i_tokenizer::{Token, TokenStream}}}};
fn token(text: &str, index: usize) -> Token {
    Token {
        text: text.to_string(),
        span: SoulSpan { line_number: 0, line_offset: index },
    }
}

fn stream_from_strs(tokens: &[&str]) -> TokenStream {
    TokenStream::new(tokens.iter().enumerate().map(|(i, &t)| token(t, i)).collect())
}

fn empty_scope() -> ScopeBuilder {
    ScopeBuilder::new(TypeScopeStack::new())
}

#[test]
fn test_simple_literal() {
    let mut stream = stream_from_strs(&["42", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Int(42))
    );

    stream = stream_from_strs(&["-42", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Int(-42))
    );

    stream = stream_from_strs(&["4.2", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Float(OrderedFloat(4.2)))
    );

    stream = stream_from_strs(&["0b11010", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Uint(0b11010))
    );

    stream = stream_from_strs(&["0xfa13", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Uint(0xfa13))
    );

    stream = stream_from_strs(&["'c'", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Char('c'))
    );

    stream = stream_from_strs(&["true", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Bool(true))
    );

    stream = stream_from_strs(&["'c'", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Char('c'))
    );


}

#[test]
fn test_complex_literal() {
    let mut stream = stream_from_strs(&["\"string\"", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Str("string".into()))
    );

    stream = stream_from_strs(&[
        "[",
            "1", ",",
            "2", ",",
            "3",
        "]", "\n"
    ]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Array{
            ty: LiteralType::Int, 
            values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)]
        })
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
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::Tuple{
            values: vec![
                Literal::Int(1), 
                Literal::Uint(0b1), 
                Literal::Float(OrderedFloat(2.0)), 
                Literal::Bool(true),
                Literal::Str("string".into()),
                Literal::Array { ty: LiteralType::Int, values: vec![Literal::Int(1), Literal::Int(2), Literal::Int(3)] }
            ]
        })
    );

    stream = stream_from_strs(&[
        "(",
            "name1", ":", "1", ",",
            "name2", ":", "1.0", ",",
        ")", "\n"
    ]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExprKind::Literal(Literal::NamedTuple{
            values: BTreeMap::from([
                (Ident("name1".into()), Literal::Int(1)), 
                (Ident("name2".into()), Literal::Float(OrderedFloat(1.0))), 
            ])
        })
    );

}

#[test]
fn test_simple_binary() {
    let mut stream = stream_from_strs(&["1", "+", "2", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let res = result.unwrap();

    let should_be = Expression::new(
        ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0, 0)), 
            BinOp::new(BinOpKind::Add, res.span), 
            Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0, 2)), 
        )),
        res.span
    );

    assert_eq_show_diff!(res.node, should_be.node);

    stream = stream_from_strs(&["1", "==", "2", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let res = result.unwrap();

    let should_be = Expression::new(
        ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0, 0)), 
            BinOp::new(BinOpKind::Eq, res.span), 
            Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0, 2)), 
        )),
        res.span
    );

    assert_eq_show_diff!(res.node, should_be.node);

    stream = stream_from_strs(&["\"hello \"", "+", "\"world\"", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let res = result.unwrap();

    let should_be = Expression::new(
        ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Str("hello ".into())), SoulSpan::new(0, 0)), 
            BinOp::new(BinOpKind::Add, res.span), 
            Expression::new(ExprKind::Literal(Literal::Str("world".into())), SoulSpan::new(0, 2)), 
        )),
        res.span
    );

    assert_eq_show_diff!(res.node, should_be.node);
}

#[test]
fn test_multiple_binary() {
    let mut stream = stream_from_strs(&["1", "+", "2", "*", "3", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // 1 + (2 * 3)
    let should_be = ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,0)),
            BinOp::new(BinOpKind::Add, SoulSpan::new(0,1)),
            Expression::new(
                ExprKind::Binary(BinaryExpr::new(
                    Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,2)),
                    BinOp::new(BinOpKind::Mul, SoulSpan::new(0,3)),
                    Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0,4)),
                )),
                SoulSpan::new(0,3)
            )
        ));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );


    stream = stream_from_strs(&["(","1", "+", "2", ")", "*", "3", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // (1 + 2) * 3
    let should_be = ExprKind::Binary(BinaryExpr::new(
        Expression::new(ExprKind::Binary(BinaryExpr::new(
                Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,2)), 
                BinOp::new(BinOpKind::Add, SoulSpan::new(0, 3)), 
                Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0, 3)),
            )), 
            SoulSpan::new(0, 3),
        ),
        BinOp::new(BinOpKind::Mul, SoulSpan::new(0, 3)),
        Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0, 4)),
    ));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );


    // stream = stream_from_strs(&["(","1", "+", "2", "*", "3", ")", "\n"]);
    // let result = get_expression(&mut stream, &mut scope, &["\n"]);
    // assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // // (1 + (2 * 3))
    // let should_be = ExprKind::Binary(BinaryExpr::new(
    //         Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,0)),
    //         BinOp::new(BinOpKind::Add, SoulSpan::new(0,1)),
    //         Expression::new(
    //             ExprKind::Binary(BinaryExpr::new(
    //                 Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,2)),
    //                 BinOp::new(BinOpKind::Mul, SoulSpan::new(0,3)),
    //                 Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0,4)),
    //             )),
    //             SoulSpan::new(0,3)
    //         )
    //     ));

    // assert_eq_show_diff!(
    //     result.as_ref().unwrap().node,
    //     should_be
    // );
}

#[test]
fn test_simple_unary() {
    let mut stream = stream_from_strs(&["!", "true", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = ExprKind::Unary(UnaryExpr{
        operator: UnaryOp::new(UnaryOpKind::Not, SoulSpan::new(0,0)), 
        expression: Box::new(Expression::new(ExprKind::Literal(Literal::Bool(true)), SoulSpan::new(0,1))),
    });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );


    stream = stream_from_strs(&["-", "1", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = ExprKind::Unary(UnaryExpr{
        operator: UnaryOp::new(UnaryOpKind::Neg, SoulSpan::new(0,0)), 
        expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,1))),
    });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );

    stream = stream_from_strs(&["(","-", "1", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = ExprKind::Unary(UnaryExpr{
        operator: UnaryOp::new(UnaryOpKind::Neg, SoulSpan::new(0,1)), 
        expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,2))),
    });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );
}






























