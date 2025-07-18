use crate::{errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind, SoulSpan}, soul_names::{NamesTypeWrapper, SOUL_NAMES}, steps::{parser::get_expressions::{parse_function_call::get_function_call, parse_variable::get_variable}, step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{BinOp, BinOpKind, BinaryExpr, ExprKind, Expression, Index, OperatorKind, UnaryExpr, UnaryOp, UnaryOpKind, Variable}, literal::Literal, spanned::Spanned, statment::{VariableRef}}, parser_response::FromTokenStream, scope::{ProgramMemmory, ScopeBuilder, ScopeKind}}, i_tokenizer::{Token, TokenStream}}}};

const ROUND_BRACKET_OPEN: SymboolKind = SymboolKind::Parenthesis(Parenthesis::RoundOpen);
const ROUND_BRACKET_CLOSED: SymboolKind = SymboolKind::Parenthesis(Parenthesis::RoundClosed);
const CLOSED_A_BRACKET: bool = true;

pub fn get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str]
) -> Result<Expression> {
    let begin_i = stream.current_index();
    let mut stacks = ExpressionStacks::new();

    let result = convert_expression(stream, scopes, &mut stacks, end_tokens);
    if result.is_err() {
        stream.go_to_index(begin_i);
        return Err(result.unwrap_err());
    }

    while let Some(operator) = stacks.symbool_stack.pop() {

        let expression = match operator.node {
            SymboolKind::BinOp(bin_op_kind) => Expression::new(
                ExprKind::Binary(get_binary_expression(&mut stacks.node_stack, bin_op_kind, operator.span)?), 
                operator.span
            ),
            SymboolKind::UnaryOp(unary_op_kind) => Expression::new(
                ExprKind::Unary(get_unary_expression(&mut stacks.node_stack, unary_op_kind, operator.span)?), 
                operator.span
            ),
            SymboolKind::Parenthesis(..) => stacks.node_stack.pop().unwrap(),
        };

        stacks.node_stack.push(expression);
    }

    if stacks.node_stack.is_empty() {
        assert!(
            !stacks.symbool_stack.is_empty(), 
            "Internal error: in get_expression() stacks.node_stack.is_empty() but typeStack is not"
        );

        return Ok(Expression::new(ExprKind::Empty, stream[begin_i].span));
    }

    if stacks.node_stack.len() > 1 {
        let mut string_builder = String::new();
        let last_index = stream.current_index()-1;
        for i in begin_i..last_index {
            string_builder.push_str(&stream[i].text);
            string_builder.push(' ');
        }

        string_builder.push_str(&stream[last_index].text);
        return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream[begin_i].span, format!("expression: '{}' is invalid (missing operator)", string_builder)))
    }

    Ok(stacks.node_stack.pop().unwrap())
}

