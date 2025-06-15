use itertools::Itertools;

use crate::meta_data::scope_and_var::scope::ScopeId;
use crate::tokenizer::token::Token;
use crate::cpp_transpiller::cpp_type::CppType;
use crate::meta_data::soul_type::soul_type::SoulType;
use crate::cpp_transpiller::namespace::get_scope_namespace;
use crate::cpp_transpiller::convert_to_cpp::variable_to_cpp::variable_to_cpp;
use crate::cpp_transpiller::convert_to_cpp::expression_to_cpp::expression_to_cpp;
use crate::meta_data::soul_names::{NamesInternalType, NamesOtherKeyWords, SOUL_NAMES};
use crate::meta_data::function::function_declaration::function_declaration::FunctionDeclaration;
use crate::meta_data::soul_error::soul_error::{new_soul_error, pass_soul_error, Result, SoulSpan};
use crate::abstract_styntax_tree::get_abstract_syntax_tree::get_stament::statment_type::statment_type::StatmentIterator;
use crate::{abstract_styntax_tree::abstract_styntax_tree::IStatment, cpp_transpiller::cpp_writer::CppWriter, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData}};

pub fn statment_to_cpp(writer: &mut CppWriter, statment_iter: &StatmentIterator, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    
    match statment {
        IStatment::Initialize{..} => initialize_to_cpp(writer, statment, meta_data, context, in_scope_id),
        IStatment::CloseScope(_) => close_scope_to_cpp(writer, statment),
        IStatment::EmptyStatment(_) => Ok(()),
        IStatment::Assignment{..} => assignment_to_cpp(writer, statment, meta_data, context, in_scope_id),
        IStatment::FunctionBody{..} => function_body_to_cpp(writer, statment_iter, statment, meta_data, context, in_scope_id),
        IStatment::FunctionCall{..} => function_call_to_cpp(writer, statment, meta_data, context, in_scope_id),
        IStatment::Return{..} => return_to_cpp(writer, statment, meta_data, context, in_scope_id),
        IStatment::Scope{..} => scope_to_cpp(writer, statment_iter, statment, meta_data, context, in_scope_id),
        IStatment::If{..} => if_to_cpp(writer, statment_iter, statment, meta_data, context, in_scope_id),
        IStatment::Else{..} => else_to_cpp(writer, statment_iter, statment, meta_data, context, in_scope_id),
        IStatment::ElseIf{..} => else_if_to_cpp(writer, statment_iter, statment, meta_data, context, in_scope_id),
    }
}

pub fn function_declaration_to_cpp(writer: &mut CppWriter, func: &FunctionDeclaration, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    const SPAN: SoulSpan = SoulSpan{line_number: 0, line_offset: 0};
    fn new_token(span: &SoulSpan) -> Token {
        Token { text: String::new(), line_number: span.line_number, line_offset: span.line_offset }
    }

    let soul_str = func.return_type.as_ref().map(|string| string.as_str())
        .unwrap_or(SOUL_NAMES.get_name(NamesInternalType::None));

    let soul_type = SoulType::from_stringed_type(soul_str, &token_from_span(&SPAN), &meta_data.type_meta_data, &context.current_generics)?;
    
    writer.push_str(
        CppType::from_soul_type(&soul_type, meta_data, context, &SPAN)?.as_str()
    );
    
    writer.push(' ');
    writer.push_str(&func.name);
    writer.push('(');

    if func.name == "main" && !func.args.is_empty() {
        writer.push_str("int __SOUL_C_argsc, char** __SOUL_C_argsv");
    }
    else {    
        let args = func.args
        .iter()
        .chain(func.optionals.iter().map(|(_name, arg)| arg))
        .sorted_by(|a, b| Ord::cmp(&a.arg_position, &b.arg_position));
    
        let args_len = func.args.len() + func.optionals.len();
        
        for (i, arg) in args.enumerate() {
            let soul_type = SoulType::from_stringed_type(&arg.value_type, &new_token(&SPAN), &meta_data.type_meta_data, &context.current_generics)?;
            writer.push_str(CppType::from_soul_type(&soul_type, meta_data, context, &SPAN)?.as_str());
            writer.push(' ');
            writer.push_str(&arg.name);
            
            if i != args_len-1 {
                writer.push_str(", ");
            }
        }
    }

    writer.push(')');
    Ok(())
}

