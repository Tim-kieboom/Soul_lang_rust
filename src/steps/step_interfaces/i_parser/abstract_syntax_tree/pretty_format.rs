use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{Arguments, ExprKind, Expression, Ident, TypeOfExpr}, soul_type::{Modifier, SoulType, TypeKind, UnionVariantKind}, statment::{ClassDecl, EnumDecl, FieldDecl, FunctionDecl, FunctionSignature, ImplBlock, InterfaceDecl, Parameter, Statment, StmtKind, StructDecl, TraitDecl, TypeEnumDecl, UnionDecl, VariableDecl}};

pub trait PrettyFormat {
    fn to_pretty_string(&self, tab: usize) -> String;
}

impl PrettyFormat for Expression {
    fn to_pretty_string(&self, tab: usize) -> String {
        let indent = |tab| "  ".repeat(tab);
        let base_indent = indent(tab);
        let child_indent = indent(tab + 1);

        match &self.node {
            ExprKind::Literal(lit) => format!("Literal({:?})", lit),
            ExprKind::Variable(Ident(name)) => format!("Variable({})", name),
            ExprKind::Binary { left, operator, right } => format!(
                                "BinaryOp({:?},{}{}{}{})",
                                operator.node,
                                child_indent,
                                left.to_pretty_string(tab + 1),
                                child_indent,
                                right.to_pretty_string(tab + 1)
                            ),
            ExprKind::Unary { operator, expression } => format!(
                                "UnaryOp({:?},{}{})",
                                operator.node,
                                child_indent,
                                expression.to_pretty_string(tab + 1)
                            ),
            ExprKind::Call { callee, arguments } => {
                                let args_str = arguments
                                    .iter()
                                    .map(|arg| arg.to_pretty_string(tab + 2))
                                    .collect::<Vec<_>>()
                                    .join(&format!(",\n{}", indent(tab + 2)));
                                format!(
                                    "Call(\n{}callee: {},\n{}args: [\n{}{}\n{}]\n{})",
                                    child_indent,
                                    callee.to_pretty_string(tab + 1),
                                    child_indent,
                                    indent(tab + 2),
                                    args_str,
                                    child_indent,
                                    base_indent,
                                )
                            },
            ExprKind::Index { collection, index } => {            
                                format!(
                                    "Call(\n{}collection: {},\n{}index: [{}]\n{})",
                                    child_indent,
                                    collection.to_pretty_string(tab + 1),
                                    child_indent,
                                    index.to_pretty_string(tab + 1),
                                    base_indent,
                                )
                            },
            ExprKind::TypeOf(type_of_expr) => {            
                                format!(
                                    "TypeOf(\n{}{}\n{})",
                                    child_indent,
                                    type_of_expr.to_pretty_string(tab+1),
                                    base_indent,
                                )
                            },
        }
    }
}

impl PrettyFormat for Arguments {
    fn to_pretty_string(&self, tab: usize) -> String {
        let indent = "  ".repeat(tab);
        if let Some(Ident(name)) = &self.name {
            format!("{}{}: {}", indent, name, self.expression.to_pretty_string(tab + 1))
        } else {
            format!("{}{}", indent, self.expression.to_pretty_string(tab))
        }
    }
}

