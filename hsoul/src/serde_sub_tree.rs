use std::fmt;
use std::fs::File;
use std::str::FromStr;
use serde::ser::SerializeSeq;
use ego_tree::{NodeRef, Tree};
use std::path::{Path, PathBuf};
use serde::de::{Visitor, SeqAccess};
use std::io::{BufReader, BufWriter};
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use crate::subfile_tree::{SubFileTree, TreeNode, TreeNodeKind};

type ResErr<T> = std::result::Result<T, Box<dyn std::error::Error>>;


#[derive(Debug, Clone, Serialize, Deserialize)]
struct SerializableNode {
    node: TreeNode,
    children_count: u32,
}

impl SubFileTree {

    pub fn get_all_file_paths(&self) -> Vec<PathBuf> {
        let mut result = Vec::with_capacity(self.files_amount);
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
                    result.push(PathBuf::from_str(&full_path).expect("should be path"));
                }
            }
        }

        result
    }

    fn flatten_tree(tree: &Tree<TreeNode>) -> Vec<SerializableNode> {
        let mut result = Vec::new();

        fn recurse(node: &NodeRef<TreeNode>, result: &mut Vec<SerializableNode>) {
            let children_count = node.children().count() as u32;
            result.push(SerializableNode {
                node: node.value().clone(),
                children_count,
            });
            for child in node.children() {
                recurse(&child, result);
            }
        }

        recurse(&tree.root(), &mut result);
        result
    }

    fn build_tree(nodes: &[SerializableNode]) -> Tree<TreeNode> {
        fn build_subtree(nodes: &[SerializableNode], start: usize, parent: &mut ego_tree::NodeMut<TreeNode>) -> usize {
            let root_data = &nodes[start];
            let mut pos = start + 1;
            for _ in 0..root_data.children_count {
                let mut child_node = parent.append(nodes[pos].node.clone());
                pos += build_subtree(nodes, pos, &mut child_node);
            }
            pos - start
        }

        let root_data = &nodes[0];
        let mut tree = Tree::new(root_data.node.clone());
        let mut root_node = tree.root_mut();
        let _ = build_subtree(nodes, 0, &mut root_node);
        tree
    }

    pub fn from_bin_file(path: &Path) -> ResErr<Self> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let this: SubFileTree = bincode::deserialize_from(reader)?;
        Ok(this)
    }

    pub fn save_to_bin_file(&self, path: &Path) -> ResErr<()> {
        let file = File::create(path)?;
        let writer = BufWriter::new(file);
        bincode::serialize_into(writer, self)?;
        Ok(())
    }

        pub fn to_tree_string(&self) -> String {
        let mut result = String::new();
        let root = self.tree.root();
        result.push_str(&root.value().name);
        result.push('\n');
        Self::build_tree_string(&root, "", &mut result);
        result
    }

    fn build_tree_string(
        node: &NodeRef<TreeNode>,
        prefix: &str,
        output: &mut String,
    ) {
        let children: Vec<_> = node.children().collect();
        let count = children.len();

        for (i, child) in children.into_iter().enumerate() {
            let is_last_child = i == count - 1;
            let connector = if is_last_child { "└── " } else { "├── " };
            output.push_str(prefix);
            output.push_str(connector);
            output.push_str(if child.value().is_public {"pub"} else {"priv"});
            output.push_str(" ");
            output.push_str(if child.value().kind == TreeNodeKind::File {"file"} else {"folder"});
            output.push_str(" ");
            output.push_str(&child.value().name);
            output.push('\n');

            let new_prefix = if is_last_child {
                format!("{prefix}    ")
            } else {
                format!("{prefix}│   ")
            };

            Self::build_tree_string(&child, &new_prefix, output);
        }
    }
}

impl Serialize for SubFileTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> 
    where S: Serializer {
        let nodes = Self::flatten_tree(&self.tree);
        let mut seq = serializer.serialize_seq(Some(nodes.len() + 1))?;
        seq.serialize_element(&self.files_amount)?;
        for node in nodes {
            seq.serialize_element(&node)?;
        }
        seq.end()
    }
}

impl<'de> Deserialize<'de> for SubFileTree {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> 
    where D: Deserializer<'de> {
        struct SubFileTreeVisitor;

        impl<'de> Visitor<'de> for SubFileTreeVisitor {
            type Value = SubFileTree;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("SubFileTree serialized as files_len followed by nodes array")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<SubFileTree, A::Error> 
            where A: SeqAccess<'de> {
                let files_len: usize = seq.next_element()?.ok_or_else(|| serde::de::Error::custom("Missing files_len"))?;

                let mut nodes = Vec::new();
                while let Some(node) = seq.next_element::<SerializableNode>()? {
                    nodes.push(node);
                }

                if nodes.is_empty() {
                    return Err(serde::de::Error::custom("No nodes found"));
                }

                let tree = SubFileTree::build_tree(&nodes);

                Ok(SubFileTree {
                    tree,
                    files_amount: files_len,
                     })
            }
        }

        deserializer.deserialize_seq(SubFileTreeVisitor)
    }
}













































