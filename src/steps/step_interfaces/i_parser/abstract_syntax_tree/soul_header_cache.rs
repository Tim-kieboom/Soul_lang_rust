use serde::{Deserialize, Serialize};
use std::{fs::File, io::{BufReader, BufWriter}, path::Path, time::SystemTime};
use crate::steps::step_interfaces::i_parser::{external_header::Header, parser_response::ParserResponse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulHeaderCache {
    pub mod_date: ModifiedDate,
    pub header: Header,
    pub parser: ParserResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModifiedDate {
    pub source_date: SystemTime,
}

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>; 
impl SoulHeaderCache {

    pub fn new(soul_file: &Path, header: Header, parser: ParserResponse,) -> ResErr<Self> {
        Ok(Self{
            mod_date: ModifiedDate::new(soul_file)?, 
            header, 
            parser
        })
    }

    pub fn from_bin_file(path: &str) -> ResErr<SoulHeaderCache> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let cache: SoulHeaderCache = bincode::deserialize_from(reader)?;
        Ok(cache)
    }

    pub fn save_to_bin_file(&self, path: &str) -> ResErr<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }

    pub fn is_cache_up_to_date(&self, soul_file: &str) -> ResErr<bool> {
        let file = File::open(soul_file)?;
        let meta_data = file.metadata()?;
        
        Ok(meta_data.modified()? == self.mod_date.source_date)
    }
}

impl ModifiedDate {
    pub fn new(soul_file: &Path) -> ResErr<Self> {
        let file = File::open(soul_file)?;
        let data = file.metadata()?;
        let source_date = data.modified()?;

        Ok(Self{source_date})
    } 
}















