use crate::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind, SoulSpan}, soul_names::{NamesOtherKeyWords, NamesTypeWrapper, SOUL_NAMES}, steps::{parser::{get_expressions::{parse_expression_group::try_get_expression_group, parse_function_call::{get_ctor, get_function_call}, parse_operator_expression::{convert_bracket_expression, get_binary_expression, get_unary_expression}, symbool::{to_symbool, Symbool, SymboolKind, ROUND_BRACKET_CLOSED, ROUND_BRACKET_OPEN}}, get_statments::parse_if_else::{get_else, get_if}}, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{BinOp, BinOpKind, ExprKind, Expression, Field, Ident, Index, OperatorKind, StaticField, Ternary, TypeOfExpr, UnaryOp, UnaryOpKind, Variable}, literal::Literal, soul_type::soul_type::SoulType, spanned::Spanned, staments::{conditionals::ElseKind, statment::{VariableRef}}}, parser_response::FromTokenStream, scope::{ProgramMemmory, ScopeBuilder, ScopeKind}}, i_tokenizer::{Token, TokenStream}}}};

const CLOSED_A_BRACKET: bool = true;

pub fn get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str]
) -> Result<Expression> {
    let mut open_bracket_stack = 0i64;
    inner_get_expression(stream, scopes, end_tokens, true, &mut open_bracket_stack, false)
}

pub fn get_expression_options(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str],
    use_literal_retention: bool,
    is_assign_var: bool,
) -> Result<Expression> {
    let mut open_bracket_stack = 0i64;
    inner_get_expression(stream, scopes, end_tokens, use_literal_retention, &mut open_bracket_stack, is_assign_var)
}

fn inner_get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder,
    end_tokens: &[&str],
    use_literal_retention: bool,
    open_bracket_stack: &mut i64,
    is_assign_var: bool,
) -> Result<Expression> {
    let begin_i = stream.current_index();
    let mut stacks = ExpressionStacks::new();

    let result = convert_expression(stream, scopes, &mut stacks, end_tokens, use_literal_retention, open_bracket_stack, is_assign_var);
    if result.is_err() {
        stream.go_to_index(begin_i);
        return Err(result.unwrap_err());
    }

    while let Some(operator) = stacks.symbool_stack.pop() {

        let expression = match operator.node {
            SymboolKind::BinOp(bin_op_kind) => get_binary_expression(&mut stacks.node_stack, BinOp::new(bin_op_kind, operator.span), operator.span)?,
            SymboolKind::UnaryOp(unary_op_kind) => get_unary_expression(&mut stacks.node_stack, UnaryOp::new(unary_op_kind, operator.span), operator.span)?,
            SymboolKind::Parenthesis(..) => stacks.node_stack.pop().unwrap(),
        };

        stacks.node_stack.push(expression);
    }

    if stacks.node_stack.is_empty() {
        assert!(
            stacks.symbool_stack.is_empty(), 
            "Internal error: in get_expression() stacks.node_stack.is_empty() but node_stack is not"
        );

        return Ok(Expression::new(ExprKind::Empty, stream[begin_i].span));
    }

    if stacks.node_stack.len() > 1 {

        let right = stacks.node_stack.pop().unwrap().node;
        let left = stacks.node_stack.pop().unwrap().node;

        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream[begin_i].span, 
            format!("expression: '{}' with '{}' is invalid (missing operator)", left.to_string(), right.to_string())
        ))
    }

    Ok(stacks.node_stack.pop().unwrap())
}

