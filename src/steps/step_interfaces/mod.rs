//! # Soul Language – Step Interfaces
//!
//! This module defines the **interface layer** between all major compilation stages
//! of the Soul language compiler.  
//! Each step has a dedicated submodule that defines the **data structures**
//! and **interfaces** used to pass information between stages.
//!
//! These steps form the full compilation pipeline, transforming raw source code
//! into C++ code output.
//! 
//! ---
//!
//! ## 🔄 Compilation Step Order
//!
//! 1. ### [`i_source_reader`]
//!    - **Purpose:** Reads raw source files line by line.
//!    - **Output:** [`SourceFileResponse`](crate::step_interfaces::i_source_reader::SourceFileResponse)
//!    - **Responsibilities:**
//!      - Loads and stores file lines with their line numbers.
//!      - Maintains mappings for embedded C-style strings.
//!      - Tracks source gaps and estimates token count for optimization.
//!
//! 2. ### [`i_tokenizer`]
//!    - **Purpose:** Converts raw text lines into lexical tokens.
//!    - **Output:** [`TokenizeResponse`](crate::step_interfaces::i_tokenizer::TokenizeResonse)
//!    - **Responsibilities:**
//!      - Breaks source into tokens with [`SoulSpan`](crate::errors::soul_error::SoulSpan) metadata.
//!      - Maintains iteration utilities for the parser.
//!      - Optionally provides line-tracking and debug visualization in `dev_mode`.
//!
//! 3. ### [`i_parser`]
//!    - **Purpose:** Converts token streams into an abstract syntax tree (AST).
//!    - **Output:** AST or intermediate syntax representation (type TBD).
//!    - **Responsibilities:**
//!      - Consumes tokens from [`TokenStream`](crate::step_interfaces::i_tokenizer::TokenStream).
//!      - Detects and reports syntax errors.
//!      - Produces structured AST nodes for later analysis.
//!
//! 4. ### [`i_sementic`] *(Semantic Analyzer)*
//!    - **Purpose:** Performs semantic checks and scope validation on the AST.
//!    - **Output:** A semantically validated AST or an annotated intermediate form.
//!    - **Responsibilities:**
//!      - Checks type consistency, symbol resolution, and scope rules.
//!      - Reports semantic and contextual errors.
//!      - Prepares the AST for the final code generation phase.
//!
//! 5. ### `i_code_generator` *(Not yet implemented)*
//!    - **Purpose:** Transforms the final analyzed AST into C++ source code.
//!    - **Planned Output:** C++ code that can be compiled into a binary using a standard C++ compiler.
//!    - **Future Responsibilities:**
//!      - Generate equivalent C++ structures, functions, and logic from Soul AST nodes.
//!      - Apply optimizations based on semantic information.
//!      - Handle imports, module generation, and symbol table translation.
//!
//! ---
//!
//! ## 🧩 Design Philosophy
//!
//! Each step is **decoupled** but **interoperable** — all data passed between them is defined here.
//! This design allows each step to evolve independently while maintaining a consistent pipeline interface.
//!
//! Developers can plug in custom tools (e.g., alternative tokenizers or analyzers) by adhering to these interfaces.
//!
//! ---
pub mod i_source_reader;
pub mod i_tokenizer;
pub mod i_sementic;
pub mod i_parser;







