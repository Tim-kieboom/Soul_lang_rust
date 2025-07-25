use std::collections::BTreeMap;
use ordered_float::OrderedFloat;
use crate::{assert_eq_show_diff, errors::soul_error::{SoulErrorKind, SoulSpan}, steps::{parser::get_expressions::parse_expression::get_expression, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{Arguments, BinOp, BinOpKind, BinaryExpr, ExprKind, Expression, FnCall, Ident, UnaryExpr, UnaryOp, UnaryOpKind, Variable}, literal::{Literal, LiteralType}, soul_type::{soul_type::SoulType, type_kind::TypeKind}, staments::statment::{VariableDecl, VariableRef}}, external_header::ExternalHeader, scope::{ScopeBuilder, ScopeKind, TypeScopeStack}}, i_tokenizer::{Token, TokenStream}}}};

fn stream_from_strs(text_tokens: &[&str]) -> TokenStream {
    let mut line_number = 0;
    let mut line_offset = 0;
    let mut tokens = vec![];
    for text in text_tokens {
        tokens.push(Token{text: text.to_string(), span: SoulSpan::new(line_number, line_offset, text.len())});
        line_offset += text.len();
        if *text == "\n" {
            line_number += 1;
            line_offset = 0;
        }
    }

    TokenStream::new(tokens)
}

fn empty_scope() -> ScopeBuilder {
    ScopeBuilder::new(TypeScopeStack::new(), ExternalHeader::new())
}