fn convert_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
    end_tokens: &[&str],
    use_literal_retention: bool,
    open_bracket_stack: &mut i64,
    is_assign_var: bool,
) -> Result<()> {

    stream.next_multiple(-1);

    while stream.next().is_some() {
		
        // for catching ')' as endToken, 
        // (YES there are 2 is_end_token() this is because of traverse_brackets() mutates the iterator DONT REMOVE PLZ)
        if is_end_token(stream.current(), end_tokens, *open_bracket_stack) {
            return Ok(());
        }

        let literal_begin = stream.current_index();
        let possible_literal = try_get_literal(stream, stacks, scopes, open_bracket_stack)?;

        if let Some(literal) = possible_literal {
            add_literal(literal, stream, scopes, stacks);
            end_expr_loop(stream, scopes, stacks)?;
            continue;
        }
        else {
            stream.go_to_index(literal_begin);

            if let Some(group) = try_get_expression_group(stream, scopes)? {
                stacks.node_stack.push(group);
                end_expr_loop(stream, scopes, stacks)?;
                continue;
            }
        }

        if try_add_statment_expression(stream, scopes, stacks)? {
            end_expr_loop(stream, scopes, stacks)?;
            continue;
        }

        let possible_scopes = scopes.lookup(stream.current_text());
        let after_generic_index = get_after_generic_index(stream)?;

        if is_end_token(stream.current(), end_tokens, *open_bracket_stack) {
            return Ok(());
        }
        else if is_ref(stream, stacks) {
            stacks.ref_stack.push(stream.current_text().clone());
        }
        else if let Some(operator) = get_operator(stream, &stacks) {
            try_add_operator(stacks, operator, stream.current_span())?;
        }
        else if let Some(var_ref) = try_get_variable(&possible_scopes) {
            add_variable(stream, stacks, var_ref, use_literal_retention, is_assign_var)?;
        }
        else if scopes.lookup_type(&stream.current_text()).is_some() {
            let ty_i = stream.current_index();
            let ty = SoulType::from_stream(stream, &scopes)?;
            let span = stream.current_span().combine(&stream[ty_i].span);
            
            if stream.peek().is_some_and(|token| token.text == "(") {
                let function = get_ctor(Spanned::new(ty, span), stream, scopes)?;
                stacks.node_stack.push(Expression::new(ExprKind::Ctor(function.node), function.span));
            }
            else {
                let mut spanned_ty = Spanned::new(ty, span);
                
                while stream.peek().is_some_and(|token| token.text == "." || token.text == "::") {
                    try_static_field_or_static_methode(&mut spanned_ty, stream, scopes, stacks)?;
                }
            }
        }
        else if is_function(stream, after_generic_index) {
            let function = get_function_call(stream, scopes)?;
            stacks.node_stack.push(Expression::new(ExprKind::Call(function.node), function.span));
        }
        else if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
            if stacks.node_stack.is_empty() {
                return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "trying to use 'typeof' without left expression (so '<missing-left> typeof <type>')"))
            }

            if let Some(sym) = stacks.symbool_stack.last() {
                merge_expressions(stacks, sym.node.get_precedence())?;
            }

            let left = stacks.node_stack.pop().unwrap();
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }

            let ty = SoulType::from_stream(stream, scopes)?;
            let span = left.span.combine(&stream.current_span());
            stacks.node_stack.push(Expression::new(ExprKind::TypeOf(TypeOfExpr{left: Box::new(left), ty}), span));
        }
        else {

            if let Err(err) = try_get_special_error(stream, scopes, literal_begin) {
                return Err(err);
            }

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                format!("token: '{}' is not valid expression", stream.current_text())
            ));
        }

        end_expr_loop(stream, scopes, stacks)?;
    }   

    Err(err_out_of_bounds(stream))
} 

fn try_add_statment_expression(stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<bool> {
    match stream.current_text().as_str() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
            add_if(stream, scopes, stacks)?
        },
        _ => return Ok(false),
    }

    Ok(true)
}

