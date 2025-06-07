use std::collections::BTreeMap;

use itertools::Itertools;

use super::operator_type::ExprOperatorType;
use crate::meta_data::{borrow_checker::borrow_checker::DeleteList, current_context::current_context::CurrentContext, function::function_declaration::function_declaration::FunctionDeclaration, soul_names::{NamesOperator, NamesTypeWrapper, SOUL_NAMES}};

#[derive(Debug, Clone, PartialEq)]
pub enum IVariable {
    Variable{name: String, type_name: String},
    // MemberExpression{parent: /*shouldBe_Variable*/Box<IVariable>, name: String, type_name: String},
}

#[derive(Debug, Clone, PartialEq)]
pub enum IExpression {
    IVariable{this: IVariable},
    BinairyExpression{left: Box<IExpression>, operator_type: ExprOperatorType, right: Box<IExpression>, type_name: String},
    Literal{value: String, type_name: String}, 
    ConstRef{expression: Box<IExpression>},
    MutRef{expression: Box<IExpression>},
    DeRef{expression: Box<IExpression>},
    Increment{variable: IVariable, is_before: bool, amount: i8},
    FunctionCall{args: Vec<IExpression>, generic_defines: BTreeMap<String, String>, function_info: FunctionDeclaration},
    EmptyExpression(),
} 

#[derive(Debug, Clone, PartialEq)]
pub enum IStatment {
    CloseScope(),
    EmptyStatment(),
    Assignment{variable: IVariable, assign: Box<IExpression>},
    Initialize{variable: IVariable, assignment: /*shouldBe_Assignment*/Option<Box<IStatment>>},
    FunctionBody{func_info: FunctionDeclaration, body: Box<BodyNode>},
    FunctionCall{this: /*shouldBe_FunctionCall*/Box<IExpression>},
    Return{expression: Option<Box<IExpression>>},
    Scope{body: Box<BodyNode>},
    If{condition: Box<IExpression>, body: Box<BodyNode>}
} 

#[derive(Debug, Clone, PartialEq)] 
pub struct BodyNode {
    pub statments: Vec<IStatment>,
    pub context: CurrentContext,
    pub delete_list: DeleteList,
}

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    pub main_nodes: Vec<IStatment>,
}

impl BodyNode {
    pub fn new(context: CurrentContext) -> Self {
        Self { statments: Vec::new(), context, delete_list: DeleteList::new() }
    }

    pub fn to_string(&self, pretty_format: bool, tab: usize) -> String {
        let mut join_symbool = "\n".to_string();
        for _ in 0..tab {
            join_symbool.push('\t');
        }
        
        let str = self.statments.iter()
            .map(|stat| stat.to_string(pretty_format))
            .chain(self.delete_list.iter().map(|var| format!("drop({})", var)))
            .join(&join_symbool);

        format!("{}{}{}{}{}", '{', join_symbool, str, join_symbool, '}')
    }
}

impl IStatment {
    pub fn to_string(&self, pretty_format: bool) -> String {
        self.internal_to_string(pretty_format, 1)
    }

    fn internal_to_string(&self, pretty_format: bool, tab: usize) -> String {
        let join_char;
        let child_join_char;
        let mut join_symbool;
        if pretty_format {
            join_symbool = "\n".to_string();
            for _ in 0..tab+1 {
                join_symbool.push('\t');
            }

            join_char = &join_symbool[..join_symbool.len()-1];
            child_join_char = &join_symbool[..];
        }
        else {
            join_char = "";
            child_join_char = "";
        }

        match self {
            IStatment::EmptyStatment() => "EmptyStatment()".to_string(),
            IStatment::Assignment { variable, assign } => format!("Assignment({}{} = {}{})", child_join_char, variable.to_string(), assign.to_string(), join_char),
            IStatment::Initialize { variable, assignment } => {
                match assignment {
                    Some(expr) => format!("Initialize({}{}{})", child_join_char, expr.internal_to_string(pretty_format, tab + 1), join_char),
                    None => format!("Initialize({}{}{})", child_join_char, variable.to_string(), join_char),
                }
            },
            IStatment::FunctionBody { func_info, body } => format!("FunctionBody({}{})", func_info.to_string(), body.to_string(pretty_format, tab)),
            IStatment::FunctionCall { this } => this.to_string(),
            IStatment::CloseScope() => "CloseScope()".to_string(),
            IStatment::Scope { body } => format!("Scope({})", body.to_string(pretty_format, tab + 1)),
            IStatment::Return { expression } => {
                match expression {
                    Some(expr) => format!("Return({})", expr.to_string()),
                    None => format!("Return()"),
                }
            },
            IStatment::If { condition, body } => {
                format!(
                    "If({}{}{})",
                    condition.to_string(),
                    child_join_char,
                    body.to_string(pretty_format, tab + 1)
                )
            },
        }
    }

    pub fn new_assignment(variable: IVariable, assign: IExpression) -> Self {
        debug_assert!(!matches!(assign, IExpression::EmptyExpression()));
        Self::Assignment { variable, assign: Box::new(assign) }
    }

    pub fn new_initialize(variable: IVariable, assignment: Option<IStatment>) -> Self {
        debug_assert!(
            assignment.as_ref().is_none_or(|assign| matches!(assign, IStatment::Assignment {..}) )
        );

        Self::Initialize { variable, assignment: assignment.map(|assign| Box::new(assign)) }
    }

