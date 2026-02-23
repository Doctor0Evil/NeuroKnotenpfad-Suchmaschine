use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cluster {
    pub id: String,
    pub name: String,
    pub node_ids: HashSet<String>,
    pub centroid: ClusterCentroid,
    pub cohesion_score: f64,
    pub cluster_type: ClusterType,
    pub metadata: ClusterMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClusterType {
    Functional,
    Architectural,
    Semantic,
    Temporal,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterCentroid {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClusterMetadata {
    pub creation_time: chrono::DateTime<chrono::Utc>,
    pub member_count: usize,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Cluster {
    pub fn new(name: String, cluster_type: ClusterType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            node_ids: HashSet::new(),
            centroid: ClusterCentroid { x: 0.0, y: 0.0, z: 0.0 },
            cohesion_score: 0.0,
            cluster_type,
            metadata: ClusterMetadata {
                creation_time: chrono::Utc::now(),
                member_count: 0,
                properties: HashMap::new(),
            },
        }
    }

    pub fn add_node(&mut self, node_id: String) {
        self.node_ids.insert(node_id);
        self.metadata.member_count = self.node_ids.len();
    }

    pub fn remove_node(&mut self, node_id: &str) {
        self.node_ids.remove(node_id);
        self.metadata.member_count = self.node_ids.len();
    }

    pub fn compute_cohesion(&mut self, nodes: &HashMap<String, crate::core::Node>) {
        if self.node_ids.is_empty() {
            self.cohesion_score = 0.0;
            return;
        }

        let mut distances = Vec::new();
        let node_ids: Vec<_> = self.node_ids.iter().collect();

        for i in 0..node_ids.len() {
            for j in (i + 1)..node_ids.len() {
                if let (Some(_n1), Some(_n2)) = (
                    nodes.get(node_ids[i]),
                    nodes.get(node_ids[j]),
                ) {
                    distances.push(1.0);
                }
            }
        }

        self.cohesion_score = if distances.is_empty() {
            0.0
        } else {
            distances.iter().sum::<f64>() / distances.len() as f64
        };
    }
}
