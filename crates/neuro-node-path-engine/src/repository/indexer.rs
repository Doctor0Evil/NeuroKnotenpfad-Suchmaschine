use crate::core::Node;
use std::collections::HashMap;

pub struct Indexer {
    index: HashMap<String, Vec<Node>>,
}

impl Indexer {
    pub fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    pub fn index_node(&mut self, node: Node) {
        self.index
            .entry(node.node_type.to_string())
            .or_insert_with(Vec::new)
            .push(node);
    }

    pub fn search_by_type(&self, node_type: &str) -> Vec<&Node> {
        self.index
            .get(node_type)
            .map(|nodes| nodes.iter().collect())
            .unwrap_or_default()
    }

    pub fn search_by_name(&self, name: &str) -> Vec<&Node> {
        self.index
            .values()
            .flat_map(|nodes| {
                nodes
                    .iter()
                    .filter(|n| n.name.contains(name))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl Default for Indexer {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for crate::core::node::NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            crate::core::node::NodeType::Repository => write!(f, "repository"),
            crate::core::node::NodeType::File => write!(f, "file"),
            crate::core::node::NodeType::Function => write!(f, "function"),
            crate::core::node::NodeType::Struct => write!(f, "struct"),
            crate::core::node::NodeType::Module => write!(f, "module"),
            crate::core::node::NodeType::Protocol => write!(f, "protocol"),
            crate::core::node::NodeType::Model => write!(f, "model"),
            crate::core::node::NodeType::Interface => write!(f, "interface"),
            crate::core::node::NodeType::Custom(s) => write!(f, "{}", s),
        }
    }
}
