use crate::meta_data::soul_error::soul_error::{new_soul_error, Result};
use super::format_stringer::{indexesof_qoutes, indexesof_qoutes_line};
use crate::{meta_data::{meta_data::MetaData, scope_and_var::var_info::{VarFlags, VarInfo}, soul_names::{NamesInternalType, NamesTypeModifiers, SOUL_NAMES}}, tokenizer::{file_line::FileLine, token::Token}};

#[allow(dead_code)]
pub fn rawstr_to_litstr_file(source_file: Vec<FileLine>, meta_data: &mut MetaData) -> Result<Vec<FileLine>> {
    let mut new_source_file = Vec::with_capacity(source_file.len());

    let qoute_iter = indexesof_qoutes(&source_file);
    for (indexes, mut line) in qoute_iter {
        rawstr_to_litstr(&mut line, meta_data, &indexes)?;
        new_source_file.push(line);
    }
    
    Ok(new_source_file)
}

#[allow(dead_code)]
pub fn rawstr_to_litstr_line(line: FileLine, meta_data: &mut MetaData) -> Result<FileLine> {
    let mut new_line = line.clone();

    let qoute_iter = indexesof_qoutes_line(&line);
    rawstr_to_litstr(&mut new_line, meta_data, &qoute_iter)?; 
    Ok(new_line)
}

fn rawstr_to_litstr(line: &mut FileLine, meta_data: &mut MetaData, indexes: &Vec<usize>) -> Result<()> {
    
    fn get_token(line: &FileLine, offset: usize) -> Token {
        Token{text: line.text.to_owned(), line_number: line.line_number as usize, line_offset: offset}
    }
    
    let soul_literal_name = SOUL_NAMES.get_name(NamesTypeModifiers::Literal);
    let soul_string_name = SOUL_NAMES.get_name(NamesInternalType::String);

    let literal_string_type_name = format!("{soul_literal_name} {soul_string_name}");

    if indexes.len() % 2 != 0 {
        return Err(new_soul_error(&get_token(&line, 0), "opening qoute (so '\"') without a closing qoute every str NEEDS TO BE CLOSED ON THE SAME LINE"))
    }

    for i in (0..indexes.len()).rev().step_by(2) {
        let begin = indexes[i-1];
        let end = indexes[i];

        let c_str = &line.text[begin..=end];

        if meta_data.type_meta_data.c_str_store.from_c_str(c_str).is_none() {
            let c_str = c_str.to_string();
            let str_name = format!("__Soul_c_str_{}__", meta_data.type_meta_data.c_str_store.len());
            meta_data.type_meta_data.c_str_store.add(c_str, str_name.clone());
            let mut var = VarInfo::new(str_name, literal_string_type_name.clone());
            var.add_var_flag(VarFlags::IsAssigned);
            meta_data.add_to_global_scope(var)
                .map_err(|msg| new_soul_error(&get_token(line, begin), format!("while trying to add c_str to global scope\n{}", msg).as_str()))?;
        }

        let c_pair = meta_data.type_meta_data.c_str_store.from_c_str(c_str).unwrap();

        line.text.replace_range(begin..=end, c_pair.name.as_str());
    }

    Ok(())
}

