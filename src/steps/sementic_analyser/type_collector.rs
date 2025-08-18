use std::collections::HashMap;
use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::{AbstractSyntacTree, GlobalKind, GlobalNode}, expression::{Arguments, Array, ExprKind, Expression, FnCall, Ident, StaticMethode, UnaryOpKind, Variable}, soul_type::{soul_type::SoulType, type_kind::{Indexed, Meth, TraitKind, TypeKind, TypeWrapper, UnionKind}}, spanned::Spanned, staments::{conditionals::{CaseDoKind, ElseKind}, enum_likes::{UnionDeclRef, UnionVariantKind}, function::FnDeclKind, objects::FieldDecl, statment::{Assignment, Block, Statment, StmtKind, VariableKind, VariableRef}}}, external_header::Header, scope::{NamedTupleCtor, ScopeKind, SoulPagePath}}, i_sementic::{ast_visitors::{AstVisitable, TypeCollector}, fault::SoulFault, sementic_scope::ScopeVisitor}}};

impl AstVisitable for TypeCollector {
    fn visit_ast(&mut self, node: &mut AbstractSyntacTree) {

        for node in node.root.iter_mut() {
            self.visit_global_node(node);
        }
    }

    fn visit_global_node(&mut self, node: &mut GlobalNode) {
        
        match &mut node.node {
            GlobalKind::ClassDecl(multi_ref) => {
                let mut class_decl = multi_ref.borrow_mut(&mut self.get_scope_mut().ref_pool);

                let class_ty = SoulType::from_type_kind(TypeKind::Class(class_decl.name.clone()));
                get_named_tuple_ctor(&class_decl.name, class_ty, &class_decl.fields, self.get_scope_mut());
                
                self.get_scope_mut().next_child();
                for methode in &mut class_decl.methodes {
                    self.visit_block(methode.node.get_body_mut());
                }
                self.get_scope_mut().to_parent();
            },
            GlobalKind::TraitDecl(_) => (),
            GlobalKind::StructDecl(struct_ref) => {
                let struct_decl = struct_ref.borrow(&self.get_scope().ref_pool);
                let class_ty = SoulType::from_type_kind(TypeKind::Class(struct_decl.name.clone()));
                get_named_tuple_ctor(&struct_decl.name, class_ty, &struct_decl.fields, self.get_scope_mut());
            },
            GlobalKind::TraitImpl(trait_impl) => {

                self.get_scope_mut().next_child();
                for methode in &mut trait_impl.methodes {
                    self.visit_block(methode.node.get_body_mut());
                }
                self.get_scope_mut().to_parent();
            },
            GlobalKind::VarDecl(variable_kind) => self.check_var_decl(variable_kind, node.span),
            GlobalKind::EnumDecl(_) |
            GlobalKind::UnionDecl(_) => {
                self.get_scope_mut().next_child();
                self.get_scope_mut().to_parent();
            },
            GlobalKind::TypeEnumDecl(_) => (),
            GlobalKind::FuncDecl(fn_decl) => self.visit_block(&mut fn_decl.body),
            GlobalKind::ExtFuncDecl(ext_fn_decl) => self.visit_block(&mut ext_fn_decl.body),
        }
    }

    fn visit_expression(&mut self, _node: &mut Expression) {}
    
