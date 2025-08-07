use once_cell::sync::Lazy;
use std::collections::HashSet;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::parser::get_statments::parse_class::get_class;
use crate::steps::parser::get_statments::parse_trait::get_trait;
use crate::steps::parser::get_statments::parse_block::get_block;
use crate::steps::parser::get_statments::parse_struct::get_struct;
use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::steps::parser::get_statments::parse_if_else::{get_else, get_if};
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::parser::parse_generic_decl::{get_generics_decl, GenericDecl};
use crate::steps::parser::get_statments::parse_function_decl::get_function_decl;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind, SoulSpan};
use crate::steps::parser::get_statments::parse_enum_like::{get_enum, get_type_enum, get_union};
use crate::steps::step_interfaces::i_parser::scope::{ScopeBuilder, ScopeKind, ScopeVisibility};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExprKind, Ident};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::function::Parameter;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::SoulType;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::{Modifier};
use crate::steps::parser::get_expressions::parse_expression::{get_expression, get_expression_options};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::StatmentBuilder;
use crate::steps::parser::get_statments::parse_var_decl_assign::{get_assignment, get_assignment_with_var, get_var_decl};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::conditionals::{ElseKind, ForDecl, SwitchDecl, WhileDecl};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::staments::statment::{Block, CloseBlock, ReturnKind, ReturnLike, Statment, StmtKind, STATMENT_ENDS};

static ASSIGN_SYMBOOLS_SET: Lazy<HashSet<&&str>> = Lazy::new(|| {
    SOUL_NAMES.assign_symbools.iter().map(|(_, str)| str).collect::<HashSet<&&str>>()
});

static END_DOT_EXPRESSION: Lazy<Vec<&str>> = Lazy::new(|| {
    let mut symbools = SOUL_NAMES
        .assign_symbools
        .iter()
        .map(|(_, symbool)| *symbool)
        .filter(|symbool| *symbool != "." && *symbool != "[")
        .collect::<Vec<&str>>();

    symbools.push("\n");
    symbools.push(";");
    symbools
});

