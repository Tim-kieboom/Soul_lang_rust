use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::AbstractSyntacTree, enum_like::EnumVariantKind, expression::{AccessField, CaseDoKind, ElseKind, Expression, ExpressionGroup, ExpressionKind, If, StaticField, UnwrapVariable, VariableName}, function::LambdaBody, object::{ClassChild, Field}, soul_type::{soul_type::SoulType, type_kind::TypeKind}, spanned::Spanned, statement::{Block, Statement, StatementKind}}, scope_builder::ScopeKind}, i_sementic::{ast_visitor::{AstAnalyser, NameResolutionAnalyser}, scope_vistitor::ScopeVisitor, soul_fault::SoulFault}}};

impl AstAnalyser for NameResolutionAnalyser {
    
    fn analyse_ast(&mut self, tree: &mut AbstractSyntacTree) {
        
        self.analyse_block(&mut tree.root);
    }
    
    fn consume(self) -> (ScopeVisitor, Vec<SoulFault>, bool) {
        self.consume_to_tuple()
    }
}

impl NameResolutionAnalyser {
    
    fn analyse_statment(&mut self, statment: &mut Statement) {

        let parent_id = self.get_scope().current_id();
        if let Some(id) = statment.node.get_scope_id() {

            if self.get_scope_mut().set_current(id).is_none() {
                panic!("could not get scope_id({}) at line: {}:{}, file: {}", id.0, statment.span.line_number, statment.span.line_offset, self.get_scope().file_path.to_string_lossy())
            }
        }

        match &mut statment.node {
            StatementKind::Trait(_) => (),
            StatementKind::Union(_) => (),
            StatementKind::CloseBlock => (),
            StatementKind::TypeEnum(_) => (),
            StatementKind::Variable(_) => (),
            
            StatementKind::Enum(enum_) => {

                match &mut enum_.variants {
                    EnumVariantKind::Int(_) => (),
                    EnumVariantKind::Expression(enum_variants) =>  {
                        
                        for variant in enum_variants {
                            self.analyse_expression(&mut variant.value);
                        }
                    },
                }
            },
            StatementKind::Struct(struct_) => {

                for field in &mut struct_.fields {
                    self.analyse_field(&mut field.node)
                }
            },
            StatementKind::Class(class) => {

                for child in &mut class.children {
                    match child {
                        ClassChild::Field(field) => self.analyse_field(&mut field.node),
                        ClassChild::Methode(methode) => self.analyse_block(&mut methode.node.block),
                        ClassChild::ImplBlock(impl_block) => self.analyse_block(&mut impl_block.node.block),
                    }
                }
            },
            StatementKind::Function(function) => self.analyse_block(&mut function.block),
            StatementKind::Expression(spanned) => self.analyse_expression(spanned),
            StatementKind::UseBlock(use_block) => self.analyse_block(&mut use_block.block),
            StatementKind::Assignment(assignment) => {
                self.analyse_expression(&mut assignment.variable);
                self.analyse_expression(&mut assignment.value);
            }
        }

        self.get_scope_mut()
            .set_current(parent_id)
            .expect("scope_id should be valid");
    }

    fn analyse_expression(&mut self, expression: &mut Expression) {
        if let Err(fault) = self.try_analyse_expression(expression) {
            self.add_fault(fault);
        }
    }

