use std::collections::BTreeMap;

use super::operator_type::OperatorType;
use crate::meta_data::{current_context::current_context::CurrentContext, function::{function_declaration::function_declaration::FunctionDeclaration}};

#[derive(Debug, Clone, PartialEq)]
pub enum IVariable {
    Variable{name: String, type_name: String},
    MemberExpression{parent: /*shouldBe_Variable*/Box<IVariable>, expression: Box<IExpression>},
}

#[derive(Debug, Clone, PartialEq)]
pub enum IExpression {
    IVariable{this: IVariable},
    BinairyExpression{left: Box<IExpression>, operator_type: OperatorType, right: Box<IExpression>, type_name: String},
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
    Assignment{variable: IVariable, assign: Box<IExpression>},
    Initialize{variable: IVariable, assignment: /*shouldBe_Assignment*/Option<Box<IStatment>>},
} 

#[derive(Debug, Clone)]
pub enum IMainNodes {
    // Function{func_info: u8, body: BodyNode, generics: u8},
} 

#[derive(Debug, Clone)]
pub enum ISyntaxNode {
    Statment{this: IStatment},
    Expression{this: IExpression},
    IMainNodes{this: IMainNodes},
} 

#[derive(Debug, Clone)] 
pub struct BodyNode {
    pub statments: Vec<IStatment>,
    pub context: CurrentContext,
}

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    pub global_variables: /*shouldBe_Initialize*/Vec<IStatment>,
    pub main_nodes: Vec<IMainNodes>,
}

impl IStatment {
    pub fn to_string(&self, pretty_format: bool) -> String {
        let join_char = if pretty_format {"\n\t"} 
                        else {" "}; 
        match self {
            IStatment::Assignment { variable, assign } => format!("Assignment({}variable: {},{}assign: {}{})", join_char, variable.to_string(), join_char, assign.to_string(), join_char),
            IStatment::Initialize { variable, assignment } => {
                let assignment_string = if let Some(expr) = assignment {expr.to_string(false)}
                                        else {"<Empty>".to_string()};

                format!("Initialize({}variable: {},{}assignment: {}{})", join_char, variable.to_string(), join_char, assignment_string, join_char)
            },
        }
    }
}

impl IVariable {
    pub fn to_string(&self) -> String {
        match self {
            IVariable::Variable { name, type_name } => format!("Variable(name: {}, type: {})", name, type_name),
            IVariable::MemberExpression { parent, expression } => format!("Variable(parent: {}, expression: {})", parent.to_string(), expression.to_string()),
        }
    }
    
    #[allow(dead_code)]
    pub fn new_variable(name: &str, type_name: &str) -> Self {
        IVariable::Variable { 
            name: name.to_string(),
            type_name: type_name.to_string(),
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
    pub fn new_binary_expression(left: IExpression, operator_type: OperatorType, right: IExpression, type_name: &str) -> Self {
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
            IExpression::Literal { value, type_name } => format!("Literal({}, type: {})", value, type_name),
            IExpression::ConstRef { expression } => format!("ConstRef({})", expression.to_string()),
            IExpression::MutRef { expression } => format!("MutRef({})", expression.to_string()),
            IExpression::DeRef { expression } => format!("Deref({})", expression.to_string()),
            IExpression::Increment { variable, is_before, amount } => format!("Increment({}, isBefore: {}, amount: {})", variable.to_string(), *is_before, *amount),
            IExpression::EmptyExpression() => "EmptyExpression()".to_string(),
            IExpression::FunctionCall { args, generic_defines, function_info } => format!("FunctionCall(info: {:?}, args: {:?}, generics: {:?})", function_info, args, generic_defines),
        }
    }
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            global_variables: Vec::new(),
            main_nodes: Vec::new(),
        }
    }
}