impl PrettyFormat for Statment {
    fn to_pretty_string(&self, tab: usize) -> String {
        let indent = |tab| "  ".repeat(tab);
        let base_indent = indent(tab);
        let child_indent = indent(tab + 1);

        match &self.node {
            StmtKind::ExprStmt(expr) => format!("ExprStmt(\n{}{}\n{})", child_indent, expr.to_pretty_string(tab + 1), base_indent),
            StmtKind::VarDecl(decl) => var_decl_pretty_string(decl, tab),
            StmtKind::FnDecl(func) => function_decl_pretty_string(func, tab),
            StmtKind::Return(expr_opt) => match expr_opt {
                                        Some(expr) => format!("Return(\n{}{}\n{})", child_indent, expr.to_pretty_string(tab + 1), base_indent),
                                        None => "Return(None)".to_string(),
                                    },
            StmtKind::If { condition, then_branch, else_branch } => {
                                        let then_str = then_branch.iter()
                                            .map(|s| s.to_pretty_string(tab + 2))
                                            .collect::<Vec<_>>()
                                            .join(&format!("\n{}", indent(tab + 2)));
                                        let else_str = else_branch.as_ref().map(|branch| {
                                            branch.iter()
                                                .map(|s| s.to_pretty_string(tab + 2))
                                                .collect::<Vec<_>>()
                                                .join(&format!("\n{}", indent(tab + 2)))
                                        });
                                        let else_block = else_str.map_or("".to_string(), |s| format!(",\n{}else_branch: [\n{}\n{}]", child_indent, s, child_indent));
                                        format!(
                                            "If(\n{}condition: {},\n{}then_branch: [\n{}{}\n{}]{} \n{})",
                                            child_indent,
                                            condition.to_pretty_string(tab + 1),
                                            child_indent,
                                            indent(tab + 2),
                                            then_str,
                                            child_indent,
                                            else_block,
                                            base_indent,
                                        )
                                    }
            StmtKind::While { condition, body } => {
                                        let body_str = body.iter()
                                            .map(|s| s.to_pretty_string(tab + 2))
                                            .collect::<Vec<_>>()
                                            .join(&format!("\n{}", indent(tab + 2)));
                                        format!(
                                            "While(\n{}condition: {},\n{}body: [\n{}{}\n{}]\n{})",
                                            child_indent,
                                            condition.to_pretty_string(tab + 1),
                                            child_indent,
                                            indent(tab + 2),
                                            body_str,
                                            child_indent,
                                            base_indent,
                                        )
                                    }
            StmtKind::Block(stmts) => {
                                        let stmts_str = stmts.iter()
                                            .map(|s| s.to_pretty_string(tab + 1))
                                            .collect::<Vec<_>>()
                                            .join(&format!("\n{}", indent(tab + 1)));
                                        format!("Block([\n{}{}\n{}])", indent(tab + 1), stmts_str, base_indent)
                                    }
            StmtKind::Assignment { target, value } => {
                                        format!(
                                            "Assignment(\n{}target: {},\n{}value: {}\n{})", 
                                            child_indent,
                                            target.to_pretty_string(tab + 1),
                                            child_indent,
                                            value.to_pretty_string(tab + 1),
                                            base_indent
                                        )
                                    },
            StmtKind::ExtFnDecl(function_decl) => ext_fn_decl_pretty_string(function_decl, tab),
            StmtKind::StructDecl(struct_decl) => struct_pretty_string(struct_decl, tab),
            StmtKind::ClassDecl(class_decl) => class_pretty_string(class_decl, tab),
            StmtKind::TraitDecl(trait_decl) => trait_pretty_string(trait_decl, tab),
            StmtKind::TraitImpl(impl_block) => trait_impl_pretty_string(impl_block, tab),
            StmtKind::InterfaceDecl(interface_decl) => interface_pretty_string(interface_decl, tab),
            StmtKind::EnumDecl(enum_decl) => enum_pretty_string(enum_decl, tab),
            StmtKind::UnionDecl(union_decl) => union_pretty_string(union_decl, tab),
            StmtKind::TypeEnumDecl(type_enum_decl) => type_enum_pretty_string(type_enum_decl, tab),
        }
    }
}

impl PrettyFormat for Parameter {
    fn to_pretty_string(&self, tab: usize) -> String {
        let indent = "  ".repeat(tab);
        let default_str = match &self.default_value {
            Some(expr) => format!(" = {}", expr.to_pretty_string(tab + 1)),
            None => "".to_string(),
        };
        format!("{}{}: {}{}", indent, self.name.0, self.ty.to_pretty_string(tab), default_str)
    }
}

