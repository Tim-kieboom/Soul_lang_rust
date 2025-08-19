use itertools::Itertools;
use std::sync::RwLockReadGuard;
use crate::{steps::step_interfaces::{i_parser::{abstract_syntax_tree::{abstract_syntax_tree::{AbstractSyntacTree, GlobalKind}, soul_type::type_kind::TypeKind, staments::{conditionals::{CaseDoKind, ElseKind, IfDecl}, enum_likes::{EnumDeclRef, TypeEnumDeclRef, UnionDeclRef}, function::{ExtFnDecl, FnDecl, FnDeclKind}, objects::{ClassDeclRef, InnerTraitDecl, StructDeclRef, TraitImpl}, statment::{Block, ReturnLike, StmtKind, VariableKind}}, visibility::{FieldAccess, Visibility}}, scope::{ScopeBuilder, ScopeKind}}, i_sementic::sementic_scope::ScopeVisitor}, utils::node_ref::{MultiRefId, MultiRefPool, MultiRefReadGuard}};

pub trait PrettyFormat {
    fn to_pretty_string(&self, ref_pool: &MultiRefPool) -> String;
}

pub trait PrettyPrint {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String;
}

impl PrettyFormat for ScopeVisitor {
    fn to_pretty_string(&self, ref_pool: &MultiRefPool) -> String {
        format!(
            "scopes: [\n\t{}\n]\ntypes: [\n\t{}\n]",
            self.get_scopes()
                .scopes
                .iter()
                .map(|scope| format!(
                    "scope{}: [\n\t\t{}\n\t]", 
                    scope.scope.self_index, 
                    scope.scope.symbols.iter().map(|sm| format!("\"{}\": {}", sm.0, sm.1.to_pretty(3, false, ref_pool))).join(",\n\t\t")
                )).join(",\n\t"),
            self.get_types()
                .iter()
                .map(|scope| format!(
                    "scope{}: [\n\t\t{}\n\t]", 
                    scope.self_index, 
                    scope.symbols.iter().map(|sm| format!("\"{}\": {}", sm.0, sm.1.to_pretty(3, false, ref_pool))).join(",\n\t\t")
                )).join(",\n\t"),
        )
    }
}

impl PrettyFormat for ScopeBuilder {
    fn to_pretty_string(&self, ref_pool: &MultiRefPool) -> String {
        format!(
            "scopes: [\n\t{}\n]\ntypes: [\n\t{}\n]",
            self.get_scopes()
                .scopes
                .iter()
                .map(|scope| format!(
                    "scope{}: [\n\t\t{}\n\t]", 
                    scope.self_index, 
                    scope.symbols.iter().map(|sm| format!("\"{}\": {}", sm.0, sm.1.to_pretty(3, false, ref_pool))).join(",\n\t\t")
                )).join(",\n\t"),
            self.get_types()
                .iter()
                .map(|scope| format!(
                    "scope{}: [\n\t\t{}\n\t]", 
                    scope.self_index, 
                    scope.symbols.iter().map(|sm| format!("\"{}\": {}", sm.0, sm.1.to_pretty(3, false, ref_pool))).join(",\n\t\t")
                )).join(",\n\t"),
        )
    }
}

impl PrettyPrint for Vec<ScopeKind> {
    fn to_pretty(&self, _tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
        
        let inner = self.iter().map(|kind| {
            match kind {
                ScopeKind::Invalid() => "<invalid>".into(),
                ScopeKind::This(this) => format!("(This={})", this.to_string(ref_pool)),
                ScopeKind::Enum(enum_decl) => format!("enum({})", enum_decl.borrow(ref_pool).name.0),
                ScopeKind::Variable(node_ref) => format!("let({})", node_ref.borrow(ref_pool).name.0),
                ScopeKind::Class(class_decl) => format!("class({})", class_decl.borrow(ref_pool).name.0),
                ScopeKind::Trait(trait_decl) => format!("trait({})", trait_decl.borrow(ref_pool).name.0),
                ScopeKind::Union(union_decl) => format!("union({})", union_decl.borrow(ref_pool).name.0),
                ScopeKind::Struct(struct_decl) => format!("struct({})", struct_decl.borrow(ref_pool).name.0),
                ScopeKind::TypeEnum(type_enum_decl) => format!("typeEnum({})", type_enum_decl.borrow(ref_pool).name.0),
                ScopeKind::TypeDefed(typedefed) => format!("type({} typeof {})", typedefed.borrow(ref_pool).name.0, typedefed.borrow(ref_pool).from_type.to_string(ref_pool)),
                ScopeKind::Functions(node_ref) => format!("func({})", node_ref.borrow(ref_pool).last().map(|fnc| fnc.get_signature().borrow(ref_pool).node.name.0.clone()).unwrap_or("".into()) ),
                ScopeKind::NamedTupleCtor(ctor) => format!("{}({})", ctor.object_type.to_string(ref_pool), ctor.values.iter().map(|(el, (ty, default))| format!("{}: {}{}", el.0, ty.to_string(ref_pool), default.as_ref().map(|el| format!(" = {}", el.node.to_string(ref_pool, 0))).unwrap_or("".into()))).join(",") ),
            }
        }).join(",");

        format!("[{}]", inner)
    }
}

