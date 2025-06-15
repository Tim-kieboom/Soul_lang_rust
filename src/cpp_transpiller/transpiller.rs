use crate::tokenizer::token::Token;
use crate::cpp_transpiller::cpp_type::CppType;
use crate::cpp_transpiller::cpp_writer::CppWriter;
use crate::meta_data::soul_type::soul_type::SoulType;
use crate::meta_data::function::internal_functions::INTERNAL_FUNCTIONS;
use crate::cpp_transpiller::convert_to_cpp::variable_to_cpp::get_var_name;
use crate::meta_data::soul_error::soul_error::{new_soul_error, Result, SoulSpan};
use crate::meta_data::current_context::current_context::{CurrentContext, CurrentGenerics};
use crate::abstract_styntax_tree::abstract_styntax_tree::{IExpression, IStatment, IVariable};
use crate::cpp_transpiller::convert_to_cpp::statment_to_cpp::{function_declaration_to_cpp, statment_to_cpp};
use crate::{abstract_styntax_tree::abstract_styntax_tree::AbstractSyntaxTree, meta_data::meta_data::MetaData};
use crate::abstract_styntax_tree::get_abstract_syntax_tree::get_stament::statment_type::statment_type::StatmentIterator;

pub fn transpiller_to_cpp(meta_data: &MetaData, statment_iter: &StatmentIterator, abstract_syntax_tree: &AbstractSyntaxTree) -> Result<String> {
    
    let mut writer = CppWriter::new();

    writer.push_str("#include \"soul_hardCodedFunctions/soul_hardCodedFunctions.h\"\n");


    forward_declare_global_vars(&mut writer, meta_data, abstract_syntax_tree)?;
    declare_c_strs(&mut writer, meta_data);
    
    forward_declare_functions(&mut writer, meta_data, &abstract_syntax_tree.global_context)?;

    let context = &abstract_syntax_tree.global_context;
    forward_declare_program_memory(&mut writer, statment_iter, meta_data, &context)?;
    for statment in &abstract_syntax_tree.main_nodes {
        statment_to_cpp(&mut writer, statment_iter, statment, meta_data, context, MetaData::GLOBAL_SCOPE_ID)?;
    }

    Ok(writer.consume_to_string())
}

fn declare_c_strs(writer: &mut CppWriter, meta_data: &MetaData) {
    for (_, pair) in meta_data.type_meta_data.c_str_store.from_name_map() {
        writer.push_str("constexpr auto ");
        writer.push_str(&pair.name);
        writer.push_str(" = __NEW_Soul_LITERAL_ARRAY_C_STR__(");
        writer.push_str(&pair.c_str);
        writer.push_str(");\n");
    }
}

fn forward_declare_program_memory(writer: &mut CppWriter, statment_iter: &StatmentIterator, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    let token = Token{text: String::new(), line_number: 0, line_offset: 0};
    let span = SoulSpan{line_number: 0, line_offset: 0};
    
    for mem in &meta_data.program_memory {

        let variable = IVariable::Variable{name: mem.make_var_name(), type_name: mem.type_name.clone(), span: span.clone()};
        let assignment = IStatment::new_assignment(variable.clone(), IExpression::new_literal(&mem.value, &mem.type_name, &token), &token);
        let init = IStatment::new_initialize(variable, Some(assignment), &token);
        statment_to_cpp(writer, statment_iter, &init, meta_data, context, MetaData::GLOBAL_SCOPE_ID)?;
    }

    Ok(())
}


fn forward_declare_global_vars(writer: &mut CppWriter, meta_data: &MetaData, abstract_syntax_tree: &AbstractSyntaxTree) -> Result<()> {
    let global_scope = meta_data.scope_store.get(&MetaData::GLOBAL_SCOPE_ID)
        .ok_or(new_soul_error(&dummy_token(), "Internal error in cpp transpiller global scope not found"))?;

    for (_, var) in &global_scope.vars {
        let soul_type = SoulType::from_stringed_type(&var.type_name, &dummy_token(), &meta_data.type_meta_data, &mut CurrentGenerics::new())?;
        if soul_type.is_literal() {
            continue;
        }
        
        let cpp_type = CppType::from_soul_type(&soul_type, meta_data, &abstract_syntax_tree.global_context, &SoulSpan{line_number: 0, line_offset: 0})?;
        writer.push_str("extern ");
        writer.push_str(cpp_type.as_str());
        writer.push(' ');
        get_var_name(writer, &var.name);
        writer.push_str(";\n");
    }

    Ok(())
}

fn forward_declare_functions(writer: &mut CppWriter, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    let last_internal_id = &INTERNAL_FUNCTIONS[INTERNAL_FUNCTIONS.len()-1].id; 

    let scope = meta_data.scope_store.get(&MetaData::GLOBAL_SCOPE_ID).unwrap();

    for (_, func) in scope.function_store.from_id.iter().filter(|func| func.0 > last_internal_id) {
        if func.name == "main" {
            continue;
        }

        function_declaration_to_cpp(writer, func, meta_data, context)?;
        writer.push_str(";\n");
    }

    Ok(())
}



fn dummy_token() -> Token {
    Token{line_number: 0, line_offset: 0, text: String::new()}
}

