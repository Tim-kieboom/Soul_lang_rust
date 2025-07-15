use crate::{errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{expression::{BinOp, BinOpKind, BinaryExpr, Expression, UnaryExpr, UnaryOp, UnaryOpKind}, soul_type::soul_type::SoulType, spanned::Spanned}, scope::ScopeBuilder}, i_tokenizer::{Token, TokenStream}}};

const ROUND_BRACKET_OPEN: SymboolKind = SymboolKind::Parenthesis(Parenthesis::RoundOpen);
const ROUND_BRACKET_CLOSED: SymboolKind = SymboolKind::Parenthesis(Parenthesis::RoundClosed);
const CLOSED_A_BRACKET: bool = true;

pub fn get_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    should_be_type: &Option<&SoulType>, 
    end_tokens: &[&str]
) -> Result<Expression> {
    let begin_i = stream.current_index();
    let mut stacks = ExpressionStacks::new();

    convert_expression(stream, scopes, &mut stacks, should_be_type, end_tokens)?;
    
    todo!()
    // Ok(())
}

fn convert_expression(
    stream: &mut TokenStream, 
    scopes: &mut ScopeBuilder, 
    stacks: &mut ExpressionStacks,
    should_be_type: &Option<&SoulType>, 
    end_tokens: &[&str]
) -> Result<()> {

    let mut open_bracket_stack = 0i64;

    stream.next_multiple(-1);

    let mut prev_token_index = stream.current_index();
    while stream.next().is_some() {
		
        // for catching ')' as endToken, 
        // (YES there are 2 isEndToken() this is because of checkBrackets() mutates the iterator DONT REMOVE PLZ)
        if is_end_token(stream.current(), end_tokens, open_bracket_stack) {
            return Ok(());
        }

        if traverse_brackets(stream, stacks, &mut open_bracket_stack) == CLOSED_A_BRACKET {
            convert_bracket_expression(stream, stacks)?;
        }

        // Literal::


    }

    Ok(())
}

fn convert_bracket_expression(stream: &mut TokenStream, stacks: &mut ExpressionStacks) -> Result<()> {
    if stacks.node_stack.len() == 1 {
        let first = stacks.symbool_stack.pop().map(|sy| sy.node);
        let second = stacks.symbool_stack.pop().map(|sy| sy.node);

        assert_eq!(first, Some(ROUND_BRACKET_CLOSED));
        assert_eq!(second, Some(ROUND_BRACKET_OPEN));
        return Ok(());
    }

    if stacks.symbool_stack.pop().is_none_or(|symbool| symbool.node != ROUND_BRACKET_CLOSED) {
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
            let expr = get_binair_expression(&mut stacks.node_stack, bin_op.clone(), symbool.span)?
                .consume_to_expression(symbool.span);

            stacks.node_stack.push(expr);
        }
        else if let SymboolKind::UnaryOp(un_op) = &symbool.node {
            let expr = get_unary_expression(&mut stacks.node_stack, un_op.clone(), symbool.span)?
                .consume_to_expression(symbool.span);
            
            stacks.node_stack.push(expr);
        }

    }

    Ok(())
}

fn get_binair_expression(
    node_stack: &mut Vec<Expression>,
    bin_op: BinOpKind,
    span: SoulSpan,
) -> Result<BinaryExpr> {
    let left = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("found binary operator '{}' but no left expression", bin_op.to_str())
        ))?;

    let right = node_stack.pop()
        .map(|expr| Box::new(expr))
        .ok_or(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            span, 
            format!("missing right expression in binary expression (so '{} {} <missing>')", left.node.to_string(), bin_op.to_str())
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

#[inline(always)]
fn is_end_token(token: &Token, end_tokens: &[&str], open_bracket_stack: i64) -> bool {
    end_tokens.iter().any(|str| str == &token.text) && is_valid_end_token(token, open_bracket_stack)
}

#[inline(always)]
fn is_valid_end_token(token: &Token, open_bracket_stack: i64) -> bool {
    token.text != ")" || (token.text == ")" && open_bracket_stack == 0)
}

struct ExpressionStacks {
    pub symbool_stack: Vec<Symbool>,
    pub type_stack: Vec<SoulType>,
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
    fn from_str(name: &str, span: SoulSpan) -> Option<Self> {
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

    fn consume_to_symbool(self, span: SoulSpan) -> Symbool {
        Symbool::new(self, span)
    }
}

impl ExpressionStacks {
    pub fn new() -> Self {
        Self { symbool_stack: vec![], type_stack: vec![], node_stack: vec![] }
    }
}



