impl PrettyPrint for TypeKind {
    fn to_pretty(&self, _tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
        self.to_string(ref_pool)
    }
}

impl PrettyFormat for AbstractSyntacTree {
    fn to_pretty_string(&self, ref_pool: &MultiRefPool) -> String {
        self.root
            .iter()
            .map(|node| node.node.to_pretty(0, true, ref_pool))
            .collect::<Vec<_>>()
            .join("\n\n")
    }
}

impl PrettyPrint for StmtKind {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            StmtKind::ExprStmt(expr) => format!("{}ExprStmt<{}> >> {};", prefix, expr.node.get_variant_name(), expr.node.to_string(ref_pool, tab)),
            StmtKind::VarDecl(var_ref) => var_ref.to_pretty(tab, is_last, ref_pool),
            StmtKind::FnDecl(fn_decl) => fn_decl.to_pretty(tab, is_last, ref_pool),
            StmtKind::ExtFnDecl(ext_fn) => ext_fn.to_pretty(tab, is_last, ref_pool),
            StmtKind::StructDecl(struc) => struc.to_pretty(tab, is_last, ref_pool),
            StmtKind::ClassDecl(class) => class.to_pretty(tab, is_last, ref_pool),
            StmtKind::TraitDecl(trait_decl) => trait_decl.borrow(ref_pool).to_pretty(tab, is_last, ref_pool),
            StmtKind::EnumDecl(enum_decl) => enum_decl.to_pretty(tab, is_last, ref_pool),
            StmtKind::UnionDecl(union_decl) => union_decl.to_pretty(tab, is_last, ref_pool),
            StmtKind::TypeEnumDecl(type_enum) => type_enum.to_pretty(tab, is_last, ref_pool),
            StmtKind::TraitImpl(trait_impl) => trait_impl.to_pretty(tab, is_last, ref_pool),
            StmtKind::Return(ReturnLike{value, delete_list, kind}) => format!(
                "{}Return >> {} {} >> free([{}])",
                prefix,
                kind.to_str(),
                value.as_ref().map(|ty| ty.node.to_string(ref_pool, tab)).unwrap_or_default(),
                delete_list.iter().join(","),
            ),
            StmtKind::Assignment(assign) => format!(
                "{}Assignment >> {} = {};",
                prefix,
                assign.target.node.to_string(ref_pool, tab),
                assign.value.node.to_string(ref_pool, tab)
            ),
            StmtKind::If(if_decl) => if_decl.to_pretty(tab, is_last, ref_pool),
            StmtKind::While(while_decl) => {
                let cond = while_decl.condition.as_ref().map(|el| el.node.to_string(ref_pool, tab)).unwrap_or("<empty>".into());
                let body = while_decl.body.to_pretty(tab + 1, true, ref_pool);
                format!(
                    "{}While >> while{}\n{}",
                    prefix,
                    cond,
                    body
                )
            }
            StmtKind::Block(block) => block.to_pretty(tab, is_last, ref_pool),
            StmtKind::CloseBlock(arr) => format!(
                "{}CloseBlock >> free([{}])\n{}",
                prefix,
                arr.delete_list.iter().join(","),
                tree_next_line_prefix(tab),
            ),
            StmtKind::For(for_decl) => {
                let el = for_decl.element.0.clone();
                let coll = for_decl.collection.node.to_string(ref_pool, tab);
                let body = for_decl.body.to_pretty(tab + 1, true, ref_pool);
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
                switch.condition.node.to_string(ref_pool, tab),
                switch.cases.iter().enumerate().map(|(i, stmt)| {
                    let is_last = i == switch.cases.len() - 1;
                    format!("\t{} => {}", stmt.if_expr.node.to_string(ref_pool, tab), stmt.do_fn.to_pretty(tab, is_last, ref_pool))
                }).collect::<Vec<_>>().join("\n")
            ),
        }
    }
}

impl PrettyPrint for CaseDoKind {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        match self {
            CaseDoKind::Block(block) => block.to_pretty(tab, is_last, ref_pool),
            CaseDoKind::Expression(spanned) => spanned.node.to_string(ref_pool, tab),
        }
    }
}

