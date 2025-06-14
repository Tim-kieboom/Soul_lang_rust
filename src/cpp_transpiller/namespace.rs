use crate::{cpp_transpiller::cpp_writer::CppWriter, meta_data::scope_and_var::scope::ScopeId};

pub fn get_scope_namespace(writer: &mut CppWriter, id: &ScopeId) {
    writer.push_str("__SOUL_ns_scp_");
    writer.push_str(&id.0.to_string());
    writer.push_str("__");
}












