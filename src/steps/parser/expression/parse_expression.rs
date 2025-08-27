use std::mem;
use crate::steps::step_interfaces::i_tokenizer::Token;
use crate::steps::step_interfaces::i_parser::scope_builder::{ProgramMemmory};
use crate::steps::parser::expression::parse_conditional::try_get_conditional;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::literal::Literal;
use crate::steps::parser::expression::symbool::{Bracket, Operator, Symbool, SymboolKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::pretty_format::ToString;
use crate::soul_names::{check_name_allow_types, could_be_name, NamesTypeWrapper, SOUL_NAMES};
use crate::steps::step_interfaces::{i_parser::scope_builder::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::parser::expression::parse_expression_groups::{get_function_call, try_get_expression_group};
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::parser::expression::merge_expression::{convert_bracket_expression, get_binary_expression, get_unary_expression, merge_expressions};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{AccessField, BinaryOperatorKind, Expression, ExpressionGroup, ExpressionKind, Index, Ternary, Tuple, UnaryOperatorKind, VariableName};


pub struct ExprOptions {
    /// `make 0 unless you are in a bracket`
    /// ```
    /// if '(' {round_bracket_stack += 1}
    /// else if ')' {round_bracket_stack -= 1}
    /// ```
    pub round_bracket_stack: i64,
}

impl Default for ExprOptions {
    fn default() -> Self {
        Self { 
            round_bracket_stack: 0,
        }
    }
}

pub fn get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str],
) -> Result<Expression> {
    inner_get_expression(stream, scopes, &mut ExprOptions::default(), false, end_tokens)
}

pub fn get_expression_options(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    mut options: ExprOptions,
    end_tokens: &[&str],
) -> Result<Expression> {
    inner_get_expression(stream, scopes, &mut options, false, end_tokens)
}

pub fn get_expression_statment(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    end_tokens: &[&str],
) -> Result<Expression> {
    inner_get_expression(stream, scopes, &mut ExprOptions::default(), true, end_tokens)
}

pub fn get_expression_statment_options(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    mut options: ExprOptions,
    end_tokens: &[&str],
) -> Result<Expression> {
    inner_get_expression(stream, scopes, &mut options, true, end_tokens)
}

fn inner_get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    options: &mut ExprOptions,
    is_statment: bool,
    end_tokens: &[&str],
) -> Result<Expression> {
    let begin_i = stream.current_index();
    let mut stacks = ExpressionStacks::new();

    loop {   
        
        match convert_expression(stream, scopes, &mut stacks, end_tokens, is_statment, options) {
            Ok(true) => break,
            Ok(false) => {
                stream.next();
                continue
            },
            Err(err) => { 
                stream.go_to_index(begin_i);
                return Err(err)
            },
        }
    }

    while let Some(operator) = stacks.symbools.pop() {

        let span = operator.span;
        let expression = match operator.node {
            SymboolKind::BinaryOperator(binary_operator) => get_binary_expression(&mut stacks, binary_operator, span)?,
            SymboolKind::UnaryOperator(unary_operator) => get_unary_expression(&mut stacks, unary_operator, span)?,
            SymboolKind::Bracket(_) => stacks.expressions.pop()
                .expect("Internal Error found Symbool::Bracket in convert expression while expressionStack is empty"),
        };
    
        stacks.expressions.push(expression);
    }

    if stacks.expressions.is_empty() {
        debug_assert!(
            stacks.symbools.is_empty(), 
            "stacks.symbools should be made empty before this"
        );

        return Ok(Expression::new(ExpressionKind::Empty, stream[begin_i].span));
    }

    if stacks.expressions.len() > 1 {

        let right = stacks.expressions.pop().unwrap().node;
        let left = stacks.expressions.pop().unwrap().node;

        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream[begin_i].span, 
            format!("expression: '{}' with '{}' is invalid (missing operator)", left.to_string(), right.to_string())
        ))
    }

    Ok(stacks.expressions.pop().unwrap())
}