impl PrettyFormat for SoulType {
    fn to_pretty_string(&self, tab: usize) -> String {
        let indent = "\t".repeat(tab);
        match self {
            SoulType::Base(kind) => {
                format_type_kind(kind, tab)
            }

            SoulType::Modifier { modifier, inner } => {
                let modi_str = match modifier {
                    Modifier::Default => "default",
                    Modifier::Literal => "literal",
                    Modifier::Const => "const",
                };
                format!(
                    "{indent}Modified({},\n{})",
                    modi_str,
                    inner.to_pretty_string(tab + 1)
                )
            }

            SoulType::Wrapper { wrapper, inner } => {
                format!(
                    "{indent}Wrapped({:?},\n{})",
                    wrapper,
                    inner.to_pretty_string(tab + 1)
                )
            }
        }
    }
}

impl PrettyFormat for TypeOfExpr {
    fn to_pretty_string(&self, tab: usize) -> String {
        match self {
            TypeOfExpr::Type(soul_type) => format!("TypeOfExpr({})", soul_type.to_pretty_string(tab)),
            TypeOfExpr::VariantPattern { union_type, variant_name, binding } => format!("TypeOfExpr(unionType: {}, variantName: {}, binding: {})", union_type.to_pretty_string(tab), variant_name.0, binding.as_ref().unwrap_or(&Ident("None".to_string())).0),
            TypeOfExpr::TypeEnum(soul_types) => format!("TypeOfExpr({:?})", soul_types.iter().map(|ty| ty.to_pretty_string(tab+1)).collect::<Vec<_>>()),
        }
    }
} 

// helper
fn ext_fn_decl_pretty_string(decl: &FunctionDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);

    let sig_str = function_signature_pretty_string(&decl.signature, tab + 1);

    format!(
        "ExtFnDecl(\n{}signature: {}\n{})",
        child_indent,
        sig_str,
        base_indent
    )
}

fn var_decl_pretty_string(decl: &VariableDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    
    let ty_str = if let Some(ty) = &decl.ty {
        ty.to_pretty_string(tab + 1)
    } else {
        "None".to_string()
    };
    let init_str = if let Some(init) = &decl.initializer {
        init.to_pretty_string(tab + 1)
    } else {
        "None".to_string()
    };
    format!(
        "LetDecl(\n{}name: {},\n{}type: {},\n{}init: {}\n{})",
        child_indent,
        decl.name.0,
        child_indent,
        ty_str,
        child_indent,
        init_str,
        base_indent,
    )
}

fn interface_pretty_string(decl: &InterfaceDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let method_indent = indent(tab + 2);

    // Print the interface name
    let name_str = &decl.signature.name.0;

    // Print each method signature
    let methods_str = if decl.methods.is_empty() {
        format!("{}None", method_indent)
    } else {
        decl.methods
            .iter()
            .map(|sig| function_signature_pretty_string(sig, tab + 2))
            .collect::<Vec<_>>()
            .join(&format!("\n"))
    };

    format!(
        "InterfaceDecl(\n{}name: {},\n{}methods: [\n{}\n{}]\n{})",
        child_indent,
        name_str,
        child_indent,
        methods_str,
        child_indent,
        base_indent
    )
}

fn function_signature_pretty_string(sig: &FunctionSignature, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);

    let name = &sig.name.0;
    let receiver = if let Some(ty) = &sig.receiver {
        format!("Some({})", ty.to_pretty_string(tab + 1))
    } else {
        "None".to_string()
    };
    let params = if sig.params.is_empty() {
        "[]".to_string()
    } else {
        let params_str = sig.params.iter()
            .map(|p| format!("{}{}: {}", child_indent, p.name.0, p.ty.to_pretty_string(tab + 2)))
            .collect::<Vec<_>>()
            .join(",\n");
        format!("[\n{}\n{}]", params_str, base_indent)
    };
    let return_type = if let Some(ty) = &sig.return_type {
        ty.to_pretty_string(tab + 1)
    } else {
        "None".to_string()
    };

    format!(
        "{}FunctionSignature(\n{}name: {},\n{}receiver: {},\n{}params: {},\n{}return_type: {}\n{})",
        base_indent,
        child_indent, name,
        child_indent, receiver,
        child_indent, params,
        child_indent, return_type,
        base_indent
    )
}

