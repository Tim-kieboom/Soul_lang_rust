use crate::soul_names::{check_name, NamesOtherKeyWords, SOUL_NAMES};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::parser::expression::parse_expression::get_expression;
use crate::steps::parser::statment::parse_function::get_function;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::Modifier;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{ExpressionKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::{Block, StatementKind, StatmentType, STATMENT_END_TOKENS};
use crate::steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::BlockBuilder, statement::Statement}, scope_builder::ScopeBuilder}, i_tokenizer::TokenStream};

pub fn get_statment(block_builder: &mut BlockBuilder, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Option<Statement>> {
    
    if stream.next_if("\n").is_none() {
        return Ok(None)
    }

    if stream.current_text() == "{" {
        
        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::InvalidInContext, stream.current_span(), "can not have a scope in global (consider making scope a function, struct or class)"))
        }

        let block = get_scope(stream, scopes)?;
        return Ok(Some(Statement::new_expression(ExpressionKind::Block(block.node), block.span)));
    }
    else if stream.current_text() == "}" {
        if scopes.is_in_global() {
            return Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, stream.current_span(), "there is a '}' without a '{'"))
        }

        stream.next();
        return Ok(Some(Statement::new(StatementKind::CloseBlock, stream.current_span())));
    }


    let statment = match get_statment_type(stream)? {
        StatmentType::Expression => {
            let expression = todo!("get Expression");
            Statement::from_expression(expression)
        }

        StatmentType::Variable => {
            todo!("get Variable")
        },
        StatmentType::Assignment => {
            todo!("get Assignment")
        },

        StatmentType::UsePath => {
            todo!("get UsePath");
            return Ok(None);
        },
        StatmentType::UseType => {
            todo!("get UseType");
            return Ok(None);
        },
        StatmentType::UseImplement => {
            todo!("get UseImplement")
        },
        StatmentType::Function => {
            let function = get_function(stream, scopes)?;
            Statement::new(StatementKind::Function(function.node), function.span)
        },
        StatmentType::FunctionCall => {
            Statement::from_expression(get_expression(stream, scopes, STATMENT_END_TOKENS)?)
        }

        StatmentType::Class => {
            todo!("get Class")
        },
        StatmentType::Trait => {
            todo!("get Trait")
        },
        StatmentType::Struct => {
            todo!("get Struct")
        },

        StatmentType::Enum => {
            todo!("get Enum")
        },
        StatmentType::Union => {
            todo!("get Union")
        },
        StatmentType::TypeEnum => {
            todo!("get TypeEnum");
            return Ok(None);
        },

        StatmentType::If => {
            let expression = todo!("get If");
            Statement::from_expression(expression)
        },
        StatmentType::Else => {
            todo!("get Else");
            return Ok(None);
        },
        StatmentType::For => {
            let expression = todo!("get For");
            Statement::from_expression(expression)
        },
        StatmentType::While => {
            let expression = todo!("get While");
            Statement::from_expression(expression)
        },
        StatmentType::Match => {
            let expression = todo!("get Match");
            Statement::from_expression(expression)
        },

        StatmentType::Type => {
            todo!("get Type");
            return Ok(None);
        },
        StatmentType::ReturnLike => {
            let expression = todo!("get ReturnLike");
            Statement::from_expression(expression)
        },
        StatmentType::CloseBlock => Statement::new(StatementKind::CloseBlock, stream.current_span()),
    };

    Ok(Some(statment))
}

fn get_statment_type(stream: &mut TokenStream) -> Result<StatmentType> {
    let begin_i = stream.current_index();
    let result = inner_get_statment_type(stream);
    stream.go_to_index(begin_i);
    result
}

fn inner_get_statment_type(stream: &mut TokenStream) -> Result<StatmentType> {
    
    let mut modifier = Modifier::Default;
    match stream.current_text() {
        val if Modifier::from_str(val) != Modifier::Default => {
            modifier = Modifier::from_str(stream.current_text());
        }

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Return) => {
            return Ok(StatmentType::ReturnLike)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::BreakLoop) => {
            return Ok(StatmentType::ReturnLike)
        }
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Fall) => {
            return Ok(StatmentType::ReturnLike)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::If) => {
            return Ok(StatmentType::If)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Else) => {
            return Ok(StatmentType::Else)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::MatchCase) => {
            return Ok(StatmentType::Match)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::WhileLoop) => {
            return Ok(StatmentType::While)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::ForLoop) => {
            return Ok(StatmentType::For)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Type) => {
            return Ok(StatmentType::Type)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Trait) => {
            return Ok(StatmentType::Trait)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::TypeEnum) => {
            return Ok(StatmentType::TypeEnum)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Union) => {
            return Ok(StatmentType::Union)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Enum) => {
            return Ok(StatmentType::Enum)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Struct) => {
            return Ok(StatmentType::Struct)
        },
        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Class) => {
            return Ok(StatmentType::Class)
        },

        val if val == SOUL_NAMES.get_name(NamesOtherKeyWords::Use) => {
            return todo!("get_use_type")
        },
        _ => (),
    }
    

    let mut has_round_bracket = false;
    let mut consecutive_parts = 0;
    loop {

        if stream.next().is_none() {
            return Err(err_out_of_bounds(stream));
        }

        if let Ok(_) = check_name(stream.current_text()) {
            consecutive_parts += 1;
        }

        match stream.current_text().as_str() {
            "(" => {
                has_round_bracket = true;
                traverse_bracket_stack(stream, "(", ")")?;
            },
            "<" => {
                traverse_bracket_stack(stream, "<", ">")?;
            },
            "[" => {
                traverse_bracket_stack(stream, "[", "]")?;
            },
            "\n" => {
                if stream.peek_is(".") {
                    () //field or methode on next line
                }
                else if has_round_bracket {
                    if stream.peek_is("{") || stream.peek_is("where") {
                        return Ok(StatmentType::Function)
                    }
                    else {
                        return Ok(StatmentType::FunctionCall)
                    }
                }
                else {
                    return Ok(StatmentType::Expression)
                }
            },
            "=" => {
                if !stream.next_till("\n") {
                    return Err(err_out_of_bounds(stream))
                }
                
                if modifier != Modifier::Default {
                    return Ok(StatmentType::Variable)
                }

                if consecutive_parts > 1 {
                    return Ok(StatmentType::Variable)
                }
                else {
                    return Ok(StatmentType::Assignment)
                }
            }
            ":=" => {
                if !stream.next_till("\n") {
                    return Err(err_out_of_bounds(stream))
                }
                return Ok(StatmentType::Variable)
            },
            "{" => {
                return Ok(StatmentType::Function)
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
                return Err(new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "')' with out '('"))
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
    
fn get_scope<'a>(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Block>> {
    let mut block_builder = BlockBuilder::new(stream.current_span());

    loop {
        let statment = match get_statment(&mut block_builder, stream, scopes)? {
            Some(val) => val,
            None => return Ok(block_builder.into_block()),
        };

        if let StatementKind::CloseBlock = statment.node {
            block_builder.push(statment);
            return Ok(block_builder.into_block());
        }

        block_builder.push(statment);
    }
}

fn err_out_of_bounds(stream: &TokenStream) -> SoulError {
    new_soul_error(SoulErrorKind::UnexpectedEnd, stream.current_span(), "unexpected end while trying to get statments")
}





