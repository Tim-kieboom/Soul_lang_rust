use std::collections::BTreeMap;
use super::operator_type::ExprOperatorType;
use crate::meta_data::soul_error::soul_error::Result;
use crate::{meta_data::{borrow_checker::borrow_checker::DeleteList, current_context::current_context::CurrentContext, function::function_declaration::function_declaration::FunctionDeclaration, meta_data::MetaData, scope_and_var::scope::ScopeId, soul_error::soul_error::{new_soul_error, SoulSpan}, soul_names::{NamesOperator, NamesTypeWrapper, SOUL_NAMES}}, tokenizer::token::Token};

#[derive(Debug, Clone, PartialEq)]
pub enum IVariable {
    Variable{name: String, type_name: String, span: SoulSpan},
    // MemberExpression{parent: /*shouldBe_Variable*/Box<IVariable>, name: String, type_name: String},
}

#[derive(Debug, Clone, PartialEq)]
pub enum IExpression {
    IVariable{this: IVariable, span: SoulSpan},
    BinairyExpression{left: Box<IExpression>, operator_type: ExprOperatorType, right: Box<IExpression>, type_name: String, span: SoulSpan},
    Literal{value: String, type_name: String, span: SoulSpan}, 
    ConstRef{expression: Box<IExpression>, span: SoulSpan},
    MutRef{expression: Box<IExpression>, span: SoulSpan},
    DeRef{expression: Box<IExpression>, span: SoulSpan},
    Increment{variable: Box<IExpression>, is_before: bool, amount: i8, span: SoulSpan},
    FunctionCall{args: Vec<IExpression>, generic_defines: BTreeMap<String, String>, function_info: Box<FunctionDeclaration>, span: SoulSpan},
    Index{this: Box<IExpression>, index: Box<IExpression>, return_type: String, span: SoulSpan},
    EmptyExpression(SoulSpan),
} 

#[derive(Debug, Clone, PartialEq)]
pub enum IStatment {
    CloseScope(SoulSpan),
    EmptyStatment(SoulSpan),
    Assignment{variable: IExpression, assign: IExpression, span: SoulSpan},
    TypeDef{type_name: String, from_type: String, span: SoulSpan},
    Initialize{variable: IVariable, assignment: /*shouldBe_Assignment*/Option<Box<IStatment>>, span: SoulSpan},
    FunctionBody{func_info: FunctionDeclaration, body: Box<BodyNode>, span: SoulSpan},
    FunctionCall{this: /*shouldBe_FunctionCall*/ IExpression, span: SoulSpan},
    Return{expression: Option<IExpression>, delete_list: DeleteList, span: SoulSpan},
    Scope{body: Box<BodyNode>, span: SoulSpan},
    If{condition: IExpression, body: Box<BodyNode>, span: SoulSpan},
    Else{body: Box<BodyNode>, span: SoulSpan},
    ElseIf{condition: IExpression, body: Box<BodyNode>, span: SoulSpan},
    While{condition: IExpression, body: Box<BodyNode>, span: SoulSpan},
    For{element: IExpression, collection: IExpression, body: Box<BodyNode>, span: SoulSpan},
} 

#[derive(Debug, Clone, PartialEq)] 
pub struct BodyNode {
    pub statments: Vec<IStatment>,
    pub context: CurrentContext,
    pub delete_list: DeleteList,
    pub scope_id: ScopeId,
}

#[derive(Debug, Clone)]
pub struct AbstractSyntaxTree {
    pub main_nodes: Vec<IStatment>,
    pub global_context: CurrentContext,
}

impl BodyNode {
    pub fn new(context: CurrentContext) -> Self {
        let scope_id = context.get_current_scope_id();
        Self { statments: Vec::new(), context, delete_list: DeleteList::new(), scope_id }
    }

    pub fn to_string(&self, pretty_format: bool, tab: usize) -> String {
        let base_indent = format!("\n{}", "\t".repeat(tab));
        let parent_indent = format!("\n{}", "\t".repeat((tab as i64-1).max(0) as usize));

        let mut result = String::new();
        result.push('{');
        result.push_str(&base_indent);
        for stmt in &self.statments {
            result.push_str(&stmt.internal_to_string(pretty_format, tab));
            result.push_str(&base_indent);
        }
        result.push_str(&parent_indent);
        result.push('}');
        result.push_str(".deletes(");
        for delete in &self.delete_list {
            result.push_str(delete);
            result.push_str(",");
        }
        result.push_str(")");
        result
    }
}

impl IStatment {
    pub fn to_string(&self, pretty_format: bool) -> String {
        self.internal_to_string(pretty_format, 0)
    }

