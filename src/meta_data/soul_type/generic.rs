use crate::abstract_styntax_tree::abstract_styntax_tree::IExpression;

#[derive(Debug, Clone, PartialEq)]
pub struct GenericValidater {
    pub possible_condition: IExpression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Generic {
    pub type_name: String,
    pub validater: Option<GenericValidater>,
}