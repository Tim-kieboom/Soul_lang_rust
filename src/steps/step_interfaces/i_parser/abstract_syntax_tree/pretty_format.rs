use itertools::Itertools;
use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{abstract_syntax_tree::AbstractSyntacTree, enum_like::{Enum, EnumVariantKind, TypeEnum, Union, UnionVariant, UnionVariantKind}, expression::{AccessField, Binary, CaseDoKind, ElseKind, Expression, ExpressionGroup, ExpressionKind, ExternalExpression, For, If, Index, Match, NamedTuple, StaticField, Ternary, Tuple, Unary, UnwrapVariable, While}, function::{Constructor, Function, FunctionCall, FunctionSignature, Lambda, LambdaBody, Parameter, StaticMethod}, generic::GenericParameter, literal::Literal, object::{Class, Field, FieldAccess, Struct, Trait, Visibility}, soul_type::soul_type::{SoulType, TypeWrapper}, spanned::Spanned, statement::{Block, Implement, StatementKind}}, scope_builder::{ScopeBuilder, ScopeKind}};

pub trait PrettyFormat {
    fn to_pretty_string(&self) -> String;
}

pub trait PrettyString {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String;
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl PrettyFormat for ScopeBuilder {
    fn to_pretty_string(&self) -> String {
        use std::fmt::Write;

        let mut string = self.get_scopes()
            .iter()
            .map(|scope| {
                let body = scope.symbols.iter()
                    .map(|(name, kind)| format!("\t{} => {},", name, kind.node.to_string()))
                    .join("\n");

                format!("scope({}) {{\n{}\n}}\n", scope.self_index, body) 
            })
            .join("");

        write!(
            string, 
            "\nprogramMemory() {{\n{}\n}}", 
            self.global_literals
                .store
                .iter()
                .map(|(literal, id)| format!("\t__soul_mem_{}({})", id.0, literal.to_string()))
                .join("\n"),
        ).expect("error writing programMemory");
        string
    }
}

impl PrettyFormat for AbstractSyntacTree {
    fn to_pretty_string(&self) -> String {
        self.root.to_pretty(0, false)
    }
}

impl PrettyString for Block {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        self.statments.iter()
            .enumerate()
            .map(|(i, statment)| {
                let is_last = i == self.statments.len() - 1;
                statment.node.to_pretty(tab, is_last)
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl PrettyString for StatementKind {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            StatementKind::Expression(spanned) => format!(
                "{}Expression<{}> >> {}", 
                prefix, 
                spanned.node.get_variant_name(),
                spanned.node.to_pretty(tab, is_last),
            ),

            StatementKind::Variable(variable_name) => format!("{}Variable >> {}", prefix, variable_name.name),
            StatementKind::Assignment(assignment) => format!("{}Assignment >> {} = {}", prefix, assignment.variable.node.to_string(), assignment.value.node.to_string()),

            StatementKind::Function(function) => function.to_pretty(tab, is_last),

            StatementKind::Class(class) => class.to_pretty(tab, is_last),
            StatementKind::Struct(struct_) => struct_.to_pretty(tab, is_last),
            StatementKind::Trait(trait_) => trait_.to_pretty(tab, is_last),

            StatementKind::Enum(enum_) => enum_.to_pretty(tab, is_last),
            StatementKind::Union(union) => union.to_pretty(tab, is_last),
            StatementKind::TypeEnum(type_enum) => type_enum.to_pretty(tab, is_last),

            StatementKind::Implement(implement) => implement.to_pretty(tab, is_last),

            StatementKind::CloseBlock => format!("{}CloseBlock\n{}", prefix, tree_next_line_prefix(tab)),
        }
    }
}

impl PrettyString for Implement {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        format!(
            "{}Implement >> use {}{}\n{}",
            prefix,
            self.ty.to_string(),
            self.impl_trait.as_ref().map(|el| format!("impl {}{}", el.name, el.generics.to_string())).unwrap_or(String::new()),
            self.block.to_pretty(tab+1, is_last),
        )
    }
}

impl PrettyString for TypeEnum {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        format!(
            "{}TypeEnum >> {} typeof[{}]",
            prefix,
            self.name,
            self.body.types.iter().map(|el| el.to_string()).join(", "),
        )
    }
}

