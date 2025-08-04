use serde::{Deserialize, Serialize};
use std::{fs::{File, OpenOptions}, io::{BufReader, BufWriter}, path::Path, time::SystemTime};
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
        if let Some(parent) = Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
    

        let file = create_or_write(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;

        let file = create_or_write(&format!("{}.date", path))?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, &self.mod_date)?;
        Ok(())
    }
}

impl ModifiedDate {
    
    pub fn from_bin_file(path: &str) -> ResErr<ModifiedDate> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data: ModifiedDate = bincode::deserialize_from(reader)?;
        Ok(data)
    }

    pub fn new(soul_file: &Path) -> ResErr<Self> {
        let file = File::open(soul_file)?;
        let data = file.metadata()?;
        let source_date = data.modified()?;

        Ok(Self{source_date})
    } 
}

fn create_or_write(path: &str) -> std::io::Result<File> {
    OpenOptions::new()
        .write(true)       
        .create(true)     
        .open(path)      
}














