use crate::{errors::soul_error::{new_soul_error, SoulErrorKind, SoulSpan}, steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::{AbstractSyntacTree, GlobalKind, GlobalNode}, expression::{Array, ArrayFiller, BinaryExpr, ExprKind, Expression, ExternalExpression, FnCall, Index, LambdaDecl, StaticField, StaticMethode, Ternary, TypeOfExpr}, soul_type::{soul_type::SoulType, type_kind::TypeKind}, spanned::Spanned, staments::{conditionals::{CaseDoKind, ElseKind, ForDecl, IfDecl, SwitchDecl, WhileDecl}, enum_likes::{UnionDeclRef, UnionVariantKind}, function::{ExtFnDecl, FnDecl, FnDeclKind, FunctionSignatureRef}, objects::{ClassDeclRef, StructDeclRef, TraitDeclRef, TraitImpl}, statment::{Assignment, Block, ReturnLike, Statment, StmtKind, VariableKind, VariableRef}}}, external_header::Header, scope::ScopeKind}, i_sementic::{ast_visitors::{AstVisitable, ExternalHeaderAnalyser}, fault::SoulFault}}, utils::node_ref::MultiRefPool};

impl AstVisitable for ExternalHeaderAnalyser {
    fn visit_ast(&mut self, node: &mut AbstractSyntacTree) {
        
        let mut temp_faults = vec![];
        for scope in self.get_scope().get_types() {
            
            for ty in scope.symbols.values() {
                if let Some(fault) = self.check_type_kind(ty, None) {
                    temp_faults.push(fault);
                }
            }
        }

        self.extent_faults(temp_faults, true);

        for node in node.root.iter_mut() {
            self.visit_global_node(node);
        }

    }

    fn visit_global_node(&mut self, node: &mut GlobalNode) {
        
        match &mut node.node {
            GlobalKind::ClassDecl(node_ref) => self.check_class_ref(node_ref, node.span),
            GlobalKind::StructDecl(node_ref) => self.check_struct_ref(node_ref, node.span),
            GlobalKind::TraitDecl(node_ref) => self.check_trait_ref(node_ref, node.span),
            GlobalKind::TraitImpl(trait_impl) => self.check_trait_impl(trait_impl, node.span),
            GlobalKind::FuncDecl(FnDecl{signature, body}) => {
                self.check_fn_signature(signature);
                self.visit_block(body);
            },
            GlobalKind::ExtFuncDecl(ExtFnDecl{signature, body}) => {
                self.check_fn_signature(signature);
                self.visit_block(body);
            },
            GlobalKind::VarDecl(variable_kind) => self.check_var_kind(variable_kind, node.span),
            GlobalKind::UnionDecl(node_ref) => self.check_union_decl(node_ref, node.span),
            GlobalKind::EnumDecl(_) => (),
            GlobalKind::TypeEnumDecl(node_ref) => {
                for ty in node_ref.borrow(&self.get_scope().ref_pool).types.iter() {
                    self.check_type(&ty, node.span);
                }
            },
        }

    }

    fn visit_expression(&mut self, node: &mut Expression) {
        self.check_expr_kind(&mut node.node, node.span)
    }