impl PrettyString for Union {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let prefix2 = tree_prefix(tab+1, is_last);
        format!(
            "{}Union >> {}{}\n{}",
            prefix,
            self.name,
            self.generics.to_string(),
            self.variants.iter().map(|el| format!("{}{}", prefix2, el.node.to_string())).join("\n")
        )
    }
}

impl ToString for UnionVariant {
    fn to_string(&self) -> String {            
        format!(
            "{}{}", 
            self.name, 
            match &self.field {
                UnionVariantKind::Tuple(tuple) => format!("({})", tuple.iter().map(|el| el.to_string()).join(",") ),
                UnionVariantKind::NamedTuple(named_tuple) => format!("({})", named_tuple.iter().map(|(key, value)| format!("{}: {}", key.0, value.to_string())).join(",") ),
            }
        )
    }
}

impl PrettyString for Enum {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        format!(
            "{}Enum >> {}\n{}",
            prefix,
            self.name,
            self.variants.to_pretty(tab+1, is_last)
        )
    }
}

impl PrettyString for EnumVariantKind {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            EnumVariantKind::Int(enum_variants) => enum_variants.iter().map(|el| format!("{}{} = {}", prefix, el.name, el.value)).join("\n"),
            EnumVariantKind::Expression(enum_variants) => enum_variants.iter().map(|el| format!("{}{} = {}", prefix, el.name, el.value.to_string())).join("\n"),
        }
    }
}

impl PrettyString for Trait {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let prefix2 = tree_prefix(tab+1, is_last);
        format!(
            "{}Struct >> {}{}\n{}",
            prefix,
            self.signature.name,
            self.signature.generics.to_string(),
            self.methodes.iter().map(|el| format!("{}Methode >>{}", prefix2, el.node.to_string())).join("\n"),
        )
    }
}

impl PrettyString for Struct {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let prefix2 = tree_prefix(tab+1, is_last);
        format!(
            "{}Struct >> {}{}\n{}",
            prefix,
            self.name,
            self.generics.to_string(),
            self.fields.iter().map(|el| format!("{}Field >>{}", prefix2, el.node.to_string())).join("\n"),
        )
    }
}

impl PrettyString for Class {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let prefix2 = tree_prefix(tab+1, is_last);
        format!(
            "{}Class >> {}{}\n{}{}",
            prefix,
            self.name,
            self.generics.to_string(),
            self.fields.iter().map(|el| format!("{}Field >>{}", prefix2, el.node.to_string())).join("\n"),
            self.methodes.iter().map(|el| format!("{}Methode >>{}", prefix2, el.node.to_pretty(tab+1, is_last))).join("\n"),
        )
    }
}

impl ToString for Field {
    fn to_string(&self) -> String {
        format!(
            "{} {}{}{}",
            self.ty.to_string(),
            self.name,
            self.vis.to_string(),
            self.default_value.as_ref().map(|el| format!(" = {}", el.to_string())).unwrap_or(String::new()),
        )
    }
}

impl ToString for FieldAccess {
    fn to_string(&self) -> String {
        if self.get.is_none() && self.set.is_none() {
            String::new()
        }
        else {
            format!(
                "{{{}{}}}",
                self.get.as_ref().map(|el| if let Visibility::Public = el {"Get "} else {"get "}).unwrap_or(""),
                self.set.as_ref().map(|el| if let Visibility::Public = el {"Set "} else {"set "}).unwrap_or(""),
            )
        }
    }
}

impl PrettyString for Function {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        format!(
            "{}Function >> {}\n{}",
            prefix,
            self.signature.to_string(),
            self.block.to_pretty(tab + 1, is_last)
        )
    }
}