pub fn get_statment(node_scope: &mut StatmentBuilder, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statment>> {

    if stream.current_text() == "\n" || stream.current_text() == ";" {

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

        return Ok(Some(Statment::new(StmtKind::Block(get_scope(node_scope, stream, scopes)?), stream.current_span())));
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
            let ret = get_return_like(stream, scopes, ReturnKind::Return)?;
            return Ok(Some(Statment::new(StmtKind::Return(ret.node), ret.span)))
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::BreakLoop) => {
            let ret = get_return_like(stream, scopes, ReturnKind::Break)?;
            return Ok(Some(Statment::new(StmtKind::Return(ret.node), ret.span)))
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Fall) => {
            let ret = get_return_like(stream, scopes, ReturnKind::Fall)?;
            return Ok(Some(Statment::new(StmtKind::Return(ret.node), ret.span)))
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop) => {
            let while_decl = get_while(stream, scopes)?;
            return Ok(Some(Statment::new(StmtKind::While(while_decl.node), while_decl.span)))
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop) => {
            return add_for_loop(stream, scopes);
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
            let if_decl = get_if(stream, scopes)?;
            return Ok(Some(Statment::new(StmtKind::If(if_decl.node), if_decl.span)))
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {
            add_else(stream, scopes, node_scope)?;
            if stream.next().is_none() {
                return Err(err_out_of_bounds(stream));
            }
            return Ok(None);
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            add_type(stream, scopes)?;
            return Ok(None);
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            let trait_decl = get_trait(stream, scopes)?;
            let name = trait_decl.node.borrow().name.0.clone();
            scopes.insert(name, ScopeKind::Trait(trait_decl.node.clone()));
            return Ok(Some(Statment::new(StmtKind::TraitDecl(trait_decl.node), trait_decl.span)));
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            let type_enum = get_type_enum(stream, scopes)?;
            let name = type_enum.node.borrow().name.0.clone();
            scopes.insert(name, ScopeKind::TypeEnum(type_enum.node.clone()));
            return Ok(Some(Statment::new(StmtKind::TypeEnumDecl(type_enum.node), type_enum.span)));
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            let union_decl = get_union(stream, scopes)?;
            let name = union_decl.node.borrow().name.0.clone();
            scopes.insert(name, ScopeKind::Union(union_decl.node.clone()));
            return Ok(Some(Statment::new(StmtKind::UnionDecl(union_decl.node), union_decl.span)))
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            let enum_decl = get_enum(stream, scopes)?;
            let name = enum_decl.node.borrow().name.0.clone();
            scopes.insert(name, ScopeKind::Enum(enum_decl.node.clone()));
            return Ok(Some(Statment::new(StmtKind::EnumDecl(enum_decl.node), enum_decl.span)));
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            let struct_decl = get_struct(stream, scopes)?;
            let name = struct_decl.node.borrow().name.0.clone();
            scopes.insert(name, ScopeKind::Struct(struct_decl.node.clone()));
            return Ok(Some(Statment::new(StmtKind::StructDecl(struct_decl.node), struct_decl.span)));
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            let class_decl = get_class(stream, scopes)?;
            let name = class_decl.node.borrow().name.0.clone();
            scopes.insert(name, ScopeKind::Class(class_decl.node.clone()));
            return Ok(Some(Statment::new(StmtKind::ClassDecl(class_decl.node), class_decl.span)));
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::SwitchCase) => {
            // let switch_decl = get_switch_case(stream, scopes)?;
            // let name = swiut
            todo!()
        },
        _ => (),
    }

    if stream.current_text() == "(" {
        let unwrap_var_decl = get_var_decl(stream, scopes)?;
        return Ok(Some(
            Statment::from_kind(
                unwrap_var_decl.node, 
                unwrap_var_decl.span
            )
        ))
    }

    let first_type_i = stream.current_index();
    let possible_res_type = SoulType::try_from_stream(stream, scopes);
    let has_valid_type = possible_res_type.as_ref().is_some_and(|res| res.is_ok());

    if let Some(result_ty) = possible_res_type {
        let _ty = result_ty?;

        let peek1_token = stream.peek()
            .ok_or(err_out_of_bounds(stream))?;

        if peek1_token.text == "(" {
            stream.go_to_index(first_type_i);
            let unwrap_var_decl = get_var_decl(stream, scopes)?;
            return Ok(Some(
                Statment::from_kind(
                    unwrap_var_decl.node, 
                    unwrap_var_decl.span
                )
            ))
        }

        let peek2_token = stream.peek_multiple(2)
            .ok_or(err_out_of_bounds(stream))?;

        if peek2_token.text == "(" {
            stream.go_to_index(first_type_i);
            let func = get_function_decl(None, stream, scopes)?;
            scopes.add_function(&func.node);
            return Ok(Some(func.node.consume_to_statment(func.span)));
        }
        else if peek2_token.text == "=" || STATMENT_ENDS.iter().any(|sym| sym == &peek2_token.text) {
            stream.go_to_index(first_type_i);
            let var = get_var_decl(stream, scopes)?;
            return Ok(Some(
                Statment::from_kind(
                    var.node, 
                    var.span
                ))
            );
        }

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
        if symbool == "=" || STATMENT_ENDS.iter().any(|sym| sym == symbool) {
            stream.go_to_index(first_type_i);
            let var = get_var_decl(stream, scopes)?;
            return Ok(Some(
                Statment::from_kind(
                    var.node, 
                    var.span
                ))
            );
        }
    }
    else if Modifier::from_str(stream.current_text()) != Modifier::Default || stream.current_text() == "let" {
        let peek2_token = stream.peek_multiple(2)
            .ok_or(err_out_of_bounds(stream))?;

        if peek2_token.text == "=" || STATMENT_ENDS.iter().any(|sym| sym == &peek2_token.text) {
            stream.go_to_index(first_type_i);
            let var = get_var_decl(stream, scopes)?;
            return Ok(Some(
                Statment::from_kind(
                    var.node, 
                    var.span
                ))
            );
        }

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
        if symbool == "=" || STATMENT_ENDS.iter().any(|sym| sym == symbool) {
            stream.go_to_index(first_type_i);
            let var = get_var_decl(stream, scopes)?;
            return Ok(Some(
                Statment::from_kind(
                    var.node, 
                    var.span
                ))
            );
        }
    }

    let type_i = stream.current_index();
    let peek_i: i64 = if SoulType::from_stream(stream, scopes).is_ok() {
        stream.current_index() as i64 - type_i as i64
    }
    else {
        if Modifier::from_str(stream.current_text()) != Modifier::Default || stream.current_text() == "let" {
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
    
    let generics = if stream[next_index].text == "<" {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        const ADD_TO_SCOPE: bool = false;
        let gene = get_generics_decl(stream, scopes, ADD_TO_SCOPE)?;
        next_index = stream.current_index();
        
        stream.go_to_index(type_i);
        Some(gene)
    }
    else {
        None
    };

    if stream[next_index].text == "." {
        const IS_ASSIGN_VAR: bool = false;
        const USE_LITERAL_RETENTION: bool = false;
        let variable = get_expression_options(stream, scopes, &END_DOT_EXPRESSION, USE_LITERAL_RETENTION, IS_ASSIGN_VAR)?;
        let span = variable.span;
        
        if stream.current_text() == "\n" || stream.current_text() == ";" {
            return Ok(Some(Statment::new(StmtKind::ExprStmt(variable), span)));
        }

        let assignment = get_assignment_with_var(variable, stream, scopes)?;
        return Ok(Some(Statment::new(StmtKind::Assignment(assignment.node), assignment.span)));
    }

    match stream[next_index].text.as_str() {
        "=" => {
            if peek_i != 1 {
                let var = get_var_decl(stream, scopes)?;
                return Ok(Some(Statment::new(StmtKind::VarDecl(var.node), var.span)))
            }
        }
        ":=" => {
            let var = get_var_decl(stream, scopes)?;
            return Ok(Some(Statment::new(StmtKind::VarDecl(var.node), var.span)))
        }
        "(" => {
            let begin_i = stream.current_index();
            stream.go_to_index(next_index);
            match func_call_or_declaration(stream, scopes, generics)? {
                FunctionKind::FunctionCall => {
                    stream.go_to_index(begin_i);
                    let expr = get_expression(stream, scopes, STATMENT_ENDS)?;
                    if !matches!(expr.node, ExprKind::Call(..)) {
                        return Err(new_soul_error(SoulErrorKind::InternalError, expr.span, format!("internal error get_expression in function call did not return function call, expr: '{}'", expr.node.to_string())))
                    }

                    let span = expr.span;
                    return Ok(Some(Statment::new(StmtKind::ExprStmt(expr), span)))
                },
                FunctionKind::FunctionDecl => {
                    stream.go_to_index(begin_i);
                    let func = get_function_decl(None, stream, scopes)?;
                    scopes.add_function(&func.node);
                    return Ok(Some(func.node.consume_to_statment(func.span)));
                },
            }
        },
        "++" | "--" => {
            let expr = get_expression(stream, scopes, STATMENT_ENDS)?;
            
            let span = expr.span;
            return Ok(Some(Spanned::new(StmtKind::ExprStmt(expr), span)));
        }
        _ => (),
    }

    if !ASSIGN_SYMBOOLS_SET.iter().any(|symb| symb == &&stream[next_index].text) {
        try_get_special_error(next_index, stream, has_valid_type, first_type_i)?;
        
        return Err(new_soul_error(
            SoulErrorKind::UnexpectedToken, 
            stream[next_index].span, 
            format!("token invalid for all statments, token: '{}'", stream[next_index].text)
        ));
    }

    let assign = get_assignment(stream, scopes)?;
    return Ok(Some(Statment::new(StmtKind::Assignment(assign.node), assign.span)));
}

fn get_while(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<WhileDecl>> {
    let while_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    if stream.current_text() == "\n" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }  
    }

    let condition = if stream.current_text() == "{" {
        None
    } 
    else {
        Some(get_expression(stream, scopes, &["{"])?)
    };
    
    let block = get_block(ScopeVisibility::All, stream, scopes, None, vec![])?;

    let span = stream[while_i].span.combine(&stream.current_span());
    Ok(Spanned::new(WhileDecl{condition, body: block.node}, span))
}

fn get_return_like(stream: &mut TokenStream, scopes: &mut ScopeBuilder, kind: ReturnKind) -> Result<Spanned<ReturnLike>> {
    let return_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    let expr = get_expression(stream, scopes, STATMENT_ENDS)?;
    let return_expr = if let ExprKind::Empty = expr.node {
        None
    }
    else {
        Some(expr)
    };

    let span = stream[return_i].span.combine(&stream.current_span());
    Ok(Spanned::new(ReturnLike{value: return_expr, kind, delete_list: vec![] }, span))
}

fn get_switch_case(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<SwitchDecl>> {
    todo!()
}

fn try_get_special_error(next_index: usize, stream: &TokenStream, has_type: bool, type_t: usize) -> Result<()> {
    if has_type && stream.is_valid_index(next_index+1) && stream[next_index+1].text == ":=" {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream[next_index+1].span, 
            format!("can not have a type: '{}' and a ':='(aka a default type-invered assign symbool) in the same variable declaration (remove '{}' or change ':=' to '=')", stream[type_t].text, stream[type_t].text)
        ));
    }

    Ok(())
}

enum FunctionKind {
    FunctionCall,
    FunctionDecl
}

fn try_add_else_to_if_branch<'a>(
    scope: &mut StatmentBuilder,
    else_branch: Spanned<ElseKind>, 
    span: SoulSpan
) -> Result<()> {
    
    let block = match scope {
        StatmentBuilder::Block(node_ref) => node_ref,
        _ => {
            return Err(new_soul_error(
                SoulErrorKind::InvalidInContext,
                span,
                "Cannot use 'else' in the global scope.",
            ));
        },
    };


    if let Some(last) = block.borrow_mut().node.statments.last_mut() {
        
        if let StmtKind::If(node) = &mut last.node {
            if node.else_branchs.last().is_some_and(|branch| matches!(branch.node, ElseKind::Else(_))) {
                return Err(new_soul_error(SoulErrorKind::InvalidInContext, span, "'else' or 'else if' can not go after 'else' stsament"))
            }

            node.else_branchs.push(else_branch);
            return Ok(());
        }
    }

    return Err(new_soul_error(
        SoulErrorKind::InvalidInContext,
        span,
        "An 'else' must follow an 'if' statement.",
    ));

}

