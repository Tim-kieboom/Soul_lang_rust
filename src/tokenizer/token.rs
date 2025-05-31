use core::fmt;
use std::ops::Index;


#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub line_number: usize,
    pub line_offset: usize,
}

pub struct TokenIterator {
    index: i64,
    tokens: Vec<Token>,
}

#[allow(dead_code)]
impl TokenIterator {

    pub fn new(tokens: Vec<Token>) -> Self {
        debug_assert!(tokens.len() > 0);
        TokenIterator { 
            index: 0, 
            tokens, 
        }
    }

    pub fn go_to_before_start(&mut self) {
        self.index = -1;
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn consume_tokens(self) -> Vec<Token> {
        self.tokens
    } 

    pub fn get_tokens_text(&self) -> Vec<&str> {
        self.tokens.iter()
                   .map(|token| token.text.as_str())
                   .collect::<Vec<&str>>()
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index.max(0) as usize]
    }

    pub fn current_index(&self) -> usize {
        self.index.max(0) as usize
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
            self.index = index as i64;
            Some(&self.tokens[self.index as usize])
        }
    }

    pub fn next_multiple(&mut self, steps: i64) -> Option<&Token> {
        let next_index = self.index as i64 + steps;
        if next_index < 0 {
            self.index = next_index;
            None
        }
        else if next_index as usize >= self.tokens.len(){
            None
        } 
        else {
            self.index = next_index;
            Some(&self.tokens[self.index as usize])
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









