use ego_tree::{NodeId, Tree};
use serde::{Deserialize, Serialize};
use std::{fs::File, io::{BufReader, BufWriter}, num::NonZeroUsize, path::{Path}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubFileTree {
    tree: Tree<TreeNode>,
    current: NodeIdSerde,
    files_len: usize,
}

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>;
impl SubFileTree {
    pub fn new() -> Self {
        let tree = Tree::new(TreeNode::new_root());
        let current = NodeIdSerde{id: tree.root().id()};
        Self{tree, current, files_len: 0}
    }

    pub fn add_subfile(&mut self, name: &str, is_public: bool) {
        self.tree.get_mut(self.current.id)
            .expect("current in subfiletree not found")
            .append(TreeNode::new_subfile(name.into(), is_public));
        
        self.files_len += 1;
    }

    pub fn push_folder(&mut self, folder: &str, is_public: bool) {
        self.current.id = self.tree.get_mut(self.current.id)
            .expect("current in subfiletree not found")
            .append(TreeNode::new_subfile(folder.into(), is_public))
            .id();  
    }

    pub fn pop_folder(&mut self) -> bool {
        if let Some(parent_id) = self.tree.get(self.current.id).expect("current in subfiletree not found").parent() {
            self.current.id = parent_id.id();
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

    pub fn from_bin_file(path: &Path) -> ResErr<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let this: Self = bincode::deserialize_from(reader)?;
        Ok(this)
    }

    pub fn save_to_bin_file(&self, path: &Path) -> ResErr<()> {
        let file = File::open(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;

        Ok(())
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

    pub fn new_root() -> Self {
        Self { name: "root".into(), is_public: true, kind: TreeNodeKind::Folder }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TreeNodeKind {
    Folder,
    File,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NodeIdSerde {
    pub id: NodeId
}

impl NodeIdSerde {
    fn new() -> Self {
        let ptr: *const NonZeroUsize = &NonZeroUsize::new(1).unwrap();
        let id = unsafe {
            let id_ptr = ptr as *const NodeId;
            *id_ptr
        };

        Self{id}
    }
}

impl Serialize for NodeIdSerde {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer {
        1usize.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NodeIdSerde {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de> {
        Ok(NodeIdSerde::new())
    }
}

















