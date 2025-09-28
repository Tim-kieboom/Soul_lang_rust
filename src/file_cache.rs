use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs::File, io::{BufReader, Write}, path::{Path, PathBuf}, time::SystemTime};
use crate::{run_options::run_options::RunOptions, steps::step_interfaces::i_parser::{header::Header, parser_response::ParserResponse}};


#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct FileCache {
    pub date: SystemTime,
    pub header: Header,
    pub parse: ParserResponse,
}

type IoResult<T> = std::io::Result<T>;
type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
impl FileCache {

    pub fn new(soul_file: &Path, header: Header, parser: ParserResponse) -> IoResult<Self> {
        Ok(Self{
            header,
            parse: parser,
            date: Self::get_date(soul_file)?,
        })
    }

    pub fn read_date(run_option: &RunOptions, file_path: &Path) -> Result<SystemTime, String> {
        let date = CachePaths::get_date(run_option, file_path);
        Self::from_disk(&date).map_err(|err| format!("error: {}, path: {}", err.to_string(), date.to_string_lossy()))
    }

    pub fn read_parse(run_option: &RunOptions, file_path: &Path) -> Result<ParserResponse, String> {
        let parse = CachePaths::get_parse(run_option, file_path);
        Self::from_disk(&parse).map_err(|err| format!("error: {}, path: {}", err.to_string(), parse.to_string_lossy()))
    }

    pub fn read_header(run_option: &RunOptions, file_path: &Path) -> Result<Header, String> {
        let header = CachePaths::get_header(run_option, file_path);
        Self::from_disk(&header).map_err(|err| format!("error: {}, path: {}", err.to_string(), header.to_string_lossy()))
    }

    pub fn write_to_disk(&self, run_option: &RunOptions, file_path: &Path) -> Result<(), String> {
        let folder = CachePaths::get_cache_folder(run_option, file_path);
        std::fs::create_dir_all(&folder)
            .map_err(|err| format!("error: {}, path: {}", err.to_string(), folder.to_string_lossy()))?;

        let date = CachePaths::get_date(run_option, file_path);
        let parse = CachePaths::get_parse(run_option, file_path);
        let header = CachePaths::get_header(run_option, file_path);

        Self::write_file(&self.date, &date).map_err(|err| format!("error: {}, path: '{}'", err.to_string(), date.to_string_lossy()))?;
        Self::write_file(&self.parse, &parse).map_err(|err| format!("error: {}, path: '{}'", err.to_string(), parse.to_string_lossy()))?;
        Self::write_file(&self.header, &header).map_err(|err| format!("error: {}, path: '{}'", err.to_string(), header.to_string_lossy()))
    }
    
    fn write_file<T: Encode>(val: &T, path: &Path) -> DynResult<()> {

        let binary = bincode::encode_to_vec(val, bincode::config::standard())?;
        Self::get_file(&path.to_string_lossy())?
            .write_all(&binary)?;
        
        Ok(())
    }

    fn from_disk<Out: Decode<()>>(path: &Path) -> DynResult<Out> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let value: Out = bincode::decode_from_reader(reader, bincode::config::standard())?;
        Ok(value)
    }

    fn get_file(path: &str) -> IoResult<File> {
        std::fs::OpenOptions::new()
            .write(true)       
            .create(true)     
            .open(path)
    }

    fn get_date(file: &Path) -> IoResult<SystemTime> {
        let file = File::open(file)?;
        let data = file.metadata()?;
        Ok(data.modified()?)
    }
}

struct CachePaths;
impl CachePaths {
    const PARSED_INCREMENTAL_FOLDER_NAME: &str = "parsedIncremental";
    const PATH_CAP: usize = Self::PARSED_INCREMENTAL_FOLDER_NAME.len() + 4/*for .hdr/.ast/.dte*/;

    pub fn get_cache_folder(run_option: &RunOptions, file_path: &Path) -> PathBuf {
        let mut dir = Self::get_dir_path(run_option, file_path);
        dir.push(file_path);
        dir
    }

    pub fn get_header(run_option: &RunOptions, file_path: &Path) -> PathBuf {
        let mut dir = Self::get_cache_folder(run_option, file_path);
        dir.push(format!("{}.hdr", Self::get_path_name(file_path).unwrap_or_else(|err| panic!("{err}")).to_string_lossy() ));
        dir
    }

    pub fn get_parse(run_option: &RunOptions, file_path: &Path) -> PathBuf {
        let mut dir = Self::get_cache_folder(run_option, file_path);
        dir.push(format!("{}.ast", Self::get_path_name(file_path).unwrap_or_else(|err| panic!("{err}")).to_string_lossy() ));
        dir
    }

    pub fn get_date(run_option: &RunOptions, file_path: &Path) -> PathBuf {
        let mut dir = Self::get_cache_folder(run_option, file_path);
        dir.push(format!("{}.dte", Self::get_path_name(file_path).unwrap_or_else(|err| panic!("{err}")).to_string_lossy() ));
        dir
    }

    fn get_dir_path(run_option: &RunOptions, file_path: &Path) -> PathBuf {
        let mut dir = PathBuf::with_capacity(Self::PATH_CAP + run_option.output_dir.as_os_str().len() + file_path.as_os_str().len());
        dir.push(&run_option.output_dir);
        dir.push(Self::PARSED_INCREMENTAL_FOLDER_NAME);
        dir
    }

    fn get_path_name<'a>(file_path: &'a Path) -> Result<&'a OsStr, String> {
        file_path.file_name().ok_or(format!("path: '{}' is does not have file_name", file_path.to_string_lossy()))
    }
}