    fn visit_statment(&mut self, node: &mut Statment) {
        
        match &mut node.node {
            StmtKind::EnumDecl(_) => (),
            StmtKind::Block(block) => self.visit_block(block),
            StmtKind::If(if_decl) => self.check_if(if_decl, node.span),
            StmtKind::ExprStmt(spanned) => self.visit_expression(spanned),
            StmtKind::For(for_decl) => self.check_for(for_decl, node.span),
            StmtKind::While(while_decl) => self.check_while(while_decl, node.span),
            StmtKind::ClassDecl(node_ref) => self.check_class_ref(node_ref, node.span),
            StmtKind::TraitDecl(node_ref) => self.check_trait_ref(node_ref, node.span),
            StmtKind::UnionDecl(node_ref) => self.check_union_decl(node_ref, node.span),
            StmtKind::Return(return_like) => self.check_return(return_like, node.span),
            StmtKind::Switch(switch_decl) => self.check_switch(switch_decl, node.span),
            StmtKind::StructDecl(node_ref) => self.check_struct_ref(node_ref, node.span),
            StmtKind::TraitImpl(trait_impl) => self.check_trait_impl(trait_impl, node.span),
            StmtKind::VarDecl(variable_kind) => self.check_var_kind(variable_kind, node.span),
            StmtKind::Assignment(assignment) => self.check_assignment(assignment, node.span),
            StmtKind::CloseBlock(_) => (),
            StmtKind::TypeEnumDecl(node_ref) => {
                for ty in node_ref.borrow(&self.get_scope().ref_pool).types.iter() {
                    self.check_type(&ty, node.span);
                }
            },
            StmtKind::ExtFnDecl(ext_fn_decl) => {
                self.check_fn_signature(&mut ext_fn_decl.signature);
                self.visit_block(&mut ext_fn_decl.body);
            },
            StmtKind::FnDecl(fn_decl) => {
                self.check_fn_signature(&mut fn_decl.signature);
                self.visit_block(&mut fn_decl.body);
            },
        }
    }

    fn visit_block(&mut self, node: &mut Block) {
        
        for statment in &mut node.statments {
            self.visit_statment(statment);
        }
    }

}

impl ExternalHeaderAnalyser {

    fn check_assignment(&mut self, node: &mut Assignment, _span: SoulSpan) {
        self.visit_expression(&mut node.target);
        self.visit_expression(&mut node.value);
    }

    fn check_switch(&mut self, node: &mut SwitchDecl, _span: SoulSpan) {
        
        self.visit_expression(&mut node.condition);
        for case in &mut node.cases {
            match &mut case.do_fn {
                CaseDoKind::Block(block) => self.visit_block(block),
                CaseDoKind::Expression(spanned) => self.visit_expression(spanned),
            }

            self.visit_expression(&mut case.if_expr);
        }
    }

    fn check_return(&mut self, node: &mut ReturnLike, _span: SoulSpan) {
        if let Some(expr) = &mut node.value {
            self.visit_expression(expr);
        }
    }

    fn check_while(&mut self, node: &mut WhileDecl, _span: SoulSpan) {
        if let Some(expr) = &mut node.condition {
            self.visit_expression(expr);
        }

        self.visit_block(&mut node.body);
    }

    fn check_for(&mut self, node: &mut ForDecl, _span: SoulSpan) {
        self.visit_expression(&mut node.collection);
        self.visit_block(&mut node.body);
    }

    fn check_expr_kind(&mut self, node: &mut ExprKind, span: SoulSpan) {
        
        match node {
            ExprKind::Empty => (),
            ExprKind::Default => (),
            ExprKind::Literal(_) => (),
            ExprKind::Variable(_) => (),
            ExprKind::If(if_decl, _) => self.check_if(if_decl, span),
            ExprKind::Index(index) => self.check_index(index, span),
            ExprKind::Ctor(fn_call) => self.check_fn_call(fn_call, span),
            ExprKind::Call(fn_call) => self.check_fn_call(fn_call, span),
            ExprKind::Array(array) => self.check_expr_array(array, span),
            ExprKind::Ternary(ternary) => self.check_ternary(ternary, span),
            ExprKind::Field(field) => self.visit_expression(&mut field.object),
            ExprKind::Lambda(lambda_decl) => self.lambda_decl(lambda_decl, span),
            ExprKind::Binary(binary_expr) => self.check_binary(binary_expr, span),
            ExprKind::TypeOf(type_of_expr) => self.check_typeof(type_of_expr, span),
            ExprKind::Deref(spanned) => self.check_expr_kind(&mut spanned.node, span),
            ExprKind::MutRef(spanned) => self.check_expr_kind(&mut spanned.node, span),
            ExprKind::ConstRef(spanned) => self.check_expr_kind(&mut spanned.node, span),
            ExprKind::Unary(unary_expr) => self.visit_expression(&mut unary_expr.expression),
            ExprKind::StaticField(static_field) => self.check_static_field(static_field, span),
            ExprKind::UnwrapVarDecl(variable_kind) => self.check_var_kind(variable_kind, span),
            ExprKind::ArrayFiller(array_filler) => self.check_array_filler(array_filler, span),
            ExprKind::StaticMethode(static_methode) => self.check_static_methode(static_methode, span),
            ExprKind::ExternalExpression(external_expression) => self.check_external_expression(external_expression, span),
            ExprKind::Tuple(tuple) => {
                for expr in tuple.values.iter_mut() {
                    self.visit_expression(expr);
                }
            },
            ExprKind::NamedTuple(named_tuple) => {
                for (_, expr) in named_tuple.values.iter_mut() {
                    self.visit_expression(expr);
                }
            },
        }
    }