    pub fn get_span(&self) -> &SoulSpan {
        match self {
            IStatment::CloseScope(soul_span) => soul_span,
            IStatment::EmptyStatment(soul_span) => soul_span,
            IStatment::Assignment{span, ..} => span,
            IStatment::TypeDef{span, ..} => span,
            IStatment::Initialize{span, ..} => span,
            IStatment::FunctionBody{span, ..} => span,
            IStatment::FunctionCall{span, ..} => span,
            IStatment::Return{span, ..} => span,
            IStatment::Scope{span, ..} => span,
            IStatment::If{span, ..} => span,
            IStatment::Else{span, ..} => span,
            IStatment::ElseIf{span, ..} => span,
            IStatment::While{span, ..} => span,
            IStatment::For{span, ..} => span,
        }
    }

    fn internal_to_string(&self, pretty_format: bool, tab: usize) -> String {
        fn indent(tab: usize) -> String {
            "\t".repeat(tab)
        }

        let (base_indent, child_indent) = if pretty_format {
            let base_indent = indent(tab);
            let child_indent = indent(tab + 1);
            (format!("\n{}", base_indent), format!("\n{}", child_indent))
        } 
        else {
            (String::new(), String::new())
        };

        match self {
            IStatment::EmptyStatment(_) => "EmptyStatment()".to_string(),
            IStatment::Assignment { variable, assign, span: _ } => format!(
                    "Assignment({}{} = {}{})",
                    child_indent,
                    variable.to_string(),
                    assign.to_string(),
                    base_indent
                ),
            IStatment::TypeDef {type_name, from_type, span: _ } => format!(
                    "TypeDef({}{} typedef {}{})",
                    child_indent,
                    type_name,
                    from_type,
                    base_indent
                ),
            IStatment::Initialize { variable, assignment, span: _ } => match assignment {
                    Some(expr) => format!(
                        "Initialize({}{}{})",
                        child_indent,
                        expr.internal_to_string(pretty_format, tab + 1),
                        base_indent
                    ),
                    None => format!(
                        "Initialize({}variable: {}{})",
                        child_indent,
                        variable.to_string(),
                        base_indent
                    ),
                },
            IStatment::FunctionBody { func_info, body, span: _ } => format!(
                    "FunctionBody({}{}){}",
                    func_info.to_string(),
                    body.to_string(pretty_format, tab + 1),
                    base_indent
                ),
            IStatment::FunctionCall { this, span: _ } => this.to_string(),
            IStatment::CloseScope(_) => "CloseScope()".to_string(),
            IStatment::Scope { body, span: _ } => format!(
                    "Scope({}{}{})",
                    child_indent,
                    body.to_string(pretty_format, tab + 1),
                    base_indent
                ),
            IStatment::Return { expression, delete_list, span: _ } => match expression {
                    Some(expr) => format!(
                        "Return({}, deletes({:?}))",
                        expr.to_string(),
                        delete_list,
                    ),
                    None => "Return()".to_string(),
                },
            IStatment::If { condition, body, span: _ } => format!(
                    "If({}{})",
                    condition.to_string(),
                    body.to_string(pretty_format, tab + 1),
                ),
            IStatment::Else { body, span:_ } =>  format!(
                    "Else({})",
                    body.to_string(pretty_format, tab + 1),
                ),
            IStatment::ElseIf { condition, body, span:_ } =>  format!(
                    "ElseIf({}{})",
                    condition.to_string(),
                    body.to_string(pretty_format, tab + 1),
                ),
            IStatment::While { condition, body, span:_ } =>  format!(
                    "While({}{})",
                    condition.to_string(),
                    body.to_string(pretty_format, tab + 1),
                ),
            IStatment::For { element, collection, body, span:_ } =>  format!(
                    "For({} in {}{})",
                    element.to_string(),
                    collection.to_string(),
                    body.to_string(pretty_format, tab + 1),
                ),
        }
    }

    pub fn new_assignment(variable: IExpression, assign: IExpression, token: &Token) -> Result<Self> {
        match variable {
            IExpression::DeRef{..} | 
            IExpression::MutRef{..} |
            IExpression::Literal{..} |
            IExpression::ConstRef{..} |
            IExpression::Increment{..} |
            IExpression::FunctionCall{..} |
            IExpression::EmptyExpression(..) |
            IExpression::BinairyExpression{..} => return Err(new_soul_error(token, format!("variable is invalid variant: '{}'", variable.get_variant_name()).as_str())),

            IExpression::Index{..} |
            IExpression::IVariable{..} => (),
        }
        
        if matches!(assign, IExpression::EmptyExpression(_)) {
            Err(new_soul_error(token, "assignment is empty"))
        }
        else {
            Ok(Self::Assignment { variable, assign, span: SoulSpan::from_token(token)})
        }
    }

