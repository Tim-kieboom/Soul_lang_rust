use core::fmt;
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub line_number: usize,
    pub line_offset: usize,
}

pub struct TokenIterator {
    index: usize,
    tokens: Vec<Token>,
}

impl TokenIterator {

    pub fn new(tokens: Vec<Token>) -> Self {
        debug_assert!(tokens.len() > 0);
        TokenIterator { 
            index: 0, 
            tokens, 
        }
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index]
    }

    pub fn current_index(&self) -> usize {
        self.index
    }

    pub fn next(&mut self) -> Option<&Token> {
        self.next_multiple(1)
    } 

    pub fn peek(&self) -> Option<&Token> {
        self.peek_multiple(1)
    }

    pub fn go_to_index(&mut self, index: usize) -> Option<&Token> {
        if index >= self.tokens.len() {
            None
        } else {
            self.index = index;
            Some(&self.tokens[self.index])
        }
    }

    pub fn next_multiple(&mut self, steps: i64) -> Option<&Token> {
        let next_index = self.index as i64 + steps;
        if next_index as usize >= self.tokens.len() || next_index < 0 {
            None
        } else {
            self.index = next_index as usize;
            Some(&self.tokens[self.index])
        }
    }

    pub fn peek_multiple(&self, steps: i64) -> Option<&Token> {
        let peek_index = (self.index as i64 + steps) as usize;
        if peek_index < self.tokens.len() {
            Some(&self.tokens[peek_index])
        } 
        else {
            None
        }
    }

}

impl Index<usize> for TokenIterator {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}

impl fmt::Debug for TokenIterator {
    fn fmt(&self, format: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        format.debug_struct("TokenIterator")
              .field("index", &self.index)
              .field("tokens", &self.tokens)
              .field("current()", &self.current())
              .finish()
    }
}