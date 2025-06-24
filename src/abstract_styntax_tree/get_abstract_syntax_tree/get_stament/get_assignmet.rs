use crate::meta_data::soul_error::soul_error::{new_soul_error, pass_soul_error, Result, SoulSpan};
use crate::{abstract_styntax_tree::{abstract_styntax_tree::{IExpression, IStatment, IVariable}, get_abstract_syntax_tree::{get_expression::get_expression::get_expression, multi_stament_result::MultiStamentResult}, operator_type::ExprOperatorType}, meta_data::{current_context::{current_context::CurrentContext, rulesets::RuleSet}, meta_data::MetaData, scope_and_var::var_info::VarFlags, soul_names::{NamesOperator, SOUL_NAMES}, soul_type::{primitive_types::DuckType, soul_type::SoulType}}, tokenizer::token::{Token, TokenIterator}};

pub struct AssignmentResult {
    pub assignment: MultiStamentResult<IStatment>,
    pub is_type: SoulType,
}

pub fn get_assignment(
    iter: &mut TokenIterator,
    meta_data: &mut MetaData,
    context: &mut CurrentContext,
    i_variable: IVariable,
    in_initialize: bool,
) -> Result<AssignmentResult> {
    let mut body_result = MultiStamentResult::new(IStatment::EmptyStatment(SoulSpan::from_token(iter.current())));

    let symbool_index = iter.current_index();
    
    let var_type = SoulType::from_stringed_type(i_variable.get_type_name(), iter.current(), &meta_data.type_meta_data, &mut context.current_generics)
        .map_err(|err| pass_soul_error(iter.current(), format!("error while trying to get type from variable of assignment").as_str(), err))?;

    let is_forward_declared = meta_data.try_get_variable(i_variable.get_name(), &context.get_current_scope_id())
        .ok_or(new_soul_error(iter.current(), format!("variable: '{}' could not be found in scope", i_variable.get_name()).as_str()))?
        .0.is_forward_declared.clone();

    if is_forward_declared && !in_initialize {
        return Err(new_soul_error(iter.current(), format!("variable: '{}' has not been initialized yet", i_variable.get_name()).as_str()));
    }

    is_symbool_allowed(&iter[symbool_index], &context, &var_type)?;

    if iter.next().is_none() {
        return Err(new_soul_error(iter.current(), "unexpected end while getting assignment"));
    }

    let increment_symbool = SOUL_NAMES.get_name(NamesOperator::Increment);
    let decrement_symbool = SOUL_NAMES.get_name(NamesOperator::Decrement);

    if &iter[symbool_index].text == increment_symbool || &iter[symbool_index].text == decrement_symbool {
        meta_data.try_get_variable_mut(i_variable.get_name(), &context.get_current_scope_id())
            .unwrap()
            .add_var_flag(VarFlags::IsAssigned);

        if var_type.to_primitive_type(&meta_data.type_meta_data).to_duck_type() != DuckType::Number {
            return Err(new_soul_error(&iter[symbool_index], format!("'{}' is only allowed for numbers", &iter[symbool_index].text).as_str()));
        }

        if !(iter.current().text == "\n" || iter.current().text == ";") {
            return Err(new_soul_error(iter.current(), format!("variable doesn't end after '{}' (add enter('\n') of ';')", iter.current().text).as_str()));
        }

        let increment_amount; 
        if &iter[symbool_index].text == increment_symbool {
            increment_amount = 1;
        } 
        else {
            increment_amount = -1;
        };

        body_result.value = IStatment::new_assignment(
            i_variable.clone(), 
            IExpression::new_increment(
                i_variable, 
                false, 
                increment_amount,
                iter.current()
            ),
            iter.current()
        )?;
        return Ok(AssignmentResult{is_type: var_type, assignment: body_result});
    }

    let begin_i = iter.current_index();

    let mut expression = get_expression(iter, meta_data, context, &Some(&var_type), is_forward_declared, &vec!["\n", ";"])
        .map_err(|err| pass_soul_error(&iter[begin_i], "while trying to get assignment expression", err))?;

    body_result.add_result(&expression.result);
    meta_data.try_get_variable_mut(i_variable.get_name(), &context.get_current_scope_id())
        .unwrap()
        .add_var_flag(VarFlags::IsAssigned);

    if var_type.is_array() || var_type.is_any_ref() || var_type.is_pointer() {

        if !expression.is_type.is_mutable() && var_type.is_mutable() {
            let error_span = iter.get_tokens_text()[begin_i..iter.current_index()].join(" ");
            return Err(new_soul_error(iter.current(), format!("variable: '{}' and expression: '{}' have diffrent mutabilitys", i_variable.get_name(), error_span).as_str()));
        }
    }

    if !is_forward_declared && !expression.is_type.is_convertable(&var_type, iter.current(), &meta_data.type_meta_data, &mut context.current_generics) {
        return Err(new_soul_error(iter.current(), format!("assignment type: '{}' is not compatible with variable type: '{}'", expression.is_type.to_string(), var_type.to_string()).as_str()));
    }

    expression.result.value = add_compount_assignment(&iter[symbool_index].text, &i_variable, expression.result.value);
    body_result.value = IStatment::new_assignment(i_variable, expression.result.value, iter.current())?;
    Ok(AssignmentResult{is_type: var_type, assignment: body_result})
}