impl ToString for ScopeKind {
    fn to_string(&self) -> String {
        match self {
            ScopeKind::Class(value) => format!("class >> {}{}", value.name, value.generics.to_string()),
            ScopeKind::Trait(value) => format!("trait >> {}{}", value.signature.name, value.signature.generics.to_string()),
            ScopeKind::Struct(value) => format!("struct >> {}{}", value.name, value.generics.to_string()),

            ScopeKind::Variable(value) => format!("Variable >> {} {}{}", value.ty.to_string(), value.name, value.initialize_value.as_ref().map(|el| format!(" = {}", el.node.to_string())).unwrap_or(String::new())),
            ScopeKind::Functions(values) => format!("Functions >> [{}]", values.iter().map(|func| func.signature.to_string()).join(", ")),

            ScopeKind::Enum(value) => format!("enum >> {}", value.name),
            ScopeKind::Union(value) => format!("union >> {}", value.name),
            ScopeKind::TypeEnum(value) => format!(
                "typeEnum >> {} [{}]", 
                value.name, 
                value.body.types.iter().map(|el| el.to_string()).join(", "),
            ),

            ScopeKind::Type(soul_type) => format!("Type >> {} ", soul_type.to_string()),
            ScopeKind::TypeDef{new_type, of_type} => format!("TypeDef >> {} typeof {}", new_type.to_string(), of_type.to_string()),
        }  
    }
}

impl ToString for FunctionSignature {
    fn to_string(&self) -> String {
        format!(
            "{}{}{}({}) {}",
            self.callee.as_ref().map(|el| format!("{}{}", el.node.extention_type.to_string(), el.node.this.as_ref().map(|el| format!(" this{} ", el.wrappers.to_string())).unwrap_or(String::new()) )).unwrap_or(String::new()),
            self.name,
            self.generics.to_string(),
            self.parameters.to_string(),
            self.return_type.as_ref().unwrap_or(&SoulType::none()).to_string()
        )
    }
}

impl ToString for Vec<Spanned<Parameter>> {
    fn to_string(&self) -> String {
        self.iter()
            .map(|param| format!(
                "{} {}", 
                param.node.ty.to_string(), 
                param.node.name, 
            ))
            .join(", ")
    }
}

impl ToString for Vec<TypeWrapper> {
    fn to_string(&self) -> String {
        self.iter()
            .map(|wrap| wrap.to_string())
            .join("")
    }
}

impl ToString for Vec<GenericParameter> {
    fn to_string(&self) -> String {
        if self.is_empty() {
            String::new()
        } 
        else {
            format!(
                "<{}>", 
                self.iter()
                    .map(|el| el.to_string())
                    .join(", ")
            )
        }
    }
}

impl ToString for Tuple {
    fn to_string(&self) -> String {
        self.values.iter()
            .map(|el| el.to_string())
            .join(", ")
    }
}

impl ToString for NamedTuple {
    fn to_string(&self) -> String {
        self.values.iter()
            .map(|(name, value)| format!("{}: {}", name, value.to_string()))
            .join(", ")
    }
}

impl PrettyString for LambdaBody {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        match self {
            LambdaBody::Block(block) => format!("\n{}", block.to_pretty(tab, is_last)),
            LambdaBody::Expression(spanned) => spanned.to_pretty(tab, is_last),
        }
    }
}

impl PrettyString for Expression {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        self.node.to_pretty(tab, is_last)
    }
}

impl PrettyString for ExpressionKind {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        match self {
            ExpressionKind::Empty => "<empty>".into(),
            ExpressionKind::Default => "<default>".into(),
            ExpressionKind::Literal(literal) => literal.to_string(),