    pub fn new_function_call(this: IExpression) -> Self {
        debug_assert!(
            matches!(this, IExpression::FunctionCall {..})
        );

        Self::FunctionCall { this: Box::new(this) }
    }

    pub fn new_function_body(func_info: FunctionDeclaration, body: BodyNode) -> Self {
        Self::FunctionBody{func_info, body: Box::new(body)}
    }

    pub fn new_scope(body: BodyNode) -> Self {
        Self::Scope { body: Box::new(body) }
    }

    pub fn new_return(expression: Option<IExpression>) -> Self {
        Self::Return { expression: expression.map(|expr| Box::new(expr)) }
    }
}

impl IVariable {
    pub fn to_string(&self) -> String {
        match self {
            IVariable::Variable { name, type_name } => format!("Variable({} {})", type_name, name),
            // IVariable::MemberExpression { parent, name, type_name } => format!("Variable(parent: {}, {} {})", parent.to_string(), type_name, name),
        }
    }
    
    #[allow(dead_code)]
    pub fn new_variable(name: &str, type_name: &str) -> Self {
        IVariable::Variable { 
            name: name.to_string(),
            type_name: type_name.to_string(),
        } 
    }

    pub fn get_name(&self) -> &String {
        match self {
            IVariable::Variable { name, .. } => name,
            // IVariable::MemberExpression { name, .. } => name,
        }
    }

    pub fn get_type_name(&self) -> &String {
        match self {
            IVariable::Variable { type_name, .. } => type_name,
            // IVariable::MemberExpression { type_name, .. } => type_name,
        }
    }
}


impl IExpression {
    #[allow(dead_code)]
    pub fn new_variable(name: &str, type_name: &str) -> Self {
        IExpression::IVariable { 
            this: IVariable::new_variable(name, type_name)
        }
    }

    #[allow(dead_code)]
    pub fn new_literal(value: &str, type_name: &str) -> Self {
        IExpression::Literal{value: value.to_string(), type_name: type_name.to_string()}
    }

    #[allow(dead_code)]
    pub fn new_increment(variable: IVariable, is_before: bool, amount: i8) -> Self {
        IExpression::Increment{variable, is_before, amount}
    }

    #[allow(dead_code)]
    pub fn new_binary_expression(left: IExpression, operator_type: ExprOperatorType, right: IExpression, type_name: &str) -> Self {
        IExpression::BinairyExpression{
            left: Box::new(left), 
            operator_type, right: Box::new(right), 
            type_name: type_name.to_string(),
        }   
    }

    #[allow(dead_code)]
    pub fn new_mutref(expression: IExpression) -> Self {
        IExpression::MutRef { expression: Box::new(expression) }
    }

    #[allow(dead_code)]
    pub fn new_constref(expression: IExpression) -> Self {
        IExpression::ConstRef { expression: Box::new(expression) }
    }

    #[allow(dead_code)]
    pub fn new_deref(expression: IExpression) -> Self {
        IExpression::DeRef { expression: Box::new(expression) }
    }

    #[allow(dead_code)]
    pub fn new_funtion_call(function_info: FunctionDeclaration, args: Vec<IExpression>, generic_defines: BTreeMap<String, String>) -> Self {
        IExpression::FunctionCall { args, generic_defines, function_info }
    }

    pub fn to_string(&self) -> String {
        match self {
            IExpression::IVariable { this } => this.to_string(),
            IExpression::BinairyExpression { left, operator_type, right, type_name } => format!("BinaryExpression({} {} {}, type: {})", left.to_string(), operator_type.to_str(), right.to_string(), type_name),
            IExpression::Literal { value, type_name } => format!("Literal({} {})", type_name, value),
            IExpression::ConstRef { expression } => format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef), expression.to_string()),
            IExpression::MutRef { expression } => format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::MutRef), expression.to_string()),
            IExpression::DeRef { expression } => format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::Pointer), expression.to_string()),
            IExpression::Increment { variable, is_before, amount } => {
                let symbool; 
                if *amount < 0 {
                    symbool = SOUL_NAMES.get_name(NamesOperator::Decrement);
                }
                else {
                    symbool = SOUL_NAMES.get_name(NamesOperator::Increment);
                };
                
                if *is_before {
                    format!("{}{}", symbool, variable.to_string())
                }
                else {
                    format!("{}{}", variable.to_string(), symbool)
                }
            },
            IExpression::EmptyExpression() => "EmptyExpression()".to_string(),
            IExpression::FunctionCall { args, generic_defines, function_info } => {
                let mut string_builder = String::new();
                string_builder.push_str("FunctionCall(");
                string_builder.push_str(&function_info.name);

                if !generic_defines.is_empty() {
                    string_builder.push('<');

                    for (i, (_template_name, type_str)) in generic_defines.iter().enumerate() {
                        string_builder.push_str(&type_str);
                        if i != generic_defines.len() - 1 {
                            string_builder.push_str(", ");
                        }
                    }
                    string_builder.push('>');
                }
                string_builder.push('(');
                for (i, arg) in args.iter().enumerate() {
                    string_builder.push_str(&arg.to_string());
                    
                    if i != args.len() - 1 {
                        string_builder.push_str(", ");
                    }
                }
                string_builder.push(')');
                string_builder.push(')');
                string_builder
            }
        }
    }
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            main_nodes: Vec::new(),
        }
    }
}

























