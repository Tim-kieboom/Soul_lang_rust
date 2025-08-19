use crate::prelude::*;
use my_macros::{CloneWithPool};
use serde::{Deserialize, Serialize};
use hsoul::subfile_tree::{SubFileTree, TreeNode, TreeNodeKind};
use std::{collections::{BTreeMap, HashMap}, path::{Component, Path, PathBuf}, process::exit, sync::Arc};
use crate::{errors::soul_error::{new_soul_error, SoulError, SoulErrorKind, SoulSpan}, steps::step_interfaces::i_parser::abstract_syntax_tree::{expression::{ExprKind, Expression, Ident}, literal::Literal, soul_type::{soul_type::SoulType, type_kind::TypeKind}, spanned::Spanned, staments::{enum_likes::{EnumDeclRef, TypeEnumDeclRef, UnionDeclRef}, function::{FnDecl, FnDeclKind, FunctionSignatureRef}, objects::{ClassDeclRef, StructDeclRef, TraitDeclRef}, statment::{SoulThis, VariableDecl, VariableRef}}}, utils::{serde_multi_ref::{MultiRef, MultiRefPool}, push::Push}};

pub type ScopeStack = InnerScopeBuilder<Vec<ScopeKind>>;
pub type TypeScopeStack = InnerScopeBuilder<TypeKind>;

pub type Scope = InnerScope<Vec<ScopeKind>>;
pub type TypeScope = InnerScope<TypeKind>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeBuilder {
    scopes: ScopeStack,
    types: Vec<TypeScope>,
    pub global_literal: ProgramMemmory,
    pub external_pages: ExternalPages,
    pub project_name: String,
    pub ref_pool: MultiRefPool,
}

#[derive(Debug, Hash, Clone, Copy, Serialize, Deserialize)]
pub struct ProgramMemmoryId(pub usize);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProgramMemmory {
    pub store: BTreeMap<Literal, ProgramMemmoryId>,
    pub last_id: ProgramMemmoryId,
}
impl ProgramMemmory {
    pub fn new() -> Self {
        Self { store: BTreeMap::new(), last_id: ProgramMemmoryId(0) }
    }

    pub fn insert(&mut self, entry: Literal) -> ProgramMemmoryId {
        if let Some(id) = self.store.get(&entry) {
            return *id;
        }
        
        let id = self.last_id;
        self.last_id.0 += 1;
        self.store.insert(entry, id);
        return id;
    }