// # Literal

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

    stream = stream_from_strs(&["'cc'", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_err());
    assert_eq!(result.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::UnexpectedToken);
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

// # Binary

#[test]
fn test_simple_binary() {
    let mut stream = stream_from_strs(&["1", "+", "2", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let res = result.unwrap();

    let should_be = Expression::new(
        ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0, 0, 1)), 
            BinOp::new(BinOpKind::Add, SoulSpan::new(0,1,1)), 
            Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0, 2, 1)), 
        )),
        SoulSpan::new(0,0,3)
    );

    assert_eq_show_diff!(res, should_be);

    stream = stream_from_strs(&["1", "==", "2", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let res = result.unwrap();

    let should_be = Expression::new(
        ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0, 0, 1)), 
            BinOp::new(BinOpKind::Eq, SoulSpan::new(0, 1, 2)), 
            Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0, 3, 1)), 
        )),
        SoulSpan::new(0,0,4)
    );

    assert_eq_show_diff!(res, should_be);

    stream = stream_from_strs(&["\"hello \"", "+", "\"world\"", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);

    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let res = result.unwrap();

    let should_be = Expression::new(
        ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Str("hello ".into())), SoulSpan::new(0, 0, 8)), 
            BinOp::new(BinOpKind::Add, SoulSpan::new(0, 8, 1)), 
            Expression::new(ExprKind::Literal(Literal::Str("world".into())), SoulSpan::new(0, 9, 7)), 
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // 1 + (2 * 3)
    let should_be = Expression::new(ExprKind::Binary(BinaryExpr::new(
            Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,0,1)),
            BinOp::new(BinOpKind::Add, SoulSpan::new(0,1,1)),
            Expression::new(
                ExprKind::Binary(BinaryExpr::new(
                    Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,2,1)),
                    BinOp::new(BinOpKind::Mul, SoulSpan::new(0,3,1)),
                    Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0,4,1)),
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


    stream = stream_from_strs(&["(","1", "+", "2", ")", "*", "3", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // (1 + 2) * 3
    let should_be = Expression::new(ExprKind::Binary(BinaryExpr::new(
        Expression::new(ExprKind::Binary(BinaryExpr::new(
                    Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1)), 
                    BinOp::new(BinOpKind::Add, SoulSpan::new(0,2,1)), 
                    Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,3,1)),
                )), 
                SoulSpan::new(0,1,3),
            ),
            BinOp::new(BinOpKind::Mul, SoulSpan::new(0,5,1)),
            Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0,6,1)),
        )),
        SoulSpan::new(0,1,6),
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be,
        "2"
    );


    stream = stream_from_strs(&["(","1", "+", "2", "*", "3", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // (1 + (2 * 3))
    let should_be = Expression::new(ExprKind::Binary(BinaryExpr::new(
        Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1)),
            BinOp::new(BinOpKind::Add, SoulSpan::new(0,2,1)),
            Expression::new(
                ExprKind::Binary(BinaryExpr::new(
                    Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,3,1)),
                    BinOp::new(BinOpKind::Mul, SoulSpan::new(0,4,1)),
                    Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0,5,1)),
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = Expression::new(
        ExprKind::Unary(UnaryExpr{
            operator: UnaryOp::new(UnaryOpKind::Not, SoulSpan::new(0,0,1)), 
            expression: Box::new(Expression::new(ExprKind::Literal(Literal::Bool(true)), SoulSpan::new(0,1,4))),
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = Expression::new(
        ExprKind::Unary(UnaryExpr{
            operator: UnaryOp::new(UnaryOpKind::Neg, SoulSpan::new(0,0,1)), 
            expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1))),
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = ExprKind::Unary(UnaryExpr{
        operator: UnaryOp::new(UnaryOpKind::Neg, SoulSpan::new(0,1,1)), 
        expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,2,1))),
    });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );

    stream = stream_from_strs(&["++", "1", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be =  Expression::new(
        ExprKind::Unary(UnaryExpr{
            operator: UnaryOp::new(UnaryOpKind::Incr{before_var: true}, SoulSpan::new(0,0,2)), 
            expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,2,1))),
        }),
        SoulSpan::new(0,0,3)
    );

    let expr = result.unwrap();
    assert_eq_show_diff!(
        expr,
        should_be
    );

    stream = stream_from_strs(&["2", "++", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be =  Expression::new(
        ExprKind::Unary(UnaryExpr{
            operator: UnaryOp::new(UnaryOpKind::Incr{before_var: false}, SoulSpan::new(0,1,2)), 
            expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,0,1))),
        }),
        SoulSpan::new(0,0,3)
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = ExprKind::Binary(BinaryExpr::new(
        Expression::new(ExprKind::Unary(UnaryExpr{
                operator: UnaryOp::new(UnaryOpKind::Neg, SoulSpan::new(0,0,1)), 
                expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1))),
            }), 
            SoulSpan::new(0,0,2),
        ), 
        BinOp::new(BinOpKind::Add, SoulSpan::new(0,2,1)), 
        Expression::new(
            ExprKind::Literal(Literal::Int(8)), 
            SoulSpan::new(0,3,1),
        ), 
    ));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );

    stream = stream_from_strs(&["(", "-", "1", ")", "+", "8", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    let should_be = ExprKind::Binary(BinaryExpr::new(
        Expression::new(ExprKind::Unary(UnaryExpr{
                operator: UnaryOp::new(UnaryOpKind::Neg, SoulSpan::new(0,1,1)), 
                expression: Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,2,1))),
            }), 
            SoulSpan::new(0,1,2),
        ), 
        BinOp::new(BinOpKind::Add, SoulSpan::new(0,4,1)), 
        Expression::new(
            ExprKind::Literal(Literal::Int(8)), 
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
    
    let var = VariableRef::new(VariableDecl{
        name: Ident(var_name.into()), 
        ty: SoulType::from_type_kind(TypeKind::Bool), 
        initializer: Some(Box::new(Expression::new(ExprKind::Literal(Literal::Bool(true)), SoulSpan::new(0,0,var_name.len())))),
        lit_retention: None,
    });
    scope.insert(var_name.into(), ScopeKind::Variable(var));

    
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    let should_be = ExprKind::Variable(Variable{name: Ident(var_name.into())});

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        should_be
    );


    stream = stream_from_strs(&["!", var_name, "\n"]);

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    let should_be = ExprKind::Unary(UnaryExpr{
            operator: UnaryOp::new(UnaryOpKind::Not, SoulSpan::new(0,0,1)),
            expression: Box::new(Expression::new(ExprKind::Variable(Variable{name: Ident(var_name.into())}), SoulSpan::new(0,1,var_name.len()))),
        });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        should_be
    );

    let var_name2 = "var2";
    stream = stream_from_strs(&[var_name2, "\n"]);
    let var2 = VariableRef::new(VariableDecl{
        name: Ident(var_name2.into()), 
        ty: SoulType::from_type_kind(TypeKind::Bool), 
        initializer: None,
        lit_retention: None,
    });
    scope.insert(var_name2.into(), ScopeKind::Variable(var2));

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::InvalidInContext);
}

