use ego_tree::{NodeId, Tree};
use std::path::{Path, PathBuf};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone)]
pub struct SubFileTree {
    pub tree: Tree<TreeNode>,
    pub files_len: usize,
}

#[derive(Debug, Clone)]
pub struct SubFileTreeBuilder {
    tree: Tree<TreeNode>,
    current: NodeId,
    files_len: usize,
    current_dir: PathBuf,
}

impl SubFileTreeBuilder {
    pub fn new(root: String) -> Self {
        let tree = Tree::new(TreeNode::new_root(root.clone()));
        let current = tree.root().id();
        let mut current_dir = PathBuf::new();
        current_dir.push(root);
        Self{tree, current, files_len: 0, current_dir }
    }

    pub fn add_subfile<S: Into<String> + AsRef<Path>>(&mut self, name: S, is_public: bool) {
        self.tree.get_mut(self.current)
            .expect("current in subfiletree not found")
            .append(TreeNode::new_subfile(name.into(), is_public));
        
        self.files_len += 1;
    }

    pub fn get_current_dir(&self) -> PathBuf {
        self.current_dir.clone()
    }

    pub fn push_folder<S: Into<String> + AsRef<Path>>(&mut self, folder: S, is_public: bool) {
        self.current_dir.push(&folder);

        self.current = self.tree.get_mut(self.current)
            .expect("current in subfiletree not found")
            .append(TreeNode::new_folder(folder.into(), is_public))
            .id();  

    }

    pub fn pop_folder(&mut self) -> bool {
        if !self.current_dir.pop() {
            return false;
        }

        if let Some(parent_id) = self.tree.get(self.current).expect("current in subfiletree not found").parent() {
            self.current = parent_id.id();
            true
        } else {
            false
        }
    }

    pub fn get_all_file_paths(&self) -> Vec<String> {
        let mut result = Vec::with_capacity(self.files_len);
        let root = self.tree.root();
        let mut stack = Vec::new();

        stack.push((root, Vec::new()));

        while let Some((node, path)) = stack.pop() {
            let node_value = node.value();

            match node_value.kind {
                TreeNodeKind::Folder => {
                    let mut new_path = path.clone();
                    new_path.push(node_value.name.clone());

                    for child in node.children().rev() {
                        stack.push((child, new_path.clone()));
                    }
                }
                TreeNodeKind::File => {
                    let mut full_path = String::new();
                    if !path.is_empty() {
                        full_path.push_str(&path.join("/"));
                        full_path.push('/');
                    }
                    full_path.push_str(&node_value.name);
                    result.push(full_path);
                }
            }
        }

        result
    }

    pub fn consume_to_sub_file_tree(self) -> SubFileTree {
        SubFileTree { tree: self.tree, files_len: self.files_len }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreeNode {
    pub name: String, 
    pub is_public: bool,
    pub kind: TreeNodeKind, 
}

impl TreeNode {
    pub fn new_subfile(name: String, is_public: bool) -> Self {
        Self { name, is_public, kind: TreeNodeKind::File }
    }

    pub fn new_folder(name: String, is_public: bool) -> Self {
        Self { name, is_public, kind: TreeNodeKind::Folder }
    } 

    pub fn new_root(name: String) -> Self {
        Self { name, is_public: true, kind: TreeNodeKind::Folder }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TreeNodeKind {
    Folder,
    File,
}






