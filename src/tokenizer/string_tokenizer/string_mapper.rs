use std::io::Result;
use super::format_stringer::{indexesof_qoutes, indexesof_qoutes_line};
use crate::{meta_data::{soul_names::{InternalType, TypeModifiers}, meta_data::MetaData, scope_and_var::var_info::{VarFlags, VarInfo}}, tokenizer::file_line::FileLine};

#[allow(dead_code)]
pub fn rawstr_to_litstr_file(source_file: Vec<FileLine>, meta_data: &mut MetaData) -> Result<Vec<FileLine>> {
    let mut new_source_file = Vec::with_capacity(source_file.len());

    let qoute_iter = indexesof_qoutes(&source_file);
    for (indexes, mut line) in qoute_iter {
        rawstr_to_litstr(&mut line, meta_data, &indexes);
        new_source_file.push(line);
    }
    
    Ok(new_source_file)
}

#[allow(dead_code)]
pub fn rawstr_to_litstr_line(line: FileLine, meta_data: &mut MetaData) -> Result<FileLine> {
    let mut new_line = line.clone();

    let qoute_iter = indexesof_qoutes_line(&line);
    rawstr_to_litstr(&mut new_line, meta_data, &qoute_iter); 
    Ok(new_line)
}

fn rawstr_to_litstr(line: &mut FileLine, meta_data: &mut MetaData, indexes: &Vec<usize>) {
    let soul_literal_name = meta_data.get_soul_name(TypeModifiers::Literal);
    let soul_string_name = meta_data.get_soul_name(InternalType::String);

    let literal_string_type_name = format!("{soul_literal_name} {soul_string_name}");

    let mut replace_offset = 0;
    for i in (0..indexes.len()).step_by(2) {
        let begin = indexes[i] + replace_offset;
        let end = indexes[i+1] + replace_offset;

        let c_str = &line.text[begin..end+1];

        if let None = meta_data.type_meta_data.c_str_store.from_c_str(c_str) {

            let c_str = c_str.to_string();
            let str_name = format!("__Soul_c_str_{}__", meta_data.type_meta_data.c_str_store.len());
            meta_data.type_meta_data.c_str_store.add(c_str, str_name.clone());
            let mut var = VarInfo::new(str_name, literal_string_type_name.clone());
            var.set_var_flag(VarFlags::IsAssigned);
            meta_data.add_to_global_scope(var);
        }

        let c_pair = meta_data.type_meta_data.c_str_store.from_c_str(c_str).unwrap();
        replace_offset += c_pair.name.len() - c_str.len();
        line.text = line.text.replace(c_str, c_pair.name.as_str());
    }
}