    fn check_index(&mut self, node: &mut Index, _span: SoulSpan) {
        self.visit_expression(&mut node.index);
        self.visit_expression(&mut node.collection);
    }

    fn check_typeof(&mut self, node: &mut TypeOfExpr, span: SoulSpan) {
        self.check_type(&node.ty, span);
        self.visit_expression(&mut node.left);
    }

    fn check_binary(&mut self, node: &mut BinaryExpr, _span: SoulSpan) {
        self.visit_expression(&mut node.left);
        self.visit_expression(&mut node.right);
    }

    fn check_static_field(&mut self, node: &mut StaticField, span: SoulSpan) {
        self.check_type(&node.object.node, span);
    }

    fn check_static_methode(&mut self, node: &mut StaticMethode, span: SoulSpan) {
        self.check_type(&node.callee.node, span);
        for arg in &mut node.arguments {
            self.visit_expression(&mut arg.expression);
        } 

        for ty in &node.generics {
            self.check_type(ty, span);
        }
    }

    fn check_fn_call(&mut self, node: &mut FnCall, span: SoulSpan) {
        if let Some(calle) = &mut node.callee {
            self.visit_expression(calle);
        }

        for ty in &node.generics {
            self.check_type(ty, span);
        }

        for arg in &mut node.arguments {
            self.visit_expression(&mut arg.expression);
        }
    }

    fn check_external_expression(&mut self, node: &mut ExternalExpression, span: SoulSpan) {

        let header = match self.get_scope().external_header.store.get(&node.path) {
            Some(val) => val,
            None => {
                self.add_fault(SoulFault::new_error(new_soul_error(SoulErrorKind::InvalidPath, span, format!("path: '{}' could not be found", node.path.0)))); 
                return;
            },
        };


        if let Err(msg) = header_contains(&header, &self.get_scope().ref_pool, &node.expr.node) {
            
            self.add_fault(SoulFault::new_error(new_soul_error(SoulErrorKind::InvalidInContext, span, format!("{} with path: '{}'", msg, node.path.0))));
            return;
        }
    }

    fn lambda_decl(&mut self, node: &mut LambdaDecl, span: SoulSpan) {
        self.visit_block(&mut node.body);
        self.check_variable(&mut node.capture.variable, span);
        
        for arg in &mut node.arguments {
            self.visit_expression(arg);
        }

        let mut lambda_sig = node.signature.borrow_mut(&mut self.get_scope_mut().ref_pool);
        for param in &mut lambda_sig.params {
            
            self.check_type(&param.node.ty, param.span);
            if let Some(expr) = &mut param.node.default_value {
                self.visit_expression(expr);
            }
        }
    }

    fn check_if(&mut self, node: &mut IfDecl, _span: SoulSpan) {
        self.visit_expression(&mut node.condition);
        self.visit_block(&mut node.body);
        for branch in &mut node.else_branchs {
            match &mut branch.node {
                ElseKind::ElseIf(spanned) => self.check_if(&mut spanned.node, spanned.span),
                ElseKind::Else(spanned) => self.visit_block(&mut spanned.node),
            }
        }
    }

