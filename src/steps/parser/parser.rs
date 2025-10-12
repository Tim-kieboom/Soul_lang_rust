use crate::errors::soul_error::{Result, SoulSpan};
use crate::steps::parser::statment::parse_statment::get_statment;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeBuilder;
use crate::steps::step_interfaces::{i_parser::parser_response::ParserResponse, i_tokenizer::tokenizer::TokenizeResonse};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::abstract_syntax_tree::{AbstractSyntacTree, BlockBuilder};

/// Builds an abstract syntax tree (AST) from a token stream and constructs scope information.
///
/// This function:
/// 1. Consumes the [`TokenStream`] from [`TokenizeResonse`].
/// 2. Initializes a [`ScopeBuilder`] with the project name to track identifiers, types,
///    and other scoped entities during parsing.
/// 3. Iteratively extracts statements from the token stream using [`get_statment`].
/// 4. Adds each parsed statement into a global [`BlockBuilder`].
/// 5. Stops when all tokens have been consumed.
/// 6. Produces an [`AbstractSyntacTree`] as the root node of the parsed program.
/// 7. Wraps the AST and collected scope data into a [`ParserResponse`].
///
/// # Parameters
/// - `tokens`: A response from the tokenizer, containing the full token stream for a file.
/// - `project_name`: The name of the current project, used to seed the root scope.
///
/// # Returns
/// - `Ok(ParserResponse)` containing:
///   - The constructed abstract syntax tree (`tree`)
///   - The scope builder (`scopes`) with all symbol information
/// - `Err(SoulError)` if parsing fails at any point (e.g., unexpected tokens).
///
/// # Notes
/// - Uses [`SoulSpan`] for attaching source positions to AST nodes for error reporting.
/// - The root block represents the global scope of the program.
/// - Parsing continues until the token stream is exhausted, even if intermediate
///   errors are reported via the scope system.
///
/// # Example
/// ```ignore
/// use crate::steps::{tokenizer::tokenize, parser::parse_ast, source_reader::read_source_file};
/// use std::fs::File;
/// use std::io::BufReader;
///
/// let file = File::open("example.soul").unwrap();
/// let reader = BufReader::new(file);
/// let source = read_source_file(reader, "    ").unwrap();
/// let tokens = tokenize(source).unwrap();
/// let parser_response = parse_ast(tokens, "my_project".to_string()).unwrap();
/// println!("AST root has {} children", parser_response.tree.root().children().len());
/// ```
pub fn parse_ast(tokens: TokenizeResonse, project_name: String) -> Result<ParserResponse> {
    let mut stream = tokens.stream;
    let mut scopes = ScopeBuilder::new(project_name);

    let mut block_builder = BlockBuilder::new(scopes.current_id(), SoulSpan::new(0,0,0));
    loop {

        if let Some(statment) = get_statment(&mut stream, &mut scopes)? {
            block_builder.push_global(statment)?;
        }

        if stream.peek().is_none() {
            break;
        }
    }
    
    let tree = AbstractSyntacTree::new(block_builder.into_block().node);
    Ok(ParserResponse{tree, scopes})
}