    fn visit_statment(&mut self, node: &mut Statment) {
        
        match &mut node.node {
            StmtKind::Return(_) => (),
            StmtKind::TraitDecl(_) => (),
            StmtKind::CloseBlock(_) => (),
            StmtKind::TypeEnumDecl(_) => (),
            StmtKind::Block(block) => self.visit_block(block),
            StmtKind::ExprStmt(spanned) => self.visit_expression(spanned),
            StmtKind::For(for_decl) => self.visit_block(&mut for_decl.body),
            StmtKind::FnDecl(fn_decl) => self.visit_block(&mut fn_decl.body),
            StmtKind::While(while_decl) => self.visit_block(&mut while_decl.body),
            StmtKind::ExtFnDecl(ext_fn_decl) => self.visit_block(&mut ext_fn_decl.body),
            StmtKind::Assignment(assignment) => self.check_assignment(assignment, node.span),
            StmtKind::VarDecl(variable_kind) => self.check_var_decl(variable_kind, node.span),

            StmtKind::EnumDecl(_) |
            StmtKind::UnionDecl(_) => {
                self.get_scope_mut().next_child();
                self.get_scope_mut().to_parent();
            },
            StmtKind::StructDecl(struct_ref) => {
                let struct_decl = struct_ref.borrow(&self.get_scope().ref_pool);
                let class_ty = SoulType::from_type_kind(TypeKind::Class(struct_decl.name.clone()));
                get_named_tuple_ctor(&struct_decl.name, class_ty, &struct_decl.fields, self.get_scope_mut());
            },
            StmtKind::ClassDecl(multi_ref) => {
                let mut class_decl = multi_ref.borrow_mut(&mut self.get_scope_mut().ref_pool);
        
                let class_ty = SoulType::from_type_kind(TypeKind::Class(class_decl.name.clone()));
                get_named_tuple_ctor(&class_decl.name, class_ty, &class_decl.fields, self.get_scope_mut());
                
                self.get_scope_mut().next_child();
                for methode in &mut class_decl.methodes {
                    self.visit_block(methode.node.get_body_mut());
                }
                self.get_scope_mut().to_parent();
            },
            StmtKind::TraitImpl(trait_impl) => {

                self.get_scope_mut().next_child();
                for methode in &mut trait_impl.methodes {
                    self.visit_block(methode.node.get_body_mut());
                }
                self.get_scope_mut().to_parent();
            },
            StmtKind::If(if_decl) => {
                self.visit_block(&mut if_decl.body);

                for branch in &mut if_decl.else_branchs {

                    match &mut branch.node {
                        ElseKind::Else(spanned) => self.visit_block(&mut spanned.node),
                        ElseKind::ElseIf(spanned) => self.visit_block(&mut spanned.node.body),
                    }
                }
            },
            StmtKind::Switch(switch_decl) => {
                
                self.get_scope_mut().next_child();
                for case in &mut switch_decl.cases {
                    
                    match &mut case.do_fn {
                        CaseDoKind::Block(block) => self.visit_block(block),
                        CaseDoKind::Expression(spanned) => {
                            self.get_scope_mut().next_child();
                            self.visit_expression(spanned);
                            self.get_scope_mut().to_parent();
                        },
                    }
                }
                self.get_scope_mut().to_parent();
            },
        }
    }

    fn visit_block(&mut self, node: &mut Block) {
        
        self.get_scope_mut().next_child();
        for statment in &mut node.statments {
            self.visit_statment(statment);
        }
        self.get_scope_mut().to_parent();
    }

}

impl TypeCollector {

    fn check_var_decl(&mut self, variable_kind: &mut VariableKind, span: SoulSpan) {
        match variable_kind {
            VariableKind::Variable(multi_ref) => {
                if multi_ref.borrow(&self.get_scope().ref_pool).ty.is_none_type() {
                    
                    let ty = if let Some(init) = &multi_ref.borrow(&self.get_scope().ref_pool).initializer {
                        
                        match try_get_exprkind_ty(&init.node, self.get_scope()) {
                            Ok(val) => match val {
                                Some(ty) => ty,
                                None => SoulType::none(),
                            },
                            Err(err) => {
                                self.add_fault(SoulFault::new_error(new_soul_error(SoulErrorKind::InvalidInContext, span, err)));
                                return;
                            }
                        }
                    }
                    else {
                        SoulType::none()
                    };

                    multi_ref.borrow_mut(&mut self.get_scope_mut().ref_pool).ty = ty;
                }
            },
            VariableKind::MultiVariable{..} => todo!(),
        }
    }

    fn check_assignment(&mut self, assignment: &mut Assignment, span: SoulSpan) {

        fn try_get_var_kind(this: &mut TypeCollector, variable: &Variable) -> Option<VariableRef> {
            let kind = this.get_scope_mut()
                .lookup(&variable.name.0)?
                .iter()
                .find(|kind| matches!(kind, ScopeKind::Variable(_)))
                .cloned()?;

            match kind {
                ScopeKind::Variable(var_ref) => Some(var_ref),
                _ => unreachable!(),
            }
        }
                
        match &assignment.target.node {

            ExprKind::Variable(variable) => {

                let mut var_ref = match try_get_var_kind(self, variable) {
                    Some(val) => val,
                    None => {
                        self.add_fault(SoulFault::new_error(new_soul_error(SoulErrorKind::InternalError, span, format!("could not find variable: '{}'", variable.name.0))));
                        return;
                    },
                };

                if !var_ref.borrow(&self.get_scope().ref_pool).ty.is_none_type() {
                    return;
                }

                if var_ref.borrow(&self.get_scope().ref_pool).ty.is_none_type() { 
                    var_ref.borrow_mut(&mut self.get_scope_mut().ref_pool).ty = match try_get_exprkind_ty(&assignment.value.node, self.get_scope()) {
                        Ok(val) => match val {
                            Some(ty) => ty,
                            None => {
                                self.add_fault(SoulFault::new_warning(new_soul_error(SoulErrorKind::InvalidType, span, "type of assignment is none")));
                                SoulType::none()
                            }
                        },
                        Err(err) => {
                            self.add_fault(SoulFault::new_warning(new_soul_error(SoulErrorKind::InvalidType, span, format!("while trying to get type of assignment {}", err))));
                            SoulType::none()
                        },
                    };
                };
            },
            _ => (),
        }
    }
}

