use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    pub name: String,
    pub node_type: NodeType,
    pub source_path: String,
    pub hash: String,
    pub depth: usize,
    pub parent_id: Option<String>,
    pub children: HashSet<String>,
    pub metadata: NodeMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NodeType {
    Repository,
    File,
    Function,
    Struct,
    Module,
    Protocol,
    Model,
    Interface,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeMetadata {
    pub language: String,
    pub signature: Option<String>,
    pub documentation: Option<String>,
    pub dependencies: Vec<String>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Node {
    pub fn new(name: String, node_type: NodeType, source_path: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            node_type,
            source_path,
            hash: String::new(),
            depth: 0,
            parent_id: None,
            children: HashSet::new(),
            metadata: NodeMetadata {
                language: String::new(),
                signature: None,
                documentation: None,
                dependencies: Vec::new(),
                properties: HashMap::new(),
            },
        }
    }

    pub fn add_child(&mut self, child_id: String) {
        self.children.insert(child_id);
    }

    pub fn remove_child(&mut self, child_id: &str) {
        self.children.remove(child_id);
    }

    pub fn with_hash(mut self, hash: String) -> Self {
        self.hash = hash;
        self
    }

    pub fn with_depth(mut self, depth: usize) -> Self {
        self.depth = depth;
        self
    }
}
