use crate::meta_data::current_context::current_context::CurrentContext;

use super::operator_type::OperatorType;

#[derive(Debug, Clone)]
pub enum IVariable {
    Variable{name: String, type_name: String},
    MemberExpression{parent: /*Variable*/Box<IVariable>, expression: Box<IExpression>},
}

#[derive(Debug, Clone)]
pub enum IExpression {
    IVariable{this: IVariable},
    BinairyExpression{left: Box<IExpression>, operator_type: OperatorType, right: Box<IExpression>, type_name: String},
    Literal{value: String, type_name: String}, 
    ConstRef{expression: Box<IExpression>},
    MutRef{expression: Box<IExpression>},
    DeRef{expression: Box<IExpression>},
    Increment{variable: IVariable, is_before: bool, amount: i8},
    // FunctionCall{name: String, return_type: String, args: Vec<Expression>, func_info: u8, generic_defines: BTreeMap<u8, u8>},
    EmptyExpression(),
} 

#[derive(Debug, Clone)]
pub enum IStatment {
    Assignment{variable: IVariable, assign: Box<IExpression>},
    Initialize{variable: IVariable, assignment: /*Assignment*/Option<Box<IStatment>>},
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
    pub global_variables: /*Initialize*/Vec<IStatment>,
    pub main_nodes: Vec<IMainNodes>,
}

impl AbstractSyntaxTree {
    pub fn new() -> Self {
        AbstractSyntaxTree {
            global_variables: Vec::new(),
            main_nodes: Vec::new(),
        }
    }
}