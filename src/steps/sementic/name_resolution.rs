use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::AbstractSyntacTree, enum_like::EnumVariantKind, expression::{CaseDoKind, ElseKind, Expression, ExpressionGroup, ExpressionKind, If, UnwrapVariable, VariableName}, function::LambdaBody, object::{ClassChild, Field}, statement::{Block, Statement, StatementKind}}, scope_builder::ScopeKind}, i_sementic::{ast_visitor::{AstAnalyser, NameResolutionAnalyser}, scope_vistitor::ScopeVisitor, soul_fault::SoulFault}}};

impl AstAnalyser for NameResolutionAnalyser {
    
    fn analyse_ast(&mut self, tree: &mut AbstractSyntacTree) {
        
        self.analyse_block(&tree.root);
    }
    
    fn consume(self) -> (ScopeVisitor, Vec<SoulFault>, bool) {
        self.consume_to_tuple()
    }
}

impl NameResolutionAnalyser {
    
    fn analyse_statment(&mut self, statment: &Statement) {

        if statment.node.is_bodied_kind() {
            self.get_scope_mut().push()
                .expect(format!("could not push scope at line: {}:{}", statment.span.line_number, statment.span.line_offset).as_str());
        }

        match &statment.node {
            StatementKind::Trait(_) => (),
            StatementKind::Union(_) => (),
            StatementKind::CloseBlock => (),
            StatementKind::TypeEnum(_) => (),
            StatementKind::Variable(_) => (),
            
            StatementKind::Enum(enum_) => {

                match &enum_.variants {
                    EnumVariantKind::Int(_) => (),
                    EnumVariantKind::Expression(enum_variants) =>  {
                        
                        for variant in enum_variants {
                            self.analyse_expression(&variant.value);
                        }
                    },
                }
            },
            StatementKind::Struct(struct_) => {

                for field in &struct_.fields {
                    self.analyse_field(&field.node)
                }
            },
            StatementKind::Class(class) => {

                for child in &class.children {
                    match child {
                        ClassChild::Field(field) => self.analyse_field(&field.node),
                        ClassChild::Methode(methode) => self.analyse_block(&methode.node.block),
                        ClassChild::ImplBlock(impl_block) => self.analyse_block(&impl_block.node.block),
                    }
                }
            },
            StatementKind::Function(function) => self.analyse_block(&function.block),
            StatementKind::Expression(spanned) => self.analyse_expression(spanned),
            StatementKind::UseBlock(use_block) => self.analyse_block(&use_block.block),
            StatementKind::Assignment(assignment) => {
                self.analyse_expression(&assignment.variable);
                self.analyse_expression(&assignment.value);
            }
        }

        if statment.node.is_bodied_kind() {
            self.get_scope_mut().pop()
                .expect(format!("could not pop scope at line: {}:{}", statment.span.line_number, statment.span.line_offset).as_str());
        }
    }