fn get_named_tuple_ctor(name: &Ident, object_type: SoulType, fields: &Vec<Spanned<FieldDecl>>, scope: &mut ScopeVisitor) {
    
    let values = fields.iter()
        .map(|field| (field.node.name.clone(), (field.node.ty.clone(), field.node.default_value.clone())))
        .collect::<HashMap<_, _>>();

    scope.insert(name.0.clone(), ScopeKind::NamedTupleCtor(NamedTupleCtor{object_type, values}));
}

fn try_get_exprkind_ty(expr: &ExprKind, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
    
    fn add_wrapper(spanned: &Box<Spanned<ExprKind>>, scope: &ScopeVisitor, wrap: TypeWrapper) -> Result<Option<SoulType>, String> {
        let mut ty = match try_get_exprkind_ty(&spanned.node, scope)? {
            Some(ty) => ty,
            None => return Err(format!("type: 'none' can not be derefed")),
        };

        ty.wrappers.push(wrap);
        Ok(Some(ty))
    }

    fn from_variable(variable: &Ident, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
        match scope.lookup(&variable.0)
            .ok_or(format!("var: '{}' not found", variable.0))?
            .iter()
            .find(|kind| matches!(kind, ScopeKind::Variable(_)))
            .ok_or(format!("var: '{}' not found", variable.0))? 
        {
            ScopeKind::Variable(multi_ref) => Ok(Some(multi_ref.borrow(&scope.ref_pool).ty.clone())),
            _ => unreachable!(),
        }
    }
    
    match expr {
        ExprKind::Empty => Ok(None),
        ExprKind::Default => Ok(None),
        ExprKind::Call(fn_call) => {
            
            try_get_fn_return_type(&fn_call, scope)
        },
        ExprKind::Index(index_expr) => {
            let index = Box::new(try_get_exprkind_ty(&index_expr.index.node, scope)?
                .ok_or("index has no type".to_string())?);
            let collection = Box::new(try_get_exprkind_ty(&index_expr.collection.node, scope)?
                .ok_or("collection of index has not type".to_string())?);

            Ok(Some(SoulType::from_type_kind(TypeKind::TraitType(TraitKind::Indexed(Indexed{collection, index})))))
        },
        ExprKind::Field(field) => {
            let object = try_get_exprkind_ty(&field.object.node, scope)?
                .ok_or("object has no type".to_string())?;

            field_type_from_type(&object.base, &field.field, scope)
        },
        ExprKind::Literal(literal) => Ok(Some(literal.to_soul_type())),
        ExprKind::Unary(unary_expr) => {
            match &unary_expr.operator.node {
                UnaryOpKind::Neg => return Ok(Some(SoulType::from_type_kind(TypeKind::Bool))),
                _ => (),
            }

            try_get_exprkind_ty(&unary_expr.expression.node, scope)
        },
        ExprKind::Variable(variable) => {
            from_variable(&variable.name, scope)
        },
        ExprKind::TypeOf(_) => Ok(Some(SoulType::from_type_kind(TypeKind::Bool))),
        ExprKind::Binary(binary_expr) => try_get_exprkind_ty(&binary_expr.left.node, scope),
        ExprKind::StaticField(static_field) => field_type_from_type(&static_field.object.node.base, &static_field.field, scope),
        ExprKind::StaticMethode(static_methode) => try_get_static_methode_return_type(&static_methode, scope),
        ExprKind::Ctor(fn_call) => try_get_fn_return_type(&fn_call, scope),
        ExprKind::UnwrapVarDecl(variable_kind) => {
            match variable_kind.as_ref() {
                VariableKind::Variable(multi_ref) => Ok(Some(multi_ref.borrow(&scope.ref_pool).ty.clone())),
                VariableKind::MultiVariable{ty, ..} => Ok(Some(ty.clone())),
            }
        },
        ExprKind::ExternalExpression(external_expression) => try_get_exprkind_ty(&external_expression.expr.node, scope),
        ExprKind::Lambda(lambda_decl) => Ok(lambda_decl.signature.borrow(&scope.ref_pool).return_type.clone()),
        ExprKind::If(_, ty) => Ok(Some(ty.clone())),
        ExprKind::Ternary(ternary) => try_get_exprkind_ty(&ternary.if_branch.node, scope),
        ExprKind::Deref(spanned) => {
            match try_get_exprkind_ty(&spanned.node, scope)? {
                Some(ty) => Ok(Some(ty.to_deref(&scope.ref_pool)?)),
                None => Err(format!("type: 'none' can not be derefed")),
            }
        },
        ExprKind::MutRef(spanned) => add_wrapper(spanned, scope, TypeWrapper::MutRef(None)),
        ExprKind::ConstRef(spanned) => add_wrapper(spanned, scope, TypeWrapper::ConstRef(None)),
        ExprKind::Array(Array{collection_type, element_type, values}) => {
            if collection_type.is_some() {
                Ok(collection_type.clone())
            }
            else if let Some(ty_) = element_type {
                let mut ty = ty_.clone();
                ty.wrappers.push(TypeWrapper::Array);
                Ok(Some(ty))
            }
            else if values.is_empty() {
                Ok(Some(SoulType::none()))
            }
            else {
                let mut ty = try_get_exprkind_ty(&values[0].node, scope)?
                    .ok_or(format!("could not get type of first expression in array"))?;
                ty.wrappers.push(TypeWrapper::Array);
                Ok(Some(ty))
            }
        },
        ExprKind::Tuple(tuple) => {
            let mut tuple_kind = vec![];
            for expr in &tuple.values {
                tuple_kind.push(
                    try_get_exprkind_ty(&expr.node, scope)?
                        .ok_or(format!("could not get type from expression"))?
                );
            }

            Ok(Some(SoulType::from_type_kind(TypeKind::Tuple(tuple_kind))))
        },
        ExprKind::NamedTuple(named_tuple) => {
            if let Some(ty) = &named_tuple.object_type {
                return Ok(Some(ty.clone()))
            }

            let mut tuple_kind = HashMap::new();
            for (name, expr) in &named_tuple.values {
                tuple_kind.insert(
                    name.clone(),
                    try_get_exprkind_ty(&expr.node, scope)?
                        .ok_or(format!("could not get type from expression: '{}'", expr.node.to_string(&scope.ref_pool, 0)))?
                );
            }

            Ok(Some(SoulType::from_type_kind(TypeKind::NamedTuple(tuple_kind))))
        },
        ExprKind::ArrayFiller(array_filler) => {
            let mut ty = try_get_exprkind_ty(&array_filler.fill_expr.node, scope)?
                .ok_or(format!("could not get type from expression: '{}'", array_filler.fill_expr.node.to_string(&scope.ref_pool, 0)))?;
            ty.wrappers.push(TypeWrapper::Array);
            Ok(Some(ty))
        },
    }
}

