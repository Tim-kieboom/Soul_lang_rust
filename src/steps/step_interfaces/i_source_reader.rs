use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FileLine {
    pub line: String,
    pub line_number: usize,
}

pub type CstrVarName = String; 
pub type RawCstr = String; 
pub type LineNumber = usize;
pub type LineOffset = usize;

#[derive(Debug)]
pub struct SourceFileResult {
    pub source_file: Vec<FileLine>,
    pub c_str_store: HashMap<RawCstr, CstrVarName>,
    pub gaps: HashMap<LineNumber, HashMap<LineOffset, i64>>,
    pub estimated_token_count: usize,
}
impl SourceFileResult {
    pub fn new() -> Self {
        Self { source_file: Vec::new(), c_str_store: HashMap::new(), gaps: HashMap::new(), estimated_token_count: 0 }
    }
}