fn add_if(stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    
    fn else_err(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not have 'else' without an 'if'")
    }
    
    loop {

        match stream.current_text().as_str() {
            val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
                let if_decl = get_if(stream, scopes)?;
                stacks.node_stack.push(Expression::new(ExprKind::If(Box::new(if_decl.node)), if_decl.span));
            },
            val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {

                let mut expr = stacks.node_stack.pop()
                    .ok_or(else_err(stream))?;

                let else_branch = get_else(stream, scopes)?;
                let is_else_if = match &else_branch.node {
                    ElseKind::ElseIf(_) => true,
                    ElseKind::Else(_) => false,
                };

                if let ExprKind::If(if_decl) = &mut expr.node {
                    if_decl.else_branchs.push(else_branch);
                }
                else {
                    return Err(else_err(stream))
                }

                stacks.node_stack.push(expr);
                
                if is_else_if {

                    if !stream.peek().is_some_and(|token| token.text == SOUL_NAMES.get_name(NamesOtherKeyWords::Else)) {
                        return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("Expected 'else' or 'else if' after 'if', but found '{}'", stream.peek().unwrap().text)))
                    }
                }
                else {
                    stream.next_multiple(-1);
                    break;
                }

            },
            _ => return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), format!("token: '{}' not allowed after 'if' (try adding 'else' first)", stream.current_text()))),
        }

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    Ok(())
}

fn end_expr_loop(stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    
    if stream.peek().is_some_and(|token| token.text == "[") {
        add_index(stream, scopes, stacks)?;
    }

    while stream.peek().is_some_and(|token| token.text == ".") {
        try_add_field_or_methode(stream, scopes, stacks)?;
    }

    if stream.peek().is_some_and(|token| token.text == "?") {
        add_ternary(stream, scopes, stacks)?;
    }

    if should_convert_to_ref(stacks) {
        add_ref(stream, scopes, stacks)?;
    }

    Ok(())
}

fn add_ternary(stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let condition = if stacks.node_stack.len() == 1 {
        Box::new(stacks.node_stack.pop().unwrap())
    }
    else {
        let last_symbool = stacks.symbool_stack
            .last()
            .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "missing operator"))?; 
        
        merge_expressions(stacks, last_symbool.node.get_precedence())?;
        Box::new(stacks.node_stack.pop().unwrap())
    };

    let if_branch = Box::new(get_expression(stream, scopes, &["\n", ":"])?);
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    if stream.current_text() == ":" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    let else_branch = Box::new(get_expression(stream, scopes, &[";", "\n", "}"])?);
    stream.next_multiple(-1);

    let span = condition.span.combine(&else_branch.span);
    let ternary = Ternary{condition, if_branch, else_branch};
    stacks.node_stack.push(Expression::new(ExprKind::Ternary(ternary), span));
    Ok(())
}

fn try_static_field_or_static_methode(ty: &mut Spanned<SoulType>, stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.peek().is_some_and(|token| token.text == "<" || token.text == "(") {
        add_static_methode(ty, stream, scopes, stacks)
    }
    else {
        add_static_field(ty, stream, stacks)
    }
}

fn try_add_field_or_methode(stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.peek().is_some_and(|token| token.text == "<" || token.text == "(") {
        add_methode(stream, scopes, stacks)
    }
    else {
        add_field(stream, stacks)
    }
}

fn add_static_field(ty: &Spanned<SoulType>, stream: &mut TokenStream, stacks: &mut ExpressionStacks) -> Result<()> {

    let span = ty.span;
    let field = Variable{name: Ident(stream.current_text().clone())};
    let field_expr = Expression::new(ExprKind::StaticField(StaticField{object: ty.clone(), field}), span.combine(&stream.current_span()));
    stacks.node_stack.push(field_expr);
    Ok(())
}

fn add_static_methode(ty: &Spanned<SoulType>, stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {

    let func = get_function_call(stream, scopes)?;
    let methode = func.node.consume_to_static_methode(ty.clone());
    stacks.node_stack.push(Expression::new(ExprKind::StaticMethode(methode), func.span));
    Ok(())
}

fn add_field(stream: &mut TokenStream, stacks: &mut ExpressionStacks) -> Result<()> {
    let object = stacks.node_stack.pop()
        .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "trying to get object of field but no there is no object"))?;

    let field = Variable{name: Ident(stream.current_text().clone())};
    let span = object.span;
    let field_expr = Expression::new(ExprKind::Field(Field{object: Box::new(object), field}), span.combine(&stream.current_span()));
    stacks.node_stack.push(field_expr);
    Ok(())
}

