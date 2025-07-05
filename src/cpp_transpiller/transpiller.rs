use crate::meta_data::soul_type::type_modifiers::TypeModifiers;
use crate::tokenizer::token::Token;
use crate::cpp_transpiller::cpp_type::CppType;
use crate::run_options::run_options::RunOptions;
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

const IGNORE_WARNGINGS: &str = 
"#if defined(__clang__) && defined(__cplusplus)\n\
#pragma clang diagnostic push\n\
#pragma clang diagnostic ignored \"-Wparentheses-equality\"\n\
#elif defined(__GNUC__) && !defined(__clang__) && defined(__cplusplus)\n\
#pragma GCC diagnostic push\n\
#pragma GCC diagnostic ignored \"-Wparentheses\"\n\
#endif";

const IGNORE_WARNGINGS_END: &str = 
"#if defined(__clang__) && defined(__cplusplus)
#pragma clang diagnostic pop
#elif defined(__GNUC__) && !defined(__clang__) && defined(__cplusplus)
#pragma GCC diagnostic pop
#endif";

pub fn transpiller_to_cpp(meta_data: &MetaData, statment_iter: &StatmentIterator, abstract_syntax_tree: &AbstractSyntaxTree, run_options: &RunOptions) -> Result<String> {
    
    let mut writer = CppWriter::new(run_options.pretty_cpp_code);

    writer.start_line();
    writer.push_str("#include \"soul_hardCodedFunctions/soul_hardCodedFunctions.h\"\n");
    writer.push_str(IGNORE_WARNGINGS);
    writer.end_line();


    forward_declare_global_vars(&mut writer, meta_data, abstract_syntax_tree)?;
    writer.end_line();
    declare_c_strs(&mut writer, meta_data);
    writer.end_line();
    
    forward_declare_functions(&mut writer, meta_data, &abstract_syntax_tree.global_context)?;
    writer.end_line();

    let context = &abstract_syntax_tree.global_context;
    forward_declare_program_memory(&mut writer, statment_iter, meta_data, &context)?;
    writer.end_line();

    for statment in &abstract_syntax_tree.main_nodes {
        statment_to_cpp(&mut writer, statment_iter, statment, meta_data, context, MetaData::GLOBAL_SCOPE_ID)?;
    }

    writer.push_str(IGNORE_WARNGINGS_END);

    Ok(writer.consume_to_string())
}

fn declare_c_strs(writer: &mut CppWriter, meta_data: &MetaData) {
    for (_, pair) in meta_data.type_meta_data.c_str_store.from_name_map() {
        writer.start_line();
        writer.push_str("constexpr char __temp");
        writer.push_str(&pair.name);
        writer.push_str("[] = ");
        writer.push_str(&pair.c_str);
        writer.push_str(";");
        writer.end_line();

        writer.start_line();
        writer.push_str("constexpr __Soul_ARRAY__<char>::AsConst ");
        writer.push_str(&pair.name);
        writer.push_str(" = __Soul_ARRAY_LiteralCtor__(char, __temp");
        writer.push_str(&pair.name);
        writer.push_str(");");
        writer.end_line();
    }
}

fn forward_declare_program_memory(writer: &mut CppWriter, statment_iter: &StatmentIterator, meta_data: &MetaData, context: &CurrentContext) -> Result<()> {
    let token = Token{text: String::new(), line_number: 0, line_offset: 0};
    let span = SoulSpan{line_number: 0, line_offset: 0};
    
    for (mem, id) in &meta_data.program_memory.store {

        if mem.is_array {
            let mut soul_type = SoulType::from_stringed_type(&mem.type_name, &token, &meta_data.type_meta_data, &context.current_generics)?;
            soul_type.remove_modifier(TypeModifiers::Const);
            soul_type.add_modifier(TypeModifiers::Literal)
                .map_err(|msg| new_soul_error(&token, &msg))?;

            
            if mem.value.is_empty() {
                writer.start_line();
                writer.push_str(CppType::from_soul_type(&soul_type, meta_data, context, &span)?.as_str());
                writer.push_str("* ");
                writer.push_str(&mem.make_var_name(*id));
                writer.push_str(" = nullptr;");
                writer.end_line();
                continue;
            }

            writer.start_line();
            writer.push_str(CppType::from_soul_type(&soul_type, meta_data, context, &span)?.as_str());
            writer.push(' ');
            writer.push_str(&mem.make_var_name(*id));
            writer.push_str("[] = {");
            writer.push_str(&mem.value);
            writer.push_str("};");
            writer.end_line();
        }
        else {
            let variable = IVariable::Variable{name: mem.make_var_name(*id), type_name: mem.type_name.clone(), span: span.clone()};
            let variable_span = span.clone();
            
            let assignment = IStatment::new_assignment(
                IExpression::IVariable{this: variable.clone(), span: variable_span}, 
                IExpression::new_literal(&mem.value, &mem.type_name, &token), 
                &token
            )?;
            
            let init = IStatment::new_initialize(variable, Some(assignment), &token);
            statment_to_cpp(writer, statment_iter, &init, meta_data, context, MetaData::GLOBAL_SCOPE_ID)?;
        }
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
        writer.start_line();
        writer.push_str("extern ");
        writer.push_str(cpp_type.as_str());
        writer.push(' ');
        get_var_name(writer, &var.name);
        writer.push_str(";");
        writer.end_line();
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

        writer.start_line();
        function_declaration_to_cpp(writer, func, meta_data, context)?;
        writer.push_str(";");
        writer.end_line();
    }

    Ok(())
}



fn dummy_token() -> Token {
    Token{line_number: 0, line_offset: 0, text: String::new()}
}

