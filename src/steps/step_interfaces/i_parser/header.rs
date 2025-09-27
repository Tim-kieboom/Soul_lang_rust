use std::collections::HashMap;
use bincode::{Decode, Encode};
use crate::file_cache::FileCache;
use serde::{Deserialize, Serialize};
use crate::{run_options::{run_options::RunOptions}, steps::step_interfaces::i_parser::{abstract_syntax_tree::soul_type::type_kind::SoulPagePath, scope_builder::{ScopeBuilder, ScopeKind}}};

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct Header {
    pub scope: HashMap<String, Vec<ScopeKind>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct ExternalHeaders(HashMap<SoulPagePath, Header>);

impl Header {
    pub fn from_scope_builder(scopes: &ScopeBuilder) -> Header {
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

impl ExternalHeaders {

    pub fn new(run_options: &RunOptions) -> Result<Self, String> {
        let pages = run_options.get_file_paths()
            .map_err(|err| err.to_err_message().join(" "))?;

        let mut headers = HashMap::with_capacity(pages.len());
        for file_path in pages {
            
            let header = FileCache::read_header(run_options, &file_path)
                .map_err(|err| err.to_string())?;
            
            headers.insert(SoulPagePath::from_path(&file_path), header);
        }

        Ok(Self(headers))
    }
}

fn starts_with_capital(text: &str) -> bool {
    text.chars()
        .next()
        .map(|c| c.is_uppercase())
        .unwrap_or(false)
}


