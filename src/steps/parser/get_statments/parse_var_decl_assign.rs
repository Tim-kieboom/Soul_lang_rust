use std::collections::HashMap;

use itertools::Itertools;
use once_cell::sync::Lazy;
use crate::steps::parser::soul_type::get_soul_type::FromWithPath;
use crate::steps::step_interfaces::i_parser::scope::ScopeKind;
use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::{i_parser::scope::ScopeBuilder, i_tokenizer::TokenStream};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::errors::soul_error::{new_soul_error, pass_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::parser::get_expressions::parse_expression::{get_expression, get_expression_options};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{Modifier, TypeKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{BinOp, BinOpKind, BinaryExpr, BoxExpr, ExprKind, Expression, Ident};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::{Assignment, VariableKind, VariableDecl, VariableRef, STATMENT_ENDS};

static ASSIGN_END_TOKENS: Lazy<Vec<&str>> = Lazy::new(|| {
    SOUL_NAMES
        .assign_symbools
        .iter()
        .map(|(_, symbool)| *symbool)
        .filter(|symbool| *symbool != "." && *symbool != "[")
        .collect::<Vec<&str>>()
});

pub fn get_var_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<VariableKind>> {
    inner_get_var_decl(stream, scopes, false)
}

pub fn get_unwrap_var(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<VariableKind>> {
    inner_get_var_decl(stream, scopes, true)
}

pub fn inner_get_var_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder, force_unwrap: bool) -> Result<Spanned<VariableKind>> {
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get initialization of variable")
    }

    let begin_i = stream.current_index();
    let possible_type = match SoulType::try_from_stream_with_path(stream, scopes) {
        Some(result) => {
            let ty = result?;
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream))
            }
            if scopes.is_in_global() && ty.modifier.is_mutable() {
                return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "global variable can not be mutable"))
            }
            Some(ty)
        },
        None => None,
    };

    let is_type_invered = possible_type.is_none();
    let is_let = stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Let);

    let modifier = if is_type_invered {
        let modi = Modifier::from_str(stream.current_text());
        if modi != Modifier::Default || stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Let) {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        modi
    }
    else {
        possible_type.as_ref()
            .unwrap()
            .modifier
            .clone()
    };

    let var_kind = if stream.current_text() == "(" {
        VarKind::Unwrap(parse_unwrap_pattern(stream, scopes)?)
    }
    else if force_unwrap {
        VarKind::UnionBinding(parse_union_binding(&possible_type, stream, scopes)?)
    }
    else {
        let var_name_index = stream.current_index();

        check_if_var_valid(&stream[var_name_index].text, stream[var_name_index].span, scopes)?;
        VarKind::Variable(Ident(stream[var_name_index].text.clone()))
    };

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if STATMENT_ENDS.iter().any(|sym| sym == stream.current_text())  {

        if scopes.is_in_global() {
            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                stream.current_span(), 
                format!("global variables HAVE TO BE assigned at init, variable '{}' is not assigned", var_kind.to_string())
            ));
        }

        let ty = if is_type_invered {
            SoulType::from_type_kind(TypeKind::None)
        } 
        else {
            possible_type.unwrap()
        };

        let variable_kind = get_variable_kind(var_kind, ty, scopes, None, None);
        return Ok(Spanned::new(variable_kind, stream[begin_i].span.combine(&stream.current_span())));
    }

    if is_type_invered {

        if modifier == Modifier::Default &&
           (stream.current_text() != ":=" && !is_let)
        {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken, 
                stream.current_span(), 
                format!("'{}' is not allowed at end of default type invered initialize variable (use ':=')", stream.current_text())
            ));
        }
    }
    else if stream.current_text() != "=" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("'{}' is not allowed at end of initialize variable (use '=')", &stream.current().text)
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let end_tokens = if force_unwrap {&["{", "\n"]} else {STATMENT_ENDS};

    if is_type_invered {

        if scopes.is_in_global() && modifier.is_mutable() {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "global variable can not be mutable"))
        }

        let begin_i = stream.current_index();
        let expr = get_expression(stream, scopes, end_tokens)
            .map_err(|err| pass_soul_error(err.get_last_kind(), stream[begin_i].span, format!("while trying to get assignment of variable: '{}'", var_kind.to_string()).as_str(), err))?;

        let (lit_retention, mut ty) = match &expr.node {
            ExprKind::Literal(literal) => (Some(expr.clone()), literal.to_soul_type()),
            _ => (None, SoulType::none()),
        };

        ty.modifier = modifier;
        ty.base = ty.base.untyped_to_typed();

        let variabel_kind = get_variable_kind(var_kind, ty, scopes, lit_retention, Some(Box::new(expr)));
        Ok(Spanned::new(variabel_kind, stream[begin_i].span.combine(&stream.current_span())))
    }
    else {
        let begin_i = stream.current_index();
        let expr = get_expression(stream, scopes, end_tokens)
            .map_err(|err| pass_soul_error(err.get_last_kind(), stream[begin_i].span, format!("while trying to get assignment of variable: '{}'", var_kind.to_string()).as_str(), err))?;

        let lit_retention = match &expr.node {
            ExprKind::Literal(..) => Some(expr.clone()),
            _ => None,
        };

        let mut ty = possible_type.unwrap();
        ty.modifier = modifier;

        let variabel_kind = get_variable_kind(var_kind, ty, scopes, lit_retention, Some(Box::new(expr)));
        Ok(Spanned::new(variabel_kind, stream[begin_i].span.combine(&stream.current_span())))
    }
}