#[test]
fn test_variable_literal_retention() {
    let var_name = "var";
    let mut stream = stream_from_strs(&[var_name, "\n"]);
    let mut scope = empty_scope();
    
    let var = VariableRef::new(VariableDecl{
        name: Ident(var_name.into()), 
        ty: SoulType::from_type_kind(TypeKind::Bool), 
        initializer: Some(Box::new(Expression::new(ExprKind::Literal(Literal::Bool(true)), SoulSpan::new(0,0,var_name.len())))),
        lit_retention: Some(Expression::new(ExprKind::Literal(Literal::Bool(true)), SoulSpan::new(0,0,var_name.len()))),
    });
    scope.insert(var_name.into(), ScopeKind::Variable(var));

    
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    let should_be = ExprKind::Literal(Literal::Bool(true));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        should_be
    );
}

// # Function

#[test]
fn test_function_call() {
    let mut scope = empty_scope();

    let mut stream = stream_from_strs(&["Println", "(", "\"hello world\"" ,")" , "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let should_be = Expression::new(
        ExprKind::Call(FnCall{
            callee: None, 
            name: Ident("Println".into()), 
            generics: vec![], 
            arguments: vec![
                Arguments{name: None, expression: Expression::new(ExprKind::Literal(Literal::Str("hello world".into())), SoulSpan::new(0,8,13))},
            ],
        }),
        SoulSpan::new(0,0,22)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);

    
    stream = stream_from_strs(&["sum", "(", "1", ",", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let should_be = Expression::new(
        ExprKind::Call(FnCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: vec![
                Arguments{name: None, expression: Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,4,1))},
                Arguments{name: None, expression: Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,6,1))},
            ],
        }),
        SoulSpan::new(0,0,8)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);


    stream = stream_from_strs(&["sum", "()", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let should_be = Expression::new(
        ExprKind::Call(FnCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: vec![],
        }),
        SoulSpan::new(0,0,5)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);


    stream = stream_from_strs(&["sum", "(", "one", "=", "1", ",", "two", "=", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let should_be = Expression::new(
        ExprKind::Call(FnCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: vec![
                Arguments{name: Some(Ident("one".to_string())), expression: Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,8,1))},
                Arguments{name: Some(Ident("two".to_string())), expression: Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,14,1))},
            ],
        }),
        SoulSpan::new(0,0,16)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);


    stream = stream_from_strs(&["sum", "(", "1", ",", "two", "=", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let should_be = Expression::new(
        ExprKind::Call(FnCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: vec![
                Arguments{name: None, expression: Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,4,1))},
                Arguments{name: Some(Ident("two".to_string())), expression: Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,10,1))},
            ],
        }),
        SoulSpan::new(0,0,12)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);


    stream = stream_from_strs(&["sum", "(", "one", "=", "1", ",", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
    let should_be = Expression::new(
        ExprKind::Call(FnCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: vec![
                Arguments{name: Some(Ident("one".to_string())), expression: Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,8,1))},
                Arguments{name: None, expression: Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,10,1))},
            ],
        }),
        SoulSpan::new(0,0,12)
    );
    let expr = result.unwrap();
    assert_eq_show_diff!(expr, should_be);

    stream = stream_from_strs(&["sum", "(", "1", ",", "2", "\n"]);
    let not_closing_fn = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(not_closing_fn.is_err());
    assert_eq!(not_closing_fn.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::UnexpectedEnd, "{}", not_closing_fn.unwrap_err().to_err_message());


    stream = stream_from_strs(&["sum", "(", "1", ",", "name", ":", ")", "\n"]);
    let not_closing_fn = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(not_closing_fn.is_err());
    assert_eq!(not_closing_fn.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::ArgError, "{}", not_closing_fn.unwrap_err().to_err_message());
}

// #[test]
// fn test_methode_call() {
//     let mut stream = stream_from_strs(&["1", ".", "sum", "(", "2", ")", "\n"]);
//     let mut scope = empty_scope();

//     let result = get_expression(&mut stream, &mut scope, &["\n"]);
//     assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message());
//     let should_be = Expression::new(
//         ExprKind::Call(FnCall{
//             callee: Some(Box::new(Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,0,0)))), 
//             name: Ident("sum".into()), 
//             generics: vec![], 
//             arguments: vec![
//                 Arguments{name: None, expression: Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,6,1))},
//             ],
//         }),
//         SoulSpan::new(0,0,10)
//     );
//     let expr = result.unwrap();
//     assert_eq_show_diff!(expr, should_be);
// }











































