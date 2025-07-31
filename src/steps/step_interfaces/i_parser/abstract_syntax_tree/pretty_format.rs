use std::sync::RwLockReadGuard;

use itertools::Itertools;

use crate::steps::step_interfaces::i_parser::{abstract_syntax_tree::{
    abstract_syntax_tree::{AbstractSyntacTree, GlobalKind}, soul_type::type_kind::TypeKind, staments::{conditionals::{ElseKind, IfDecl}, enum_likes::{EnumDeclRef, TypeEnumDeclRef, UnionDeclRef}, function::{ExtFnDecl, FnDecl, FnDeclKind}, objects::{ClassDeclRef, InnerTraitDecl, StructDeclRef, TraitImpl}, statment::{Block, ReturnLike, StmtKind, VariableDecl}}, visibility::{FieldAccess, Visibility}}, scope::{ScopeBuilder, ScopeKind}};

pub trait PrettyFormat {
    fn to_pretty_string(&self) -> String;
}

pub trait PrettyPrint {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String;
}

impl PrettyFormat for ScopeBuilder {
    fn to_pretty_string(&self) -> String {
        format!(
            "scopes: [\n\t{}\n]\ntypes: [\n\t{}\n]",
            self.get_scopes()
                .scopes
                .iter()
                .map(|scope| format!(
                    "scope{}: [\n\t\t{}\n\t]", 
                    scope.self_index, 
                    scope.symbols.iter().map(|sm| format!("\"{}\": {}", sm.0, sm.1.to_pretty(3, false))).join(",\n\t\t")
                )).join(",\n\t"),
            self.get_types()
                .iter()
                .map(|scope| format!(
                    "scope{}: [\n\t\t{}\n\t]", 
                    scope.self_index, 
                    scope.symbols.iter().map(|sm| format!("\"{}\": {}", sm.0, sm.1.to_pretty(3, false))).join(",\n\t\t")
                )).join(",\n\t"),
        )
    }
}

impl PrettyPrint for Vec<ScopeKind> {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        
        let inner = self.iter().map(|kind| {
            match kind {
                ScopeKind::Invalid() => "<invalid>".into(),
                ScopeKind::Variable(node_ref) => format!("let({})", node_ref.borrow().name.0),
                ScopeKind::Struct(struct_decl) => format!("struct({})", struct_decl.borrow().name.0),
                ScopeKind::Class(class_decl) => format!("class({})", class_decl.borrow().name.0),
                ScopeKind::Trait(trait_decl) => format!("trait({})", trait_decl.borrow().name.0),
                ScopeKind::Functions(node_ref) => format!("func({})", node_ref.borrow().last().map(|fnc| fnc.get_signature().borrow().name.0.clone()).unwrap_or("".into()) ),
                ScopeKind::Enum(enum_decl) => format!("enum({})", enum_decl.borrow().name.0),
                ScopeKind::Union(union_decl) => format!("union({})", union_decl.borrow().name.0),
                ScopeKind::TypeEnum(type_enum_decl) => format!("typeEnum({})", type_enum_decl.borrow().name.0),
            }
        }).join(",");

        format!("[{}]", inner)
    }
}

impl PrettyPrint for TypeKind {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        self.to_string()
    }
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
            StmtKind::TraitDecl(trait_decl) => trait_decl.borrow().to_pretty(tab, is_last),
            StmtKind::EnumDecl(enum_decl) => enum_decl.to_pretty(tab, is_last),
            StmtKind::UnionDecl(union_decl) => union_decl.to_pretty(tab, is_last),
            StmtKind::TypeEnumDecl(type_enum) => type_enum.to_pretty(tab, is_last),
            StmtKind::TraitImpl(trait_impl) => trait_impl.to_pretty(tab, is_last),
            StmtKind::Return(ReturnLike{value, delete_list, kind}) => format!(
                "{}Return >> {} {} >> free([{}])",
                prefix,
                kind.to_str(),
                value.as_ref().map(|ty| ty.node.to_string()).unwrap_or_default(),
                delete_list.iter().join(","),
            ),
            StmtKind::Assignment(assign) => format!(
                "{}Assignment >> {} = {};",
                prefix,
                assign.target.node.to_string(),
                assign.value.node.to_string()
            ),
            StmtKind::If(if_decl) => if_decl.to_pretty(tab, is_last),
            StmtKind::While(while_decl) => {
                let cond = while_decl.condition.as_ref().map(|el| el.node.to_string()).unwrap_or("<empty>".into());
                let body = while_decl.body.to_pretty(tab + 1, true);
                format!(
                    "{}While >> while{}\n{}",
                    prefix,
                    cond,
                    body
                )
            }
            StmtKind::Block(block) => block.to_pretty(tab, is_last),
            StmtKind::CloseBlock(arr) => format!(
                "{}CloseBlock >> free([{}])\n{}",
                prefix,
                arr.delete_list.iter().join(","),
                tree_next_line_prefix(tab),
            ),
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
            },
            StmtKind::Switch(switch) =>format!(
                "SwitchCase >> match {}\n{}", 
                switch.condition.node.to_string(),
                switch.cases.iter().enumerate().map(|(i, stmt)| {
                    let is_last = i == switch.cases.len() - 1;
                    stmt.do_fn.to_pretty(tab, is_last)
                }).collect::<Vec<_>>().join("\n")
            ),
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
            GlobalKind::TraitDecl(trait_decl) => trait_decl.borrow().to_pretty(tab, true),
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

