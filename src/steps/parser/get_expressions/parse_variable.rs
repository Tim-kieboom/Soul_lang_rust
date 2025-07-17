use crate::errors::soul_error::{new_soul_error, Result, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::Variable;
use crate::steps::step_interfaces::i_tokenizer::TokenStream;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statment::VariableRef;

pub fn get_variable(
    stream: &mut TokenStream, 
    variable: &VariableRef
) -> Result<Variable> {

    if variable.borrow().initializer.is_none() {
        return Err(new_soul_error(
            SoulErrorKind::InvalidInContext, 
            stream.current_span(), 
            format!("'{}' can not be used before it is assigned", variable.borrow().name.0)
        ));
    }

    return Ok(Variable{name: variable.borrow().name.clone()});
}





























