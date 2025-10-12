use bincode::{Decode, Encode};
use serde::{Deserialize, Serialize};
use std::{ffi::OsStr, fs::File, io::{BufReader, Write}, path::{Path, PathBuf}, time::SystemTime};
use crate::{run_options::run_options::RunOptions, steps::step_interfaces::i_parser::{header::Header, parser_response::ParserResponse}};

/// A cached representation of a parsed file, including its header, parse tree, and modification date.
/// The struct can be serialized and deserialized using `serde` and `bincode`.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode)]
pub struct FileCache {
    /// The file system modification date of the original source file.
    pub date: SystemTime,
    /// The parsed header information from the file.
    pub header: Header,
    /// The parser's response (AST or equivalent) for the file.
    pub parse: ParserResponse,
}

type IoResult<T> = std::io::Result<T>;
type DynResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;
impl FileCache {

    /// Creates a new [`FileCache`] from a given file path, header, and parser result.
    ///
    /// # Arguments
    /// * `soul_file` - Path to the source file to cache.
    /// * `header` - Parsed header data.
    /// * `parser` - Parsed AST or other parser response.
    ///
    /// # Returns
    /// An `std::io::Result` containing a new [`FileCache`] instance or an IO error.s
    pub fn new(soul_file: &Path, header: Header, parser: ParserResponse) -> IoResult<Self> {
        Ok(Self{
            header,
            parse: parser,
            date: Self::get_date(soul_file)?,
        })
    }

    /// Reads the cached modification date for a given file from disk.
    ///
    /// # Arguments
    /// * `run_option` - Global runtime options.
    /// * `file_path` - Path to the source file.
    ///
    /// # Returns
    /// A result containing the [`SystemTime`] of the cached date, or an error message.
    pub fn read_date(run_option: &RunOptions, file_path: &Path) -> Result<SystemTime, String> {
        let date = CachePaths::get_date(run_option, file_path);
        Self::from_disk(&date).map_err(|err| format!("error: {}, path: {}", err.to_string(), date.to_string_lossy()))
    }

    /// Reads the cached parse tree for a given file from disk.
    ///
    /// # Arguments
    /// * `run_option` - Global runtime options.
    /// * `file_path` - Path to the source file.
    ///
    /// # Returns
    /// A result containing the [`ParserResponse`], or an error message.
    pub fn read_parse(run_option: &RunOptions, file_path: &Path) -> Result<ParserResponse, String> {
        let parse = CachePaths::get_parse(run_option, file_path);
        Self::from_disk(&parse).map_err(|err| format!("error: {}, path: {}", err.to_string(), parse.to_string_lossy()))
    }

    /// Reads the cached header data for a given file from disk.
    ///
    /// # Arguments
    /// * `run_option` - Global runtime options.
    /// * `file_path` - Path to the source file.
    ///
    /// # Returns
    /// A result containing the [`Header`], or an error message.
    pub fn read_header(run_option: &RunOptions, file_path: &Path) -> Result<Header, String> {
        let header = CachePaths::get_header(run_option, file_path);
        Self::from_disk(&header).map_err(|err| format!("error: {}, path: {}", err.to_string(), header.to_string_lossy()))
    }

    /// Writes this [`FileCache`] instance to disk in binary form, storing separate files for date, parse, and header.
    ///
    /// # Arguments
    /// * `run_option` - Global runtime options.
    /// * `file_path` - Path to the source file.
    ///
    /// # Returns
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



