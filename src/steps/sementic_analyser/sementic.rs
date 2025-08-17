use crate::run_options::run_options::RunOptions;
use crate::steps::step_interfaces::i_sementic::sementic_scope::ScopeVisitor;
use crate::steps::step_interfaces::i_parser::parser_response::ParserResponse;
use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind, SoulSpan};
use crate::steps::step_interfaces::i_sementic::sementic_respone::SementicAnalyserResponse;
use crate::steps::step_interfaces::i_sementic::ast_visitors::{AstVisitable, ExternalHeaderAnalyser, TypeCollector};

pub fn sementic_analyse_ast(parser: ParserResponse, run_options: &RunOptions) -> Result<SementicAnalyserResponse> {

    const SHOULD_RESET: bool = true;
    let ParserResponse{scopes, mut tree} = parser;

    let mut scope = ScopeVisitor::new(scopes, run_options)
        .map_err(|err| new_soul_error(SoulErrorKind::InternalError, SoulSpan::new(0,0,0), format!("Internal Error: while trying to get ScopeVisitor\n{}", err.to_string())))?;
    
    let mut faults = vec![];

    let mut header_analyser = ExternalHeaderAnalyser::new(scope, faults, SHOULD_RESET);
    header_analyser.visit_ast(&mut tree);
    (scope, faults) = header_analyser.consume();

    let mut type_analyser = TypeCollector::new(scope, faults, SHOULD_RESET);
    type_analyser.visit_ast(&mut tree);
    (scope, faults) = type_analyser.consume();

    Ok(SementicAnalyserResponse{tree, scope, faults})
}













