use std::{ops::Index, slice::Iter};
use crate::{errors::soul_error::SoulSpan};

#[derive(Debug, Clone)]
pub struct TokenizeResonse {
    pub stream: TokenStream,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub text: String,
    pub span: SoulSpan,
}

#[derive(Debug, Clone)]
pub struct TokenStream {
    #[cfg(feature="dev_mode")]
    current: String,
    #[cfg(feature="dev_mode")]
    current_line_string: String,
    #[cfg(feature="dev_mode")]
    current_line: usize,

    tokens: Vec<Token>,
    index: i64,
}


impl Token {
    pub fn new(text: String, span: SoulSpan) -> Self {
        Self{span, text}
    }
}

impl TokenStream {
    #[cfg(not(feature="dev_mode"))]
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { 
            index: 0, 
            tokens, 
        }
    }

    #[cfg(feature="dev_mode")]
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut this = TokenStream { 
            current: "<tokenstream uninit>".into(),
            current_line_string: String::new(),
            current_line: 0,
            index: 0, 
            tokens, 
        };

        this.update_line_string(0);
        this
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn ref_tokens(&self) -> &Vec<Token> {
        &self.tokens
    } 

    pub fn iter(&'_ self) -> Iter<'_, Token> {
        self.tokens.iter()
    }
    
    pub fn current_starts_with(&self, strs: &[&str]) -> bool {
        if self.index < 0 {
            return false
        }

        let start = self.index as usize;
        let end = start + strs.len();

        if end > self.tokens.len() {
            return false
        }

        self.tokens[start..end]
            .iter()
            .map(|token| token.text.as_str())
            .eq(strs.iter().copied())
    }

    pub fn current(&self) -> &Token {
        &self.tokens[self.index.max(0) as usize]
    }

    pub fn current_text(&self) -> &String {
        &self.tokens[self.index.max(0) as usize].text
    }

    pub fn current_span(&self) -> SoulSpan {
        self.tokens[self.index.max(0) as usize].span
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

    pub fn peek_is(&self, text: &str) -> bool {
        self.peek().is_some_and(|token| token.text == text)
    }

    ///keeps calling '.next()' till 'token.text == text' or reached end
    pub fn next_till(&mut self, text: &str) -> bool {
        loop {
            if self.current_text() == text {
                return true
            }

            if self.next().is_none() {
                return false
            }
        }
    }

    pub fn is_valid_index(&self, index: usize) -> bool {
        index < self.tokens.len()
    }

    pub fn go_to_index(&mut self, index: usize) -> Option<&Token> {
        if index >= self.tokens.len() {
            None
        } else {
            #[cfg(feature="dev_mode")]
            self.change_token(index);
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
            #[cfg(feature="dev_mode")]
            self.change_token(next_index as usize);
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

    #[cfg(feature="dev_mode")]
    fn change_token(&mut self, index: usize) {
        self.current = self.tokens[index].text.clone();
        let old_line = self.tokens[self.index as usize].span.line_number; 
        let new_line = self.tokens[index].span.line_number;

        if new_line != old_line {
            self.update_line_string(new_line);
            self.current_line = new_line;
        }

    }

    #[cfg(feature="dev_mode")]
    fn update_line_string(&mut self, line_number: usize) {
        use itertools::Itertools;

        self.current_line_string = self.tokens
            .iter()
            .flat_map(|token| if token.span.line_number == line_number {Some(&token.text)} else {None})
            .join(" ")
    }
}

impl Index<usize> for TokenStream {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}


