    pub fn new_initialize(variable: IVariable, assignment: Option<IStatment>, token: &Token) -> Self {
        debug_assert!(
            assignment.as_ref().is_none_or(|assign| matches!(assign, IStatment::Assignment {..}) )
        );

        Self::Initialize { variable, assignment: assignment.map(|assign| Box::new(assign)), span: SoulSpan::from_token(token) }
    }

    pub fn new_function_call(this: IExpression, token: &Token) -> Self {
        debug_assert!(
            matches!(this, IExpression::FunctionCall {..})
        );

        Self::FunctionCall { this, span: SoulSpan::from_token(token) }
    }

    pub fn new_function_body(func_info: FunctionDeclaration, body: BodyNode, token: &Token) -> Self {
        Self::FunctionBody{func_info, body: Box::new(body), span: SoulSpan::from_token(token)}
    }

    pub fn new_scope(body: BodyNode, token: &Token) -> Self {
        Self::Scope { body: Box::new(body), span: SoulSpan::from_token(token) }
    }

    pub fn new_return(expression: Option<IExpression>, delete_list: Vec<String>, token: &Token) -> Self {
        Self::Return { expression, delete_list, span: SoulSpan::from_token(token) }
    }

    pub fn new_if(condition: IExpression, body: BodyNode, token: &Token) -> Self {
        Self::If { condition, body: Box::new(body), span: SoulSpan::from_token(token) }
    }

    pub fn new_else(body: BodyNode, token: &Token) -> Self {
        Self::Else { body: Box::new(body), span: SoulSpan::from_token(token) }
    }

    pub fn new_while(condition: IExpression, body: BodyNode, token: &Token) -> Self {
        Self::While { condition, body: Box::new(body), span: SoulSpan::from_token(token) }
    }

    pub fn new_for(element: IExpression, collection: IExpression, body: BodyNode, token: &Token) -> Self {
        Self::For { element, collection, body: Box::new(body), span: SoulSpan::from_token(token) }
    }

    pub fn new_type_def(type_name: String, from_type: String, token: &Token) -> Self {
        Self::TypeDef { type_name, from_type, span: SoulSpan::from_token(token) }
    }

}

impl IVariable {
    pub fn to_string(&self) -> String {
        match self {
            IVariable::Variable { name, type_name, span:_ } => format!("Variable({} {})", type_name, name),
            // IVariable::MemberExpression { parent, name, type_name } => format!("Variable(parent: {}, {} {})", parent.to_string(), type_name, name),
        }
    }
    
