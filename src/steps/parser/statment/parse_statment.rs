use crate::steps::parser::statment::parse_function::get_function;
use crate::steps::parser::statment::parse_generics_decl::{get_generics_decl, GenericDecl};
use crate::steps::parser::statment::parse_variable::get_variable;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::type_kind::TypeKind;
use crate::steps::step_interfaces::i_parser::parser_response::FromTokenStream;
use crate::steps::step_interfaces::i_parser::scope_builder::ScopeKind;
use crate::steps::parser::expression::parse_expression::{get_expression, get_expression_options, ExprOptions};
use crate::soul_names::{check_name_allow_types, NamesOtherKeyWords, SOUL_NAMES};
use crate::errors::soul_error::{new_soul_error, Result, SoulError, SoulErrorKind};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::spanned::Spanned;
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::soul_type::soul_type::{Modifier, SoulType};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::statement::{Assignment, Block, StatementKind, StatementType, STATMENT_END_TOKENS};
use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::expression::{Expression, ExpressionKind, ReturnKind, ReturnLike, VariableName};
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


    let statment = match get_statement_type(stream)? {
        StatementType::Expression => {
            let expression = get_expression(stream, scopes, STATMENT_END_TOKENS)?;
            Statement::from_expression(expression)
        }

        StatementType::Variable => {
            let variable = get_variable(stream, scopes)?;
            let variable_name = VariableName::new(&variable.node.name);
            scopes.insert(variable_name.name.0.clone(), ScopeKind::Variable(variable.node));

            Statement::new(StatementKind::Variable(variable_name), variable.span)
        },
        StatementType::Assignment => {
            let assignment = get_assignment(stream, scopes)?;
            Statement::new(StatementKind::Assignment(assignment.node), assignment.span)
        },

        StatementType::Use => {
            todo!("get Use");
            return Ok(None);
        },
        StatementType::Function => {
            let function = get_function(stream, scopes)?;
            Statement::new(StatementKind::Function(function.node), function.span)
        },
        StatementType::FunctionCall => {
            Statement::from_expression(get_expression(stream, scopes, STATMENT_END_TOKENS)?)
        }

        StatementType::Class => {
            todo!("get Class")
        },
        StatementType::Trait => {
            todo!("get Trait")
        },
        StatementType::Struct => {
            todo!("get Struct")
        },

        StatementType::Enum => {
            todo!("get Enum")
        },
        StatementType::Union => {
            todo!("get Union")
        },
        StatementType::TypeEnum => {
            todo!("get TypeEnum");
            return Ok(None);
        },

        StatementType::If => {
            let expression = todo!("get If");
            Statement::from_expression(expression)
        },
        StatementType::Else => {
            todo!("get Else");
            return Ok(None);
        },
        StatementType::For => {
            let expression = todo!("get For");
            Statement::from_expression(expression)
        },
        StatementType::While => {
            let expression = todo!("get While");
            Statement::from_expression(expression)
        },
        StatementType::Match => {
            let expression = todo!("get Match");
            Statement::from_expression(expression)
        },

        StatementType::Type => {
            parse_type_def(stream, scopes)?;
            return Ok(None);
        },
        StatementType::Return => {
            let expression = get_return_like(ReturnKind::Return, stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::Break => {
            let expression = get_return_like(ReturnKind::Break, stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::Fall => {
            let expression = get_return_like(ReturnKind::Fall, stream, scopes)?;
            Statement::from_expression(expression)
        },
        StatementType::CloseBlock => Statement::new(StatementKind::CloseBlock, stream.current_span()),
    };

    Ok(Some(statment))
}

fn get_assignment(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Spanned<Assignment>> {
    let assign_i = stream.current_index();
    let variable = get_expression(stream, scopes, &["="])?;
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let value = get_expression(stream, scopes, STATMENT_END_TOKENS)?;
    let span = stream[assign_i].span.combine(&stream.current_span());
    Ok(Spanned::new(Assignment{variable, value}, span))
}

fn parse_type_def(stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<()> {
    debug_assert_eq!(stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Type));
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let type_i = stream.current_index();
    let new_type = SoulType::from_stream(stream, scopes)?;
    if !matches!(new_type.base, TypeKind::Unknown(_)) {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("type: '{}' is invalid", stream[type_i].text)))
    }

    let name = new_type.base.to_name_string();

    if stream.current_text() != SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof) {
        return Err(new_soul_error(SoulErrorKind::InvalidType, stream.current_span(), format!("token: '{}', should be '{}'", stream.current_text(), SOUL_NAMES.get_name(NamesOtherKeyWords::Typeof))))
    }
    
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream))
    }

    let of_type = SoulType::from_stream(stream, scopes)?;

    scopes.insert(name, ScopeKind::TypeDef{new_type, of_type});
    Ok(())
}

fn get_return_like(kind: ReturnKind, stream: &mut TokenStream, scopes: &mut ScopeBuilder) -> Result<Expression> {
    let return_i = stream.current_index();
    if stream.next().is_none() {
        return Err(err_out_of_bounds(stream));
    }  

    let expr = get_expression(stream, scopes, STATMENT_END_TOKENS)?;
    let value = if let ExpressionKind::Empty = expr.node {
        None
    }
    else {
        Some(Box::new(expr))
    };

    let span = stream[return_i].span.combine(&stream.current_span());
    Ok(Expression::new(ExpressionKind::ReturnLike(ReturnLike{value, kind, delete_list: vec![]}), span))
}

fn get_statement_type(stream: &mut TokenStream) -> Result<StatementType> {
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
            }
            ":=" => {
                if !stream.next_till("\n") {
                    return Err(err_out_of_bounds(stream))
                }
                return Ok(StatementType::Variable)
            },
            "{" => {
                return Ok(StatementType::Function)
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