fn add_for_loop(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statment>> {
    let for_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    check_name(stream.current_text())
        .map_err(|msg| new_soul_error(SoulErrorKind::InvalidName, stream.current_span(), msg))?;
    
    let name_i = stream.current_index();
    let el_parameter = Spanned::new(Parameter{name: Ident(stream[name_i].text.clone()), ty: SoulType::none(), default_value: None}, stream[name_i].span);
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    if stream.current_text() != "in" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedToken, stream.current_span(), format!("in for statment'{}' should be 'in' (format is 'for <element> in <collection> {{}}')", stream.current_text())))
    }
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    let collection = get_expression(stream, scopes, &["{"])?;

    let block = get_block(ScopeVisibility::All, stream, scopes, None, vec![el_parameter])?;

    let span = stream[for_i].span.combine(&stream.current_span());
    return Ok(Some(Statment::new(StmtKind::For(ForDecl{collection, element: Ident(stream[name_i].text.clone()), body: block.node}), span)))
}

fn add_else(stream: &mut TokenStream, scopes: &mut ScopeBuilder, node_scope: &mut StatmentBuilder) -> Result<()> {
    let else_i = stream.current_index();
    let else_branch = get_else(stream, scopes)?;
    try_add_else_to_if_branch(node_scope, else_branch, stream[else_i].span)?;

    Ok(())
}