impl PrettyPrint for RwLockReadGuard<'_, InnerTraitDecl> {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let header = format!("{}Trait {} >>", prefix, self.name.0);

        let mut methods = self.methodes
            .iter()
            .enumerate()
            .map(|(i, sig)| {
                let last = i == self.methodes.len() - 1;
                let inner_prefix = tree_prefix(tab + 1, last);
                format!("{} {};", inner_prefix, sig.to_string())
            })
            .join("\n");

        if methods.is_empty() {
            methods = prefix;
        }

        format!("{}\n{}", header, methods)
    }
}

impl PrettyPrint for TypeEnumDeclRef {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        let this = self.borrow();
        let types = this.types.iter().map(|t| t.to_string()).join(", ");
        format!("TypeEnum {} = [{}];", this.name.0, types)
    }
}

impl PrettyPrint for UnionDeclRef {
    fn to_pretty(&self, tab: usize, _is_last: bool) -> String {
        let this = self.borrow();
        let indent_str = indent(tab);
        let variants = this.variants
            .iter()
            .map(|v| format!("{}{}{}", indent_str, v.name.0, v.field.to_string()))
            .join(",\n");
        format!("Union {} >>\n{}", this.name.0, variants)
    }
}

impl PrettyPrint for EnumDeclRef {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        let this = self.borrow();
        let variants = this.variants
            .iter()
            .map(|v| format!("{}({:?})", v.name.0, v.value))
            .join(", ");
        format!("Enum {} >> [{}]", this.name.0, variants)
    }
}

impl PrettyPrint for TraitImpl {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let mut methods = self.methodes
            .iter()
            .map(|fn_decl| fn_decl.to_pretty(tab + 1, true))
            .join("\n\n");

        if methods.is_empty() {
            methods = tree_prefix(tab, is_last);
        }

        format!("Impl {} for {} >>\n{}", self.trait_name.0, self.for_type.to_string(), methods)
    }
}

impl PrettyPrint for ClassDeclRef {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let this = self.borrow();
        
        let prefix = tree_prefix(tab, is_last);
        let generics = if this.generics.is_empty() {
            "".to_string()
        } else {
            format!("<{}>", this.generics.iter().map(|g| g.to_string()).join(", "))
        };
        let header = format!("{}Class {}{} >>", prefix, this.name.0, generics);

        let last_index = this.fields.len() - 1;
        let is_empty = this.methodes.is_empty();

        let fields = this.fields.iter().enumerate().map(|(i, f)| {
            let is_last_field = i == last_index && is_empty;
            let field_prefix = tree_prefix(tab + 1, is_last_field);
            let access = match (&f.vis.get, &f.vis.set) {
                (Some(Visibility::Public), Some(Visibility::Public)) => "{Get;Set;}",
                (Some(Visibility::Public), _) => "{Get;}",
                (_, Some(Visibility::Public)) => "{Set;}",
                _ => "",
            };
            let default = f.default_value
                .as_ref()
                .map(|v| format!(" = {}", v.node.to_string()))
                .unwrap_or_default();
            format!("{}{} {}: {}{}", field_prefix, access, f.name.0, f.ty.to_string(), default)
        });

        let len = this.methodes.len();
        let methods = this.methodes.iter().enumerate().map(|(i, m)| {
            let is_last_method = i == len - 1;
            m.node.to_pretty(tab + 1, is_last_method)
        });

        let mut body = fields.chain(methods).join("\n");
        if body.is_empty() {
            body = prefix;
        }

        format!("{}\n{}", header, body)
    }
}

impl PrettyPrint for FnDeclKind {

    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        match self {
            FnDeclKind::Fn(fn_decl) => fn_decl.to_pretty(tab, is_last),
            FnDeclKind::InternalFn(node_ref) => node_ref.to_string(),
            FnDeclKind::ExtFn(ext_fn_decl) => ext_fn_decl.to_pretty(tab, is_last),
        }
    }
}

impl PrettyPrint for StructDeclRef {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let header = format!("{}Struct {}{} >>", prefix, self.borrow().name.0, 
            if self.borrow().generics.is_empty() {
                "".to_string()
            } else {
                format!("<{}>", self.borrow().generics.iter().map(|g| g.to_string()).join(", "))
            });

        let mut body = self.borrow().fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let last = i == self.borrow().fields.len() - 1;
                let inner_prefix = tree_prefix(tab + 1, last);
                if let Some(value) = &f.default_value {
                    format!("{}{} {} {} = {}", inner_prefix, f.ty.to_string(), f.name.0, f.vis.to_pretty(0, false), value.node.to_string())
                } else {
                    format!("{}{} {} {}", inner_prefix, f.ty.to_string(), f.name.0, f.vis.to_pretty(0, false))
                }
            })
            .join("\n");

        if body.is_empty() {
            body = prefix;
        }

        format!("{}\n{}", header, body)
    }
}

impl PrettyPrint for FieldAccess {
    fn to_pretty(&self, _tab: usize, _is_last: bool) -> String {
        if self.get.is_none() && self.set.is_none() {
            return String::new()
        }

        let get = if let Some(get) = &self.get {
            match get {
                Visibility::Public => "Get;",
                Visibility::Private => "get;",
            }
        }
        else {""};

        let set = if let Some(set) = &self.set {
            match set {
                Visibility::Public => "Set;",
                Visibility::Private => "set;",
            }
        }
        else {""};

        format!("{{{}{}}}", get, set)
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

impl PrettyPrint for ExtFnDecl {
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let sig = self.signature.to_string();
        let body = self.body.to_pretty(tab + 1, true); 
        format!("{}ExtFnDecl >> {}\n{}", prefix, sig, body)
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
    fn to_pretty(&self, tab: usize, is_last: bool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let init = match &self.initializer {
            Some(expr) => format!(" = {}", expr.node.to_string()),
            None => "".to_string(),
        };
        format!("{}Var {} {}{}", prefix, self.ty.to_string(), self.name.0, init)
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











































