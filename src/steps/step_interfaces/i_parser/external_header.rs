use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{self, BufReader}, path::Path};
use crate::{cache_file::get_cache_path_header, run_options::run_options::RunOptions, steps::step_interfaces::i_parser::{abstract_syntax_tree::{soul_type::type_kind::TypeKind}, scope::{ExternalPages, ScopeKind, SoulPagePath}}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub scope: HashMap<String, Vec<ScopeKind>>,
    pub types: HashMap<String, TypeKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalHeader {
    pub store: HashMap<SoulPagePath, Header>, 
}

impl Header {
    pub fn from<const N: usize, const M: usize>(scope: [(String, Vec<ScopeKind>); N], types: [(String, TypeKind); M]) -> Self {
        Self{scope: HashMap::from(scope), types: HashMap::from(types)}
    }

    pub fn from_bin_file<P: AsRef<Path>>(path: &P) -> io::Result<Header> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: Header = bincode::deserialize_from(reader)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;
        Ok(data)
    }
}

impl ExternalHeader {
    pub fn new(pages: ExternalPages, run_options: &RunOptions) -> io::Result<Self> {
        let mut store = HashMap::new();
        for (name, is_internal) in pages.pages {
            
            const ADD_SOUL_EXTENTION: bool = true;

            let path = if let Some(path) = is_internal.0 {
                path
            }
            else {
                get_cache_path_header(run_options, &name.to_path_buf(ADD_SOUL_EXTENTION))
            };

            let header = Header::from_bin_file(&path)
                .map_err(|err| io::Error::new(err.kind(), format!("while trying to get header.bin from: '{}', {}", path.to_string_lossy(), err.to_string())))?;
            
            store.insert(name, header);
        }

        Ok(Self{store})
    }

    pub fn from<const N: usize>(arr: [(SoulPagePath, Header); N]) -> Self {
        Self{store: HashMap::from(arr)}
    }

    pub fn save_to_file(&self, path: &str) -> io::Result<()> {
        let data = bincode::serialize(self).unwrap();
        std::fs::write(path, data)
    }

    pub fn from_bin_file(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: Self = bincode::deserialize_from(reader)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;
        Ok(data)
    }
}















