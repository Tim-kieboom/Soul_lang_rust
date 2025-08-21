use std::collections::{BTreeMap, HashMap};

use ordered_float::OrderedFloat;

use crate:: {assert_eq_show_diff, errors::soul_error::{SoulErrorKind, SoulSpan}, soul_tuple, steps::{parser::expression::parse_expression::get_expression, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{Array, Binary, BinaryOperator, BinaryOperatorKind, Expression, ExpressionGroup, ExpressionKind, Ident, NamedTuple, Tuple, Unary, UnaryOperator, UnaryOperatorKind, VariableName}, function::{FunctionCall}, literal::{Literal, LiteralType}, soul_type::{soul_type::SoulType, type_kind::TypeKind}}, scope_builder::{ProgramMemmory, ProgramMemmoryId, ScopeBuilder, ScopeKind, Variable}}, i_tokenizer::{Token, TokenStream}}}};

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
    ScopeBuilder::new()
}

fn soul_mem_name(id: usize) -> Ident {
    ProgramMemmory::to_program_memory_name(&ProgramMemmoryId(id))
}

// # Literal

#[test]
fn test_simple_literal() {
    let mut stream = stream_from_strs(&["42", "\n"]);
    let mut scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Int(42))
    );

    stream = stream_from_strs(&["-42", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Int(-42))
    );

    stream = stream_from_strs(&["4.2", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Float(OrderedFloat(4.2)))
    );

    stream = stream_from_strs(&["0b11010", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Uint(0b11010))
    );

    stream = stream_from_strs(&["0xfa13", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Uint(0xfa13))
    );

    stream = stream_from_strs(&["'c'", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Char('c'))
    );

    stream = stream_from_strs(&["true", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Bool(true))
    );

    stream = stream_from_strs(&["'c'", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Char('c'))
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
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::Str("string".into()))
    );

    stream = stream_from_strs(&[
        "[",
            "1", ",",
            "2", ",",
            "3",
        "]", "\n"
    ]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(Ident("__soul_mem_0".into()), LiteralType::Array(Box::new(LiteralType::Int))))
    );

    // should get turn by literal into '[12, 12, 12]'
    stream = stream_from_strs(&["[", "for", "3", "=>", "12", "]", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(Ident("__soul_mem_1".into()), LiteralType::Array(Box::new(LiteralType::Int))))
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
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(Ident("__soul_mem_0".into()), LiteralType::Tuple(vec![
            LiteralType::Int,
            LiteralType::Uint,
            LiteralType::Float,
            LiteralType::Bool,
            LiteralType::Str,
            LiteralType::Array(Box::new(LiteralType::Int))
        ])))
    );

    stream = stream_from_strs(&[
        "(",
            "name1", ":", "1", ",",
            "name2", ":", "1.0", ",",
        ")", "\n"
    ]);
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::Literal(Literal::ProgramMemmory(Ident("__soul_mem_0".into()), LiteralType::NamedTuple(BTreeMap::from([
            (Ident("name1".into()), LiteralType::Int),
            (Ident("name2".into()), LiteralType::Float),
        ]))))
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
            Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0, 0, 1)), 
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,1,1)), 
            Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0, 2, 1)), 
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
            Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0, 0, 1)), 
            BinaryOperator::new(BinaryOperatorKind::Eq, SoulSpan::new(0, 1, 2)), 
            Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0, 3, 1)), 
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
            Expression::new(ExpressionKind::Literal(Literal::Str("hello ".into())), SoulSpan::new(0, 0, 8)), 
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0, 8, 1)), 
            Expression::new(ExpressionKind::Literal(Literal::Str("world".into())), SoulSpan::new(0, 9, 7)), 
        )),
        SoulSpan::new(0,0,16)
    );

    assert_eq_show_diff!(res, should_be);
}

