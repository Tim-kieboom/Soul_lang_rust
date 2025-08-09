use std::{collections::HashMap, fs::File, io::{self, BufReader}};
use serde::{Deserialize, Serialize};
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::soul_type::type_kind::TypeKind, scope::{ExternalPages, ScopeKind}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub scope: Vec<ScopeKind>,
    pub types: Vec<TypeKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalHeader {
    pub store: HashMap<String, Header>, 
}

impl Header {
    pub fn from<const N: usize, const M: usize>(scope: [ScopeKind; N], types: [TypeKind; M]) -> Self {
        Self{scope: Vec::from(scope), types: Vec::from(types)}
    }

    pub fn from_bin_file(path: &str) -> io::Result<Header> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: Header = bincode::deserialize_from(reader)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;
        Ok(data)
    }
}

impl ExternalHeader {
    pub fn new(pages: ExternalPages) -> io::Result<Self> {
        let mut store = HashMap::new();
        for (name, path) in pages.store {
            let header = Header::from_bin_file(&path)?;
            store.insert(name.0, header);
        }

        Ok(Self{store})
    }

    pub fn from<const N: usize>(arr: [(String, Header); N]) -> Self {
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























