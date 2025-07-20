use itertools::Itertools;

use crate::steps::step_interfaces::i_parser::abstract_syntax_tree::{
    abstract_syntax_tree::{AbstractSyntacTree, GlobalKind},
    statment::{Block, ClassDecl, ElseKind, EnumDecl, ExtFnDecl, FnDecl, IfDecl, StmtKind, StructDecl, TraitDecl, TraitImpl, TypeEnumDecl, UnionDecl, VariableDecl, Visibility},
};

pub trait PrettyFormat {
    fn to_pretty_string(&self) -> String;
}

pub trait PrettyPrint {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String;
}

impl PrettyFormat for AbstractSyntacTree {
    fn to_pretty_string(&self) -> String {
        self.root
            .iter()
            .map(|node| node.node.to_pretty(0, true))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl PrettyPrint for StmtKind {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            StmtKind::ExprStmt(expr) => format!("{}ExprStmt<{}> >> {};", prefix, expr.node.get_variant_name(), expr.node.to_string()),
            StmtKind::VarDecl(var_ref) => var_ref.borrow().to_pretty(tab, is_last),
            StmtKind::FnDecl(fn_decl) => fn_decl.to_pretty(tab, is_last),
            StmtKind::ExtFnDecl(ext_fn) => ext_fn.to_pretty(tab, is_last),
            StmtKind::StructDecl(struc) => struc.to_pretty(tab, is_last),
            StmtKind::ClassDecl(class) => class.to_pretty(tab, is_last),
            StmtKind::TraitDecl(trait_decl) => trait_decl.to_pretty(tab, is_last),
            StmtKind::EnumDecl(enum_decl) => enum_decl.to_pretty(tab, is_last),
            StmtKind::UnionDecl(union_decl) => union_decl.to_pretty(tab, is_last),
            StmtKind::TypeEnumDecl(type_enum) => type_enum.to_pretty(tab, is_last),
            StmtKind::TraitImpl(trait_impl) => trait_impl.to_pretty(tab, is_last),
            StmtKind::Return(ret) => format!(
                "{}Return >> return {};",
                prefix,
                ret.value.as_ref().map(|ty| ty.node.to_string()).unwrap_or_default()
            ),
            StmtKind::Assignment(assign) => format!(
                "{}Assignment >> {} = {};",
                prefix,
                assign.target.node.to_string(),
                assign.value.node.to_string()
            ),
            StmtKind::If(if_decl) => if_decl.to_pretty(tab, is_last),
            StmtKind::While(while_decl) => {
                let cond = while_decl.condition.node.to_string();
                let body = while_decl.body.to_pretty(tab + 1, true);
                format!(
                    "{}While >> while ({})\n{}",
                    prefix,
                    cond,
                    body
                )
            }
            StmtKind::Block(block) => block.to_pretty(tab, is_last),
            StmtKind::CloseBlock(_) => format!("{}CloseBlock >>", prefix),
            StmtKind::For(for_decl) => {
                let el = for_decl.element.0.clone();
                let coll = for_decl.collection.node.to_string();
                let body = for_decl.body.to_pretty(tab + 1, true);
                format!(
                    "{}For >> for {} in {}\n{}",
                    prefix,
                    el,
                    coll,
                    body
                )
            }
        }
    }
}

impl PrettyPrint for ElseKind {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            ElseKind::ElseIf(if_decl) => if_decl.to_pretty(tab, is_last),
            ElseKind::Else(block) => {
                let body = block.to_pretty(tab + 1, true);
                format!("{}Else >> else\n{}", prefix, body)
            }
        }
    }
}

impl PrettyPrint for IfDecl {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let cond = self.condition.node.to_string();
        let mut output = vec![format!("{}If >> if ({})", prefix, cond)];

        output.push(self.body.to_pretty(tab + 1, true));

        for (i, e) in self.else_branchs.iter().enumerate() {
            let last = i == self.else_branchs.len() - 1;
            output.push(e.to_pretty(tab + 1, last));
        }

        output.join("\n")
    }
}

impl PrettyPrint for GlobalKind {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let content = match self {
            GlobalKind::ClassDecl(class) => class.to_pretty(tab, true),
            GlobalKind::StructDecl(struc) => struc.to_pretty(tab, true),
            GlobalKind::TraitDecl(trait_decl) => trait_decl.to_pretty(tab, true),
            GlobalKind::TraitImpl(impl_block) => impl_block.to_pretty(tab, true),
            GlobalKind::FuncDecl(fn_decl) => fn_decl.to_pretty(tab, true),
            GlobalKind::ExtFuncDecl(fn_decl) => fn_decl.to_pretty(tab, true),
            GlobalKind::VarDecl(var) => var.borrow().to_pretty(tab, true),
            GlobalKind::EnumDecl(enum_decl) => enum_decl.to_pretty(tab, true),
            GlobalKind::UnionDecl(union_decl) => union_decl.to_pretty(tab, true),
            GlobalKind::TypeEnumDecl(type_enum) => type_enum.to_pretty(tab, true),
        };
        format!("{}{}", prefix, content)
    }
}