    #[allow(dead_code)]
    pub fn new_variable(name: &str, type_name: &str, token: &Token) -> Self {
        IVariable::Variable { 
            name: name.to_string(),
            type_name: type_name.to_string(),
            span: SoulSpan::from_token(token)
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

    pub fn get_span(&self) -> &SoulSpan {
        match self {
            IVariable::Variable { span,.. } => span,
        }
    } 
}


impl IExpression {
    pub fn new_variable(name: &str, type_name: &str, token: &Token) -> Self {
        IExpression::IVariable { 
            this: IVariable::new_variable(name, type_name, token),
            span: SoulSpan::from_token(token)
        }
    }

    pub fn new_literal(value: &str, type_name: &str, token: &Token) -> Self {
        IExpression::Literal{
            value: value.to_string(), 
            type_name: type_name.to_string(),
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_increment(variable: IExpression, is_before: bool, amount: i8, token: &Token) -> Self {
        IExpression::Increment{
            variable: Box::new(variable), 
            is_before, 
            amount,
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_binary_expression(left: IExpression, operator_type: ExprOperatorType, right: IExpression, type_name: &str, token: &Token) -> Self {
        IExpression::BinairyExpression{
            left: Box::new(left), 
            operator_type, right: Box::new(right), 
            type_name: type_name.to_string(),
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_mutref(expression: IExpression, token: &Token) -> Self {
        IExpression::MutRef { 
            expression: Box::new(expression),
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_constref(expression: IExpression, token: &Token) -> Self {
        IExpression::ConstRef { 
            expression: Box::new(expression),
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_deref(expression: IExpression, token: &Token) -> Self {
        IExpression::DeRef { 
            expression: Box::new(expression),
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_funtion_call(function_info: FunctionDeclaration, args: Vec<IExpression>, generic_defines: BTreeMap<String, String>, token: &Token) -> Self {
        IExpression::FunctionCall { 
            args, 
            generic_defines, 
            function_info: Box::new(function_info),
            span: SoulSpan::from_token(token),
        }
    }

    pub fn new_index(this: IExpression, index: IExpression, return_type: String, token: &Token) -> Self {
        IExpression::Index { 
            this: Box::new(this), 
            index: Box::new(index), 
            return_type, 
            span: SoulSpan::from_token(token), 
        } 
    }

    pub fn get_span(&self) -> &SoulSpan {
        match self {
            IExpression::Index{span, ..} => span,
            IExpression::DeRef{span, ..} => span,
            IExpression::MutRef{span, ..} => span,
            IExpression::Literal{span, ..} => span,
            IExpression::ConstRef{span, ..} => span,
            IExpression::Increment{span, ..} => span,
            IExpression::IVariable{span, ..} => span,
            IExpression::FunctionCall{span, ..} => span,
            IExpression::BinairyExpression{span, ..} => span,
            IExpression::EmptyExpression(soul_span) => soul_span,
        }
    }

    pub fn try_get_name(&self) -> Option<&String> {
        match self {
            IExpression::Literal{..} => None,
            IExpression::EmptyExpression(..) => None,
            IExpression::BinairyExpression{..} => None,
            IExpression::IVariable{this, ..} => Some(this.get_name()),
            IExpression::Index{this, ..} => this.try_get_name(),
            IExpression::DeRef{expression, ..} => expression.try_get_name(),
            IExpression::MutRef{expression, ..} => expression.try_get_name(),
            IExpression::ConstRef{expression, ..}=> expression.try_get_name(),
            IExpression::Increment{variable, ..} => variable.try_get_name(),
            IExpression::FunctionCall{function_info, ..} => Some(&function_info.name),
        }
    }

    pub fn try_get_type_name(&self) -> Option<&String> {
        match self {
            IExpression::EmptyExpression(..) => None,
            IExpression::Index{return_type, ..} => Some(return_type),
            IExpression::Literal{type_name, ..} => Some(type_name),
            IExpression::IVariable{this, ..} => Some(this.get_type_name()),
            IExpression::DeRef{expression, ..} => expression.try_get_name(),
            IExpression::MutRef{expression, ..} => expression.try_get_name(),
            IExpression::BinairyExpression{type_name, ..} => Some(type_name),
            IExpression::ConstRef{expression, ..}=> expression.try_get_name(),
            IExpression::Increment{variable, ..} => variable.try_get_type_name(),
            IExpression::FunctionCall{function_info, ..} => function_info.return_type.as_ref(),
        }
    }

    pub fn get_variant_name(&self) -> &str {
        match self {
            IExpression::Index{..} => "Index",
            IExpression::DeRef{..} => "DeRef",
            IExpression::MutRef{..} => "MutRef",
            IExpression::Literal{..} => "Literal",
            IExpression::ConstRef{..} => "ConstRef",
            IExpression::IVariable{..} => "IVariable",
            IExpression::Increment{..} => "Increment",
            IExpression::FunctionCall{..} => "FunctionCall",
            IExpression::EmptyExpression(..) => "EmptyExpression",
            IExpression::BinairyExpression{..} => "BinairyExpression",
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            IExpression::IVariable { this, span:_ } => this.to_string(),
            IExpression::BinairyExpression { left, operator_type, right, type_name, span:_ } => format!("BinaryExpression({} {} {}, type: {})", left.to_string(), operator_type.to_str(), right.to_string(), type_name),
            IExpression::Literal { value, type_name, span:_ } => format!("Literal({} {})", type_name, value),
            IExpression::ConstRef { expression, span:_ } => format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::ConstRef), expression.to_string()),
            IExpression::MutRef { expression, span:_ } => format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::MutRef), expression.to_string()),
            IExpression::DeRef { expression, span:_ } => format!("{}{}", SOUL_NAMES.get_name(NamesTypeWrapper::Pointer), expression.to_string()),
            IExpression::Index { this, index, return_type , span:_ } => format!("Index({}[{}], type: {})", this.to_string(), index.to_string(), return_type.to_string()),
            IExpression::Increment { variable, is_before, amount, span:_ } => {
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
            IExpression::EmptyExpression(_) => "EmptyExpression()".to_string(),
            IExpression::FunctionCall { args, generic_defines, function_info, span:_ } => {
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
            global_context: CurrentContext::new(MetaData::GLOBAL_SCOPE_ID),
        }
    }
}