fn field_type_from_type(base: &TypeKind, field_var: &Variable, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
    
    match &base {
        TypeKind::Class(ident) |
        TypeKind::Struct(ident) |
        TypeKind::TypeDefed(ident) => get_ident_field_type(ident, field_var, scope),
        
        TypeKind::Function(multi_ref) => match &multi_ref.borrow(&scope.ref_pool).node.return_type {
            Some(ty) => field_type_from_type(&ty.base, field_var, scope),
            None => Err(format!("tryed to index from function: '{}', but function has not return type", multi_ref.borrow(&scope.ref_pool).node.name.0)),
        },
        
        TypeKind::Tuple(types) => {
            let idx = match field_var.name.0.parse::<usize>() {
                Ok(num) => num,
                Err(_) => return Err(format!("tuple types only have positive interger field names (so use for example 'tuple.0' not 'tuple.{}')", field_var.name.0)),
            };

            match types.get(idx) {
                Some(ty) => Ok(Some(ty.clone())),
                None => Err(format!("tuple field: '{}' is out of bounds only from 0 till {}", idx, types.len()-1)),
            }
        },
        TypeKind::NamedTuple(types) => {
            match types.get(&field_var.name) {
                Some(ty) => Ok(Some(ty.clone())),
                None => Err(format!("field: '{}' not found in namedTuple type", field_var.name.0)),
            }
        },
        TypeKind::UnionVariant(union_variant) => {
            
            let union_ref = match &union_variant.union {
                UnionKind::Union(ident) => {
                    get_union(ident, scope)
                        .ok_or(format!("union not found in scope"))?
                },
                UnionKind::External(external_type) => {
                    get_path_union(&external_type.path, &external_type.name, scope)?
                        .ok_or(format!("union not found in scope"))?
                },
            };

            let union = union_ref.borrow(&scope.ref_pool);
            let variant = &union.variants.iter().find(|var| var.name == union_variant.variant)
                .ok_or(format!("union variant: '{}' not found", union_variant.variant.0))?.field;
            
            match &variant {
                UnionVariantKind::Tuple(types) => {
                    let idx = match field_var.name.0.parse::<usize>() {
                        Ok(num) => num,
                        Err(_) => return Err(format!("tuple types only have positive interger field names (so use for example 'tuple.0' not 'tuple.{}')", field_var.name.0)),
                    };

                    match types.get(idx) {
                        Some(ty) => Ok(Some(ty.clone())),
                        None => Err(format!("tuple field: '{}' is out of bounds only from 0 till {}", idx, types.len()-1)),
                    }
                },
                UnionVariantKind::NamedTuple(types) => {
                    match types.get(&field_var.name) {
                        Some(ty) => Ok(Some(ty.clone())),
                        None => Err(format!("field: '{}' not found in namedTuple type", field_var.name.0)),
                    }
                },
            }
        },
        
        TypeKind::ExternalType(spanned) => {
            let header = match scope.external_header.store.get(&spanned.node.path) {
                Some(header) => header,
                None => return Err(format!("path: '{}' not found", spanned.node.path.0)),
            };

            for kind in header.scope.get(&spanned.node.path.0).ok_or(format!("path: '{}' not found", spanned.node.path.0))?.iter() {
                match kind {
                    ScopeKind::Enum(multi_ref) => return field_type_from_type(&TypeKind::Enum(multi_ref.borrow(&scope.ref_pool).name.clone()), field_var, scope),
                    ScopeKind::Class(multi_ref) => return field_type_from_type(&TypeKind::Class(multi_ref.borrow(&scope.ref_pool).name.clone()), field_var, scope),
                    ScopeKind::Union(multi_ref) => return field_type_from_type(&TypeKind::Union(multi_ref.borrow(&scope.ref_pool).name.clone()), field_var, scope),
                    ScopeKind::Trait(multi_ref) => return field_type_from_type(&TypeKind::Trait(multi_ref.borrow(&scope.ref_pool).name.clone()), field_var, scope),
                    ScopeKind::Struct(multi_ref) => return field_type_from_type(&TypeKind::Struct(multi_ref.borrow(&scope.ref_pool).name.clone()), field_var, scope),
                    ScopeKind::TypeEnum(multi_ref) => return field_type_from_type(&TypeKind::TypeEnum(multi_ref.borrow(&scope.ref_pool).name.clone(), multi_ref.borrow(&scope.ref_pool).types.clone()), field_var, scope),
                    ScopeKind::TypeDefed(multi_ref) => return field_type_from_type(&TypeKind::TypeDefed(multi_ref.borrow(&scope.ref_pool).name.clone()), field_var, scope),
                    _ => (),
                }
            }

            Err(format!("type not found"))
        },
        TypeKind::ExternalPath(_) => Ok(Some(SoulType::from_type_kind(base.clone()))),
        
        TypeKind::TraitType(kind) => {
            Ok(Some(SoulType::from_type_kind(TypeKind::TraitType(kind.clone()))))  
        },
        _ => Err(format!("typeKind: '{}' can not have fields", base.get_variant(&scope.ref_pool))),
    }
}