impl PrettyPrint for ElseKind {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            ElseKind::ElseIf(if_decl) => format!(
                "{}Else If >> else if {}\n{}",
                prefix,
                if_decl.node.condition.node.to_string(ref_pool, tab),
                if_decl.node.body.to_pretty(tab + 1, true, ref_pool),
            ),
            ElseKind::Else(block) => {
                let body = block.node.to_pretty(tab + 1, true, ref_pool);
                format!("{}Else >> else\n{}", prefix, body)
            }
        }
    }
}

impl PrettyPrint for IfDecl {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let cond = self.condition.node.to_string(ref_pool, tab);
        let mut output = vec![format!("{}If >> if ({})", prefix, cond)];

        output.push(self.body.to_pretty(tab + 1, true, ref_pool));

        for (i, e) in self.else_branchs.iter().enumerate() {
            let last = i == self.else_branchs.len() - 1;
            output.push(e.node.to_pretty(tab + 1, last, ref_pool));
        }

        output.join("\n")
    }
}

impl PrettyPrint for GlobalKind {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let content = match self {
            GlobalKind::ClassDecl(class) => class.to_pretty(tab, true, ref_pool),
            GlobalKind::StructDecl(struc) => struc.to_pretty(tab, true, ref_pool),
            GlobalKind::TraitDecl(trait_decl) => trait_decl.borrow(ref_pool).to_pretty(tab, true, ref_pool),
            GlobalKind::TraitImpl(impl_block) => impl_block.to_pretty(tab, true, ref_pool),
            GlobalKind::FuncDecl(fn_decl) => fn_decl.to_pretty(tab, true, ref_pool),
            GlobalKind::ExtFuncDecl(fn_decl) => fn_decl.to_pretty(tab, true, ref_pool),
            GlobalKind::VarDecl(var) => var.to_pretty(tab, true, ref_pool),
            GlobalKind::EnumDecl(enum_decl) => enum_decl.to_pretty(tab, true, ref_pool),
            GlobalKind::UnionDecl(union_decl) => union_decl.to_pretty(tab, true, ref_pool),
            GlobalKind::TypeEnumDecl(type_enum) => type_enum.to_pretty(tab, true, ref_pool),
        };
        format!("{}{}", prefix, content)
    }
}

impl PrettyPrint for MultiRefReadGuard<InnerTraitDecl> {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let header = format!("{}Trait {} >>", prefix, self.name.0);

        let mut methods = self.methodes
            .iter()
            .enumerate()
            .map(|(i, sig)| {
                let last = i == self.methodes.len() - 1;
                let inner_prefix = tree_prefix(tab + 1, last);
                format!("{} {};", inner_prefix, sig.to_string(ref_pool))
            })
            .join("\n");

        if methods.is_empty() {
            methods = prefix;
        }

        format!("{}\n{}", header, methods)
    }
}

impl PrettyPrint for TypeEnumDeclRef {
    fn to_pretty(&self, _tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
        let this = self.borrow(ref_pool);
        let types = this.types.iter().map(|t| t.to_string(ref_pool)).join(", ");
        format!("TypeEnum {} = [{}];", this.name.0, types)
    }
}

impl PrettyPrint for UnionDeclRef {
    fn to_pretty(&self, tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
        let this = self.borrow(ref_pool);
        let indent_str = indent(tab);
        let variants = this.variants
            .iter()
            .map(|v| format!("{}{}{}", indent_str, v.name.0, v.field.to_string(ref_pool)))
            .join(",\n");
        format!("Union {} >>\n{}", this.name.0, variants)
    }
}

impl PrettyPrint for EnumDeclRef {
    fn to_pretty(&self, _tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
        let this = self.borrow(ref_pool);
        let variants = this.variants
            .iter()
            .map(|v| format!("{}({:?})", v.name.0, v.value))
            .join(", ");
        format!("Enum {} >> [{}]", this.name.0, variants)
    }
}

impl PrettyPrint for TraitImpl {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let mut methods = self.methodes
            .iter()
            .map(|fn_decl| fn_decl.node.to_pretty(tab + 1, true, ref_pool))
            .join("\n\n");

        if methods.is_empty() {
            methods = tree_prefix(tab, is_last);
        }

        format!("Impl {} for {} >>\n{}", self.trait_name.0, self.for_type.to_string(ref_pool), methods)
    }
}

impl PrettyPrint for ClassDeclRef {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let this = self.borrow(ref_pool);
        
        let prefix = tree_prefix(tab, is_last);
        let generics = if this.generics.is_empty() {
            "".to_string()
        } else {
            format!("<{}>", this.generics.iter().map(|g| g.to_string(ref_pool)).join(", "))
        };
        let header = format!("{}Class {}{} >>", prefix, this.name.0, generics);

        let last_index = this.fields.len().saturating_sub(1);
        let is_empty = this.methodes.is_empty();

