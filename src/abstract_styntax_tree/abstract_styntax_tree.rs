use std::collections::BTreeMap;
use super::operator_type::ExprOperatorType;
use crate::{meta_data::{borrow_checker::borrow_checker::DeleteList, current_context::current_context::CurrentContext, function::function_declaration::function_declaration::FunctionDeclaration, meta_data::MetaData, soul_error::soul_error::SoulSpan, soul_names::{NamesOperator, NamesTypeWrapper, SOUL_NAMES}}, tokenizer::token::Token};

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
    Increment{variable: IVariable, is_before: bool, amount: i8, span: SoulSpan},
    FunctionCall{args: Vec<IExpression>, generic_defines: BTreeMap<String, String>, function_info: Box<FunctionDeclaration>, span: SoulSpan},
    EmptyExpression(),
} 

#[derive(Debug, Clone, PartialEq)]
pub enum IStatment {
    CloseScope(),
    EmptyStatment(),
    Assignment{variable: IVariable, assign: IExpression, span: SoulSpan},
    Initialize{variable: IVariable, assignment: /*shouldBe_Assignment*/Option<Box<IStatment>>, span: SoulSpan},
    FunctionBody{func_info: FunctionDeclaration, body: Box<BodyNode>, span: SoulSpan},
    FunctionCall{this: /*shouldBe_FunctionCall*/ IExpression, span: SoulSpan},
    Return{expression: Option<IExpression>, span: SoulSpan},
    Scope{body: Box<BodyNode>, span: SoulSpan},
    If{condition: IExpression, body: Box<BodyNode>, span: SoulSpan},
    Else{body: Box<BodyNode>, span: SoulSpan},
    ElseIf{condition: IExpression, body: Box<BodyNode>, span: SoulSpan},
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
    pub global_context: CurrentContext,
}

impl BodyNode {
    pub fn new(context: CurrentContext) -> Self {
        Self { statments: Vec::new(), context, delete_list: DeleteList::new() }
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
        result
    }
}

impl IStatment {
    pub fn to_string(&self, pretty_format: bool) -> String {
        self.internal_to_string(pretty_format, 0)
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
            IStatment::EmptyStatment() => "EmptyStatment()".to_string(),
            IStatment::Assignment { variable, assign, span: _ } => format!(
                    "Assignment({}{} = {}{})",
                    child_indent,
                    variable.to_string(),
                    assign.to_string(),
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
            IStatment::CloseScope() => "CloseScope()".to_string(),
            IStatment::Scope { body, span: _ } => format!(
                    "Scope({}{}{})",
                    child_indent,
                    body.to_string(pretty_format, tab + 1),
                    base_indent
                ),
            IStatment::Return { expression, span: _ } => match expression {
                    Some(expr) => format!(
                        "Return({})",
                        expr.to_string(),
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
        }
    }

    pub fn new_assignment(variable: IVariable, assign: IExpression, token: &Token) -> Self {
        debug_assert!(!matches!(assign, IExpression::EmptyExpression()));
        Self::Assignment { variable, assign, span: SoulSpan::from_token(token)}
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

    pub fn new_return(expression: Option<IExpression>, token: &Token) -> Self {
        Self::Return { expression, span: SoulSpan::from_token(token) }
    }

    pub fn new_if(condition: IExpression, body: BodyNode, token: &Token) -> Self {
        Self::If { condition, body: Box::new(body), span: SoulSpan::from_token(token) }
    }

    pub fn new_else(body: BodyNode, token: &Token) -> Self {
        Self::Else { body: Box::new(body), span: SoulSpan::from_token(token) }
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
    #[allow(dead_code)]
    pub fn new_variable(name: &str, type_name: &str, token: &Token) -> Self {
        IExpression::IVariable { 
            this: IVariable::new_variable(name, type_name, token),
            span: SoulSpan::from_token(token)
        }
    }

    #[allow(dead_code)]
    pub fn new_literal(value: &str, type_name: &str, token: &Token) -> Self {
        IExpression::Literal{
            value: value.to_string(), 
            type_name: type_name.to_string(),
            span: SoulSpan::from_token(token),
        }
    }

    #[allow(dead_code)]
    pub fn new_increment(variable: IVariable, is_before: bool, amount: i8, token: &Token) -> Self {
        IExpression::Increment{
            variable, 
            is_before, 
            amount,
            span: SoulSpan::from_token(token),
        }
    }

    #[allow(dead_code)]
    pub fn new_binary_expression(left: IExpression, operator_type: ExprOperatorType, right: IExpression, type_name: &str, token: &Token) -> Self {
        IExpression::BinairyExpression{
            left: Box::new(left), 
            operator_type, right: Box::new(right), 
            type_name: type_name.to_string(),
            span: SoulSpan::from_token(token),
        }
    }

    #[allow(dead_code)]
    pub fn new_mutref(expression: IExpression, token: &Token) -> Self {
        IExpression::MutRef { 
            expression: Box::new(expression),
            span: SoulSpan::from_token(token),
        }
    }

    #[allow(dead_code)]
    pub fn new_constref(expression: IExpression, token: &Token) -> Self {
        IExpression::ConstRef { 
            expression: Box::new(expression),
            span: SoulSpan::from_token(token),
        }
    }

    #[allow(dead_code)]
    pub fn new_deref(expression: IExpression, token: &Token) -> Self {
        IExpression::DeRef { 
            expression: Box::new(expression),
            span: SoulSpan::from_token(token),
        }
    }

    #[allow(dead_code)]
    pub fn new_funtion_call(function_info: FunctionDeclaration, args: Vec<IExpression>, generic_defines: BTreeMap<String, String>, token: &Token) -> Self {
        IExpression::FunctionCall { 
            args, 
            generic_defines, 
            function_info: Box::new(function_info),
            span: SoulSpan::from_token(token),
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
            IExpression::EmptyExpression() => "EmptyExpression()".to_string(),
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