            ExpressionKind::Index(Index{collection, index}) => format!("{}[{}]", collection.to_pretty(tab + 1, is_last), index.to_pretty(tab + 1, is_last)),
            ExpressionKind::Lambda(Lambda{signature, arguments, body, capture:_}) => format!("{}({}) => {}", signature.mode.get_lambda_name(), arguments.to_string(), body.to_pretty(tab + 1, is_last)),
            ExpressionKind::Constructor(Constructor{calle, arguments}) => format!("{}(|ctor|{})", calle.to_string(), arguments.to_string()),
            ExpressionKind::FunctionCall(FunctionCall{name, callee, generics, arguments}) => format!("{}{}{}({})", callee.as_ref().map(|el| format!("{}.",el.to_string())).unwrap_or(String::new()), name, generic_to_string(generics), arguments.to_string()),

            ExpressionKind::AccessField(AccessField{object, field}) => format!("{}.{}", object.to_pretty(tab + 1, is_last), field.name),
            ExpressionKind::StaticField(StaticField{object, field}) => format!("{}.{}", object.to_string(), field.name),
            ExpressionKind::StaticMethod(StaticMethod{callee, name, generics, arguments}) => format!("{}.{}{}({})", callee.node.to_string(), name, generic_to_string(generics), arguments.to_string()),

            ExpressionKind::UnwrapVariable(unwrap_variable) => match unwrap_variable {
                UnwrapVariable::Variable(variable_name) => variable_name.name.0.clone(),
                UnwrapVariable::MultiVariable{vars, ty, initializer} => format!("{}({}){}", ty.to_string(), vars.iter().map(|el| &el.name.0).join(", "), initializer.as_ref().map(|el| format!(" = {}", el.to_string())).unwrap_or(String::new()) ),
            },
            ExpressionKind::ExternalExpression(ExternalExpression{path, expr}) => format!("{}::{}", path.0, expr.to_pretty(tab + 1, is_last)),

            ExpressionKind::Unary(Unary{operator, expression}) => format!("{}{}", operator.node.to_str(), expression.to_pretty(tab + 1, is_last)),
            ExpressionKind::Binary(Binary{left, operator, right}) => format!("{} {} {}", left.to_pretty(tab + 1, is_last), operator.node.to_str(), right.to_pretty(tab + 1, is_last)),

            ExpressionKind::If(if_) => if_.to_pretty(tab, is_last),
            ExpressionKind::For(For{element, collection, block}) => format!("for {}{}\n{}", element.as_ref().map(|el| format!("{} in ", el.to_string())).unwrap_or("".into()), collection.to_pretty(tab + 1, is_last), block.to_pretty(tab+1, is_last)),
            ExpressionKind::While(While{condition, block}) => format!("while {}\n{}", condition.as_ref().map(|el| el.node.to_pretty(tab + 1, is_last)).unwrap_or("true".into()), block.to_pretty(tab+1, is_last)),
            ExpressionKind::Match(Match{condition, cases}) => format!("match {}\n{}{}", condition.to_pretty(tab, is_last), tree_prefix(tab+1, is_last), cases.iter().map(|el| format!("{} => \n{}", el.if_expr.to_string(), el.do_fn.to_pretty(tab, is_last))).join(format!("\n{}", tree_prefix(tab+1, is_last)).as_str()) ),
            ExpressionKind::Ternary(Ternary{condition, if_branch, else_branch}) => format!("{} ? {} : {}", condition.to_pretty(tab, is_last), if_branch.to_pretty(tab, is_last), else_branch.to_pretty(tab, is_last)),

            ExpressionKind::Deref(spanned) => format!("*{}", spanned.to_pretty(tab, is_last)),
            ExpressionKind::MutRef(spanned) => format!("&{}", spanned.to_pretty(tab, is_last)),
            ExpressionKind::ConstRef(spanned) => format!("@{}", spanned.to_pretty(tab, is_last)),

            ExpressionKind::Block(block) => block.to_pretty(tab, is_last),
            ExpressionKind::ReturnLike(return_like) => format!("{} {} >> free[{}]", return_like.kind.to_str(), return_like.value.as_ref().map(|el| el.to_string()).unwrap_or(String::new()), return_like.delete_list.iter().join(", ")),
            ExpressionKind::ExpressionGroup(expression_group) => expression_group.to_string(),
            ExpressionKind::Variable(var_name) => var_name.name.0.clone(),
        }
    }
}

