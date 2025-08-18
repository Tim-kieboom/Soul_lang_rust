use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs::File, io::{self, BufReader}, path::Path};
use crate::{cache_file::get_cache_path_header, prelude::CloneWithPool, run_options::run_options::RunOptions, steps::step_interfaces::i_parser::{abstract_syntax_tree::{soul_type::type_kind::TypeKind}, scope::{ExternalPages, ScopeKind, SoulPagePath}}, utils::serde_multi_ref::MultiRefPool};

#[derive(Debug, Clone)]
pub struct Header {
    pub scope: HashMap<String, Vec<ScopeKind>>,
    pub types: HashMap<String, TypeKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeHeader {
    pub pool: MultiRefPool,
    pub scope: HashMap<String, Vec<ScopeKind>>,
    pub types: HashMap<String, TypeKind>,
}

#[derive(Debug, Clone)]
pub struct ExternalHeader {
    pub store: HashMap<SoulPagePath, Header>, 
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerdeExternalHeader {
    pub store: HashMap<SoulPagePath, SerdeHeader>, 
}

impl Header {
    pub fn from<const N: usize, const M: usize>(scope: [(String, Vec<ScopeKind>); N], types: [(String, TypeKind); M]) -> Self {
        Self{scope: HashMap::from(scope), types: HashMap::from(types)}
    }

    pub fn to_serde_header(&self, src_pool: &mut MultiRefPool) -> SerdeHeader {
        let mut this = SerdeHeader{pool: MultiRefPool::new(), scope: HashMap::new(), types: HashMap::new()};
        let dst_pool = &mut this.pool;

        this.scope = self.scope.iter().map(|(name, el)| {
            (
                name.clone(),
                el.iter().map(|el| el.clone_change_ref_pool(src_pool, dst_pool)).collect()
            )
        }).collect();

        this
    }

    pub fn from_serde_header(serde: SerdeHeader, dst_pool: &mut MultiRefPool) -> Header {
        let mut this = Header{scope: HashMap::new(), types: HashMap::new()};

        this.scope = serde.scope.iter().map(|(name, el)| {
            (
                name.clone(),
                el.iter().map(|el| el.clone_change_ref_pool(&serde.pool, dst_pool)).collect()
            )
        }).collect();

        this
    }

    pub fn from_bin_file<P: AsRef<Path>>(path: &P, dst_pool: &mut MultiRefPool) -> io::Result<Header> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: SerdeHeader = bincode::deserialize_from(reader)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;
        Ok(Self::from_serde_header(data, dst_pool))
    }
}

impl ExternalHeader {
    pub fn new(pages: ExternalPages, run_options: &RunOptions, pool: &mut MultiRefPool) -> io::Result<Self> {
        let mut store = HashMap::new();
        for (name, is_internal) in pages.pages {
            
            const ADD_SOUL_EXTENTION: bool = true;

            let path = if let Some(path) = is_internal.0 {
                path
            }
            else {
                get_cache_path_header(run_options, &name.to_path_buf(ADD_SOUL_EXTENTION))
            };

            let header = Header::from_bin_file(&path, pool)
                .map_err(|err| io::Error::new(err.kind(), format!("while trying to get header.bin from: '{}', {}", path.to_string_lossy(), err.to_string())))?;
            
            store.insert(name, header);
        }

        Ok(Self{store})
    }

    pub fn from<const N: usize>(arr: [(SoulPagePath, Header); N]) -> Self {
        Self{store: HashMap::from(arr)}
    }

    pub fn to_serde_external_header(&self, src_pool: &mut MultiRefPool) -> SerdeExternalHeader {
        SerdeExternalHeader {
            store: self.store.iter().map(|(k, v)| (k.clone(), v.to_serde_header(src_pool))).collect()
        }
    }

    pub fn from_serde_external_header(serde: SerdeExternalHeader, src_pool: &mut MultiRefPool) -> Self {
        Self {
            store: serde.store.iter().map(|(k, v)| (k.clone(), Header::from_serde_header(v.clone(), src_pool))).collect()
        }
    }

    pub fn save_to_file(&self, path: &str, src_pool: &mut MultiRefPool) -> io::Result<()> {
        let data = bincode::serialize(&self.to_serde_external_header(src_pool)).unwrap();
        std::fs::write(path, data)
    }

    pub fn from_bin_file(path: &str, src_pool: &mut MultiRefPool) -> io::Result<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: SerdeExternalHeader = bincode::deserialize_from(reader)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidInput, err.to_string()))?;
        Ok(Self::from_serde_external_header(data, src_pool))
    }
}