impl PrettyPrint for TraitDecl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let indent_str = indent(tab);
        let methods = self.methods
            .iter()
            .map(|sig| format!("{}fn {};", indent_str, sig.to_string()))
            .join("\n");

        format!("Trait {} >>\n{}", self.name.0, methods)
    }
}

impl PrettyPrint for TypeEnumDecl {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        let types = self.types.iter().map(|t| t.to_string()).join(", ");
        format!("TypeEnum {} = [{}];", self.name.0, types)
    }
}

impl PrettyPrint for UnionDecl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let indent_str = indent(tab);
        let variants = self.variants
            .iter()
            .map(|v| format!("{}{:?}", indent_str, v.fields))
            .join(",\n");
        format!("Union {} >>\n{}", self.name.0, variants)
    }
}

impl PrettyPrint for EnumDecl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let indent_str = indent(tab);
        let variants = self.variants
            .iter()
            .map(|v| format!("{}{:?}", indent_str, v.value))
            .join(",\n");
        format!("Enum {} >>\n{}", self.name.0, variants)
    }
}

impl PrettyPrint for ExtFnDecl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let sig = self.signature.to_string();
        let body = self.body.to_pretty(tab + 1, true);
        format!("External fn {} >>\n{}", sig, body)
    }
}

impl PrettyPrint for TraitImpl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let methods = self.methods
            .iter()
            .map(|fn_decl| fn_decl.to_pretty(tab + 1, true))
            .join("\n\n");
        format!("Impl {} for {} >>\n{}", self.trait_name.0, self.for_type.to_string(), methods)
    }
}

impl PrettyPrint for ClassDecl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let indent_str = indent(tab);
        let generics = if self.generics.is_empty() {
            "".to_string()
        } else {
            format!("<{}>", self.generics.iter().map(|g| g.to_string()).join(", "))
        };
        let fields = self.fields.iter().map(|f| {
            let access = match (&f.vis.get, &f.vis.set) {
                (Some(Visibility::Public), Some(Visibility::Public)) => "pub",
                (Some(Visibility::Public), _) => "pub(get)",
                (_, Some(Visibility::Public)) => "pub(set)",
                _ => "priv",
            };
            let default = f.default_value
                .as_ref()
                .map(|v| format!(" = {}", v.node.to_string()))
                .unwrap_or_default();
            format!("{}{} {}: {}{}", indent_str, access, f.name.0, f.ty.to_string(), default)
        }).join("\n");
        let methods = self.methods.iter()
            .map(|sig| format!("{}fn {};", indent_str, sig.node.to_string()))
            .join("\n");

        format!("Class {}{} >>\n{}\n{}", self.signature.0, generics, fields, methods)
    }
}

impl PrettyPrint for StructDecl {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let indent_str = indent(tab);
        let generics = if self.generics.is_empty() {
            "".to_string()
        } else {
            format!("<{}>", self.generics.iter().map(|g| g.to_string()).join(", "))
        };
        let fields = self.fields
            .iter()
            .map(|f| format!("{}{}: {}", indent_str, f.name.0, f.ty.to_string()))
            .join("\n");

        format!("Struct {}{} >>\n{}", self.name.0, generics, fields)
    }
}

impl PrettyPrint for FnDecl {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let sig = self.signature.to_string();
        let body = self.body.to_pretty(tab + 1, true); 
        format!("{}FnDecl >> {}\n{}", prefix, sig, body)
    }
}

impl PrettyPrint for Block {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        self.statments.iter().enumerate().map(|(i, stmt)| {
            let is_last = i == self.statments.len() - 1;
            stmt.node.to_pretty(tab, is_last)
        }).collect::<Vec<_>>().join("\n")
    }
}

impl PrettyPrint for VariableDecl {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        let init = match &self.initializer {
            Some(expr) => format!(" = {}", expr.node.to_string()),
            None => "".to_string(),
        };
        format!("Var {}: {}{}", self.name.0, self.ty.to_string(), init)
    }
}

fn indent(level: usize) -> String {
    "    ".repeat(level)
}

fn tree_prefix(indent: usize, is_last: bool) -> String {
    if indent == 0 {
        return String::new();
    }

    let mut prefix = String::new();

    for _ in 0..indent - 1 {
        prefix.push_str("│   ");
    }

    prefix.push_str(if is_last { "└── " } else { "├── " });

    prefix
}