    pub fn to_program_memory_name(this: &ProgramMemmoryId) -> Ident {
        Ident(format!("__soul_mem_{}", this.0))
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SoulPagePath(pub String);
impl SoulPagePath {
    pub fn from_path(path: &PathBuf) -> Self {
        let mut soul_path = String::new();
        let mut components = path.components().peekable();

        while let Some(component) = components.next() {
            
            if let Component::Normal(os_str) = component {
                if soul_path.len() > 0 {
                    soul_path.push_str("::");
                }

                if components.peek().is_none() {
                    let path = PathBuf::from(&os_str);
                    let stem = path.file_stem()
                        .map(|s| s.to_string_lossy())
                        .unwrap_or_else(|| os_str.to_string_lossy());
                    
                    soul_path.push_str(&stem);
                } 
                else {
                    soul_path.push_str(&os_str.to_string_lossy());
                }
            }
        }

        Self(soul_path)
    } 

    pub fn to_path_buf(&self, add_soul_extention: bool) -> PathBuf {
        let mut path = PathBuf::new();

        for token in self.0.split("::") {
            path.push(Path::new(token));
        }
        
        if add_soul_extention {
            append_extension(&path, "soul")
        }
        else {
            path
        }
    }

    pub fn get_last_name(&self) -> String {
        self.0
            .split("::")
            .last()
            .unwrap_or("")
            .to_string()
    }
}

fn append_extension(path: &Path, ext: &str) -> PathBuf {
    let mut os_string = path.as_os_str().to_owned();
    os_string.push(".");
    os_string.push(ext);
    PathBuf::from(os_string)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct IsInternalLib(pub Option<PathBuf>);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalPages {
    pub pages: HashMap<SoulPagePath, IsInternalLib>,
    pub books: HashMap<SoulPagePath, IsInternalLib>,
}

impl ExternalPages {
    pub fn new() -> Self { 
        Self{pages: HashMap::new(), books: HashMap::new()}
    }

    pub fn from_subfile_tree(subfile_tree: Arc<SubFileTree>) -> Self { 
        
        fn to_soul_path(path: &Vec<String>, node_value: &TreeNode) -> SoulPagePath {
            let mut full_path = String::new();
            if !path.is_empty() {
                full_path.push_str(&path.join("::"));
                full_path.push_str("::");
            }
            full_path.push_str(&node_value.name);
            SoulPagePath(full_path)
        }

        let root = subfile_tree.tree.root();
        let mut stack = Vec::new();
        
        let mut pages = HashMap::with_capacity(subfile_tree.files_len);
        let mut books = HashMap::with_capacity(subfile_tree.files_len);

        stack.push((root, Vec::new()));
        while let Some((node, path)) = stack.pop() {
            let node_value = node.value();

            match node_value.kind {
                TreeNodeKind::Folder => {
                    let mut new_path = path.clone();
                    new_path.push(node_value.name.clone());
                    books.insert(to_soul_path(&path, node_value), IsInternalLib(None));

                    for child in node.children().rev() {
                        stack.push((child, new_path.clone()));
                        books.insert(to_soul_path(&path, node_value), IsInternalLib(None));
                    }
                }
                TreeNodeKind::File => {
                    pages.insert(to_soul_path(&path, node_value), IsInternalLib(None));
                }
            }
        }

        Self{pages, books}
    }

    pub fn push_internal(&mut self, path: SoulPagePath, mut path_to_bin: PathBuf) {
        if self.pages.contains_key(&path) {
            return;
        }
        
        let mut path_buf = path.to_path_buf(false);
        self.pages.insert(path, IsInternalLib(Some(path_to_bin.clone())));
        
        while !path_buf.pop() && !path_to_bin.pop() {
            let path_exist = self.books.insert(
                SoulPagePath::from_path(&path_buf), 
                IsInternalLib(Some(path_to_bin.clone()))
            ).is_some();

            if path_exist {
                break;
            }
        }
    }

    pub fn push(&mut self, path: SoulPagePath) {
        if self.pages.contains_key(&path) {
            return;
        }

        let mut path_buf = path.to_path_buf(false);
        self.pages.insert(path, IsInternalLib(None));

        while !path_buf.pop() {
            let path_exist = self.books.insert(
                SoulPagePath::from_path(&path_buf), 
                IsInternalLib(None)
            ).is_some();

            if path_exist {
                break;
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnerScopeBuilder<T> {
    pub scopes: Vec<InnerScope<T>>,
    pub current: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InnerScope<T> {
    pub parent_index: Option<usize>,
    pub children: Vec<usize>,
    pub self_index: usize,

    pub symbols: HashMap<String, T>,

    pub visibility_mode: ScopeVisibility,
}

impl ScopeBuilder {
    pub fn new(external_books: ExternalPages, project_name: String, ref_pool: MultiRefPool) -> Self {
        Self { scopes: ScopeStack::new(), global_literal: ProgramMemmory::new(), project_name, types: vec![], external_pages: external_books, ref_pool }
    }

    pub fn fill_with_type_stack(&mut self, type_stack: TypeScopeStack) {
        self.types = type_stack.scopes;
    }

    pub fn __consume_to_tuple(self) -> (ScopeStack, Vec<TypeScope>, ProgramMemmory, ExternalPages, String, MultiRefPool) {
        let Self{scopes, types, global_literal, external_pages: external_books, project_name, ref_pool} = self;
        (scopes, types, global_literal, external_books, project_name, ref_pool)
    }

    pub fn get_scopes(&self) -> &InnerScopeBuilder<Vec<ScopeKind>> {
        &self.scopes
    }

    pub fn is_external_page_or_book(&self, path: &SoulPagePath) -> Option<IsInternalLib> {
        if let Some(res) = self.is_external_book(path) {
            Some(res)
        }
        else if let Some(res) = self.is_external_page(path) {
            Some(res)
        }
        else {
            None
        }
    }

    pub fn is_external_page(&self, path: &SoulPagePath) -> Option<IsInternalLib> {
        self.external_pages.pages.get(path).cloned()
    }

    pub fn is_external_book(&self, path: &SoulPagePath) -> Option<IsInternalLib> {
        self.external_pages.books.get(path).cloned()
    }

    pub fn get_types(&self) -> &Vec<InnerScope<TypeKind>> {
        &self.types
    }

    pub fn get_global_scope(&self) -> &InnerScope<Vec<ScopeKind>>{
        &self.scopes.scopes[InnerScopeBuilder::<()>::GLOBAL_SCOPE_INDEX]
    }

    pub fn get_global_types(&self) -> &InnerScope<TypeKind>{
        &self.types[InnerScopeBuilder::<()>::GLOBAL_SCOPE_INDEX]
    }

    pub fn current_scope(&self) -> &InnerScope<Vec<ScopeKind>> {
        &self.scopes.scopes[self.scopes.current]
    }

    pub fn current_index(&self) -> usize {
        self.scopes.current
    }

    pub fn push(&mut self, scope_visability: ScopeVisibility) {
        self.scopes.push_current(scope_visability);
    }

    pub fn remove_current(&mut self, span: SoulSpan) {
        self.scopes.remove_current(span);
    }

    pub fn push_from(&mut self, parent_index: usize, scope_visability: ScopeVisibility) {
        self.scopes.push(parent_index, scope_visability);
    }

    pub fn pop(&mut self, span: SoulSpan) {
        self.scopes.pop(span);
    }

    pub fn is_in_global(&self) -> bool {
        self.scopes.is_in_global()
    } 

    ///only looks in current scope
    pub fn flat_lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        self.scopes.flat_lookup(name)
    }

    ///looks in current scope and parent scopes of ScopeVisibilty is All
    pub fn lookup(&self, name: &str) -> Option<&Vec<ScopeKind>> {
        self.scopes.lookup(name)
    }

    pub fn add_this_type(&mut self, ty: TypeKind) {
        self.scopes.insert("This".into(), vec![ScopeKind::This(ty)]);
    }

    pub fn try_get_this_type(&self) -> Option<&TypeKind> {
        match self.scopes.lookup("This")?.last()? {
            ScopeKind::This(ty) => Some(ty),
            _ => unreachable!()
        }
    }

    pub fn add_function(&mut self, fn_decl: &FnDeclKind) {
        
        let mut signature_ref = fn_decl.get_signature().clone();
        let signature = signature_ref.owned_borrow(&self.ref_pool);

        if let Some(kinds) = self.scopes.flat_lookup_mut(&signature.node.name.0) {
            
            let possible_funcs = kinds.iter_mut().find(|kind| matches!(kind, ScopeKind::Functions(..)));
            if let Some(ScopeKind::Functions(funcs)) = possible_funcs {
                funcs.borrow_mut(&self.ref_pool).push(fn_decl.clone());
            }
            else {
                self.scopes.insert(
                    signature.node.name.0.clone(), 
                    vec![ScopeKind::Functions(OverloadedFunctions::new(vec![fn_decl.clone()], &mut self.ref_pool))]
                );
            }
        }
        else {
            self.scopes.insert(
                signature.node.name.0.clone(), 
                vec![ScopeKind::Functions(OverloadedFunctions::new(vec![fn_decl.clone()], &mut self.ref_pool))]
            );
        }
    } 

    pub fn insert(&mut self, name: String, kind: ScopeKind) {
        self.scopes.insert_to_vec(name, kind)
    } 

    pub fn add_variable(&mut self, var_decl: VariableDecl) -> Result<VariableRef, String> {
        
        let single_var = var_decl.name.0.clone();

        let var_ref = VariableRef::new(var_decl, &mut self.ref_pool);
        self.insert(single_var, ScopeKind::Variable(var_ref.clone()));
        return Ok(var_ref);
    }

    pub fn insert_this_variable(&mut self, this: Spanned<SoulThis>) {

        let this_var = ScopeKind::Variable(MultiRef::new(
            VariableDecl{
                name: Ident("this".into()), 
                ty: this.node.ty, 
                initializer: Some(Box::new(Expression::new(ExprKind::Empty, this.span))), 
                lit_retention: None,
            },
            &mut self.ref_pool
        ));
        
        self.scopes.insert_to_vec("this".into(), this_var);
    }

    pub fn insert_type(&mut self, name: String, kind: TypeKind) -> Result<(), String> {
        #[cfg(debug_assertions)]
        if self.scopes.current > self.types.len() -1 {
            return Ok(());
        }

        let types = &mut self.types[self.scopes.current];

        if types.get(&name).is_some() {
            return Err(format!("type: '{}' already exists", name));
        }

        types.symbols.insert(name, kind);
        Ok(())
    }  

    pub fn insert_global(&mut self, name: String, kind: ScopeKind) {
        self.scopes.insert_global_to_vec(name, kind)
    }

    pub fn lookup_type(&self, name: &str) -> Option<&TypeKind> {
        let mut current_index = Some(self.scopes.current);

        if self.types.is_empty() {
            return None;
        }

        while let Some(index) = current_index {
            #[cfg(debug_assertions)]
            if index > self.types.len() {
                break;
            }
            
            let scope = &self.types[index];

            if let Some(kind) = scope.get(name) {
                return Some(kind);
            }

            match scope.visibility_mode {
                ScopeVisibility::All => current_index = scope.parent_index,
                ScopeVisibility::GlobalOnly => {
                    current_index = if index == 0 { None } else { Some(0) };
                }
            }
        }

        None
    }
}

impl<T> InnerScopeBuilder<T> {
    pub const GLOBAL_SCOPE_INDEX: usize = 0; 

    pub fn new() -> Self {
        Self { 
            scopes: vec![InnerScope::<T>::new_global()],  
            current: Self::GLOBAL_SCOPE_INDEX 
        }
    }

    pub fn push(&mut self, parent_index: usize, scope_visability: ScopeVisibility) {
        self.current = self.scopes.len();
        self.scopes[parent_index].children.push(self.current);
        self.scopes.push(InnerScope::<T>::new_child(self.current, parent_index, scope_visability));
    }

    pub fn push_current(&mut self, scope_visability: ScopeVisibility) {
        let parent_index = self.current;
        self.current = self.scopes.len();
        self.scopes[parent_index].children.push(self.current);
        self.scopes.push(InnerScope::<T>::new_child(self.current, parent_index, scope_visability));
    }

    pub fn remove_current(&mut self, span: SoulSpan) {
        let current = self.current;
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
            let self_index = self.scopes[parent_index].children.iter().enumerate().find(|(_i, el)| **el == current).unwrap().0;
            self.scopes[parent_index].children.remove(self_index);
        } 
        else {
            println!("{}", new_soul_error(SoulErrorKind::UnexpectedEnd, span, "can not remove global scope").to_err_message().join("\n"));
            exit(1)
        }
    }

    pub fn pop(&mut self, span: SoulSpan) {
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
        } else {
            println!("{}", new_soul_error(SoulErrorKind::UnmatchedParenthesis, span, "somewhere in program there is a '}}' without a '{{' (probably near the '}}' before this one)").to_err_message().join("\n"));
            exit(1)
        }
    }

    pub fn try_pop(&mut self, span: SoulSpan) -> Result<(), SoulError> {
        if let Some(parent_index) = self.scopes[self.current].parent_index {
            self.current = parent_index;
            Ok(())
        } 
        else {
            Err(new_soul_error(SoulErrorKind::UnmatchedParenthesis, span, "somewhere in program there is a '}}' without a '{{' (probably near the '}}' before this one)"))
        }
    }

    pub fn is_in_global(&self) -> bool {
        self.current == Self::GLOBAL_SCOPE_INDEX
    } 

    pub fn flat_lookup(&self, name: &str) -> Option<&T> {
        let scope = &self.scopes[self.current];

        if let Some(kinds) = scope.get(name) {
            return Some(kinds);
        }

        None
    }
        
    pub fn flat_lookup_mut(&mut self, name: &str) -> Option<&mut T> {
        let scope = &mut self.scopes[self.current];

        if let Some(kinds) = scope.get_mut(name) {
            return Some(kinds);
        }

        None
    }

    pub fn lookup(&self, name: &str) -> Option<&T> {
        let mut current_index = Some(self.current);

        while let Some(index) = current_index {
            let scope = &self.scopes[index];

            if let Some(kinds) = scope.get(name) {
                return Some(kinds);
            }

            match scope.visibility_mode {
                ScopeVisibility::All => current_index = scope.parent_index,
                ScopeVisibility::GlobalOnly => {
                    current_index = if index == 0 { None } else { Some(0) };
                }
            }
        }

        None
    }

    /// Inserts a new symbol. 
    /// Returns `true` if the symbol was newly inserted; 
    /// returns `false` if a symbol with the same name already existed. 
    pub fn insert(&mut self, name: String, kind: T) -> bool {
        
        self.current_mut()
            .symbols
            .insert(name, kind)
            .is_none()
    }

    pub fn insert_to_vec<V>(&mut self, name: String, kind: V) 
    where 
        T: Push<V> + Default
    {
        self.current_mut()
            .symbols
            .entry(name)
            .or_default()
            .push(kind);
    }

    pub fn insert_global(&mut self, name: String, kind: T) {
        self.global_mut()
            .symbols
            .insert(name, kind);
    }

    pub fn insert_global_to_vec<V>(&mut self, name: String, kind: V) 
    where 
        T: Push<V> + Default
    {
        self.global_mut()
            .symbols
            .entry(name)
            .or_default()
            .push(kind);
    }

    pub fn current(&self) -> &InnerScope<T> {
        &self.scopes[self.current]
    }

    pub fn current_mut(&mut self) -> &mut InnerScope<T> {
        &mut self.scopes[self.current]
    }

    pub fn global_mut(&mut self) -> &mut InnerScope<T> {
        &mut self.scopes[Self::GLOBAL_SCOPE_INDEX]
    }
}

pub trait LookUpScope<T> {fn lookup(&self, name: &str, current: usize) -> Option<&T>;}
impl LookUpScope<TypeKind> for Vec<InnerScope<TypeKind>> {
    fn lookup(&self, name: &str, current: usize) -> Option<&TypeKind> {
        let mut current_index = Some(current);

        while let Some(index) = current_index {
            let scope = &self[index];

            if let Some(kinds) = scope.get(name) {
                return Some(kinds);
            }

            match scope.visibility_mode {
                ScopeVisibility::All => current_index = scope.parent_index,
                ScopeVisibility::GlobalOnly => {
                    current_index = if index == 0 { None } else { Some(0) };
                }
            }
        }

        None
    }
}

impl<T> InnerScope<T> {
    pub fn new_global() -> Self {
        Self {
            parent_index: None,
            children: vec![],
            self_index: 0,
            symbols: HashMap::new(),
            visibility_mode: ScopeVisibility::GlobalOnly, // does not matter for global
        }
    }

    pub fn new_child(self_index: usize, parent_index: usize, vis: ScopeVisibility) -> Self {
        Self {
            self_index,
            children: vec![],
            symbols: HashMap::new(),
            parent_index: Some(parent_index),
            visibility_mode: vis,
        }
    }

    pub fn get(&self, name: &str) -> Option<&T> {
        self.symbols.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut T> {
        self.symbols.get_mut(name)
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScopeVisibility {
    All,         // Can access child -> parent -> ... -> global
    GlobalOnly,  // Can only access global scope
}

#[derive(Debug, Clone, CloneWithPool, Serialize, Deserialize)]
pub enum ScopeKind {
    Invalid(),
    Variable(VariableRef),
    Struct(StructDeclRef),
    Class(ClassDeclRef),

    Trait(TraitDeclRef),

    Functions(OverloadedFunctions),
    NamedTupleCtor(NamedTupleCtor),

    This(TypeKind),
    Enum(EnumDeclRef),
    Union(UnionDeclRef),
    TypeEnum(TypeEnumDeclRef),
    TypeDefed(TypeDefedRef),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NamedTupleCtor {
    pub object_type: SoulType,
    pub values: HashMap<Ident, (SoulType, Option<Expression>)>
}

pub type TypeDefedRef = MultiRef<TypeDefed>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeDefed {
    pub name: Ident,
    pub from_type: SoulType, 
}

pub type OverloadedFunctions = MultiRef<Vec<FnDeclKind>>;

impl OverloadedFunctions {
    pub fn from_fn(decl: FnDecl, ref_pool: &mut MultiRefPool) -> Self {
        Self::new(vec![FnDeclKind::Fn(decl)], ref_pool)
    }

    pub fn from_internal_fn(sig: FunctionSignatureRef, ref_pool: &mut MultiRefPool) -> Self {
        Self::new(vec![FnDeclKind::InternalFn(sig)], ref_pool)
    }
}




