fn convert_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
    end_tokens: &[&str],
    is_statment: bool,
    options: &mut ExprOptions,
) -> Result<bool> {
    const CLOSED_A_BRACKET: bool = true;
    const BREAK_LOOP: Result<bool> = Ok(true);
    const CONTINUE_LOOP: Result<bool> = Ok(false);

    if is_end_token(stream.current(), end_tokens, options) {
        return BREAK_LOOP
    }

    let first_literal_begin = stream.current_index();

    if let Some(literal) = Literal::try_from_stream(stream, scopes)? {

        add_literal(literal, stream, scopes, stacks, first_literal_begin);
        end_loop(stream, scopes, stacks)?;
        return CONTINUE_LOOP
    }

    let begin_i = stream.current_index();
    stream.go_to_index(first_literal_begin);
    if let Some(group) = try_get_expression_group(stream, scopes)? {
        
        add_group_expressions(group, stacks)?;
        end_loop(stream, scopes, stacks)?;
        return CONTINUE_LOOP
    }
    else {
        stream.go_to_index(begin_i);
    }

    if let CLOSED_A_BRACKET = traverse_brackets(stream, stacks, options) {
        convert_bracket_expression(stream, stacks)?;
    }
    
    let second_literal_begin = stream.current_index();
    if let Some(literal) = Literal::try_from_stream(stream, scopes)? {

        add_literal(literal, stream, scopes, stacks, second_literal_begin);
        end_loop(stream, scopes, stacks)?;
        return CONTINUE_LOOP
    }

    if try_get_conditional(stream, scopes, stacks, is_statment)? {
        stream.next_multiple(-1);
        end_loop(stream, scopes, stacks)?;
        return CONTINUE_LOOP
    }

    if is_end_token(stream.current(), end_tokens, options) {
        return BREAK_LOOP
    }

    if let Some(ref_kind) = get_ref(stream, stacks) {
        stacks.refs.push(ref_kind);
    }
    else if let Some(operator) = get_operator(stream, stacks) {
        try_add_operator(stacks, operator, stream.current_span())?;
    }
    else if could_be_variable(stream) {
        
        let variable = VariableName::new(stream.current_text());
        stacks.expressions.push(
            Expression::new(
                ExpressionKind::Variable(variable), 
                stream.current_span()
            )
        );
    }
    else {

        let current = stream.current_text();

        if could_be_name(current) {

            check_name_allow_types(current)
                .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;
        }

        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken,
            stream.current_span(), 
            format!("token: '{}' is not valid expression", stream.current_text())
        ));
    }

    end_loop(stream, scopes, stacks)?;
    return CONTINUE_LOOP
}



pub fn traverse_brackets(
    stream: &mut TokenStream, 
    stacks: &mut ExpressionStacks, 
    options: &mut ExprOptions,
) -> bool {

    const START: &str = "(";
    const END: &str = ")";

    let token = stream.current_text();
    if token == START {
        stacks.symbools.push(Symbool::new(SymboolKind::Bracket(Bracket::RoundOpen), stream.current_span()));
        stream.next();
        
        options.round_bracket_stack += 1;
    } 
    else if token == END {
        stacks.symbools.push(Symbool::new(SymboolKind::Bracket(Bracket::RoundClose), stream.current_span()));
        stream.next();

        options.round_bracket_stack -= 1;
        if options.round_bracket_stack >= 0 {
            return true;
        }
    }

    false
}

pub fn add_literal(
    literal: Literal, 
    stream: &TokenStream,
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
    begin_i: usize,
) {

    let span = stream[begin_i].span.combine(&stream.current_span());

    let expression = match &literal {
        Literal::Tuple{..} |
        Literal::Array{..} |
        Literal::NamedTuple{..} => {
            let literal_type = literal.get_literal_type();
            let id = scopes.global_literals.insert(literal);
            let name = ProgramMemmory::to_program_memory_name(&id);
            ExpressionKind::Literal(Literal::ProgramMemmory(name, literal_type))
        },
        _ => ExpressionKind::Literal(literal),
    };

    stacks.expressions.push(Expression::new(expression, span));
}

fn add_group_expressions(mut group: Expression, stacks: &mut ExpressionStacks) -> Result<()> {

    if let ExpressionKind::ExpressionGroup(ExpressionGroup::Tuple(tuple)) = &mut group.node {
        
        if is_single_tuple(&tuple) {
            stacks.expressions.push(mem::take(&mut tuple.values[0]));
            return Ok(())
        }
    }
    
    stacks.expressions.push(group);
    Ok(())
}

fn try_add_operator(
    stacks: &mut ExpressionStacks,
    mut operator: Operator,
    span: SoulSpan
) -> Result<()> {

    merge_expressions(stacks, operator.get_precedence())?;

    if operator == Operator::Binary(BinaryOperatorKind::Sub) && 
       is_minus_negative_unary(stacks) 
    {
        operator = Operator::Unary(UnaryOperatorKind::Neg)
    }

    stacks.symbools.push(operator.to_symbool(span));
    Ok(())
}


fn get_operator(stream: &TokenStream, stacks: &ExpressionStacks) -> Option<Operator> {
    let mut operator = Operator::from_str(stream.current_text());
    if unary_is_before_expression(&operator, stacks) {
        return operator
    }

    match &mut operator {
        Some(Operator::Unary(UnaryOperatorKind::Increment{before_var})) => *before_var = false,
        Some(Operator::Unary(UnaryOperatorKind::Decrement{before_var})) => *before_var = false,
        Some(_) => (),
        None => return None,
    }

    operator
}

