use crate::{errors::soul_error::SoulSpan, steps::step_interfaces::i_tokenizer::token_stream::TokenStream};

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


