fn add_methode(stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) -> Result<()> {
    let object = stacks.node_stack.pop()
        .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "trying to get object of field but no there is no object"))?;

    let mut func = get_function_call(stream, scopes)?;
    func.node.callee = Some(Box::new(object));
    stacks.node_stack.push(Expression::new(ExprKind::Call(func.node), func.span));
    Ok(())
}

fn add_literal(literal: Literal, stream: &mut TokenStream, scopes: &mut ScopeBuilder, stacks: &mut ExpressionStacks) {
    let expr_kind = match &literal {
        Literal::Tuple{..} |
        Literal::Array{..} |
        Literal::NamedTuple{..} => {
            let literal_ty = literal.get_literal_type();
            let id = scopes.global_literal.insert(literal);
            let name = ProgramMemmory::to_program_memory_name(&id);
            ExprKind::Literal(Literal::ProgramMemmory(name, literal_ty))
        },
        _ => ExprKind::Literal(literal),
    };

    stacks.node_stack.push(Expression::new(expr_kind, stream.current_span()));
}

fn try_get_special_error(stream: &mut TokenStream, scopes: &mut ScopeBuilder, literal_begin: usize) -> Result<()> {
    let begin_i = stream.current_index();
    
    if stream.current_text() == "[" {
        stream.go_to_index(literal_begin);
        let result = Literal::from_stream(stream, scopes);
        if let Err(err) = result {
            return Err(pass_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                "invalid array in expression",
                err
            ));
        }
    }
    else if stream.peek().is_some_and(|token| token.text == "[") {
        stream.go_to_index(literal_begin);
        let result = SoulType::from_stream(stream, scopes);
        if let Err(err) = result {
            return Err(pass_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                "invalid collection type of array in expression",
                err
            ));
        }
    }
    else if stream.current_text() == "(" {
        stream.go_to_index(literal_begin);
        let result = Literal::from_stream(stream, scopes);
        if let Err(err) = result {
            return Err(pass_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                "invalid tuple in expression",
                err
            ));
        }
    }

    stream.go_to_index(begin_i);
    Ok(())
}

fn try_add_operator(
    stacks: &mut ExpressionStacks,
    mut operator: OperatorKind,
    span: SoulSpan
) -> Result<()> {

    merge_expressions(stacks, operator.get_precedence())?;

    if operator == OperatorKind::BinOp(BinOpKind::Sub) && 
       is_minus_negative_unary(stacks) 
    {
        operator = OperatorKind::UnaryOp(UnaryOpKind::Neg)
    }

    stacks.symbool_stack.push(to_symbool(operator, span));
    Ok(())
}

fn merge_expressions(stacks: &mut ExpressionStacks, current_precedence: u8) -> Result<()> {
        fn last_precedence(stacks: &mut ExpressionStacks) -> u8 {
        stacks.symbool_stack.last().unwrap().node.get_precedence()
    }
    
    while !stacks.symbool_stack.is_empty() &&
          last_precedence(stacks) >= current_precedence 
    {
        let operator = stacks.symbool_stack.pop().unwrap();
        let expression = match operator.node {
            SymboolKind::BinOp(bin_op_kind) => get_binary_expression(&mut stacks.node_stack, BinOp::new(bin_op_kind, operator.span), operator.span)?,
            SymboolKind::UnaryOp(unary_op_kind) => get_unary_expression(&mut stacks.node_stack, UnaryOp::new(unary_op_kind, operator.span), operator.span)?,
            SymboolKind::Parenthesis(..) => panic!("Internal error this should not be possible, precedence should be 0 and all valid ops > 0"),
        };

        stacks.node_stack.push(expression);
    }

    Ok(())
}

