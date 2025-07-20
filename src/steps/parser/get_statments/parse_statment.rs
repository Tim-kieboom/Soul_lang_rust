use once_cell::sync::Lazy;
use std::collections::HashSet;
use crate::steps::parser::get_expressions::parse_expression::get_expression;
use crate::steps::parser::get_statments::parse_function_decl::get_function_decl;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::step_interfaces::i_parser::scope::{ScopeBuilder, ScopeKind};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExprKind, Ident};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::Modifier;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::{Block, CloseBlock, Return, Statment, StmtKind, VariableDecl, VariableRef};

static ASSIGN_SYMBOOLS_SET: Lazy<HashSet<&&str>> = Lazy::new(|| {
    SOUL_NAMES.assign_symbools.iter().map(|(_, str)| str).collect::<HashSet<&&str>>()
});

pub fn get_statment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statment>> {
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
    }

    if stream.current().text == "\n" {

        if stream.next().is_none() {
            return Ok(None)
        }
    }

    if stream.current_text() == "{" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not have a scope in global (consider making scope a function, struct or class)"))
        }

        return Ok(Some(Statment::new(StmtKind::Block(get_scope(stream, scopes)?), stream.current_span())));
    }
    else if stream.current_text() == "}" {
        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "there is a '}' without a '{'"))
        }

        stream.next();
        return Ok(Some(Statment::new(StmtKind::CloseBlock(CloseBlock{delete_list:vec![]}), stream.current_span())));
    }

    match stream.current_text() {
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Return) => {
            let return_i = stream.current_index();
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }  

            let expr = get_expression(stream, scopes, &["\n", ";"])?;
            let return_expr = if let ExprKind::Empty = expr.node {
                None
            }
            else {
                Some(expr)
            };

            let span = stream[return_i].span.combine(&stream.current_span());
            return Ok(Some(Statment::new(StmtKind::Return(Return{value: return_expr}), span)));
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            todo!()
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            todo!()
        },
        _ => (),
    }

    let begin_i = stream.current_index();
    let possible_res_type = SoulType::try_from_stream(stream, scopes);

    if let Some(result_ty) = possible_res_type {
        if let Err(err) = result_ty {
            return Err(err);
        } 

        let peek2_token = stream.peek_multiple(2)
            .ok_or(err_out_of_bounds(stream))?;

        let symbool_index = if peek2_token.text == ">" {
            let begin_gen_i = stream.current_index();
            get_symbool_after_generic(stream, begin_gen_i)?;
            let sym = stream.current_index();
            
            stream.go_to_index(begin_gen_i);
            sym
        }
        else {
            stream.current_index()
        };

        let symbool = &stream[symbool_index].text;

        if symbool == "=" {
            stream.go_to_index(begin_i);
            return Ok(Some(
                Statment::from_kind(
                    get_var_decl(stream, scopes)?, 
                    stream.current_span()
                ))
            );
        }
    }

    let type_i = stream.current_index();
    let peek_i: i64 = if SoulType::from_stream(stream, scopes).is_ok() {
        stream.current_index() as i64 - type_i as i64
    }
    else {
        if Modifier::from_str(stream.current_text()) != Modifier::Default {
            2i64
        }
        else {
            1i64
        }
    };

    stream.go_to_index(type_i);

    let mut next_index = (stream.current_index() as i64 + peek_i) as usize;
    
    //check if next_index is valid index
    stream.peek_multiple(peek_i).ok_or(err_out_of_bounds(stream))?;

    if stream[next_index].text == "<" {
        get_symbool_after_generic(stream, next_index)?;
        next_index = stream.current_index();
        
        stream.go_to_index(type_i);
    }

    match stream[next_index].text.as_str() {
        "=" => {
            if peek_i != 1 {
                //var decl
            }
        }
        ":=" => {
            //var decl
        }
        "(" => {
            let begin_i = stream.current_index();
            stream.go_to_index(next_index);
            match func_call_or_declaration(stream, scopes)? {
                FunctionKind::FunctionCall => {
                    stream.go_to_index(begin_i);
                    let expr = get_expression(stream, scopes, &["\n", ";"])?;
                    if !matches!(expr.node, ExprKind::Call(..)) {
                        return Err(new_soul_error(SoulErrorKind::InternalError, expr.span, format!("internal error get_expression in function call did not return function call, expr: '{}'", expr.node.to_string())))
                    }

                    let span = expr.span;
                    return Ok(Some(Statment::new(StmtKind::ExprStmt(expr), span)))
                },
                FunctionKind::FunctionDecl => {
                    stream.go_to_index(begin_i);
                    let func = get_function_decl(stream, scopes)?;
                    scopes.add_function(func.node.clone());
                    return Ok(Some(Statment::new(StmtKind::FnDecl(func.node), func.span)));
                },
            }
        }
        _ => (),
    }

    if !ASSIGN_SYMBOOLS_SET.iter().any(|symb| symb == &&stream[next_index].text) {
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream[next_index].span, 
            format!("token invalid for all statments, token: '{}'", stream[next_index].text)
        ));
    }

    //assignment
    todo!();
}