    fn check_ternary(&mut self, node: &mut Ternary, _span: SoulSpan) {
        self.visit_expression(&mut node.condition);
        self.visit_expression(&mut node.else_branch);
        self.visit_expression(&mut node.if_branch);
    }

    fn check_expr_array(&mut self, node: &mut Array, span: SoulSpan) {
        if let Some(ty) = &node.collection_type {
            self.check_type(ty, span);
        }

        if let Some(ty) = &node.element_type {
            self.check_type(ty, span);
        }

        for expr in &mut node.values {
            self.visit_expression(expr);
        }
    }

    fn check_array_filler(&mut self, node: &mut ArrayFiller, span: SoulSpan) {
        self.visit_expression(&mut node.amount);
        self.visit_expression(&mut node.fill_expr);
        if let Some(expr) = &mut node.index {
            self.check_variable(expr, span);
        }
    }

    fn check_union_decl(&mut self, node: &mut UnionDeclRef, span: SoulSpan) {
        let union = node.borrow(&self.get_scope().ref_pool);

        for variant in &union.variants {
            
            match &variant.field {
                UnionVariantKind::Tuple(soul_types) => {
                    for ty in soul_types {
                        self.check_type(ty, span);
                    }
                },
                UnionVariantKind::NamedTuple(hash_map) => {
                    for (_, ty) in hash_map {
                        self.check_type(ty, span);
                    }
                },
            }
        }   
    }

    fn check_var_kind(&mut self, node: &mut VariableKind, span: SoulSpan) {
        
        match node {
            VariableKind::Variable(node_ref) => self.check_variable(node_ref, span),
            VariableKind::MultiVariable { vars, ty, initializer, lit_retention:_ } => {
                for (_, var) in vars {
                    self.check_variable(var, span);
                }

                self.check_type(ty, span);
                if let Some(init) = initializer {
                    self.visit_expression(init);
                }
            },
        }
    } 

    fn check_variable(&mut self, node: &mut VariableRef, span: SoulSpan) {
        let mut variable = node.borrow_mut(&mut self.get_scope_mut().ref_pool);
        self.check_type(&variable.ty, span);
        
        if let Some(init) = &mut variable.initializer {
            self.visit_expression(init);
        }

        //dont need to check literal_retention
    }

    fn check_trait_impl(&mut self, node: &mut TraitImpl, span: SoulSpan) {
        self.check_type(&node.for_type, span);

        for meth in node.methodes.iter_mut() {

            self.check_fn_kind(meth);
        }
    }

    fn check_trait_ref(&mut self, node: &mut TraitDeclRef, _span: SoulSpan) {
        let mut trait_decl = node.borrow_mut(&mut self.get_scope_mut().ref_pool);

        for meth in trait_decl.methodes.iter_mut() {

            self.check_fn_signature(meth);
        }
    }

    fn check_struct_ref(&mut self, node: &mut StructDeclRef, _span: SoulSpan) {
        let mut struct_decl = node.borrow_mut(&mut self.get_scope_mut().ref_pool);
 
        for field in struct_decl.fields.iter_mut() {

            self.check_type(&field.node.ty, field.span);
            if let Some(expr) = &mut field.node.default_value {
                self.visit_expression(expr);
            }
        }
    }

    fn check_class_ref(&mut self, node: &mut ClassDeclRef, _span: SoulSpan) {
        let mut class_decl = node.borrow_mut(&mut self.get_scope_mut().ref_pool);
        
 
        for field in class_decl.fields.iter_mut() {

            self.check_type(&field.node.ty, field.span);
            if let Some(expr) = &mut field.node.default_value {
                self.visit_expression(expr);
            }
        }

        for meth in class_decl.methodes.iter_mut() {

            self.check_fn_kind(meth);
        }
    }

    fn check_fn_kind(&mut self, fn_kind: &mut Spanned<FnDeclKind>) {
        
        let (mut signature, body) = match &mut fn_kind.node {
            FnDeclKind::Fn(FnDecl{signature, body}) => (signature, body),
            FnDeclKind::Ctor(FnDecl{signature, body}) => (signature, body),
            FnDeclKind::ExtFn(ExtFnDecl{signature, body}) => (signature, body),
            
            FnDeclKind::InternalFn(_) => return,
            FnDeclKind::InternalCtor(_) => return,
        };

        self.check_fn_signature(&mut signature);
        self.visit_block(body);
    }