fn add_ternary(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let condition = if stacks.expressions.len() == 1 {
        Box::new(stacks.expressions.pop().unwrap())
    }
    else {
        let last_symbool = stacks.symbools
            .last()
            .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "missing operator"))?; 

        merge_expressions(stacks, last_symbool.node.get_precedence())?;
        Box::new(stacks.expressions.pop().unwrap())
    };

    let if_branch = Box::new(get_expression(stream, scopes, &["\n", ":"])?);
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if("\n").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    if stream.next_if(":").is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let else_branch = Box::new(get_expression(stream, scopes, &[";", "\n", "}"])?);
    stream.next_multiple(-1);

    let span = condition.span.combine(&else_branch.span);
    let ternary = Ternary{condition, else_branch, if_branch};
    stacks.expressions.push(Expression::new(ExpressionKind::Ternary(ternary), span));
    Ok(())
}

fn add_field_or_methode(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
) -> Result<()> {
    if stream.next_multiple(2).is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.peek_is("<") || stream.peek_is("(") {
        let start_i = stream.current_index();
        let result = add_methode(stream, scopes, stacks);
        
        // Ambiguity handling:
        // if for example 'val.0 < val.1' this is seen as a methode 
        // because of the '.' in 'val.0' before '<' (the '<' makes it seem like a genric in methode)
        // so is methode fails it might be a field instead
        if let Err(err) = result {

            stream.go_to_index(start_i);
            match add_field(stream, stacks) {
                Ok(val) => Ok(val),
                Err(_) => Err(err),
            }
        }
        else {
            result
        }
    }
    else {
        add_field(stream, stacks)
    }
}

fn add_field(stream: &mut TokenStream, stacks: &mut ExpressionStacks) -> Result<()> {
    let object = stacks.expressions.pop()
        .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "trying to get object of field but no there is no object"))?;

    let span = object.span;
    let field = VariableName::new(stream.current_text());
    let field_expr = Expression::new(
        ExpressionKind::AccessField(
            AccessField{object: Box::new(object), field}
        ),
        span.combine(&stream.current_span(),
    ));

    stacks.expressions.push(field_expr);
    Ok(())
}

fn add_methode(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    let object = stacks.expressions.pop()
        .ok_or(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "trying to get object of field but no there is no object"))?;

    let mut function = match get_function_call(stream, scopes) {
        Ok(val) => val,
        Err(err) => {stacks.expressions.push(object); return Err(err)},
    };

    function.node.callee = Some(Box::new(object));
    let expression = Expression::new(
        ExpressionKind::FunctionCall(function.node), 
        function.span,
    );

    stacks.expressions.push(expression);
    Ok(())
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

    let collection = Box::new(stacks.expressions.pop()
        .ok_or(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            "indexer without collection (e.g. '[1]' instead of 'array[1]')"
        ))?);

    let indexer = Expression::new(ExpressionKind::Index(Index{collection, index}), stream.current_span());
    stacks.expressions.push(indexer);

    Ok(())
}

fn add_ref(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
) -> Result<()> {
    debug_assert!(!stacks.refs.is_empty());

    while let Some(ref_kind) = stacks.refs.pop() {
        let mut expression = stacks.expressions.pop()
            .ok_or(new_soul_error(
                SoulErrorKind::InvalidInContext, 
                stream.current_span(), 
                "trying to ref without expression to ref (e.g. '@'/'&' should be '@obj'/'&obj')"
            ))?;

        if let ExpressionKind::Literal(literal) = expression.node {
            expression = turn_into_program_memory(literal, scopes, expression.span);            
        }

        let span = expression.span.combine(&ref_kind.span);
        let any_ref = match ref_kind.node {
            RefKind::Deref => Expression::new(ExpressionKind::Deref(Box::new(expression)), span),
            RefKind::MutRef => Expression::new(ExpressionKind::MutRef(Box::new(expression)), span),
            RefKind::ConstRef => Expression::new(ExpressionKind::ConstRef(Box::new(expression)), span),
        };

        stacks.expressions.push(any_ref);
    }

    Ok(())
}

fn turn_into_program_memory(
    literal: Literal, 
    scopes: &mut ScopeBuilder, 
    span: SoulSpan,
) -> Expression {
    let (name, literal_type) = match literal {
        Literal::ProgramMemmory(name, ty) => (name, ty),
        _ => {
            let ty = literal.get_literal_type();
            let id = scopes.global_literals.insert(literal);
            (ProgramMemmory::to_program_memory_name(&id), ty)
        }
    };

    let program_memory = Literal::ProgramMemmory(name, literal_type);
    Expression::new(ExpressionKind::Literal(program_memory), span)
}