pub fn parse_unwrap_pattern(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<HashMap<Ident, Option<Ident>>> {
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get initialization of variable")
    }

    if stream.current_text() != "(" {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream.current_span(), 
            format!("token: '{}' invalid for variable unwrap pattern should be '(' (e.g 'let (one, two) = (1, 2)' )", stream.current_text())
        ));
    }

    let mut idents = HashMap::new();
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == "\n" {

            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
        }

        let possible_name = if stream.peek().is_some_and(|token| token.text == ":") {
            let var_i = stream.current_index();

            if stream.next_multiple(2).is_none() {
                return Err(err_out_of_bounds(stream));
            }

            if stream.current_text() == "\n" {

                if stream.next().is_none() {
                    return Err(err_out_of_bounds(stream));
                }
            }
            Some(Ident(stream[var_i].text.clone()))
        }
        else {
            None
        };
        
        check_if_var_valid(&stream.current_text(), stream.current_span(), scopes)?;
        
        match possible_name {
            Some(name) => idents.insert(name, Some(Ident(stream.current_text().clone()))),
            None => idents.insert(Ident(stream.current_text().clone()), None),
        };

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == ")" {
            break;
        }
        else if stream.current_text() != "," {
            return Err(new_soul_error(
                SoulErrorKind::UnexpectedToken,
                stream.current_span(), 
                format!("token: '{}' should be ','", stream.current_text())
            ));
        }
    }

    Ok(idents)
}

pub fn parse_union_binding(possible_types: &Option<SoulType>, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Ident> {
    if possible_types.is_none() {
        return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "missing type"))
    }

    let var_name_index = stream.current_index();

    check_if_var_valid(&stream[var_name_index].text, stream[var_name_index].span, scopes)?;
    Ok(Ident(stream[var_name_index].text.clone()))
}

pub fn get_variable_kind(var_kind: VarKind, ty: SoulType, scopes: &mut ScopeBuilder, lit_retention: Option<Spanned<ExprKind>>, initializer: Option<BoxExpr>) -> VariableKind {
    match var_kind {
        VarKind::Variable(ident) |
        VarKind::UnionBinding(ident) => {
            let var_ref = VariableRef::new(
                VariableDecl{name: ident.clone(), ty, initializer, lit_retention}, 
                &mut scopes.ref_pool
            );
            let scope_kind = var_ref.clone();
            let var_kind = var_ref;

            scopes.insert(ident.0, ScopeKind::Variable(scope_kind));
            VariableKind::Variable(var_kind)            
        },
        VarKind::Unwrap(idents) => {
            let init = if initializer.is_some() {
                Some(Box::new(Spanned::new(ExprKind::Empty, SoulSpan::new(0,0,0))))
            } 
            else {
                None
            };

            let vars = idents.into_iter().map(|(name, rename)| {
                let var_name = rename.unwrap_or(name.clone());

                let var_ref = VariableRef::new(
                    VariableDecl{name: var_name.clone() , ty: SoulType::none(), initializer: init.clone(), lit_retention: None},
                    &mut scopes.ref_pool
                );
                scopes.insert(var_name.0, ScopeKind::Variable(var_ref.clone()));
                (name, var_ref)
            }).collect();

            VariableKind::MultiVariable{vars, ty, initializer, lit_retention}
        },
    }
}