    fn check_fn_signature(&mut self, fn_sig: &mut FunctionSignatureRef) {
        let mut signature = fn_sig.borrow_mut(&mut self.get_scope_mut().ref_pool);
        
        if let Some(calle) = &signature.node.calle {
            self.check_type(&calle.node.ty, calle.span);
        }

        for param in signature.node.params.iter_mut() {

            self.check_type(&param.node.ty, param.span);
            if let Some(expr) = &mut param.node.default_value {
                self.visit_expression(expr);
            }
        } 

        if let Some(ty) = &signature.node.return_type {
            self.check_type(ty, signature.span);
        }
    }

    fn check_type(&mut self, ty: &SoulType, span: SoulSpan) {

        if let Some(fault) = self.check_type_kind(&ty.base, Some(span)) {
            self.add_fault(fault);
        }
    }

    fn check_type_kind(&self, ty: &TypeKind, span: Option<SoulSpan>) -> Option<SoulFault> {

        if let TypeKind::ExternalType(external_type) = &ty {

            let span = if let Some(val) = span {val} else {external_type.span};
            let header = match self.get_scope().external_header.store.get(&external_type.node.path) {
                Some(val) => val,
                None => {
                    return Some(SoulFault::new_error(new_soul_error(SoulErrorKind::InvalidPath, span, format!("path: '{}' could not be found", external_type.node.path.0)))); 
                },
            };

            
            if !header.types.contains_key(&external_type.node.name.0) && !header.scope.contains_key(&external_type.node.name.0)  {
                return Some(SoulFault::new_error(new_soul_error(SoulErrorKind::InvalidInContext, span, format!("'{}' does not exist in path: '{}'", external_type.node.name.0, external_type.node.path.0))));
            }
        }
        None
    }
}

fn header_contains(header: &Header, ref_pool: &MultiRefPool, expr: &ExprKind) -> Result<(), String> {

    let expr_name = match &expr {
        ExprKind::Call(fn_call) => &fn_call.name.0,
        ExprKind::Ctor(fn_call) => &fn_call.name.0,
        ExprKind::Variable(variable) => &variable.name.0,
        _ => return Err(format!("expression: '{}' can not be external expression", expr.get_variant_name())),
    }; 

    if expr_name.contains("::") {
        let mut splits = expr_name.split("::");
        let (union_name, variant_name) = (splits.next().unwrap(), splits.next().unwrap());
        let kinds = header.scope.get(union_name)
            .ok_or(format!("union is not in external page"))?;

        let union = kinds.into_iter().find(|el| matches!(el, ScopeKind::Union(_)));

        let has_variant = match union {
            Some(ScopeKind::Union(node_ref)) => node_ref.borrow(ref_pool).variants.iter().any(|variant| variant.name.0 == variant_name),
            _ => return Err(format!("found type: '{}' but type is not union", union_name))
        };

        if !has_variant {
            return Err(format!("variant: '{}' is not a part of union: '{}'", variant_name, union_name))
        }

        return Ok(())
    }

    let kinds = header.scope.get(expr_name)
        .ok_or(format!("expression is not in external page"))?;

    let (correct_type, kind_name) = match expr {
        ExprKind::Ctor(_) => (kinds.iter().any(|kind| matches!(kind, ScopeKind::Functions(_))), "Ctor"),
        ExprKind::Call(_) => (kinds.iter().any(|kind| matches!(kind, ScopeKind::Functions(_))), "function"),
        ExprKind::Variable(_) => (kinds.iter().any(|kind| matches!(kind, ScopeKind::Variable(_))), "variable"),
        _ => unreachable!(),
    };

    if !correct_type {
        return Err(format!("{} is not in external page but {} of same name is", expr.get_variant_name(), kind_name));
    }

    Ok(())
}
