fn add_type(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    loop {
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if stream.current_text() == SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
            break;
        }

        if stream.current_text() == "\n" {
            return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), format!("can not end line of type statment without '{}'", SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof))));
        }
    } 

    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() == "\n" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }
    }

    SoulType::from_stream(stream, scopes)?;
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }

    if stream.current_text() != ";" && stream.current_text() != "\n" {
        return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), format!("token: '{}' is incorrect end of type", stream.current_text())));
    }

    Ok(())
}

fn func_call_or_declaration(stream: &mut TokenStream, scopes: &mut ScopeBuilder, generics: Option<GenericDecl>) -> Result<FunctionKind> {
    go_to_symbool_after_brackets(stream)?;

    let is_curly_bracket = stream.current_text() == "{";

    let possible_result = SoulType::try_from_stream(stream, scopes);
    let is_type = possible_result.is_some();
    let is_generic = generics.is_some_and(|gen_decl| gen_decl.generics.iter().any(|generic| &generic.name.0 == stream.current_text()));

    if is_generic {
        Ok(FunctionKind::FunctionDecl)
    }
    else if is_type {
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

fn go_to_symbool_after_brackets<'a>(stream: &mut TokenStream) -> Result<()> {
    if stream.current_text() != "(" {
        return Err(new_soul_error(
            SoulErrorKind::UnmatchedParenthesis,
            stream.current_span(), 
            format!("unexpected token: '{}' while trying to open args (args is not opened add '(')", stream.current_text()),
        ));
    }

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

fn get_scope<'a>(node_scope: &mut StatmentBuilder, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Block> {
    let mut statments = Vec::new();

    loop {
        let statment = match get_statment(node_scope, stream, scopes)? {
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

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}









