fn try_get_literal(
    stream: &mut TokenStream, 
    stacks: &mut ExpressionStacks, 
    scopes: &mut ScopeBuilder, 
    open_bracket_stack: &mut i64,
) -> Result<Option<Literal>> {
    
    match Literal::try_from_stream(stream, scopes) {
        Some(Ok(lit)) => Ok(Some(lit)),
        Some(Err(e)) => return Err(e),
        None => {
            if traverse_brackets(stream, stacks, open_bracket_stack) == CLOSED_A_BRACKET {
                convert_bracket_expression(stream, stacks)?;
            }

            match Literal::try_from_stream(stream, scopes) {
                Some(Ok(lit)) => Ok(Some(lit)),
                Some(Err(e)) => return Err(e),
                None => Ok(None),
            }
        },
    }
}

fn add_ref(
    stream: &mut TokenStream,
    scopes: &mut ScopeBuilder,
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    debug_assert!(!stacks.ref_stack.is_empty());
    
    while let Some(to_ref) = stacks.ref_stack.pop() {
        let mut expression = stacks.node_stack.pop()
            .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "trying to ref without expression to ref (e.g. '@'/'&' should be '@obj'/'&obj')"))?;
        
        let is_const_ref = match to_ref.as_str() {
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => true,
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => false,
            _ => return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), format!("Internal error token: '{}' 'let Some(to_ref) = stacks.ref_stack.pop()' is not const or mut ref", to_ref))),
        };

        if let ExprKind::Literal(literal) = expression.node {
            let literal_ty = literal.get_literal_type();
            let id = scopes.global_literal.insert(literal);

            let name = ProgramMemmory::to_program_memory_name(&id);
            let variable = Literal::ProgramMemmory(name, literal_ty);
            expression = Expression::new(ExprKind::Literal(variable), stream.current_span());
        }

        let ref_expr = if is_const_ref {
            Expression::new(ExprKind::ConstRef(Box::new(expression)), stream.current_span())
        }
        else {
            Expression::new(ExprKind::MutRef(Box::new(expression)), stream.current_span())
        };

        stacks.node_stack.push(ref_expr);
    }

    Ok(())
}

fn get_after_generic_index(stream: &TokenStream) -> Result<usize> {
    let mut peek_i = 1;

    if !stream.peek_multiple(peek_i as i64).is_some_and(|token| token.text == "<") {
        return Ok(stream.current_index() + peek_i);
    }

    loop {
        peek_i += 1;
        let next = match stream.peek_multiple(peek_i as i64) {
            Some(val) => val,
            None => return Err(err_out_of_bounds(stream)),
        };

        if next.text == ">" {
            peek_i += 1;
            return Ok(stream.current_index() + peek_i);
        }
    }
}

fn add_index(
    stream: &mut TokenStream,
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    } 

    let begin_i = stream.current_index();
    let index = Box::new(get_expression(stream, scopes, &["]"])
        .map_err(|err| pass_soul_error(
            SoulErrorKind::ArgError, 
            stream[begin_i].span, 
            "while trying to get index", 
            err
        ))?);

    let collection = Box::new(stacks.node_stack.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            "indexer without collection (e.g. '[1]' instead of 'array[1]')"
        ))?);

    let indexer = Expression::new(ExprKind::Index(Index{collection, index}), stream.current_span());
    stacks.node_stack.push(indexer);

    Ok(())
}

fn add_variable(stream: &mut TokenStream, stacks: &mut ExpressionStacks, var_ref: &VariableRef, use_literal_retention: bool, is_assign_var: bool) -> Result<()> {
    
    let var = var_ref.borrow();
    let variable = Variable{name: var.name.clone()};

    if var.initializer.is_none() && !is_assign_var {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            format!("'{}' can not be used before it is assigned", variable.name.0)
        ));
    }
    
    if let Some(literal) = var.lit_retention.clone() {
    
        if use_literal_retention {
            stacks.node_stack.push(literal.clone());
            return Ok(());
        }
    }

    stacks.node_stack.push(Expression::new(
        ExprKind::Variable(variable), 
        stream.current_span()
    ));

    Ok(())
}