fn struct_pretty_string(decl: &StructDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let field_indent = indent(tab + 2);

    let fields_str = if decl.fields.is_empty() {
        format!("{}None", field_indent)
    } else {
        decl.fields
            .iter()
            .map(|f| field_pretty_string(f, tab + 2))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "StructDecl(\n{}name: {},\n{}fields: [\n{}\n{}]\n{})",
        child_indent,
        decl.name.0,
        child_indent,
        fields_str,
        child_indent,
        base_indent
    )
}

fn class_pretty_string(decl: &ClassDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let field_indent = indent(tab + 2);
    let method_indent = indent(tab + 2);

    let fields_str = if decl.fields.is_empty() {
        format!("{}None", field_indent)
    } else {
        decl.fields
            .iter()
            .map(|f| field_pretty_string(f, tab + 2))
            .collect::<Vec<_>>()
            .join("\n")
    };

    let methods_str = if decl.methods.is_empty() {
        format!("{}None", method_indent)
    } else {
        decl.methods
            .iter()
            .map(|m| function_signature_pretty_string(&m.node, tab + 2))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "ClassDecl(\n{}name: {},\n{}fields: [\n{}\n{}],\n{}methods: [\n{}\n{}]\n{})",
        child_indent,
        decl.name.0,
        child_indent,
        fields_str,
        child_indent,
        child_indent,
        methods_str,
        child_indent,
        base_indent
    )
}

fn field_pretty_string(field: &FieldDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);

    let default_str = if let Some(expr) = &field.default_value {
        expr.to_pretty_string(tab + 1)
    } else {
        "None".to_string()
    };

    let vis_get = match &field.vis.get {
        Some(v) => format!("{:?}", v),
        None => "default".to_string(),
    };
    let vis_set = match &field.vis.set {
        Some(v) => format!("{:?}", v),
        None => "disallow".to_string(),
    };

    format!(
        "{}FieldDecl(name: {}, ty: {}, default: {}, vis: {{ get: {}, set: {} }})",
        base_indent,
        field.name.0,
        field.ty.to_pretty_string(tab + 1),
        default_str,
        vis_get,
        vis_set
    )
}