fn convert_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
    end_tokens: &[&str]
) -> Result<()> {

    let mut open_bracket_stack = 0i64;

    stream.next_multiple(-1);

    while stream.next().is_some() {
		
        // for catching ')' as endToken, 
        // (YES there are 2 is_end_token() this is because of traverse_brackets() mutates the iterator DONT REMOVE PLZ)
        if is_end_token(stream.current(), end_tokens, open_bracket_stack) {
            return Ok(());
        }

        let literal_begin = stream.current_index();
        let possible_literal = try_get_literal(stream, stacks, scopes, &mut open_bracket_stack)?;

        let possible_scopes = scopes.lookup(stream.current_text());
        let after_generic_index = get_after_generic_index(stream)?;
        
        if is_end_token(stream.current(), end_tokens, open_bracket_stack) {
            return Ok(());
        }
        else if is_ref(stream, stacks) {
            stacks.ref_stack.push(stream.current_text().clone());
        }
        else if let Some(operator) = get_operator(stream, &stacks) {
            try_add_operator(stacks, operator, stream.current_span())?;
        }
        else if let Some(var_ref) = try_get_variable(&possible_scopes) {
            add_variable(stream, stacks, var_ref)?;
        }
        else if let Some(literal) = possible_literal {
            stacks.node_stack.push(Expression::new(ExprKind::Literal(literal), stream.current_span()));
        }
        else if is_function(stream, after_generic_index) {
            let function = get_function_call(stream, scopes)?;
            stacks.node_stack.push(Expression::new(ExprKind::Call(function), stream.current_span()));
        }
        else {

            try_get_special_error(stream, scopes, literal_begin)?;

            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                format!("token: '{}' is not valid expression", stream.current_text())
            ));
        }

        if stream.peek().is_some_and(|token| token.text == "[") {
            add_index(stream, scopes, stacks)?;
        }

        if should_convert_to_ref(stacks) {
            add_ref(stream, scopes, stacks)?;
        }
    }   

    Err(err_out_of_bounds(stream))
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
    fn last_precedence(stacks: &mut ExpressionStacks) -> u8 {
        stacks.symbool_stack.last().unwrap().node.get_precedence()
    }
    
    let current_precedence = operator.get_precedence();

    while !stacks.symbool_stack.is_empty() &&
          last_precedence(stacks) >= current_precedence 
    {
        let operator = stacks.symbool_stack.pop().unwrap();
        let expression = match operator.node {
            SymboolKind::BinOp(bin_op_kind) => Expression::new(
                ExprKind::Binary(get_binary_expression(&mut stacks.node_stack, bin_op_kind, operator.span)?), 
                operator.span
            ),
            SymboolKind::UnaryOp(unary_op_kind) => Expression::new(
                ExprKind::Unary(get_unary_expression(&mut stacks.node_stack, unary_op_kind, operator.span)?), 
                operator.span
            ),
            SymboolKind::Parenthesis(..) => panic!("Internal error this should not be possible, precedence should be 0 and all valid ops > 0"),
        };

        stacks.node_stack.push(expression);
    }

    if operator == OperatorKind::BinOp(BinOpKind::Sub) && 
       is_minus_negative_unary(stacks) 
    {
        operator = OperatorKind::UnaryOp(UnaryOpKind::Neg)
    }

    stacks.symbool_stack.push(to_symbool(operator, span));
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

        if let ExprKind::Literal(literal) = &expression.node {
            let id = scopes.global_literal.insert(literal.clone());

            let name = ProgramMemmory::to_program_memory_name(&id);
            let variable = ExprKind::Variable(Variable{name: name.clone()});
            expression = Expression::new(variable, stream.current_span());
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
        return Ok(peek_i);
    }

    loop {
        peek_i += 1;
        let next = match stream.peek_multiple(peek_i as i64) {
            Some(val) => val,
            None => return Err(err_out_of_bounds(stream)),
        };

        if next.text == ">" {
            peek_i += 1;
            return Ok(peek_i);
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

fn add_variable(stream: &mut TokenStream, stacks: &mut ExpressionStacks, var_ref: &VariableRef) -> Result<()> {
    let variable = get_variable(stream, var_ref)?;
    
    if let Some(literal) = &var_ref.borrow().lit_retention {
        stacks.node_stack.push(Expression::new(ExprKind::Literal(literal.clone()), stream.current_span()));
    }
    else {
        stacks.node_stack.push(Expression::new(ExprKind::Variable(variable), stream.current_span()));
    }

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

fn convert_bracket_expression(stream: &mut TokenStream, stacks: &mut ExpressionStacks) -> Result<()> {
    if stacks.node_stack.len() == 1 {
        let first = stacks.symbool_stack.pop().map(|sy| sy.node);
        let mut second = stacks.symbool_stack.pop();

        if first != Some(ROUND_BRACKET_CLOSED) {
            return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression first symbool is not ')'"));
        }
        else if second.is_none() {
            return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is not None"));
        }

        match &second.as_ref().unwrap().node {
            SymboolKind::BinOp(..) => return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is binary operator")),
            SymboolKind::UnaryOp(unary) => {
                let expr = get_unary_expression(&mut stacks.node_stack, unary.clone(), second.as_ref().unwrap().span)?;
                stacks.node_stack.push(Expression::new(ExprKind::Unary(expr), second.as_ref().unwrap().span));
                second = stacks.symbool_stack.pop();
            },
            SymboolKind::Parenthesis(..) =>(),
        }

        if !second.is_some_and(|sy| sy.node == ROUND_BRACKET_OPEN) {
            return Err(new_soul_error(SoulErrorKind::InternalError, stream.current_span(), "while doing convert_bracket_expression second symbool is not None"));
        }

        return Ok(());
    }

    if !stacks.symbool_stack.pop().is_some_and(|symbool| symbool.node == ROUND_BRACKET_CLOSED) {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis, 
            stream.peek_multiple(-1).unwrap_or(stream.current()).span, 
            "in 'getBracketBinairyExpression()': symoboolStack top is not ')'"
        ));
    }

    while let Some(symbool) = stacks.symbool_stack.last() {
        if symbool.node == ROUND_BRACKET_OPEN {
            break;
        }

        if let SymboolKind::BinOp(bin_op) = &symbool.node {
            let expr = get_binary_expression(&mut stacks.node_stack, bin_op.clone(), symbool.span)?
                .consume_to_expression(symbool.span);

            stacks.node_stack.push(expr);
        }
        else if let SymboolKind::UnaryOp(un_op) = &symbool.node {
            let expr = get_unary_expression(&mut stacks.node_stack, un_op.clone(), symbool.span)?
                .consume_to_expression(symbool.span);
            
            stacks.node_stack.push(expr);
        }

        stacks.symbool_stack.pop();
    }

    Ok(())
}

fn get_binary_expression(
    node_stack: &mut Vec<Expression>,
    bin_op: BinOpKind,
    span: SoulSpan,
) -> Result<BinaryExpr> {
    let right = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found binary operator '{}' but no expression", bin_op.to_str())
        ))?;

    let left = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("missing right expression in binary expression (so '{} {} <missing>')", right.node.to_string(), bin_op.to_str())
        ))?;

    Ok(BinaryExpr{left, operator: BinOp::new(bin_op, span), right})
}