fn end_loop(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks, 
) -> Result<()> { 

    loop {
        
        let symbool = AfterExpressionSymbools::from_context(stream, stacks);
        match symbool {
            AfterExpressionSymbools::Ref => add_ref(stream, scopes, stacks)?,
            AfterExpressionSymbools::Index => add_index(stream, scopes, stacks)?,
            AfterExpressionSymbools::Ternary => add_ternary(stream, scopes, stacks)?,
            AfterExpressionSymbools::FieldOrMethode => add_field_or_methode(stream, scopes, stacks)?,

            AfterExpressionSymbools::None => break,
        }
    }

    Ok(())
}


fn get_ref(stream: &TokenStream, stacks: &ExpressionStacks) -> Option<Spanned<RefKind>> {

    if could_be_ref(stacks) {

        RefKind::from_str(&stream.current_text())
            .map(|el| Spanned::new(el, stream.current_span()))
    }
    else {
        None
    }
}

enum AfterExpressionSymbools {
    None,
    Index,
    FieldOrMethode,
    Ref,
    Ternary
}

impl AfterExpressionSymbools {
    pub fn from_context(stream: &TokenStream, stacks: &ExpressionStacks) -> Self {

        if Self::should_convert_to_ref(stacks) {
            return Self::Ref
        }
        
        let peek_i = if stream.peek_is("\n") {2} else {1};

        let token = match stream.peek_multiple(peek_i) {
            Some(val) => val.text.as_str(),
            None => return Self::None,
        };

        match token {
            "[" => Self::Index,
            "." => Self::FieldOrMethode,
            "?" => Self::Ternary,
            _ => Self::None,
        }
    }

    fn should_convert_to_ref(stacks: &ExpressionStacks) -> bool {
        !stacks.refs.is_empty() && !stacks.expressions.is_empty()
    }
}

fn could_be_variable(stream: &mut TokenStream) -> bool {
    check_name_allow_types(stream.current_text()).is_ok()
}

fn is_single_tuple(tuple: &Tuple) -> bool {
    tuple.values.len() == 1
}

fn is_minus_negative_unary(stacks: &ExpressionStacks) -> bool {
    stacks.expressions.is_empty() || 
    !stacks.symbools.is_empty()
}

fn unary_is_before_expression(operator: &Option<Operator>, stacks: &ExpressionStacks) -> bool {
    operator.is_none() || 
    stacks.expressions.is_empty() || 
    !has_no_operators(stacks) 
}

fn has_no_operators(stacks: &ExpressionStacks) -> bool {
    stacks.symbools.is_empty() || 
    stacks.symbools.iter().all(|sy| matches!(sy.node, SymboolKind::Bracket(..)))
}

fn is_end_token(token: &Token, end_tokens: &[&str], options: &ExprOptions) -> bool {
    end_tokens.iter().any(|str| str == &token.text) && 
    end_token_special_cases(token, options)

}
fn end_token_special_cases(token: &Token, options: &ExprOptions) -> bool {
    token.text != ")" || 
    // Special case: if one of the end_tokens is ')',
    // it is only valid if there are no unmatched opening
    // parentheses left in the stack.
    // This ensures that all "(" have been properly closed.
    (token.text == ")" && options.round_bracket_stack == 0)
}

fn could_be_ref(stacks: &ExpressionStacks) -> bool {
    // Case 1: If there are no prior expressions, then `*`/`@`/`&` must be a ref symbool.
    // Example: `*ref`
    stacks.expressions.is_empty() || 
    // Case 2: If the counts of operators (`symbools`) and expressions line up,
    // then `*`/`@`/`&` is more likely acting as a ref symbool than as operator.
    // Example: For `1 + 2 * 3`, we have symbols.len() = 1 and expressions.len() = 2,
    // so it's treated as operator instead of ref symbool.
    stacks.symbools.len() == stacks.expressions.len() 
}

#[derive(Debug, Clone, Default)]
pub struct ExpressionStacks {
    pub expressions: Vec<Expression>,
    pub symbools: Vec<Symbool>,
    pub refs: Vec<Spanned<RefKind>>,
}

#[derive(Debug, Clone)]
pub enum RefKind {
    ConstRef,
    MutRef,
    Deref,
}

impl ExpressionStacks {
    pub fn new() -> Self {
        Self{..Default::default()}
    }
}

impl RefKind {
    pub fn from_str(text: &str) -> Option<Self> {
        match text {
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::MutRef) => Some(Self::MutRef),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::Pointer) => Some(Self::Deref),
            val if val == SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef) => Some(Self::ConstRef),
            _ => None,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            RefKind::Deref => SOUL_NAMES.get_name(NamesTypeWrapper::Pointer),
            RefKind::MutRef => SOUL_NAMES.get_name(NamesTypeWrapper::MutRef),
            RefKind::ConstRef => SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef),
        }
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while parsing exprestion")
}







































