use crate::abstract_styntax_tree::abstract_styntax_tree::IExpression;
use crate::cpp_transpiller::convert_to_cpp::expression_to_cpp::expression_to_cpp;
use crate::meta_data::scope_and_var::scope::ScopeId;
use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulSpan};
use crate::{abstract_styntax_tree::abstract_styntax_tree::IVariable, cpp_transpiller::{cpp_type::CppType, cpp_writer::CppWriter}, meta_data::{current_context::current_context::CurrentContext, meta_data::MetaData, soul_type::soul_type::SoulType}, tokenizer::token::Token};

pub fn variable_to_cpp(writer: &mut CppWriter, variable: &IExpression, meta_data: &MetaData, context: &CurrentContext, in_scope_id: ScopeId, add_type: bool) -> Result<()> {
    match variable {
        IExpression::DeRef{..} |
        IExpression::MutRef{..} |
        IExpression::Literal{..} |
        IExpression::ConstRef{..} |
        IExpression::Increment{..} |
        IExpression::FunctionCall{..} |
        IExpression::EmptyExpression{..} |
        IExpression::BinairyExpression{..} => return Err(new_soul_error(&token_from_span(variable.get_span()), format!("invalid variant of variable: '{}'", variable.get_variant_name()).as_str())),

        IExpression::IVariable{this, span:_} => {
            match this {
                IVariable::Variable { name, type_name, span } => {
                    
                    if !add_type {
                        let soul_type = SoulType::from_stringed_type(&type_name, &token_from_span(span), &meta_data.type_meta_data, &context.current_generics)?;
                        writer.push_str(CppType::from_soul_type(&soul_type, meta_data, context, span)?.as_str());
                        writer.push(' ');
                    }

                    get_var_name(writer, &name);
                },
            }
        },
        IExpression::Index{..} => {
            expression_to_cpp(writer, variable, meta_data, context, in_scope_id)?;
        },
    }
   
    

    Ok(())
}

pub fn get_var_name(writer: &mut CppWriter, name: &str) {
    const ILLIGAL_CPP_NAMES: &[&str] = &[
        "bool", 
        "char", "wchar_t", "char8_t", "char16_t", "char32_t",
        "short", "int", "long",
        "float", "double", "void"
    ];
    
    if ILLIGAL_CPP_NAMES.iter().any(|il| il == &name) {
        writer.push_str("__var_");
    }
    writer.push_str(&name);
}

fn token_from_span(span: &SoulSpan) -> Token {
    Token{line_number: span.line_number, line_offset: span.line_offset, text: String::new()}
}




































