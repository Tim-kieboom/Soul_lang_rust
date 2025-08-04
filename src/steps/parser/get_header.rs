use crate::steps::step_interfaces::i_parser::scope::ScopeBuilder;
use crate::steps::step_interfaces::i_parser::{external_header::Header};

pub fn get_header(scopes: &ScopeBuilder) -> Header {
    let mut header = Header{scope: vec![], types: vec![]};
    
    for (name, scopes) in &scopes.get_global_scope().symbols {
        
        if starts_with_capital(name.as_str()) {
            header.scope.extend_from_slice(scopes.as_slice());
        }
    }

    for (name, ty) in &scopes.get_global_types().symbols {
        
        if starts_with_capital(name.as_str()) {
            header.types.push(ty.clone());
        }
    }

    header
}

fn starts_with_capital(text: &str) -> bool {
    text.chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}

















































