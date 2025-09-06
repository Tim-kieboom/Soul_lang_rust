use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{BufReader, Write}, path::{Path, PathBuf}, time::SystemTime};
use crate::{run_options::run_options::RunOptions, steps::step_interfaces::i_parser::{header::Header, parser_response::ParserResponse}};


#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct FileCache {
    pub date: SystemTime,
    pub header: Header,
    pub parse: ParserResponse,
}

type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
impl FileCache {

    pub fn new(
        soul_file: &Path,
        header: Header,
        parser: ParserResponse,
    ) -> DynResult<Self> {
        Ok(Self{
            header,
            parse: parser,
            date: get_file_date(soul_file)?,
        })
    }

    pub fn read_date(run_option: &RunOptions, file_path: &Path) -> DynResult<SystemTime> {
        Self::from_disk(&get_cache_date_path(run_option, file_path))
    }

    pub fn read_parse(run_option: &RunOptions, file_path: &Path) -> DynResult<ParserResponse> {
        Self::from_disk(&get_cache_path_parse(run_option, file_path))
    }

    pub fn read_header(run_option: &RunOptions, file_path: &Path) -> DynResult<Header> {
        Self::from_disk(&get_cache_path_header(run_option, file_path))
    }

    pub fn write_to_disk(&self, run_option: &RunOptions, file_path: &Path) -> DynResult<()> {
        std::fs::create_dir_all(get_cache_path(run_option, file_path))?;

        Self::write_file(&self.date, &get_cache_date_path(run_option, file_path))?;
        Self::write_file(&self.parse, &get_cache_path_parse(run_option, file_path))?;
        Self::write_file(&self.header, &get_cache_path_header(run_option, file_path))
    }
    
    fn write_file<T: Encode>(val: &T, path: &Path) -> DynResult<()> {

        let binary = bincode::encode_to_vec(val, bincode::config::standard())?;
        create_or_write(&path.to_string_lossy())?
            .write_all(&binary)?;
        
        Ok(())
    }

    fn from_disk<Out: Decode<()>>(path: &Path) -> DynResult<Out> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let value: Out = bincode::decode_from_reader(reader, bincode::config::standard())?;
        Ok(value)
    }
}

fn get_file_date(file: &Path) -> DynResult<SystemTime> {
    let file = File::open(file)?;
    let data = file.metadata()?;
    Ok(data.modified()?)
}

fn create_or_write(path: &str) -> std::io::Result<File> {
    std::fs::OpenOptions::new()
        .write(true)       
        .create(true)     
        .open(path)      
}

const PARSED_INCREMENTAL_FOLDER: &str = "parsedIncremental";
const PATH_CAP: usize = PARSED_INCREMENTAL_FOLDER.len() + 4/*for .hdr/.ast/.dte*/;

fn get_cache_path(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = PathBuf::with_capacity(PATH_CAP + run_option.output_dir.as_os_str().len() + file_path.as_os_str().len());
    dir.push(&run_option.output_dir);
    dir.push(PARSED_INCREMENTAL_FOLDER);
    dir.push(file_path);
    dir
}

fn get_cache_path_header(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = PathBuf::with_capacity(PATH_CAP + run_option.output_dir.as_os_str().len() + file_path.as_os_str().len());
    dir.push(&run_option.output_dir);
    dir.push(PARSED_INCREMENTAL_FOLDER);
    dir.push(format!("{}.hdr", file_path.to_str().unwrap_or(&file_path.to_string_lossy())));
    dir
}

fn get_cache_path_parse(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = PathBuf::with_capacity(PATH_CAP + run_option.output_dir.as_os_str().len() + file_path.as_os_str().len());
    dir.push(&run_option.output_dir);
    dir.push(PARSED_INCREMENTAL_FOLDER);
    dir.push(format!("{}.ast", file_path.to_str().unwrap_or(&file_path.to_string_lossy())));
    dir
}

fn get_cache_date_path(run_option: &RunOptions, file_path: &Path) -> PathBuf {
    let mut dir = PathBuf::with_capacity(PATH_CAP + run_option.output_dir.as_os_str().len() + file_path.as_os_str().len());
    dir.push(&run_option.output_dir);
    dir.push(PARSED_INCREMENTAL_FOLDER);
    dir.push(format!("{}.dte", file_path.to_str().unwrap_or(&file_path.to_string_lossy())));
    dir
}