fn check_if_var_valid(var_name: &str, span: SoulSpan, scopes: &mut ScopeBuilder) -> Result<()> {
    if let Err(msg) = check_name(var_name) {
        return Err(new_soul_error(SoulErrorKind::InvalidName, span, msg))
    }
    
    let possible_scope_kinds = scopes.flat_lookup(var_name);
    let possible_var = possible_scope_kinds
        .filter(|scope_kinds| {
            scope_kinds.iter().any(|kind| matches!(kind, ScopeKind::Variable(_)))
        });

    if possible_var.is_some() {
        return Err(new_soul_error(
            SoulErrorKind::NotFoundInScope, 
            span, 
            format!("variable '{}' already exists in scope", var_name)
        ));
    }

    Ok(())
}

pub fn get_assignment_with_var(variable: Expression, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Assignment>> {
        
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing assignment")
    }
    
    let symbool_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let expr = get_expression(stream, scopes, STATMENT_ENDS)?;
    
    let expression = get_compount_assignment(stream, symbool_i, &variable, expr)?;
    
    let span = variable.span.combine(&expression.span);
    Ok(Spanned::new(Assignment{target: variable, value: expression}, span))
}

pub fn get_assignment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Assignment>> {
    
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpeced end while parsing assignment")
    }

    const IS_ASSIGN_VAR: bool = true;
    const USE_LITERAL_RETENTION: bool = false;
    let variable = get_expression_options(stream, scopes, &ASSIGN_END_TOKENS, USE_LITERAL_RETENTION, IS_ASSIGN_VAR)?;
    let symbool_i = stream.current_index();

    let lit_retention = if let ExprKind::Variable(var) = &variable.node {
        
        let possible_var = {
            let scope = scopes.lookup(&var.name.0);
            try_get_variable(&scope).cloned()
        };

        if let Some(mut var_ref) = possible_var {
            let lit_ret = &mut var_ref.borrow_mut(&mut scopes.ref_pool).lit_retention;
            std::mem::take(lit_ret)
        }
        else {
            None
        }
    }
    else {
        None
    };

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    let expr = get_expression(stream, scopes, STATMENT_ENDS)?;
    
    let assign = if let Some(literal) = lit_retention {Expression::new(literal.node, variable.span)} else {variable.clone()};
    let expression = get_compount_assignment(stream, symbool_i, &assign, expr)?;
    
    let span = variable.span.combine(&expression.span);
    Ok(Spanned::new(Assignment{target: variable, value: expression}, span))
}

fn get_compount_assignment(stream: &TokenStream, symbool_i: usize, variable: &Expression, expression: Expression) -> Result<Expression> {
    let op_kind = match stream[symbool_i].text.as_str() {
        "=" => return Ok(expression),
        "+=" => BinOpKind::Add,
        "-=" => BinOpKind::Sub,
        "*=" => BinOpKind::Mul,
        "/=" => BinOpKind::Div,
        "%=" => BinOpKind::Mod,
        "&=" => BinOpKind::BitAnd,
        "|=" => BinOpKind::BitOr,
        "^=" => BinOpKind::BitXor,
        _ => return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream[symbool_i].span, format!("symbool: '{}' unknown symbool for assignment", stream[symbool_i].text))),
    };

    let span = expression.span;
    let operator = BinOp::new(op_kind, stream[symbool_i].span);
    Ok(Expression::new(ExprKind::Binary(
        BinaryExpr{
            left: Box::new(variable.clone()), 
            operator,
            right: Box::new(expression),
        }), 
        span
    ))
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

pub enum VarKind {
    Variable(Ident),
    Unwrap(HashMap<Ident, Option<Ident>>),
    UnionBinding(Ident),
}
impl VarKind {
    pub fn to_string(&self) -> String {
        match self {
            VarKind::Variable(ident) => ident.0.clone(),
            VarKind::Unwrap(idents) => format!("({})", idents.iter().map(|(name, rename)| {
                match rename {
                    Some(rname) => format!("{}: {}", name.0, rname.0),
                    None => name.0.clone(),
                }
            }).join(",")),
            VarKind::UnionBinding(ident) => ident.0.clone(),
        }
    }
}





