fn if_to_cpp(writer: &mut CppWriter, statment_iter: &StatmentIterator, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (condition, body, _) = match statment {
        IStatment::If{condition, body, span} => (condition, body, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error if_to_cpp() called while statment is not If")),
    };

    writer.push_str("if(");
    expression_to_cpp(writer, condition, meta_data, context, in_scope_id)?;
    writer.push_str(") {\n");
    for statment in &body.statments {
        statment_to_cpp(writer, statment_iter, &statment, meta_data, context, in_scope_id)?;
    }
    writer.push_str("}\n");

    Ok(())
}

fn else_to_cpp(writer: &mut CppWriter, statment_iter: &StatmentIterator, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (body, _) = match statment {
        IStatment::Else{body, span} => (body, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error else_to_cpp() called while statment is not Else")),
    };

    writer.push_str("else {\n");
    for statment in &body.statments {
        statment_to_cpp(writer, statment_iter, &statment, meta_data, context, in_scope_id)?;
    }
    writer.push_str("}\n");

    Ok(())
}

fn else_if_to_cpp(writer: &mut CppWriter, statment_iter: &StatmentIterator, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (condition, body, _) = match statment {
        IStatment::ElseIf{condition, body, span} => (condition, body, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error else_if_to_cpp() called while statment is not ElseIf")),
    };

    writer.push_str("else if(");
    expression_to_cpp(writer, condition, meta_data, context, in_scope_id)?;
    writer.push_str(") {\n");
    for statment in &body.statments {
        statment_to_cpp(writer, statment_iter, &statment, meta_data, context, in_scope_id)?;
    }
    writer.push_str("}\n");

    Ok(())
}

fn scope_to_cpp(writer: &mut CppWriter, statment_iter: &StatmentIterator, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (body, _) = match statment {
        IStatment::Scope{body, span} => (body, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error scope_to_cpp() called while statment is not Scope")),
    };

    writer.push_str(" {\n");
    for statment in &body.statments {
        statment_to_cpp(writer, statment_iter, &statment, meta_data, context, in_scope_id)?;
    }
    writer.push_str("}\n");

    Ok(())
}

fn return_to_cpp(writer: &mut CppWriter, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (possible_expression, _) = match statment {
        IStatment::Return{expression, span} => (expression, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error return_to_cpp() called while statment is not Return")),
    };

    writer.push_str(SOUL_NAMES.get_name(NamesOtherKeyWords::Return));
    writer.push(' ');

    if let Some(expression) = possible_expression {
        expression_to_cpp(writer, expression, meta_data, context, in_scope_id)?;
    }

    writer.push_str(";\n");
    Ok(())
}

fn function_call_to_cpp(writer: &mut CppWriter, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (this, _) = match statment {
        IStatment::FunctionCall{this, span} => (this, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error function_call_to_cpp() called while statment is not FunctionCall")),
    };

    expression_to_cpp(writer, this, meta_data, context, in_scope_id)?;
    writer.push_str(";\n");
    Ok(())
}

fn function_body_to_cpp(writer: &mut CppWriter, statment_iter: &StatmentIterator, func_body: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (body, func_info, _) = match func_body {
        IStatment::FunctionBody{body, func_info, span} => (body, func_info, span),
        _ => return Err(new_soul_error(&token_from_span(func_body.get_span()), "Internal error function_body_to_cpp() called while statment is not FunctionBody")),
    };

    let function_bodys = body.statments
        .iter()
        .filter_map(|stat| match &stat {
            IStatment::FunctionBody{func_info, ..} => Some((stat, func_info.in_scope_id.clone())),
            _ => None,
        });

    for (inner_function, func_in_scope_id) in function_bodys {

        writer.push_str("namespace ");
        get_scope_namespace(writer, &body.scope_id);
        writer.push_str("{\n");

        let scope = meta_data.scope_store.get(&body.scope_id).unwrap();
        for (_, func) in scope.function_store.from_id.iter() {
            function_declaration_to_cpp(writer, func, meta_data, context)?;
            writer.push_str(";\n");
        }

        function_body_to_cpp(writer, statment_iter, inner_function, meta_data, context, func_in_scope_id)?;
        writer.push_str("}\n");
    }

    function_declaration_to_cpp(writer, func_info, meta_data, context)?;
    writer.push_str(" {\n");
    if func_info.name == "main" && !func_info.args.is_empty() {
        let arg_name = &func_info.args[0].name;
        writer.push_str(format!("auto __var_args = __Soul_ARRAY__<__Soul_ARRAY__<char>>(__SOUL_C_argsc);\nfor(int i = 0; i < __SOUL_C_argsc; i++){}\n\t__var_args[i] = str((const char*)__SOUL_C_argsv[i]);\n{}\nauto const* {} = &__var_args;\n", '{', '}', arg_name).as_str());
    }

    for statment in body.statments.iter().filter(|stat| !matches!(stat, IStatment::FunctionBody{..})) {
        statment_to_cpp(writer, statment_iter, &statment, meta_data, context, in_scope_id)?;
    }
    writer.push_str("}\n");

    Ok(())
}

fn close_scope_to_cpp(writer: &mut CppWriter, statment: &IStatment) -> Result<()> {
    match statment {
        IStatment::CloseScope(_) => (),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error close_scope_to_cpp() called while statment is not CloseScope")),
    };

    writer.push('}');
    Ok(())
}

fn initialize_to_cpp(writer: &mut CppWriter, statment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (variable, possible_assignment, span) = match statment {
        IStatment::Initialize{variable, assignment, span} => (variable, assignment, span),
        _ => return Err(new_soul_error(&token_from_span(statment.get_span()), "Internal error initialize_to_cpp() called while statment is not Initialize")),
    };

    if let Some(assignment) = possible_assignment {
        assignment_to_cpp(writer, assignment.as_ref(), meta_data, context, in_scope_id)?;
    }
    else {
        variable_to_cpp(writer, variable, meta_data, context)
            .map_err(|err| pass_soul_error(&token_from_span(span), "while trying to convert initialize", err))?;

        writer.push_str(";\n");
    }


    Ok(())
}

fn assignment_to_cpp(writer: &mut CppWriter, assignment: &IStatment, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId) -> Result<()> {
    let (variable, assign_expression, span) = match assignment {
        IStatment::Assignment{variable, assign, span} => (variable, assign, span),
        _ => return Err(new_soul_error(&token_from_span(assignment.get_span()), "Internal error assignment_to_cpp() called while statment is not Assignment")),
    };

    variable_to_cpp(writer, variable, meta_data, context)
        .map_err(|err| pass_soul_error(&token_from_span(span), "while trying to convert assignment", err))?;
    
    writer.push_str(" = ");

    expression_to_cpp(writer, assign_expression, meta_data, context, in_scope_id)?;
    writer.push_str(";\n");

    Ok(())
}

fn token_from_span(span: &SoulSpan) -> Token {
    Token{line_number: span.line_number, line_offset: span.line_offset, text: String::new()}
}










