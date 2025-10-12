//! # Soul Language Tokenizer
//!
//! This module defines the `Token`, `TokenStream`, and `TokenizeResponse` types,
//! which form the **tokenization step** of the Soul language compiler pipeline.
//!
//! Each compilation step in the Soul compiler implements a structural interface
//! to provide consistent data passing between phases (see `step_interfaces`).
//! 

use std::{ops::Index, slice::Iter};
use crate::{errors::soul_error::SoulSpan};

/// Represents the output of the tokenization phase.
///
/// Contains a [`TokenStream`] which holds all tokens
/// produced by the tokenizer for a given source input.
#[derive(Debug, Clone)]
pub struct TokenizeResonse {
    pub stream: TokenStream,
}

/// A single lexical token in the Soul language.
///
/// Each token holds its textual value and source span information.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    /// The raw text representation of the token.
    pub text: String,
    /// Source location data for this token.
    pub span: SoulSpan,
}

/// A sequence of [`Token`]s with utilities for traversal,
/// lookahead, and state management.
///
/// The `TokenStream` is used by later compilation steps
/// (parser, analyzer, etc.) to process tokens sequentially.
///
/// In `dev_mode`, it also tracks the current line and text
/// for debugging and development visualization.
#[derive(Debug, Clone)]
pub struct TokenStream {
    #[cfg(feature="dev_mode")]
    current: String,
    #[cfg(feature="dev_mode")]
    current_line_string: String,
    #[cfg(feature="dev_mode")]
    current_line: i64,

    tokens: Vec<Token>,
    index: i64,
}


impl Token {

    /// Creates a new [`Token`] from text and span data.
    ///
    /// # Arguments
    /// * `text` - The textual content of the token.
    /// * `span` - The span indicating where this token appears in the source.
    ///
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::Token;
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let current_line_number = 0;
    /// let current_line_offset = 0;
    /// let raw_token = "if";
    /// let span = SoulSpan::new(current_line_number, current_line_offset, raw_token.len());
    /// 
    /// let token = Token::new(raw_token.to_string(), span);
    /// ```
    pub fn new(text: String, span: SoulSpan) -> Self {
        Self{span, text}
    }
}

impl TokenStream {
    /// Creates a new [`TokenStream`] from a list of tokens (feature = not("dev_mode")).
    ///
    /// # Arguments
    /// * `tokens` - The vector of tokens to wrap.
    #[cfg(not(feature="dev_mode"))]
    pub fn new(tokens: Vec<Token>) -> Self {
        TokenStream { 
            index: 0, 
            tokens, 
        }
    }

    /// Creates a new [`TokenStream`] with debugging support (feature = "dev_mode").
    ///
    /// Initializes the stream to track the current token, line, and line string.
    ///
    /// # Arguments
    /// * `tokens` - The vector of tokens to wrap.
    #[cfg(feature="dev_mode")]
    pub fn new(tokens: Vec<Token>) -> Self {
        let mut this = TokenStream { 
            current: "<tokenstream uninit>".into(),
            current_line_string: String::new(),
            current_line: 0,
            index: 0, 
            tokens, 
        };

        this.index = -1;
        this.current_line = -1;
        
        this.change_token(0);
        
        this.index = 0;
        this.current_line = 0;

        this
    }

    /// Returns the number of tokens in the stream.
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Returns an immutable reference to the underlying token vector.
    pub fn ref_tokens(&self) -> &Vec<Token> {
        &self.tokens
    } 