fn format_type_kind(kind: &TypeKind, tab: usize) -> String {
    let indent = "\t".repeat(tab);
    match kind {
        TypeKind::UntypedInt => format!("{indent}UntypedInt"),
        TypeKind::SystemInt => format!("{indent}SystemInt"),
        TypeKind::Int(size) => format!("{indent}Int({size})"),
        TypeKind::UntypedUint => format!("{indent}UntypedUint"),
        TypeKind::SystemUint => format!("{indent}SystemUint"),
        TypeKind::Uint(size) => format!("{indent}Uint({size})"),
        TypeKind::UntypedFloat => format!("{indent}UntypedFloat"),
        TypeKind::Float(size) => format!("{indent}Float({size})"),
        TypeKind::Char(size) => format!("{indent}Char({size})"),
        TypeKind::Bool => format!("{indent}Bool"),
        TypeKind::Str => format!("{indent}Str"),
        TypeKind::Custom(ident) => format!("{indent}Custom({})", ident.0),
        TypeKind::Tuple(types) => {
                let inner = types
                    .iter()
                    .map(|t| t.to_pretty_string(tab + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{indent}Tuple([\n{inner}\n{indent}])")
            }
        TypeKind::Function { params, return_type } => {
                let param_str = params
                    .iter()
                    .map(|t| t.to_pretty_string(tab + 1))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!(
                    "{indent}Function([\n{}\n{indent}] -> {})",
                    param_str,
                    return_type.to_pretty_string(tab + 1)
                )
            }
        TypeKind::Struct { name:_, fields, implements } => {
            let fields_str = fields
                .iter()
                .map(|f| format!("{}\t{}", indent, var_decl_pretty_string(f, tab + 1)))
                .collect::<Vec<_>>()
                .join(",\n");
            let implements_str = if implements.is_empty() {
                String::new()
            } else {
                let imps = implements
                    .iter()
                    .map(|i| i.name.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("\n{indent}\timplements: [{}]", imps)
            };
            format!("{indent}Struct {{\n{fields_str}{implements_str}\n{indent}}}")
        }

        TypeKind::Class { name:_, fields, methods, implements } => {
            let fields_str = fields
                .iter()
                .map(|f| format!("{}\t{}", indent, var_decl_pretty_string(f, tab + 1)))
                .collect::<Vec<_>>()
                .join(",\n");
            let methods_str = methods
                .iter()
                .map(|m| format!("{}\t{}", indent, function_decl_pretty_string(m, tab + 1)))
                .collect::<Vec<_>>()
                .join(",\n");
            let implements_str = if implements.is_empty() {
                String::new()
            } else {
                let imps = implements
                    .iter()
                    .map(|i| i.name.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("\n{indent}\timplements: [{}]", imps)
            };
            format!("{indent}Class {{\n{fields_str}\n{methods_str}{implements_str}\n{indent}}}")
        }

        TypeKind::Interface { name:_, methods } => {
            let methods_str = methods
                .iter()
                .map(|m| format!("{}\t{}", indent, m.name.0.as_str()))
                .collect::<Vec<_>>()
                .join(",\n");
            format!("{indent}Interface {{\n{methods_str}\n{indent}}}")
        }

        TypeKind::Trait { name:_, methods, requires } => {
            let methods_str = methods
                .iter()
                .map(|m| format!("{}\t{}", indent, m.name.0.as_str()))
                .collect::<Vec<_>>()
                .join(",\n");
            let requires_str = if requires.is_empty() {
                String::new()
            } else {
                let reqs = requires
                    .iter()
                    .map(|r| r.name.0.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("\n{indent}\trequires: [{}]", reqs)
            };
            format!("{indent}Trait {{\n{methods_str}{requires_str}\n{indent}}}")
        }

        TypeKind::Enum { name:_, variants } => {
            let variants_str = variants
                .iter()
                .map(|v| {
                    if let Some(val) = v.value {
                        format!("{}\t{} = {}", indent, v.name.0, val)
                    } else {
                        format!("{}\t{}", indent, v.name.0)
                    }
                })
                .collect::<Vec<_>>()
                .join(",\n");
            format!("{indent}Enum {{\n{variants_str}\n{indent}}}")
        }

        TypeKind::Union { name:_, variants } => {
            let variants_str = variants
                .iter()
                .map(|v| {
                    let fields_str = match &v.fields[..] {
                        [] => String::from(""),
                        fields => fields.iter().map(|f| match f {
                            UnionVariantKind::Unit => "Unit".to_string(),
                            UnionVariantKind::Tuple(types) => {
                                let tstr = types.iter().map(|t| t.to_pretty_string(tab + 2)).collect::<Vec<_>>().join(", ");
                                format!("Tuple({})", tstr)
                            }
                            UnionVariantKind::Struct(vars) => {
                                let vstr = vars.iter().map(|f| var_decl_pretty_string(f, tab + 2)).collect::<Vec<_>>().join(", ");
                                format!("Struct({})", vstr)
                            }
                        }).collect::<Vec<_>>().join(", "),
                    };
                    if fields_str.is_empty() {
                        format!("{}\t{}", indent, v.name.0)
                    } else {
                        format!("{}\t{}: {}", indent, v.name.0, fields_str)
                    }
                })
                .collect::<Vec<_>>()
                .join(",\n");
            format!("{indent}Union {{\n{variants_str}\n{indent}}}")
        }
    }
}

fn function_decl_pretty_string(func: &FunctionDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    
    let params_str = func.signature.params.iter()
        .map(|p| p.to_pretty_string(tab + 2))
        .collect::<Vec<_>>()
        .join(&format!(",\n{}", indent(tab + 2)));
    let body_str = func.body.iter()
        .map(|s| s.to_pretty_string(tab + 2))
        .collect::<Vec<_>>()
        .join(&format!("\n{}", indent(tab + 2)));
    let ret_str = if let Some(ret) = &func.signature.return_type {
        ret.to_pretty_string(tab + 1)
    } else {
        "None".to_string()
    };
    format!(
        "FnDecl(\n{}name: {},\n{}params: [\n{}{}\n{}],\n{}return_type: {},\n{}body: [\n{}{}\n{}]\n{})",
        child_indent,
        func.signature.name.0,
        child_indent,
        indent(tab + 2),
        params_str,
        child_indent,
        child_indent,
        ret_str,
        child_indent,
        indent(tab + 2),
        body_str,
        child_indent,
        base_indent,
    )
}

fn trait_pretty_string(decl: &TraitDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let method_indent = indent(tab + 2);

    let name_str = &decl.signature.name.0;

    let methods_str = if decl.methods.is_empty() {
        format!("{}None", method_indent)
    } else {
        decl.methods
            .iter()
            .map(|sig| function_signature_pretty_string(sig, tab + 2))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "TraitDecl(\n{}name: {},\n{}methods: [\n{}\n{}]\n{})",
        child_indent,
        name_str,
        child_indent,
        methods_str,
        child_indent,
        base_indent
    )
}

fn trait_impl_pretty_string(impl_block: &ImplBlock, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let method_indent = indent(tab + 2);

    let trait_name = &impl_block.trait_name.0;
    let for_type = impl_block.for_type.to_pretty_string(tab + 1);

    let methods_str = if impl_block.methods.is_empty() {
        format!("{}None", method_indent)
    } else {
        impl_block.methods
            .iter()
            .map(|m| function_decl_pretty_string(m, tab + 2))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "TraitImpl(\n{}trait: {},\n{}for: {},\n{}methods: [\n{}\n{}]\n{})",
        child_indent,
        trait_name,
        child_indent,
        for_type,
        child_indent,
        methods_str,
        child_indent,
        base_indent
    )
}

fn enum_pretty_string(decl: &EnumDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let variant_indent = indent(tab + 2);

    let name_str = &decl.signature.name.0;

    let variants_str = if decl.variants.is_empty() {
        format!("{}None", variant_indent)
    } else {
        decl.variants
            .iter()
            .map(|var| format!("{}{}", variant_indent, var.name.0))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "EnumDecl(\n{}name: {},\n{}variants: [\n{}\n{}]\n{})",
        child_indent,
        name_str,
        child_indent,
        variants_str,
        child_indent,
        base_indent
    )
}

fn union_pretty_string(decl: &UnionDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let field_indent = indent(tab + 2);

    let name_str = &decl.signature.name.0;

    let fields_str = if decl.variants.is_empty() {
        format!("{}None", field_indent)
    } else {
        decl.variants
            .iter()
            .map(|field| format!("{}{}", field_indent, field.name.0))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "UnionDecl(\n{}name: {},\n{}fields: [\n{}\n{}]\n{})",
        child_indent,
        name_str,
        child_indent,
        fields_str,
        child_indent,
        base_indent
    )
}

fn type_enum_pretty_string(decl: &TypeEnumDecl, tab: usize) -> String {
    let indent = |tab| "  ".repeat(tab);
    let base_indent = indent(tab);
    let child_indent = indent(tab + 1);
    let variant_indent = indent(tab + 2);

    let name_str = &decl.signature.name.0;

    let variants_str = if decl.types.is_empty() {
        format!("{}None", variant_indent)
    } else {
        decl.types
            .iter()
            .map(|variant| format!("{}{}", variant_indent, variant.to_pretty_string(tab)))
            .collect::<Vec<_>>()
            .join("\n")
    };

    format!(
        "TypeEnumDecl(\n{}name: {},\n{}variants: [\n{}\n{}]\n{})",
        child_indent,
        name_str,
        child_indent,
        variants_str,
        child_indent,
        base_indent
    )
}


