fn get_union(name: &Ident, scope: &ScopeVisitor) -> Option<UnionDeclRef> {
    match scope.lookup(&name.0)?.iter().find(|kind| matches!(kind, ScopeKind::Union(_)))? {
        ScopeKind::Union(multi_ref) => Some(multi_ref.clone()),
        _ => unreachable!(),
    }
}

fn get_path_union(path: &SoulPagePath, name: &Ident, scope: &ScopeVisitor) -> Result<Option<UnionDeclRef>, String> {

    let header = match scope.external_header.store.get(path) {
        Some(header) => header,
        None => return Err(format!("path: '{}' not found", path.0)),
    };

    fn get_union(name: &Ident, header: &Header) -> Option<UnionDeclRef> {
        match header.scope.get(&name.0)?.iter().find(|kind| matches!(kind, ScopeKind::Union(_)))? {
            ScopeKind::Union(multi_ref) => Some(multi_ref.clone()),
            _ => unreachable!(),
        }
    }

    Ok(get_union(name, header))
}


fn get_ident_field_type(ident: &Ident, field_var: &Variable, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
    let kinds = scope.lookup(&ident.0)
        .ok_or(format!("object type: '{}' is not found in scope", ident.0))?;

    let kind = kinds.iter().find(|kind| match kind {
        ScopeKind::Struct(_) |
        ScopeKind::Class(_) |
        ScopeKind::TypeDefed(_) => true, 
        _ => false,
    }).ok_or(format!("object type: '{}' is not found in scope", ident.0))?;

    match kind {
        ScopeKind::Class(multi_ref) => {
            let class_decl = multi_ref.borrow(&scope.ref_pool);
            let field = class_decl.fields.iter().find(|el| el.node.name == field_var.name)
                .ok_or(format!("field: '{}' does not exists", field_var.name.0))?;

            Ok(Some(field.node.ty.clone()))
        },
        ScopeKind::Struct(multi_ref) => {
            let struct_decl = multi_ref.borrow(&scope.ref_pool);
            let field = struct_decl.fields.iter().find(|el| el.node.name == field_var.name)
            .ok_or(format!("field: '{}' does not exists", field_var.name.0))?;
        
            Ok(Some(field.node.ty.clone()))
        },
        ScopeKind::TypeDefed(multi_ref) => {
            field_type_from_type(&multi_ref.borrow(&scope.ref_pool).from_type.base, field_var, scope)
        },
        _ => unreachable!(),
    }
}