    fn analyse_expression(&mut self, expression: &Expression) {
        
        if expression.node.is_bodied_kind() {
            self.get_scope_mut().push()
                .expect(format!("could not push scope at line: {}:{}", expression.span.line_number, expression.span.line_offset).as_str());
        }

        match &expression.node {
            ExpressionKind::Empty => (),
            ExpressionKind::Default => (),
            ExpressionKind::Literal(_) => (),
            ExpressionKind::StaticField(_) => (),
            ExpressionKind::ExternalExpression(_) => (),
            
            ExpressionKind::Index(index) => {
                self.analyse_expression(&index.collection);
                self.analyse_expression(&index.index);
            },
            ExpressionKind::Lambda(lambda) => {
                match &lambda.body {
                    LambdaBody::Block(block) => self.analyse_block(block),
                    LambdaBody::Expression(spanned) => self.analyse_expression(spanned),
                }
            },
            ExpressionKind::FunctionCall(function_call) => {
                
                for argument in &function_call.arguments.values {
                    self.analyse_expression(argument);
                }
            },
            ExpressionKind::StructConstructor(struct_constructor) => {
                
                for (_, expression) in &struct_constructor.arguments.values {
                    self.analyse_expression(expression);
                }
            },
            ExpressionKind::AccessField(access_field) => {
                self.analyse_expression(&access_field.object);
            },
            ExpressionKind::StaticMethod(static_method) => {

                for argument in &static_method.arguments.values {
                    self.analyse_expression(argument);
                }
            },
            ExpressionKind::Variable(variable_name) => {
                self.check_variable(variable_name, expression.span);
            },
            ExpressionKind::UnwrapVariable(unwrap_variable) => {
                match unwrap_variable {
                    UnwrapVariable::Variable(variable_name) => {
                        self.check_variable(variable_name, expression.span);
                    },
                    UnwrapVariable::MultiVariable{vars, ty:_, initializer} => {
                        for variable_name in vars {
                            self.check_variable(variable_name, expression.span);
                        }
                        
                        if let Some(expression) = initializer {
                            self.analyse_expression(&expression);
                        }
                    },
                }
            },
            ExpressionKind::Unary(unary) => {
                self.analyse_expression(&unary.expression);
            },
            ExpressionKind::Binary(binary) => {
                self.analyse_expression(&binary.left);
                self.analyse_expression(&binary.right);
            },
            ExpressionKind::If(if_decl) => self.analyse_if(if_decl),
            ExpressionKind::For(for_decl) => {
                self.analyse_expression(&for_decl.collection);
                self.analyse_block(&for_decl.block);
            },
            ExpressionKind::While(while_decl) => {
                if let Some(condition) = &while_decl.condition {
                    self.analyse_expression(condition);
                }
                self.analyse_block(&while_decl.block);
            },
            ExpressionKind::Match(match_decl) => {
                self.analyse_expression(&match_decl.condition);
                for case in &match_decl.cases {

                    self.analyse_expression(&case.if_expr);
                    match &case.do_fn {
                        CaseDoKind::Block(spanned) => self.analyse_block(&spanned.node),
                        CaseDoKind::Expression(spanned) => self.analyse_expression(spanned),
                    }
                }
            },
            ExpressionKind::Ternary(ternary) => {
                self.analyse_expression(&ternary.condition);
                self.analyse_expression(&ternary.if_branch);
                self.analyse_expression(&ternary.else_branch);
            },
            ExpressionKind::Deref(spanned) => {
                self.analyse_expression(spanned);
            },
            ExpressionKind::MutRef(spanned) => {
                self.analyse_expression(spanned);
            },
            ExpressionKind::ConstRef(spanned) => {
                self.analyse_expression(spanned);
            },
            ExpressionKind::Block(block) => {
                self.analyse_block(block);
            },
            ExpressionKind::ReturnLike(return_like) => {
                if let Some(expression) = &return_like.value {
                    self.analyse_expression(expression);
                }
            },
            ExpressionKind::ExpressionGroup(expression_group) => {
                match expression_group {
                    ExpressionGroup::Tuple(tuple) => {
                        
                        for expression in &tuple.values {
                            self.analyse_expression(expression);
                        } 
                    },
                    ExpressionGroup::Array(array) => {
                        
                        for expression in &array.values {
                            self.analyse_expression(expression);
                        } 
                    },
                    ExpressionGroup::NamedTuple(named_tuple) =>{
                        
                        for (_, expression) in &named_tuple.values {
                            self.analyse_expression(expression);
                        } 
                    },
                    ExpressionGroup::ArrayFiller(array_filler) =>{
                        
                        self.analyse_expression(&array_filler.amount);
                        self.analyse_expression(&array_filler.fill_expr );
                    },
                }
            },
        }

        if expression.node.is_bodied_kind() {
            self.get_scope_mut().pop()
                .expect(format!("could not pop scope at line: {}:{}", expression.span.line_number, expression.span.line_offset).as_str());
        }
    }

    fn analyse_if(&mut self, if_decl: &If) {
        self.analyse_expression(&if_decl.condition);
        self.analyse_block(&if_decl.block);

        for branch in &if_decl.else_branchs {
            
            match &branch.node {
                ElseKind::Else(spanned) => self.analyse_block(&spanned.node),
                ElseKind::ElseIf(spanned) => self.analyse_if(&spanned.node),
            }
        }
    }

    fn check_variable(&mut self, variable_name: &VariableName, span: SoulSpan) {
        if !self.get_scope_mut()
            .lookup(&variable_name.name.0)
            .is_some_and(|kinds| kinds.iter().any(|el| matches!(el.node, ScopeKind::Variable(_))))
        {
            self.add_error(
                new_soul_error(SoulErrorKind::InvalidName, Some(span), format!("variable: '{}' not found in scope", variable_name.name))
            );
        }
    }

    fn analyse_field(&mut self, field: &Field) {
        if let Some(expression) = &field.default_value {
            self.analyse_expression(expression);
        }
    }

    fn analyse_block(&mut self, block: &Block) {
        
        for statment in block.statments.iter() {
            self.analyse_statment(statment);
        }
    }


}