enum FunctionKind {
    FunctionCall,
    FunctionDecl
}

fn func_call_or_declaration(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<FunctionKind> {
    go_to_symbool_after_brackets(stream, stream.current_index())?;

    let is_curly_bracket = stream.current_text() == "{";

    let possible_result = SoulType::try_from_stream(stream, scopes);
    let is_type = possible_result.is_some();

    if is_type {
        possible_result.unwrap()?;
        Ok(FunctionKind::FunctionDecl)
    }
    else if is_curly_bracket {
        Ok(FunctionKind::FunctionDecl)
    }
    else {
        Ok(FunctionKind::FunctionCall)
    }
}

fn go_to_symbool_after_brackets<'a>(stream: &mut TokenStream, start_i: usize) -> Result<()> {
    if &stream[start_i].text != "(" {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis,
            stream.current_span(), 
            format!("unexpected token: '{}' while trying to open args (args is not opened add '(')", stream.current_text()),
        ));
    }

    stream.go_to_index(start_i);
    let mut stack = 1;

    loop {
        if stream.next().is_none() {
            break Err(new_soul_error(
                SoulErrorKind::UnmatchedParenthesis,
                stream.current_span(), 
                format!("unexpected token: '{}' while trying to close args (args is not closed add ')')", stream.current_text())
            ));
        }

        if stream.current().text == "(" {
            stack += 1;
        }
        else if stream.current().text == ")" {
            stack -= 1;
        }

        if stack == 0 {
            stream.next();
            break Ok(());
        }
    }
}

fn get_var_decl(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<VariableRef> {
    fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
        new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get initialization of variable")
    }

    let possible_type = match SoulType::try_from_stream(stream, scopes) {
        Some(val) => Some(val?),
        None => None,
    };

    let is_type_invered = possible_type.is_none();

    let modifier = if is_type_invered {
        let modi = Modifier::from_str(stream.current_text());
        if modi != Modifier::Default {

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

    let var_name_index = stream.current_index();
    if let Err(msg) = check_name(&stream[var_name_index].text) {
        return Err(new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))
    }

    let possible_scope_kinds = scopes.flat_lookup(&stream[var_name_index].text);
    let possible_var = possible_scope_kinds
        .filter(|scope_kinds| {
            scope_kinds.iter().any(|kind| matches!(kind, ScopeKind::Variable(_)))
        });

    if possible_var.is_some() {
        return Err(new_soul_error(
            SoulErrorKind::NotFoundInScope, 
            stream[var_name_index].span, 
            format!("variable '{}' already exists in scope", &stream[var_name_index].text)
        ));
    }

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" || stream.current_text() == ";" {
        if is_type_invered {
            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                stream.current_span(), 
                format!("variable '{}' can not have no type and no assignment (add type 'int foo' or assignment 'foo := 1')", &stream[var_name_index].text)
            ));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(
                SoulErrorKind::InvalidEscapeSequence, 
                stream.current_span(), 
                format!("global variables HAVE TO BE assigned at init, variable '{}' is not assigned", &stream[var_name_index].text)
            ));
        }

        let ty = possible_type.unwrap();
        let name = Ident(stream[var_name_index].text.clone());
        let var_decl: VariableRef = VariableRef::new(
            VariableDecl{name, ty, initializer: None, lit_retention: None}
        );

        scopes.insert(stream[var_name_index].text.clone(), ScopeKind::Variable(var_decl.clone()));
        return Ok(var_decl);
    }

    if is_type_invered {

        if modifier == Modifier::Default &&
           stream.current_text() != ":=" 
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

    if is_type_invered {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }


    }


    todo!()
    // Ok(VariableRef::new(RefCell::new()))
}

fn get_symbool_after_generic<'a>(stream: &'a mut TokenStream, start_i: usize) -> Result<()> {
    if &stream[start_i].text != "<" {
        return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "unexpected start while trying to get generic (generic is not opened add '<')"));
    }

    stream.go_to_index(start_i);
    let mut stack = 1;

    loop {
        if stream.next().is_none() {
            break Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "unexpected end while trying to get generic (generic is not closed add '>')"));
        }

        if stream.current().text == "<" {
            stack += 1;
        }
        else if stream.current().text == ">" {
            stack -= 1;
        }

        if stack == 0 {
            stream.next();
            break Ok(());
        }
    }
}

fn get_scope(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Block> {
    let mut statments = Vec::new();

    loop {
        let statment = match get_statment(stream, scopes)? {
            Some(val) => val,
            None => return Ok(Block{statments}),
        };

        let should_end = matches!(statment.node, StmtKind::CloseBlock(_));

        statments.push(statment);
        
        if should_end {
            return Ok(Block{statments});
        }
    }
}






