#[test]
fn test_multiple_binary() {
    // let mut stream = stream_from_strs(&["1", "+", "2", "*", "3", "\n"]);
    // let mut scope = empty_scope();
    // let result = get_expression(&mut stream, &mut scope, &["\n"]);
    // assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message());

    // // 1 + (2 * 3)
    // let should_be = Expression::new(ExprKind::Binary(BinaryExpr::new(
    //         Expression::new(ExprKind::Literal(Literal::Int(1)), SoulSpan::new(0,0,1)),
    //         BinOp::new(BinOpKind::Add, SoulSpan::new(0,1,1)),
    //         Expression::new(
    //             ExprKind::Binary(BinaryExpr::new(
    //                 Expression::new(ExprKind::Literal(Literal::Int(2)), SoulSpan::new(0,2,1)),
    //                 BinOp::new(BinOpKind::Mul, SoulSpan::new(0,3,1)),
    //                 Expression::new(ExprKind::Literal(Literal::Int(3)), SoulSpan::new(0,4,1)),
    //             )),
    //             SoulSpan::new(0,2,3)
    //         )
    //     )),
    //     SoulSpan::new(0,0,5)
    // );

    // let expr = result.unwrap();
    // assert_eq_show_diff!(
    //     expr,
    //     should_be
    // );

    let mut scope = empty_scope();
    let mut stream = stream_from_strs(&["(","1", "+", "2", ")", "*", "3", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    // (1 + 2) * 3
    let should_be = Expression::new(ExpressionKind::Binary(Binary::new(
        Expression::new(ExpressionKind::Binary(Binary::new(
                    Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1)), 
                    BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,2,1)), 
                    Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,3,1)),
                )), 
                SoulSpan::new(0,1,3),
            ),
            BinaryOperator::new(BinaryOperatorKind::Mul, SoulSpan::new(0,5,1)),
            Expression::new(ExpressionKind::Literal(Literal::Int(3)), SoulSpan::new(0,6,1)),
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    // (1 + (2 * 3))
    let should_be = Expression::new(ExpressionKind::Binary(Binary::new(
        Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1)),
            BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,2,1)),
            Expression::new(
                ExpressionKind::Binary(Binary::new(
                    Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,3,1)),
                    BinaryOperator::new(BinaryOperatorKind::Mul, SoulSpan::new(0,4,1)),
                    Expression::new(ExpressionKind::Literal(Literal::Int(3)), SoulSpan::new(0,5,1)),
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
            expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Bool(true)), SoulSpan::new(0,1,4))),
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
            expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1))),
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
        expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,2,1))),
    });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        should_be
    );

    stream = stream_from_strs(&["++", "1", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be =  Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Increment{before_var: true}, SoulSpan::new(0,0,2)), 
            expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,2,1))),
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be =  Expression::new(
        ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Increment{before_var: false}, SoulSpan::new(0,1,2)), 
            expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,0,1))),
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
    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));

    let should_be = ExpressionKind::Binary(Binary::new(
        Expression::new(ExpressionKind::Unary(Unary{
                operator: UnaryOperator::new(UnaryOperatorKind::Neg, SoulSpan::new(0,0,1)), 
                expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,1,1))),
            }), 
            SoulSpan::new(0,0,2),
        ), 
        BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,2,1)), 
        Expression::new(
            ExpressionKind::Literal(Literal::Int(8)), 
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
                expression: Box::new(Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,2,1))),
            }), 
            SoulSpan::new(0,1,2),
        ), 
        BinaryOperator::new(BinaryOperatorKind::Add, SoulSpan::new(0,4,1)), 
        Expression::new(
            ExpressionKind::Literal(Literal::Int(8)), 
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
    
    let var = Variable{
        name: Ident(var_name.into()), 
        ty: SoulType::from_type_kind(TypeKind::Bool), 
        initialize_value: Some(Expression::new(ExpressionKind::Literal(Literal::Bool(true)), SoulSpan::new(0,0,var_name.len()))),
    };
    scope.insert(var_name.into(), ScopeKind::Variable(var));

    
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    let should_be = ExpressionKind::Variable(VariableName{name: Ident::new(var_name)});

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        should_be
    );


    stream = stream_from_strs(&["!", var_name, "\n"]);

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    let should_be = ExpressionKind::Unary(Unary{
            operator: UnaryOperator::new(UnaryOperatorKind::Not, SoulSpan::new(0,0,1)),
            expression: Box::new(Expression::new(ExpressionKind::Variable(VariableName{name: Ident::new(var_name)}), SoulSpan::new(0,1,var_name.len()))),
        });

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        should_be
    );

    let var_name2 = "var2";
    stream = stream_from_strs(&[var_name2, "\n"]);
    let var2 = Variable{
        name: Ident(var_name2.into()), 
        ty: SoulType::from_type_kind(TypeKind::Bool), 
        initialize_value: None,
    };
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
    
    let var = Variable{
        name: Ident(var_name.into()), 
        ty: SoulType::from_type_kind(TypeKind::Bool), 
        initialize_value: Some(Expression::new(ExpressionKind::Literal(Literal::Bool(true)), SoulSpan::new(0,0,var_name.len()))),
    };
    scope.insert(var_name.into(), ScopeKind::Variable(var));

    
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{:#?}", result.unwrap_err());
    
    let should_be = ExpressionKind::Literal(Literal::Bool(true));

    assert_eq_show_diff!(
        result.as_ref().unwrap().node, 
        should_be
    );
}

// # group (tuple, array and namedTuple that contain non literal expressions)

