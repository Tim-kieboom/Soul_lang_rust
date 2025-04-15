use std::io::Result;
use crate::{abstract_styntax_tree::abstract_styntax_tree::AbstractSyntaxTree, meta_data::{self, meta_data::MetaData}, tokenizer::token::TokenIterator};

#[allow(dead_code)]
pub fn get_abstract_syntax_tree_file(iterator: &mut TokenIterator, meta_data: &mut MetaData) -> Result<AbstractSyntaxTree> {
    let mut tree = AbstractSyntaxTree::new();
    
    
    todo!();
}

#[allow(dead_code)]
pub fn add_to_abstract_syntax_tree_line(tree: &mut AbstractSyntaxTree, iterator: &mut TokenIterator, meta_data: &mut MetaData) -> Result<AbstractSyntaxTree> {
    
    
    
    todo!();
}