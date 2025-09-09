use crate::errors::soul_error::Result;
use crate::soul_names::OPERATOR_ASSIGN_SYMBOOLS;
use crate::{errors::soul_error::{new_soul_error, SoulError, SoulErrorKind}, soul_names::{check_name_allow_types, NamesOtherKeyWords, SOUL_NAMES}, steps::step_interfaces::{i_parser::abstract_syntax_tree::soul_type::soul_type::Modifier, i_tokenizer::TokenStream}};


pub fn get_statement_type(stream: &mut TokenStream) -> Result<StatementType> {
    let begin_i = stream.current_index();
    let result = inner_get_statment_type(stream);
    stream.go_to_index(begin_i);
    result
}

fn inner_get_statment_type(stream: &mut TokenStream) -> Result<StatementType> {
    
    let mut modifier = Modifier::Default;
    match stream.current_text() {
        val if Modifier::from_str(val) != Modifier::Default => {
            modifier = Modifier::from_str(stream.current_text());
        }

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Return) => {
            return Ok(StatementType::Return)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::BreakLoop) => {
            return Ok(StatementType::Break)
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Fall) => {
            return Ok(StatementType::Fall)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
            return Ok(StatementType::If)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {
            return Ok(StatementType::Else)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::MatchCase) => {
            return Ok(StatementType::Match)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop) => {
            return Ok(StatementType::While)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop) => {
            return Ok(StatementType::For)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            return Ok(StatementType::Type)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            return Ok(StatementType::Trait)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            return Ok(StatementType::TypeEnum)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            return Ok(StatementType::Union)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            return Ok(StatementType::Enum)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            return Ok(StatementType::Struct)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            return Ok(StatementType::Class)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Use) => {
            return Ok(StatementType::Use)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Let) => {
            return Ok(StatementType::Variable)
        }
        _ => (),
    }
    
    if modifier == Modifier::Default {
        stream.next_multiple(-1);
    }

    let mut has_round_bracket = false;
    let mut consecutive_parts = 0;
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if let Ok(_) = check_name_allow_types(stream.current_text()) {
            consecutive_parts += 1;
        }

        match stream.current_text().as_str() {
            "(" => {
                has_round_bracket = true;
                traverse_bracket_stack(stream, "(", ")")?;
                if consecutive_parts == 0 {
                    consecutive_parts = 1
                }
            },
            "<" => {
                traverse_bracket_stack(stream, "<", ">")?;
                if consecutive_parts == 0 {
                    consecutive_parts = 1
                }
            },
            "[" => {
                traverse_bracket_stack(stream, "[", "]")?;
                if consecutive_parts == 0 {
                    consecutive_parts = 1
                }
            },
            "\n" => {
                if stream.peek_is(".") {
                    () //field or methode on next line
                }
                else if has_round_bracket {
                    if stream.peek_is("{") || stream.peek_is("where") {
                        return Ok(StatementType::Function)
                    }
                    else {
                        return Ok(StatementType::FunctionCall)
                    }
                }
                else {
                    return Ok(StatementType::Expression)
                }
            },
            ":=" => {
                if !stream.next_till("\n") {
                    return Err(err_out_of_bounds(stream))
                }
                return Ok(StatementType::Variable)
            },
            "{" => {
                if has_round_bracket {
                    return Ok(StatementType::Function)
                }

                return Ok(StatementType::StructContructor)
            },
            "=" => {
                if !stream.next_till("\n") {
                    return Err(err_out_of_bounds(stream))
                }
                
                if modifier != Modifier::Default {
                    return Ok(StatementType::Variable)
                }

                if consecutive_parts > 1 {
                    return Ok(StatementType::Variable)
                }
                else {
                    return Ok(StatementType::Assignment)
                }
            },
            "." => {
                consecutive_parts = 0;
            },
            val if OPERATOR_ASSIGN_SYMBOOLS.iter().any(|symbool| *symbool == val) => {
                if !stream.next_till("\n") {
                    return Err(err_out_of_bounds(stream))
                }
                return Ok(StatementType::Assignment)
            },
            _ => (),
        }
    }
}

fn traverse_bracket_stack(stream: &mut TokenStream, open: &str, close: &str) -> Result<()> {
    let mut stack = 0;
    loop {

        if stream.current_text() == open {
            stack += 1
        } 
        else if stream.current_text() == close {
            
            if stack == 0 {
                return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "')' with out '('"))
            }

            stack -= 1
        }

        if stack == 0 {
            break Ok(())
        }
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream))
        }
    }
}

pub enum StatementType {
    Expression,

    Variable,
    Assignment,

    Use,

    Function,
    FunctionCall,
    StructContructor,

    Class,
    Trait,
    Struct,

    Enum,
    Union,
    TypeEnum,

    If,
    For,
    Else,
    While,
    Match,

    Type,

    Return,
    Break,
    Fall,
}


fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span_some(), "unexpected end while trying to get statmentType")
}





