fn get_unary_expression(
    node_stack: &mut Vec<Expression>,
    unary_op: UnaryOpKind,
    span: SoulSpan, 
) -> Result<UnaryExpr> {
    let expr = node_stack.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found unary operator '{}' but no expression", unary_op.to_str())
        ))?;
    
    Ok(UnaryExpr{operator: UnaryOp::new(unary_op, span), expression: Box::new(expr)})
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

fn is_minus_negative_unary(stacks: &ExpressionStacks) -> bool {
    stacks.node_stack.is_empty() || !stacks.symbool_stack.is_empty()
}

fn should_convert_to_ref(stacks: &ExpressionStacks) -> bool {
    !stacks.ref_stack.is_empty() && !stacks.node_stack.is_empty()
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
    stream[after_generic_index].text == "("
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

type Symbool = Spanned<SymboolKind>;

#[derive(Debug, Clone, PartialEq)]
pub enum SymboolKind {
    BinOp(BinOpKind),
    UnaryOp(UnaryOpKind),
    Parenthesis(Parenthesis),
}


#[derive(Debug, Clone, PartialEq)]
pub enum Parenthesis {
    RoundOpen,
    RoundClosed,
}

impl SymboolKind {
    pub fn from_str(name: &str) -> Option<Self> {
        let bin_op = BinOpKind::from_str(name);
        if bin_op != BinOpKind::Invalid {
            return Some(Self::BinOp(bin_op));
        }
        
        let un_op = UnaryOpKind::from_str(name);
        if un_op != UnaryOpKind::Invalid {
            return Some(Self::UnaryOp(un_op));
        }

        match name {
            "(" => Some(ROUND_BRACKET_OPEN),
            ")" => Some(ROUND_BRACKET_CLOSED),
            _ => None
        }
    }

    pub fn get_precedence(&self) -> u8 {
        match self {
            SymboolKind::Parenthesis(..) => 0,
            SymboolKind::BinOp(bin_op_kind) => bin_op_kind.get_precedence(),
            SymboolKind::UnaryOp(unary_op_kind) => unary_op_kind.get_precedence(),
        }
    }

    fn consume_to_symbool(self, span: SoulSpan) -> Symbool {
        Symbool::new(self, span)
    }
}

impl ExpressionStacks {
    pub fn new() -> Self {
        Self { symbool_stack: vec![], ref_stack: vec![], node_stack: vec![] }
    }
}

fn to_symbool(sy: OperatorKind, span: SoulSpan) -> Symbool {
    match sy {
        OperatorKind::BinOp(bin_op_kind) => Symbool::new(SymboolKind::BinOp(bin_op_kind), span),
        OperatorKind::UnaryOp(unary_op_kind) => Symbool::new(SymboolKind::UnaryOp(unary_op_kind), span),
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing exprestion")
}





































