        let fields = this.fields.iter().enumerate().map(|(i, f)| {
            let is_last_field = i == last_index && is_empty;
            let field_prefix = tree_prefix(tab + 1, is_last_field);
            let access = match (&f.node.vis.get, &f.node.vis.set) {
                (Some(Visibility::Public), Some(Visibility::Public)) => "{Get;Set;}",
                (Some(Visibility::Public), _) => "{Get;}",
                (_, Some(Visibility::Public)) => "{Set;}",
                _ => "",
            };
            let default = f.node.default_value
                .as_ref()
                .map(|v| format!(" = {}", v.node.to_string(ref_pool, tab)))
                .unwrap_or_default();
            format!("{}{} {}: {}{}", field_prefix, access, f.node.name.0, f.node.ty.to_string(ref_pool), default)
        });


        let len = this.methodes.len();
        let methods = this.methodes.iter().enumerate().map(|(i, m)| {
            let is_last_method = i == len - 1;
            m.node.to_pretty(tab + 1, is_last_method, ref_pool)
        });

        let mut body = fields.chain(methods).join("\n");
        if body.is_empty() {
            body = prefix;
        }

        format!("{}\n{}", header, body)
    }
}

impl PrettyPrint for FnDeclKind {

    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        match self {
            FnDeclKind::InternalFn(node_ref) => node_ref.to_string(ref_pool),
            FnDeclKind::Fn(fn_decl) => fn_decl.to_pretty(tab, is_last, ref_pool),
            FnDeclKind::InternalCtor(multi_ref) => multi_ref.to_string(ref_pool),
            FnDeclKind::Ctor(fn_decl) => fn_decl.to_pretty(tab, is_last, ref_pool),
            FnDeclKind::ExtFn(ext_fn_decl) => ext_fn_decl.to_pretty(tab, is_last, ref_pool),
        }
    }
}

impl PrettyPrint for StructDeclRef {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let header = format!("{}Struct {}{} >>", prefix, self.borrow(ref_pool).name.0, 
            if self.borrow(ref_pool).generics.is_empty() {
                "".to_string()
            } else {
                format!("<{}>", self.borrow(ref_pool).generics.iter().map(|g| g.to_string(ref_pool)).join(", "))
            });

        let mut body = self.borrow(ref_pool).fields
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let last = i == self.borrow(ref_pool).fields.len() - 1;
                let inner_prefix = tree_prefix(tab + 1, last);
                if let Some(value) = &f.node.default_value {
                    format!("{}{} {} {} = {}", inner_prefix, f.node.ty.to_string(ref_pool), f.node.name.0, f.node.vis.to_pretty(0, false, ref_pool), value.node.to_string(ref_pool, tab))
                } else {
                    format!("{}{} {} {}", inner_prefix, f.node.ty.to_string(ref_pool), f.node.name.0, f.node.vis.to_pretty(0, false, ref_pool))
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
    fn to_pretty(&self, _tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
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
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let sig = self.signature.to_string(ref_pool);
        let body = self.body.to_pretty(tab + 1, true, ref_pool); 
        format!("{}FnDecl >> {}\n{}", prefix, sig, body)
    }
}

impl PrettyPrint for ExtFnDecl {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        let sig = self.signature.to_string(ref_pool);
        let body = self.body.to_pretty(tab + 1, true, ref_pool); 
        format!("{}ExtFnDecl >> {}\n{}", prefix, sig, body)
    }
}

impl PrettyPrint for Block {
    fn to_pretty(&self, tab: usize, _is_last: bool, ref_pool: &MultiRefPool) -> String {
        self.statments.iter().enumerate().map(|(i, stmt)| {
            let is_last = i == self.statments.len() - 1;
            stmt.node.to_pretty(tab, is_last, ref_pool)
        }).collect::<Vec<_>>().join("\n")
    }
}

impl PrettyPrint for VariableKind {
    fn to_pretty(&self, tab: usize, is_last: bool, ref_pool: &MultiRefPool) -> String {
        let prefix = tree_prefix(tab, is_last);
        match self {
            VariableKind::Variable(node_ref) => {
                let node = node_ref.borrow(ref_pool);
                let init = match &node.initializer {
                    Some(expr) => format!(" = {}", expr.node.to_string(ref_pool, tab)),
                    None => "".to_string(),
                };
                format!("{}Var {} {}{}", prefix, node.ty.to_string(ref_pool), node.name.0, init)
            },
            VariableKind::MultiVariable{vars, ty, initializer, lit_retention:_} => {
                
                let init = match &initializer {
                    Some(expr) => format!(" = {}", expr.node.to_string(ref_pool, tab)),
                    None => "".to_string(),
                };
                format!("{}Var {} ({}){}", prefix, ty.to_string(ref_pool), vars.iter().map(|(name, var)| format!("{}: {}", name.0, var.borrow(ref_pool).name.0)).join(","), init)
            },
        }
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











