    /// Returns an iterator over all tokens in the stream.
    pub fn iter(&'_ self) -> Iter<'_, Token> {
        self.tokens.iter()
    }
    
    /// Checks if the sequence of tokens starting at the current index
    /// matches the given array of string slices.
    ///
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::default());
    /// let second = Token::new("if".to_string(), SoulSpan::default());
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);;
    /// 
    /// assert!(stream.current_starts_with(&["else", "if"]))
    /// ```
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

    /// Returns a reference to the current token in the stream.
    ///
    /// If the index is invalid (< 0), returns the first token.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::default());
    /// let second = Token::new("if".to_string(), SoulSpan::default());
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current(), &first)
    /// ```
    pub fn current(&self) -> &Token {
        &self.tokens[self.index.max(0) as usize]
    }

    /// Returns the text of the current token.
    ///     
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::default());
    /// let second = Token::new("if".to_string(), SoulSpan::default());
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_text(), "else")
    /// ```
    pub fn current_text(&self) -> &String {
        &self.current().text
    } 

    /// Returns `true` if the current token’s text matches the given string.
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::default());
    /// let second = Token::new("if".to_string(), SoulSpan::default());
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert!(stream.current_is("else"))
    /// ```
    pub fn current_is(&self, text: &str) -> bool {
        self.current().text == text
    }   

    /// Returns the [`SoulSpan`] of the current token.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_span(), first.span)
    /// ```
    pub fn current_span(&self) -> SoulSpan {
        self.current().span
    }

    /// Returns the current token’s span wrapped in `Some()`
    /// (useful for APIs expecting `Option<SoulSpan>`).
    ///     
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_span_some(), Some(first.span))
    /// ```
    pub fn current_span_some(&self) -> Option<SoulSpan> {
        Some(self.current().span)
    }

    /// Returns the current token index, clamped to 0 if negative.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_index(), 0)
    /// ```
    pub fn current_index(&self) -> usize {
        self.index.max(0) as usize
    }

    /// Advances the token stream by one and returns the new current token.
    ///
    /// Returns `None` if there are no more tokens.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current(), &first);
    /// assert_eq!(stream.next(), Some(&second));
    /// assert_eq!(stream.current(), &second);
    /// assert_eq!(stream.next(), None);
    /// ```
    pub fn next(&mut self) -> Option<&Token> {
        self.next_multiple(1)
    }

    /// Advances if the current token matches the given text, otherwise stays put.
    ///
    /// Returns the current token (after potential advancement).
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current(), &first);
    /// assert_eq!(stream.next_if("not current text"), Some(&first)); // returned current instead of next
    /// assert_eq!(stream.current(), &first);
    /// assert_eq!(stream.next_if("else"), Some(&second)); // returned next
    /// assert_eq!(stream.current(), &second);
    /// ```
    pub fn next_if(&mut self, text: &str) -> Option<&Token> {
        if self.current_text() == text {
            self.next_multiple(1)
        }
        else {
            Some(self.current())
        }
    }

    /// Returns the next token without advancing.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current(), &first);
    /// assert_eq!(stream.peek(), Some(&second));
    /// assert_eq!(stream.current(), &first);
    /// ```
    pub fn peek(&self) -> Option<&Token> {
        self.peek_multiple(1)
    }

    /// Returns `true` if the next token matches the given text.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert!(stream.peek_is("if"));
    /// ```
    pub fn peek_is(&self, text: &str) -> bool {
        self.peek().is_some_and(|token| token.text == text)
    }

    /// Returns `true` if the token `steps` ahead matches the given text.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert!(stream.peek_multiple_is(1, "if"));
    /// assert!(stream.peek_multiple_is(2, "true"));
    /// ```
    pub fn peek_multiple_is(&self, steps: i64, text: &str) -> bool {
        self.peek_multiple(steps).is_some_and(|token| token.text == text)
    }

    ///keeps calling '.next()' till 'token.text == text' or reached end
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_index(), 0);
    /// assert!(stream.next_till("true"));
    /// assert_eq!(stream.current_index(), 2);
    /// ```
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

    /// Advances until a token with the given text is found, or end of stream.
    ///
    /// Returns `true` if found, `false` if the stream ended first.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert!(stream.is_valid_index(0));
    /// assert!(stream.is_valid_index(1));
    /// assert!(stream.is_valid_index(2));
    /// assert!(!stream.is_valid_index(3)); //out of bounds
    /// ```
    pub fn is_valid_index(&self, index: usize) -> bool {
        index < self.tokens.len()
    }

    
    /// Moves the current position to the given index and returns that token.
    ///
    /// Returns `None` if the index is out of range.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_index(), 0);
    /// assert_eq!(stream.go_to_index(2), Some(&third));
    /// assert_eq!(stream.current_index(), 2);
    /// assert_eq!(stream.go_to_index(3), None); //out of bounds
    /// 
    /// 
    /// ```
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

    /// Advances the stream by the given number of steps and returns the new token.
    ///
    /// Returns `None` if stepping would exceed stream bounds.
    /// 
    /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_index(), 0);
    /// assert_eq!(stream.next_multiple(1000), None); // out of bounds stayed as current
    /// assert_eq!(stream.current_index(), 0);
    /// assert_eq!(stream.next_multiple(2), Some(&third));
    /// assert_eq!(stream.current_index(), 2);
    /// 
    /// ```
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

    /// Peeks ahead by a number of steps without advancing the index.
    /// 
        /// # Example
    /// ```
    /// use soul_lang_rust::steps::step_interfaces::i_tokenizer::{Token, TokenStream};
    /// use soul_lang_rust::errors::soul_error::SoulSpan;
    /// 
    /// let first = Token::new("else".to_string(), SoulSpan::new(0,0,4));
    /// let second = Token::new("if".to_string(), SoulSpan::new(0,4,2));
    /// let third = Token::new("true".to_string(), SoulSpan::new(0,6,4));
    /// let mut stream = TokenStream::new(vec![
    ///     first.clone(),
    ///     second.clone(),
    ///     third.clone(),
    /// ]);
    /// 
    /// assert_eq!(stream.current_index(), 0);
    /// assert_eq!(stream.peek_multiple(1000), None);
    /// assert_eq!(stream.current_index(), 0);
    /// assert_eq!(stream.peek_multiple(2), Some(&third));
    /// assert_eq!(stream.current_index(), 0);
    /// 
    /// ```
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
        let old_line = if self.index < 0 {-1} else {self.tokens[self.index as usize].span.line_number as i64}; 
        let new_line = self.tokens[index].span.line_number as i64;

        if new_line != old_line {
            self.update_line_string(new_line as i64);
            self.current_line = new_line as i64;
        }

    }

    #[cfg(feature="dev_mode")]
    fn update_line_string(&mut self, line_number: i64) {
        use itertools::Itertools;

        self.current_line_string = self.tokens
            .iter()
            .flat_map(|token| if token.span.line_number as i64 == line_number {Some(&token.text)} else {None})
            .join(" ")
    }
}

impl Index<usize> for TokenStream {
    type Output = Token;

    fn index(&self, index: usize) -> &Self::Output {
        &self.tokens[index]
    }
}


















