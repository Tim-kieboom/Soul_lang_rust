use crate::abstract_styntax_tree::abstract_styntax_tree::IExpression;

#[derive(Debug, Clone)]
pub struct GenericValidater {
    pub possible_condition: Option<IExpression>,
}

#[derive(Debug, Clone)]
pub struct Generic {
    pub type_name: String,
    pub validater: GenericValidater,
}