    fn try_analyse_expression(&mut self, expression: &mut Expression) -> Result<(), SoulFault> {
       
        let parent_id = self.get_scope().current_id();
        if let Some(id) = expression.node.get_scope_id() {

            if self.get_scope_mut().set_current(id).is_none() {
                panic!("could not get scope_id({}) at line: {}:{}, file: {}", id.0, expression.span.line_number, expression.span.line_offset, self.get_scope().file_path.to_string_lossy())
            }
        }

        match &mut expression.node {
            ExpressionKind::Empty => (),
            ExpressionKind::Default => (),
            ExpressionKind::Literal(_) => (),
            ExpressionKind::StaticField(_) => (),
            ExpressionKind::ExternalExpression(_) => (),
            
            ExpressionKind::Index(index) => {
                self.try_analyse_expression(&mut index.collection)?;
                self.try_analyse_expression(&mut index.index)?;
            },
            ExpressionKind::Lambda(lambda) => {
                match &mut lambda.body {
                    LambdaBody::Block(block) => self.analyse_block(block),
                    LambdaBody::Expression(spanned) => self.try_analyse_expression(spanned)?,
                }
            },
            ExpressionKind::FunctionCall(function_call) => {
                
                for argument in &mut function_call.arguments.values {
                    self.try_analyse_expression(argument)?;
                }
            },
            ExpressionKind::StructConstructor(struct_constructor) => {
                
                for (_, expression) in &mut struct_constructor.arguments.values {
                    self.try_analyse_expression(expression)?;
                }
            },
            ExpressionKind::AccessField(access_field) => {
                if let Err(fault) = self.try_analyse_expression(&mut access_field.object) {
                    
                    let AccessField{object, field} = std::mem::take(access_field);
                    if let Spanned{node: ExpressionKind::Variable(variable), span} = *object {
                        *expression = Expression::new(
                            ExpressionKind::StaticField(StaticField{object: SoulType::from_type_kind(TypeKind::Unknown(variable.name)), field}),
                            span
                        );
                    }
                    else {
                        return Err(fault)
                    }
                }
            },
            ExpressionKind::StaticMethod(static_method) => {

                for argument in &mut static_method.arguments.values {
                    self.try_analyse_expression(argument)?;
                }
            },
            ExpressionKind::Variable(variable_name) => {
                self.check_variable(variable_name, expression.span)?;
            },
            ExpressionKind::UnwrapVariable(unwrap_variable) => {
                match unwrap_variable {
                    UnwrapVariable::Variable(variable_name) => {
                        self.check_variable(variable_name, expression.span)?;
                    },
                    UnwrapVariable::MultiVariable{vars, ty:_, initializer} => {
                        for variable_name in vars {
                            self.check_variable(variable_name, expression.span)?;
                        }
                        
                        if let Some(expression) = initializer {
                            self.try_analyse_expression(expression)?;
                        }
                    },
                }
            },
            ExpressionKind::Unary(unary) => {
                self.try_analyse_expression(&mut unary.expression)?;
            },
            ExpressionKind::Binary(binary) => {
                self.try_analyse_expression(&mut binary.left)?;
                self.try_analyse_expression(&mut binary.right)?;
            },
            ExpressionKind::If(if_decl) => self.analyse_if(if_decl)?,
            ExpressionKind::For(for_decl) => {
                self.try_analyse_expression(&mut for_decl.collection)?;
                self.analyse_block(&mut for_decl.block);
            },
            ExpressionKind::While(while_decl) => {
                if let Some(condition) = &mut while_decl.condition {
                    self.try_analyse_expression(condition)?;
                }
                self.analyse_block(&mut while_decl.block);
            },
            ExpressionKind::Match(match_decl) => {
                self.try_analyse_expression(&mut match_decl.condition)?;
                for case in &mut match_decl.cases {

                    self.try_analyse_expression(&mut case.if_expr)?;
                    match &mut case.do_fn {
                        CaseDoKind::Block(spanned) => {
                            self.analyse_block(&mut spanned.node);
                        }
                        CaseDoKind::Expression(spanned) => self.try_analyse_expression(spanned)?,
                    }
                }
            },
            ExpressionKind::Ternary(ternary) => {
                self.try_analyse_expression(&mut ternary.condition)?;
                self.try_analyse_expression(&mut ternary.if_branch)?;
                self.try_analyse_expression(&mut ternary.else_branch)?;
            },
            ExpressionKind::Deref(spanned) => {
                self.try_analyse_expression(spanned)?;
            },
            ExpressionKind::MutRef(spanned) => {
                self.try_analyse_expression(spanned)?;
            },
            ExpressionKind::ConstRef(spanned) => {
                self.try_analyse_expression(spanned)?;
            },
            ExpressionKind::Block(block) => {
                self.analyse_block(block);
            },
            ExpressionKind::ReturnLike(return_like) => {
                if let Some(expression) = &mut return_like.value {
                    self.try_analyse_expression(expression)?;
                }
            },
            ExpressionKind::ExpressionGroup(expression_group) => {
                match expression_group {
                    ExpressionGroup::Tuple(tuple) => {
                        
                        for expression in &mut tuple.values {
                            self.try_analyse_expression(expression)?;
                        } 
                    },
                    ExpressionGroup::Array(array) => {
                        
                        for expression in &mut array.values {
                            self.try_analyse_expression(expression)?;
                        } 
                    },
                    ExpressionGroup::NamedTuple(named_tuple) =>{
                        
                        for (_, expression) in &mut named_tuple.values {
                            self.try_analyse_expression(expression)?;
                        } 
                    },
                    ExpressionGroup::ArrayFiller(array_filler) =>{
                        
                        self.try_analyse_expression(&mut array_filler.amount)?;
                        self.try_analyse_expression(&mut array_filler.fill_expr)?;
                    },
                }
            },
        }

        self.get_scope_mut()
            .set_current(parent_id)
            .expect("scope_id should be valid");

        Ok(())
    }


    fn analyse_if(&mut self, if_decl: &mut If) -> Result<(), SoulFault> {
        self.try_analyse_expression(&mut if_decl.condition)?;
        self.analyse_block(&mut if_decl.block);

        for branch in &mut if_decl.else_branchs {
            
            match &mut branch.node {
                ElseKind::Else(spanned) => self.analyse_block(&mut spanned.node),
                ElseKind::ElseIf(spanned) => self.analyse_if(&mut spanned.node)?,
            }
        }

        Ok(())
    }

    fn check_variable(&mut self, variable_name: &VariableName, span: SoulSpan) -> Result<(), SoulFault> {
        if !self.get_scope_mut()
            .lookup(&variable_name.name.0)
            .is_some_and(|kinds| kinds.iter().any(|el| matches!(el.node, ScopeKind::Variable(_))))
        {
            Err(
                SoulFault::new_error(new_soul_error(SoulErrorKind::InvalidName, Some(span), format!("variable: '{}' not found in scope", variable_name.name)))
            )
        }
        else {
            Ok(())
        }
    }

    fn analyse_field(&mut self, field: &mut Field) {
        if let Some(expression) = &mut field.default_value {
            self.analyse_expression(expression);
        }
    }

    fn analyse_block(&mut self, block: &mut Block) {
        
        for statment in &mut block.statments {
            self.analyse_statment(statment);
        }
    }


}




