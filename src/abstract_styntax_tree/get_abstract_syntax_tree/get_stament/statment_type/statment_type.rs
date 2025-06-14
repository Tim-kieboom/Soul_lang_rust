use core::fmt;
use std::ops::Index;
use crate::{abstract_styntax_tree::abstract_styntax_tree::{IStatment, IVariable}, meta_data::{function::function_declaration::function_declaration::{FunctionDeclaration, FunctionID}, scope_and_var::scope::ScopeId}, tokenizer::token::Token};

pub struct StatmentTypeInfo {
    pub statment_types: Vec<StatmentType>, 
    pub scope_start_index: Vec<usize>,
    pub open_bracket_stack: i64, 
}
impl StatmentTypeInfo {
    pub fn new(open_bracket_stack: i64) -> Self {
        Self { statment_types: Vec::new(), scope_start_index: Vec::new(), open_bracket_stack }
    }

    pub fn with_capacity(open_bracket_stack: i64, capacity: usize) -> Self {
        Self { statment_types: Vec::with_capacity(capacity), scope_start_index: Vec::new(), open_bracket_stack }
    }
}

#[derive(Clone, PartialEq)]
#[repr(u32)]
pub enum StatmentType {
    CloseScope{begin_body_index: usize},
    EmptyStatment,
    Assignment,
    Initialize{is_mutable: bool, is_assigned: bool, var: IVariable},
    FunctionBody{func_info: FunctionDeclaration, end_body_index: usize},
    FunctionCall,
    Return{begin_body_index: usize},
    Scope{end_body_index: usize},
    If{end_body_index: usize},
    Else{end_body_index: usize},
    ElseIf{end_body_index: usize},
}

impl StatmentType {

    #[allow(dead_code)]
    ///this fn is to insure the StatmentType.variants == IStatment.variants
    fn _insure_impl(statment: IStatment) -> StatmentType {
        match statment {
            IStatment::CloseScope(_) => StatmentType::CloseScope{begin_body_index: 0},
            IStatment::EmptyStatment(_) => StatmentType::EmptyStatment,
            IStatment::Assignment{..} => StatmentType::Assignment,
            IStatment::Initialize{..} => StatmentType::Initialize{is_assigned:false, is_mutable:false, var: IVariable::new_variable("", "", &Token{line_number: 0, line_offset: 0, text: String::new()})},
            IStatment::FunctionBody{..} => StatmentType::FunctionBody{func_info:FunctionDeclaration::new(String::new(), None, Vec::new(), false, FunctionID(0), ScopeId(0)), end_body_index: 0},
            IStatment::FunctionCall{..} => StatmentType::FunctionCall,
            IStatment::Scope{..} => StatmentType::Scope{end_body_index: 0},
            IStatment::Return{..} => StatmentType::Return{begin_body_index: 0},
            IStatment::If{..} => StatmentType::If{end_body_index: 0},
            IStatment::Else{..} => StatmentType::Else{end_body_index: 0},
            IStatment::ElseIf{..} => StatmentType::ElseIf{end_body_index: 0},
        }
    }

    pub fn set_end_body_index(&mut self, index: usize) {
        match self {
            StatmentType::Return{..} |
            StatmentType::Assignment |
            StatmentType::FunctionCall |
            StatmentType::EmptyStatment |
            StatmentType::CloseScope{..} |
            StatmentType::Initialize{..} => panic!("Internal error trying to StatmentType::set_end_body_index() to: {:#?}", self),
            
            StatmentType::FunctionBody{end_body_index, ..} => *end_body_index = index,  
            StatmentType::Scope { end_body_index } => *end_body_index = index,
            StatmentType::If { end_body_index } => *end_body_index = index,
            StatmentType::Else { end_body_index } => *end_body_index = index,
            StatmentType::ElseIf { end_body_index } => *end_body_index = index,
        }
    }

}

impl fmt::Debug for StatmentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CloseScope{..} => write!(f, "CloseScope"),
            Self::EmptyStatment => write!(f, "EmptyStatment"),
            Self::Assignment => write!(f, "Assignment"),
            Self::Initialize{..} => write!(f, "Initialize"),
            Self::FunctionBody{..} => write!(f, "FunctionBody"),
            Self::FunctionCall => write!(f, "FunctionCall"),
            Self::Return{..} => write!(f, "Return"),
            Self::Scope{..} => write!(f, "Scope"),
            Self::If{..} => write!(f, "If"),
            Self::Else{..} => write!(f, "Else"),
            Self::ElseIf{..} => write!(f, "ElseIf"),
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

    pub fn get_consume_statments(self) -> Vec<StatmentType> {
        self.statments
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