fn try_get_variable<'a>(possible_scopes: &'a Option<&Vec<ScopeKind>>) -> Option<&'a VariableRef> {
    
    possible_scopes
        .as_ref()?
        .iter()
        .find_map(|kind| {
            if let ScopeKind::Variable(var) = kind {
                Some(var)
            } else {
                None
            }
        })
}

fn traverse_brackets(
    stream: &mut TokenStream, 
    stacks: &mut ExpressionStacks, 
    open_bracket_stack: &mut i64,
) -> bool {
    let token = stream.current_text();
    if token == "(" {
        let symbool = ROUND_BRACKET_OPEN.consume_to_symbool(stream.current_span());
        
        stacks.symbool_stack.push(symbool);
        stream.next();
        
        *open_bracket_stack += 1;
    } 
    else if token == ")" {
        let symbool = ROUND_BRACKET_CLOSED.consume_to_symbool(stream.current_span());

        stacks.symbool_stack.push(symbool);
        stream.next();

        *open_bracket_stack -= 1;
        if *open_bracket_stack >= 0 {
            return true;
        }
    }

    false
}

fn get_operator(stream: &TokenStream, stacks: &ExpressionStacks) -> Option<OperatorKind> {
    let mut op = OperatorKind::from_str(stream.current_text());
    if op.is_none() || unary_is_before_expr(&op, stacks) {
        return op;
    }

    let operator = op.as_mut().unwrap();
    if let OperatorKind::UnaryOp(unary) = operator {

        match unary {
            UnaryOpKind::Incr{before_var} => *before_var = false,
            UnaryOpKind::Decr{before_var} => *before_var = false,
            _ => (),
        }
    }

    op
}

fn is_minus_negative_unary(stacks: &ExpressionStacks) -> bool {
    stacks.node_stack.is_empty() || !stacks.symbool_stack.is_empty()
}

fn should_convert_to_ref(stacks: &ExpressionStacks) -> bool {
    !stacks.ref_stack.is_empty() && !stacks.node_stack.is_empty()
}

fn unary_is_before_expr(op: &Option<OperatorKind>, stacks: &ExpressionStacks) -> bool {
    op.is_none() || stacks.node_stack.is_empty() || !has_no_operators(stacks) 
}

fn has_no_operators(stacks: &ExpressionStacks) -> bool {
    stacks.symbool_stack.is_empty() || stacks.symbool_stack.iter().all(|sy| matches!(sy.node, SymboolKind::Parenthesis(..)))
}

fn is_ref(stream: &TokenStream, stacks: &ExpressionStacks) -> bool {
    is_token_any_ref(stream.current()) && (stacks.node_stack.is_empty() || !stacks.symbool_stack.is_empty())
}

fn is_token_any_ref(token: &Token) -> bool {
    token.text == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) ||
    token.text == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef)
}

fn is_function(stream: &TokenStream, after_generic_index: usize) -> bool {
    stream[after_generic_index].text == "(" || stream[after_generic_index].text == "()"
}

fn is_end_token(token: &Token, end_tokens: &[&str], open_bracket_stack: i64) -> bool {
    end_tokens.iter().any(|str| str == &token.text) && is_valid_end_token(token, open_bracket_stack)
}

fn is_valid_end_token(token: &Token, open_bracket_stack: i64) -> bool {
    token.text != ")" || (token.text == ")" && open_bracket_stack == 0)
}

pub struct ExpressionStacks {
    pub symbool_stack: Vec<Symbool>,
    pub ref_stack: Vec<String>,
    pub node_stack: Vec<Expression>,
}
impl ExpressionStacks {
    pub fn new() -> Self {
        Self { symbool_stack: vec![], ref_stack: vec![], node_stack: vec![] }
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing exprestion")
}





































































