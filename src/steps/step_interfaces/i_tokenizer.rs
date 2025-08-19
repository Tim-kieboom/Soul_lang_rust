use std::{ops::Index, slice::Iter};
use crate::errors::soul_error::SoulSpan;

pub struct TokenizeResonse {
    pub stream: TokenStream,
}

pub struct Token {
    pub text: String,
    pub span: SoulSpan,
}

pub struct TokenStream {
    #[cfg(feature="dev_mode")]
    current: String, //this is to see what the current is while debugging
    #[cfg(feature="dev_mode")]
    current_line: String, //this is to see what the current is while debugging
    #[cfg(feature="dev_mode")]
    current_line_number: usize, //this is to see what the current is while debugging

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
        Self { 
            index: 0, 
            tokens, 
        }
    }

    #[cfg(feature="dev_mode")]
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut this = Self { 
            current: "<tokenstream not yet started>".to_string(),
            current_line: String::new(),
            current_line_number: 0,
            index: 0, 
            tokens, 
        };

        this.current_line = this.get_current_line(0);
        this
    }

    pub fn reset(&mut self) {
        self.go_to_index(0);
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

    pub fn current(&self) -> &Token {
        &self.tokens[self.index.max(0) as usize]
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

    pub fn peek_starts_with(&self, amount: i64, strs: &[&str]) -> bool {
        if self.index < 0 || self.peek_multiple(amount).is_none() {
            return false
        }

        let start = (self.index + amount) as usize;
        let end = start + strs.len();

        if end > self.tokens.len() {
            return false
        }

        self.tokens[start..end]
            .iter()
            .map(|token| token.text.as_str())
            .eq(strs.iter().copied())
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

    pub fn is_valid_index(&self, index: usize) -> bool {
        index < self.tokens.len()
    }

    pub fn go_to_index(&mut self, index: usize) -> Option<&Token> {
        if index >= self.tokens.len() {
            None
        } else {
            #[cfg(feature="dev_mode")]
            self.change_current(index);
            self.index = index as i64;
            Some(&self.tokens[self.index as usize])
        }
    }

    pub fn next_multiple(&mut self, steps: i64) -> Option<&Token> {
        let next_index = self.index as i64 + steps;
        if next_index < 0 {
            self.index = next_index;
            #[cfg(feature="dev_mode")]
            {self.current = format!("<token stream index negative: {}>", self.index);}
            None
        }
        else if next_index as usize >= self.tokens.len(){
            None
        } 
        else {
            #[cfg(feature="dev_mode")]
            self.change_current(next_index as usize);
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
    fn change_current(&mut self, index: usize) {
        self.current = self.tokens[index].text.clone();
        if self.tokens[self.index as usize].span.line_number != self.tokens[index].span.line_number {
            self.current_line = self.get_current_line(index);
            self.current_line_number = self.tokens[index].span.line_number;
        }
    }

    #[cfg(feature="dev_mode")]
    fn get_current_line(&self, index: usize) -> String {
        use itertools::Itertools;
        
        let line = self.tokens[index].span.line_number;
        self.tokens.iter().flat_map(|token| if token.span.line_number == line {Some(&token.text)} else {None}).join(" ")
    }
}

impl Index<usize> for TokenStream {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}



















