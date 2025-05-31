use core::fmt;
use std::ops::Index;

use crate::{abstract_styntax_tree::abstract_styntax_tree::{IStatment, IVariable}, meta_data::function::function_declaration::function_declaration::{FunctionDeclaration, FunctionID}};

#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum StatmentType {
    CloseScope,
    EmptyStatment,
    Assignment,
    Initialize{is_mutable: bool, is_assigned: bool, var: IVariable},
    FunctionBody{func_info: FunctionDeclaration},
    FunctionCall,
    Scope,
}

impl StatmentType {

    #[allow(dead_code)]
    ///this fn is to insure the StatmentType.variants == IStatment.variants
    fn _insure_impl(statment: IStatment) -> StatmentType {
        match statment {
            IStatment::CloseScope() => StatmentType::CloseScope,
            IStatment::EmptyStatment() => StatmentType::EmptyStatment,
            IStatment::Assignment{..} => StatmentType::Assignment,
            IStatment::Initialize{..} => StatmentType::Initialize{is_assigned:false, is_mutable:false, var: IVariable::new_variable("", "")},
            IStatment::FunctionBody{..} => StatmentType::FunctionBody{func_info:FunctionDeclaration::new(String::new(), None, Vec::new(), false, FunctionID(0))},
            IStatment::FunctionCall{..} => StatmentType::FunctionCall,
            IStatment::Scope{..} => StatmentType::Scope,
        }
    }

}

impl fmt::Debug for StatmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CloseScope => write!(f, "CloseScope"),
            Self::EmptyStatment => write!(f, "EmptyStatment"),
            Self::Assignment => write!(f, "Assignment"),
            Self::Initialize{..} => write!(f, "Initialize"),
            Self::FunctionBody{..} => write!(f, "FunctionBody"),
            Self::FunctionCall => write!(f, "FunctionCall"),
            Self::Scope => write!(f, "Scope"),
        }
    }
}

pub struct StatmentIterator {
    statments: Vec<StatmentType>,
    index: i64
}
impl StatmentIterator {
    pub fn new(statments: Vec<StatmentType>) -> Self {
        Self {statments, index: -1}
    }

    pub fn len(&self) -> usize {
        self.statments.len()
    }

    pub fn current(&self) -> &StatmentType {
        &self.statments[self.index.max(0) as usize]
    }

    pub fn current_index(&self) -> usize {
        self.index.max(0) as usize
    }

    pub fn next(&mut self) -> Option<&StatmentType> {
        self.next_multiple(1)
    } 

    pub fn peek(&self) -> Option<&StatmentType> {
        self.peek_multiple(1)
    }

    pub fn go_to_index(&mut self, index: usize) -> Option<&StatmentType> {
        if index >= self.statments.len() {
            None
        } else {
            self.index = index as i64;
            Some(&self.statments[self.index as usize])
        }
    }

    pub fn next_multiple(&mut self, steps: i64) -> Option<&StatmentType> {
        let next_index = self.index as i64 + steps;
        if next_index < 0 {
            self.index = next_index;
            None
        }
        else if next_index as usize >= self.statments.len(){
            None
        } 
        else {
            self.index = next_index;
            Some(&self.statments[self.index as usize])
        }
    }

    pub fn peek_multiple(&self, steps: i64) -> Option<&StatmentType> {
        let peek_index = (self.index as i64 + steps) as usize;
        if peek_index < self.statments.len() {
            Some(&self.statments[peek_index])
        } 
        else {
            None
        }
    }

    pub fn get_statments_mut(&mut self) -> &mut Vec<StatmentType> {
        &mut self.statments
    }
}

impl Index<usize> for StatmentIterator {
    type Output = StatmentType;

    fn index(&self, index: usize) -> &Self::Output {
        &self.statments[index]
    }
}

impl fmt::Debug for StatmentIterator {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format.debug_struct("TokenIterator")
              .field("index", &self.index)
              .field("tokens", &self.statments)
              .field("current()", &self.current())
              .finish()
    }
}