#[test]
fn test_group_expressions() {
// 
    let mut stream = stream_from_strs(&[
        "[",
            "func", "(", ")", ",",
            "2", ",",
            "3",
        "]", "\n"
    ]);
    let mut scope = empty_scope();

    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(ExpressionKind::FunctionCall(FunctionCall{ callee: None, name: Ident("func".into()), generics: vec![], arguments: Tuple::from([])}), SoulSpan::new(0,1,6)),
        Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,8,1)),
        Expression::new(ExpressionKind::Literal(Literal::Int(3)), SoulSpan::new(0,10,1)),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Array(Array{collection_type: None, element_type: None, values: values.clone()}))
    );

    let mut stream = stream_from_strs(&[
        "[",
            "[", "func", "(", ")", ",", "1", "]", ",",
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
                    Expression::new(ExpressionKind::FunctionCall(FunctionCall{ callee: None, name: Ident("func".into()), generics: vec![], arguments: soul_tuple![]}), SoulSpan::new(0,2,6)),
                    Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,9,1)),
                ], 
                collection_type: None, 
                element_type: None,
            })), 
            SoulSpan::new(0,1,10),
        ),
        Expression::new(
            ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(0), LiteralType::Array(Box::new(LiteralType::Int)))),
            SoulSpan::new(0,14,1),
        ),
        Expression::new(
            ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(1), LiteralType::Array(Box::new(LiteralType::Int)))),
            SoulSpan::new(0,18,1),
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
            "func", "(", ")", ",",
            "2", ",",
            "3",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(ExpressionKind::FunctionCall(FunctionCall{ callee: None, name: Ident("func".into()), generics: vec![], arguments: soul_tuple![]}), SoulSpan::new(0,1,6)),
        Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,8,1)),
        Expression::new(ExpressionKind::Literal(Literal::Int(3)), SoulSpan::new(0,10,1)),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values: values.clone()}))
    );

    stream = stream_from_strs(&[
        "(",
            "func", "(", ")", ",",
            "2", ",",
            "(", "3", ",", "true", ")",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = vec![
        Expression::new(ExpressionKind::FunctionCall(FunctionCall{ callee: None, name: Ident("func".into()), generics: vec![], arguments: soul_tuple![]}), SoulSpan::new(0,1,6)),
        Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,8,1)),
        Expression::new(
            ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(0), LiteralType::Tuple(vec![LiteralType::Int, LiteralType::Bool]))),
            SoulSpan::new(0,17,1),
        ),
    ];

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(Tuple{values: values.clone()}))
    );
//

    stream = stream_from_strs(&[
        "(",
            "field", ":", "func", "(", ")", ",",
            "field2", ":", "2", ",",
            "field3", ":", "3",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = HashMap::from([
        (Ident("field".into()), Expression::new(ExpressionKind::FunctionCall(FunctionCall{ callee: None, name: Ident("func".into()), generics: vec![], arguments: soul_tuple![]}), SoulSpan::new(0,7,6))),
        (Ident("field2".into()), Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,21,1))),
        (Ident("field3".into()), Expression::new(ExpressionKind::Literal(Literal::Int(3)), SoulSpan::new(0,30,1))),
    ]);

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::NamedTuple(NamedTuple{values: values.clone() }))
    );

    stream = stream_from_strs(&[
        "(",
            "field", ":", "func", "(", ")", ",",
            "field2", ":", "2", ",",
            "field3", ":", "(", "field", ":", "1", ")",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    
    let values = HashMap::from([
        (Ident("field".into()), Expression::new(ExpressionKind::FunctionCall(FunctionCall{ callee: None, name: Ident("func".into()), generics: vec![], arguments: soul_tuple![]}), SoulSpan::new(0,7,6))),
        (Ident("field2".into()), Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,21,1))),
        (Ident("field3".into()), Expression::new(ExpressionKind::Literal(Literal::ProgramMemmory(soul_mem_name(0), LiteralType::NamedTuple(BTreeMap::from([(Ident("field".into()), LiteralType::Int)])))), SoulSpan::new(0,38,1))),
    ]);

    assert!(result.is_ok(), "error: {}", result.unwrap_err().to_err_message().join("\n"));
    assert_eq_show_diff!(
        result.as_ref().unwrap().node,
        ExpressionKind::ExpressionGroup(ExpressionGroup::NamedTuple(NamedTuple{values: values.clone() }))
    );

    stream = stream_from_strs(&[
        "(",
            "field", ":", "func", "(", ")", ",",
            "field", ":", "2", ",",
        ")", "\n"
    ]);
    
    scope = empty_scope();
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_err(), "{:#?}", result.unwrap());
    assert_eq!(result.unwrap_err().get_last_kind(), SoulErrorKind::InvalidName);

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


    stream = stream_from_strs(&["sum", "(", "one", "=", "1", ",", "two", "=", "2", ")", "\n"]);
    let result = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(result.is_ok(), "{}", result.unwrap_err().to_err_message().join("\n"));
    let should_be = Expression::new(
        ExpressionKind::FunctionCall(FunctionCall{
            callee: None, 
            name: Ident("sum".into()), 
            generics: vec![], 
            arguments: soul_tuple![
                Expression::new(ExpressionKind::Literal(Literal::Int(1)), SoulSpan::new(0,8,1)),
                Expression::new(ExpressionKind::Literal(Literal::Int(2)), SoulSpan::new(0,14,1)),
            ],
        }),
        SoulSpan::new(0,0,16)
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
    stream = stream_from_strs(&["sum", "(", "1", ",", "name", ":", ")", "\n"]);
    let not_closing_fn = get_expression(&mut stream, &mut scope, &["\n"]);
    assert!(not_closing_fn.is_err());
    assert_eq!(not_closing_fn.as_ref().unwrap_err().get_last_kind(), SoulErrorKind::ArgError, "{}", not_closing_fn.unwrap_err().to_err_message().join("\n"));
}
















