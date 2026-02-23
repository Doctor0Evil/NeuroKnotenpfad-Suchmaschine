use crate::{core::Node, engine::QueryContext};
use std::collections::VecDeque;

pub struct PathResolver {
    max_depth: usize,
}

impl PathResolver {
    pub fn new() -> Self {
        Self { max_depth: 32 }
    }

    pub fn resolve(&self, context: &QueryContext, nodes: &[Node]) -> anyhow::Result<Vec<String>> {
        let mut path = Vec::new();
        let mut queue = VecDeque::new();

        let start_nodes: Vec<_> = nodes
            .iter()
            .filter(|n| context.query.contains(&n.name))
            .collect();

        for node in start_nodes {
            queue.push_back((node.id.clone(), 0));
        }

        while let Some((node_id, depth)) = queue.pop_front() {
            if depth > self.max_depth {
                break;
            }

            path.push(node_id.clone());

            for node in nodes {
                if node.parent_id.as_ref() == Some(&node_id) {
                    queue.push_back((node.id.clone(), depth + 1));
                }
            }
        }

        Ok(path)
    }
}

impl Default for PathResolver {
    fn default() -> Self {
        Self::new()
    }
}