fn try_get_fn_return_type(call: &FnCall, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
    
    let name = &call.name.0; 
    let callee = match call.callee.as_ref().map(|expr| try_get_exprkind_ty(&expr.node, scope)) {
        Some(expr) => match expr? {
            Some(ty) => Some(ty),
            None => None,
        },
        None => None,
    };
    inner_get_fn_return_type(&name, callee, &call.generics, &call.arguments, scope)
}

fn try_get_static_methode_return_type(call: &StaticMethode, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
    
    let name = &call.name.0; 
    let callee = Some(call.callee.node.clone());
    inner_get_fn_return_type(&name, callee, &call.generics, &call.arguments, scope)
}

fn inner_get_fn_return_type(name: &str, callee: Option<SoulType>, generics: &Vec<SoulType>, arguments: &Vec<Arguments>, scope: &ScopeVisitor) -> Result<Option<SoulType>, String> {
    
    fn eq_calle(func: &FnDeclKind, scope: &ScopeVisitor, calle: &Option<SoulType>) -> bool {
        func.get_signature().borrow(&scope.ref_pool).node.calle.as_ref().is_none_or(|this| calle.as_ref() == Some(&this.node.ty))
    }
    
    let func_with_same_calle = |kinds: &Vec<ScopeKind>, calle: &Option<SoulType>| kinds.iter().any(|kind| {
        
        match kind {
            ScopeKind::Functions(multi_ref) => multi_ref.borrow(&scope.ref_pool).iter().any(|func| eq_calle(func, scope, calle)),
            _ => false,
        }
    });
    
    match scope.lookup_fn(name, func_with_same_calle, &callee) {
        
        Some(funcs) => funcs.iter().flat_map(|kind| match kind {
                ScopeKind::Functions(multi_ref) => multi_ref.borrow(&scope.ref_pool)
                    .iter()
                    .find(|func| eq_calle(func, scope, &callee))
                    .map(|func| func.get_signature().borrow(&scope.ref_pool).node.return_type.clone()),
                _ => None,
            }).next().ok_or("function not found".into()),
        
        None => {
            if let Some(cal) = callee {
                let meth = Meth{
                    methode: Box::new(StaticMethode{
                        callee: Spanned::new(cal, SoulSpan::new(0,0,0)), 
                        name: Ident(name.to_string()), 
                        generics: generics.clone(), 
                        arguments: arguments.clone(),
                    })
                };
                Ok(Some(SoulType::from_type_kind(TypeKind::TraitType(TraitKind::Meth(meth)))))
            }
            else {
                Ok(None)
            }
        },
    }
}




