impl PrettyString for CaseDoKind { 
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        match self {
            CaseDoKind::Block(block) => block.node.to_pretty(tab+2, is_last),
            CaseDoKind::Expression(spanned) => format!("{}{}", tree_prefix(tab+2, is_last), spanned.to_pretty(tab+2, is_last)),
        }
    }
}

fn generic_to_string(types: &Vec<SoulType>) -> String {
    if types.is_empty() {
        "".into()
    }
    else {
        format!("<{}>", types.iter().map(|el| el.to_string()).join(", "))
    }
}

impl PrettyString for If {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab+1, is_last);
        format!(
            "if {}\n{}{}", 
            self.condition.to_string(),
            self.block.to_pretty(tab + 1, is_last),
            self.else_branchs.iter().map(|el| match &el.node {
                ElseKind::ElseIf(if_) => format!("\n{}else {}", prefix, if_.node.to_pretty(tab, is_last)),
                ElseKind::Else(else_) => format!("{}else\n{}", prefix, else_.node.to_pretty(tab+1, is_last)),
            }).join("\n"),
        )
    }
} 

impl ToString for Expression {
    fn to_string(&self) -> String {
        self.node.to_string()
    }
}

impl ToString for ExpressionKind {
    fn to_string(&self) -> String {
        match self {
            ExpressionKind::Empty => "<empty>".into(),
            ExpressionKind::Default => "<default>".into(),
            ExpressionKind::Literal(literal) => literal.to_string(),
            
            ExpressionKind::Index(Index{collection, index}) => format!("{}[{}]", collection.to_string(), index.to_string()),
            ExpressionKind::Lambda(Lambda{signature, arguments, body:_, capture:_}) => format!("{}({})", signature.mode.get_lambda_name(), arguments.to_string()),
            ExpressionKind::Constructor(Constructor{calle, arguments}) => format!("{}(|ctor|{})", calle.to_string(), arguments.to_string()),
            ExpressionKind::FunctionCall(FunctionCall{name, callee, generics, arguments}) => format!("{}{}{}({})", callee.as_ref().map(|el| format!("{}.",el.to_string())).unwrap_or(String::new()), name, generic_to_string(generics), arguments.to_string()),
            
            ExpressionKind::AccessField(AccessField{object, field}) => format!("{}.{}", object.to_string(), field.name),
            ExpressionKind::StaticField(StaticField{object, field}) => format!("{}.{}", object.to_string(), field.name),
            ExpressionKind::StaticMethod(StaticMethod{callee, name, generics, arguments}) => format!("{}.{}{}({})", callee.node.to_string(), name, generic_to_string(generics), arguments.to_string()),
            
            ExpressionKind::UnwrapVariable(unwrap_variable) => match unwrap_variable {
                UnwrapVariable::Variable(variable_name) => variable_name.name.0.clone(),
                UnwrapVariable::MultiVariable{vars, ty, initializer} => format!("{}({}){}", ty.to_string(), vars.iter().map(|el| &el.name.0).join(", "), initializer.as_ref().map(|el| format!(" = {}", el.to_string())).unwrap_or(String::new()) ),
            },
            ExpressionKind::ExternalExpression(ExternalExpression{path, expr}) => format!("{}::{}", path.0, expr.to_string()),
            
            ExpressionKind::Unary(Unary{operator, expression}) => format!("{}{}", operator.node.to_str(), expression.to_string()),
            ExpressionKind::Binary(Binary{left, operator, right}) => format!("{} {} {}", left.to_string(), operator.node.to_str(), right.to_string()),
            
            ExpressionKind::If(If{condition, block:_, else_branchs}) => format!(
                "if {}, {}", 
                condition.to_string(),
                else_branchs.iter().map(|el| match &el.node {
                    ElseKind::ElseIf(if_) => format!("else if {}", if_.node.condition.to_string()),
                    ElseKind::Else(_) => "else".into(),
                }).join(", "),
            ),
            ExpressionKind::For(For{element, collection, block:_}) => format!("for {}{}", element.as_ref().map(|el| format!("{} in ", el.to_string())).unwrap_or("".into()), collection.to_string()),
            ExpressionKind::While(While{condition, block:_}) => format!("while {}", condition.as_ref().map(|el| el.node.to_string()).unwrap_or("true".into())),
            ExpressionKind::Match(Match{condition, cases:_}) => format!("match {}", condition.to_string()),
            ExpressionKind::Ternary(Ternary{condition, if_branch, else_branch}) => format!("{} ? {} : {}", condition.to_string(), if_branch.to_string(), else_branch.to_string()),
            
            ExpressionKind::Deref(spanned) => format!("*{}", spanned.to_string()),
            ExpressionKind::MutRef(spanned) => format!("&{}", spanned.to_string()),
            ExpressionKind::ConstRef(spanned) => format!("@{}", spanned.to_string()),
            
            ExpressionKind::Block(_) => "BlockExpression".into(),
            ExpressionKind::ReturnLike(return_like) => format!("{} {} >> free[{}]", return_like.kind.to_str(), return_like.value.as_ref().map(|el| el.to_string()).unwrap_or(String::new()), return_like.delete_list.iter().join(", ")),
            ExpressionKind::ExpressionGroup(expression_group) => expression_group.to_string(),
            ExpressionKind::Variable(var_name) => var_name.name.0.clone(),
        }
    }
}

