//! # Soul Language – Source Reader
//!
//! This module defines the **source reading step** in the Soul language compiler pipeline.
//! It is responsible for reading raw source files, breaking them into lines, and preparing
//! data structures that assist the tokenizer and later compiler stages.
//!
//! ## Responsibilities
//! - Read the input source code file line-by-line.
//! - Store each line along with its corresponding line number.
//! - Maintain mappings for embedded C strings (`c_str_store`).
//! - Track line gaps for debugging or source position reconstruction.
//! - Estimate token count to optimize tokenizer allocations.


use std::collections::{BTreeMap, HashMap};

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
pub struct SourceFileResponse {
    pub source_file: Vec<FileLine>,
    pub c_str_store: HashMap<RawCstr, CstrVarName>,
    pub gaps: HashMap<LineNumber, BTreeMap<LineOffset, i64>>,
    pub estimated_token_count: usize,
}
impl SourceFileResponse {
    pub fn new() -> Self {
        Self { source_file: Vec::new(), c_str_store: HashMap::new(), gaps: HashMap::new(), estimated_token_count: 0 }
    }
}














