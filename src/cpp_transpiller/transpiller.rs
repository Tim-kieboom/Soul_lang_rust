use crate::cpp_transpiller::cpp_type::CppType;
use crate::meta_data::current_context::current_context::{CurrentContext, CurrentGenerics};
use crate::meta_data::function::function_declaration::function_declaration::FunctionDeclaration;
use crate::meta_data::function::internal_functions::{self, INTERNAL_FUNCTIONS};
use crate::meta_data::scope_and_var::scope::ScopeId;
use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulSpan};
use crate::meta_data::soul_names::{NamesInternalType, SOUL_NAMES};
use crate::meta_data::soul_type::soul_type::SoulType;
use crate::tokenizer::token::Token;
use crate::{abstract_styntax_tree::abstract_styntax_tree::AbstractSyntaxTree, meta_data::meta_data::MetaData};

pub fn transpiller_to_cpp(meta_data: &MetaData, abstract_syntax_tree: &AbstractSyntaxTree) -> Result<String> {
    
    let mut string_builder = String::new();

    string_builder.push_str("#include \"soul_hardCodedFunctions/soul_hardCodedFunctions.h\"\n");

    forward_declare_global_vars(&mut string_builder, meta_data, abstract_syntax_tree)?;
    declare_c_strs(&mut string_builder, meta_data);
    
    forward_declare_functions(&mut string_builder, meta_data, &abstract_syntax_tree.global_context)?;

    

    Ok(string_builder)
}

fn declare_c_strs(string_builder: &mut String, meta_data: &MetaData) {
    for (_, pair) in meta_data.type_meta_data.c_str_store.from_name_map() {
        string_builder.push_str("constexpr const char* ");
        string_builder.push_str(&pair.name);
        string_builder.push_str(" = ");
        string_builder.push_str(&pair.c_str);
        string_builder.push_str(";\n");
    }
}

fn forward_declare_global_vars(string_builder: &mut String, meta_data: &MetaData, abstract_syntax_tree: &AbstractSyntaxTree) -> Result<()> {
    let global_scope = meta_data.scope_store.get(&MetaData::GLOBAL_SCOPE_ID)
        .ok_or(new_soul_error(&dummy_token(), "Internal error in cpp transpiller global scope not found"))?;

    for (_, var) in &global_scope.vars {
        let soul_type = SoulType::from_stringed_type(&var.type_name, &dummy_token(), &meta_data.type_meta_data, &mut CurrentGenerics::new())?;
        if soul_type.is_literal() {
            continue;
        }
        
        let cpp_type = CppType::from_soul_type(&soul_type, meta_data, &abstract_syntax_tree.global_context, &SoulSpan{line_number: 0, line_offset: 0})?;
        string_builder.push_str("extern ");
        string_builder.push_str(cpp_type.as_str());
        get_var_name(string_builder, &var.name);
        string_builder.push_str(";\n");
    }

    Ok(())
}

fn forward_declare_functions(string_builder: &mut String, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    let last_internal_id = &INTERNAL_FUNCTIONS[INTERNAL_FUNCTIONS.len()-1].id; 

    for (id, scope) in &meta_data.scope_store {
        if scope.function_store.is_empty() {
            continue;
        }
        
        if id != &MetaData::GLOBAL_SCOPE_ID {
            get_scope_namespace(string_builder, id);
            string_builder.push_str(" {\n");
        }

        for (_, func) in scope.function_store.from_id.iter().filter(|func| func.0 > last_internal_id) {
            if func.name == "main" {
                continue;
            }

            function_declaration_to_cpp(string_builder, func, meta_data, context)?;
            string_builder.push_str(";\n");
        }

        if id != &MetaData::GLOBAL_SCOPE_ID {
            string_builder.push_str("};\n");
        }
    }

    Ok(())
}

fn get_var_name(string_builder: &mut String, name: &str) {
    const ILLIGAL_CPP_NAMES: &[&str] = &[
        "bool", 
        "char", "wchar_t", "char8_t", "char16_t", "char32_t",
        "short", "int", "long",
        "float", "double", "void"
    ];
    
    if ILLIGAL_CPP_NAMES.iter().any(|il| il == &name) {
        string_builder.push_str("__var_");
    }
    string_builder.push_str(&name);
}

fn get_scope_namespace(string_builder: &mut String, id: &ScopeId) {
    string_builder.push_str("namespace __SOUL_ns_scp_");
    string_builder.push_str(&id.0.to_string());
    string_builder.push_str("__");
}

fn function_declaration_to_cpp(string_builder: &mut String, func: &FunctionDeclaration, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    const SPAN: SoulSpan = SoulSpan{line_number: 0, line_offset: 0};

    let soul_str = func.return_type.as_ref().map(|string| string.as_str())
        .unwrap_or(SOUL_NAMES.get_name(NamesInternalType::None));

    let soul_type = SoulType::from_stringed_type(soul_str, &token_from_span(&SPAN), &meta_data.type_meta_data, &context.current_generics)?;
    
    string_builder.push_str(
        CppType::from_soul_type(&soul_type, meta_data, context, &SPAN)?.as_str()
    );
    
    string_builder.push_str(&func.name);
    string_builder.push('(');

    string_builder.push(')');

    Ok(())
}

fn token_from_span(span: &SoulSpan) -> Token {
    Token{line_number: span.line_number, line_offset: span.line_offset, text: String::new()}
}

fn dummy_token() -> Token {
    Token{line_number: 0, line_offset: 0, text: String::new()}
}
