impl ToString for ExpressionGroup {
    fn to_string(&self) -> String {
        match self {
            ExpressionGroup::Tuple(tuple) => tuple.to_string(),
            ExpressionGroup::NamedTuple(named_tuple) => named_tuple.to_string(),
            ExpressionGroup::Array(array) => format!(
                "{}[{}{}]", 
                array.collection_type.as_ref().map(|el| el.to_string()).unwrap_or(String::new()),
                array.element_type.as_ref().map(|el| format!("{}: ", el.to_string())).unwrap_or(String::new()),
                array.values.iter().map(|el| el.to_string()).join(", "),
            ),
            ExpressionGroup::ArrayFiller(array_filler) => format!(
                "{}[{}for {}{} => {}]",
                array_filler.collection_type.as_ref().map(|el| el.to_string()).unwrap_or(String::new()),
                array_filler.element_type.as_ref().map(|el| format!("{}: ", el.to_string())).unwrap_or(String::new()),
                array_filler.index.as_ref().map(|el| format!("{} in ", el.name)).unwrap_or(String::new()),
                array_filler.amount.to_string(),
                array_filler.fill_expr.to_string(),
            ),
        }
    }
}

impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Int(num) => format!("{}", num),
            Literal::Uint(num) => format!("{}", num),
            Literal::Float(num) => format!("{}", num),
            
            Literal::Bool(bool_) => format!("{}", bool_),
            
            Literal::Char(char_) => format!("{}", char_),
            Literal::Str(string) => format!("{}", string),
            
            Literal::Tuple{values} => format!("({})", values.iter().map(|el| el.to_string()).join(", ")),
            Literal::Array{ty:_, values} => format!("[{}]", values.iter().map(|el| el.to_string()).join(", ")),
            Literal::NamedTuple{values} => format!("({})", values.iter().map(|el| format!("{}: {}", el.0, el.1.to_string())).join(", ")),
            
            Literal::ProgramMemmory(ident, _) => format!("{}", ident),
        }
    }
}

fn tree_prefix(tab: usize, is_last: bool) -> String {
    if tab == 0 {
        return String::new();
    }

    let mut prefix = String::new();

    for _ in 0..tab - 1 {
        prefix.push_str("│   ");
    }

    prefix.push_str(if is_last { "└── " } else { "├── " });

    prefix
}

fn tree_next_line_prefix(indent: usize) -> String {
    if indent == 0 {
        return String::new();
    }

    let mut prefix = String::new();

    for _ in 0..indent {
        prefix.push_str("│   ");
    }

    prefix
}











































