use std::collections::HashMap;
use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::{scope_builder::{ScopeBuilder, ScopeKind}};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Header {
    pub scope: HashMap<String, Vec<ScopeKind>>,
}


impl Header {
    pub fn from_scope(scopes: &ScopeBuilder) -> Header {
        let mut header = Self{scope: HashMap::new()};
        
        for (name, scopes) in &scopes.get_global_scope().symbols {
            
            if starts_with_capital(name.as_str()) {
                
                let scope = scopes.iter()
                    .cloned()
                    .map(|el| el.node)
                    .collect();

                header.scope.insert(name.clone(), scope);
            }
        }

        header
    }
}

fn starts_with_capital(text: &str) -> bool {
    text.chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}


