//! # Soul Language Tokenizer
//!
//! This module defines the `Token`, `TokenStream`, and `TokenizeResponse` types,
//! which form the **tokenization step** of the Soul language compiler pipeline.
//!
//! Each compilation step in the Soul compiler implements a structural interface
//! to provide consistent data passing between phases (see `step_interfaces`).
//! 

pub mod tokenizer;
pub mod token_stream;

pub use crate::steps::step_interfaces::i_tokenizer::tokenizer::Token;
pub use crate::steps::step_interfaces::i_tokenizer::token_stream::TokenStream;