fn add_compount_assignment(symbool: &str, i_variable: &IVariable, expression: IExpression) -> IExpression {
    let op = match symbool {
        "=" => return expression,
        "+=" => ExprOperatorType::Add,
        "-=" => ExprOperatorType::Sub,
        "*=" => ExprOperatorType::Mul,
        "/=" => ExprOperatorType::Div,
        "%=" => ExprOperatorType::Modulo,
        "&=" => ExprOperatorType::BitWiseAnd,
        "|=" => ExprOperatorType::BitWiseOr,
        "^=" => ExprOperatorType::BitWiseXor,
        _ => panic!("symbool: '{}' unknown", symbool)
    };

    IExpression::BinairyExpression { 
        left: Box::new(IExpression::IVariable {this: i_variable.clone(), span: i_variable.get_span().clone()}), 
        operator_type: op, right: Box::new(expression), 
        type_name: i_variable.get_type_name().clone(),
        span: i_variable.get_span().clone()
    }
}

fn is_symbool_allowed(
    symbool: &Token,
    context: &CurrentContext,
    soul_type: &SoulType,
) -> Result<()> {
    const ALLOWED_DEFAULT_SYMBOOLS: [&str; 12] = ["-=", "+=", "*=", "/=", "&=", "|=", "^=", "%=","=", "++", "--", "."];
    const ALLOWED_ARRAY_SYMBOOLS: [&str; 4] = ["[", "=", "+=", "."];
    const ALLOWED_POINTER_SYMBOOLS: [&str; 2] = ["=", "."];

    const ALLOWED_CONST_ARRAY_SYMBOOLS: [&str; 3] = ["[", "=", "."];
    // const ALLOWED_REF_SYMBOOLS: [&str; 2] = ["=", "."];
    // const ALLOWED_MUT_REF_SYMBOOLS: [&str; 2] = ["=", "."];
    const ALLOWED_CONST_SYMBOOLS: [&str; 2] = ["=", "."];

    // !!!!! this is temp should add operator system

    let err_symbool = |allowed_symbols: &[&str]| {
        new_soul_error(symbool, format!("type '{}' in ruleSet '{:?}' does not allow symbool '{}' only allows '{:?}'", soul_type.to_string(), context.rulesets, symbool.text, allowed_symbols).as_str())
    };

    let allowed_symbols: &[&str];

    if !soul_type.is_mutable() {

        if soul_type.is_array() {
            allowed_symbols = &ALLOWED_CONST_ARRAY_SYMBOOLS;
        }
        else {
            allowed_symbols = &ALLOWED_CONST_SYMBOOLS;
        }
    }
    else if soul_type.is_array() {
        allowed_symbols = &ALLOWED_ARRAY_SYMBOOLS;
    }
    else if soul_type.is_pointer() {

        if context.rulesets.contains(RuleSet::Unsafe) {
            allowed_symbols = &ALLOWED_DEFAULT_SYMBOOLS;
        }
        else {
            allowed_symbols = &ALLOWED_POINTER_SYMBOOLS;
        }
    }
    else {
        allowed_symbols = &ALLOWED_DEFAULT_SYMBOOLS; 
    }

    if !allowed_symbols.iter().any(|str| str == &symbool.text) {
        return Err(err_symbool(allowed_symbols));
    }

    return Ok(());
}



