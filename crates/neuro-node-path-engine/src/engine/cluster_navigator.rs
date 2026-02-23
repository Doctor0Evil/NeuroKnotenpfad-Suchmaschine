use crate::{core::Cluster, engine::QueryContext};
use dashmap::DashMap;
use std::sync::Arc;

pub struct ClusterNavigator {
    visited_clusters: Vec<String>,
}

impl ClusterNavigator {
    pub fn new() -> Self {
        Self {
            visited_clusters: Vec::new(),
        }
    }

    pub fn navigate(
        &self,
        context: &QueryContext,
        clusters: &Arc<DashMap<String, Cluster>>,
    ) -> anyhow::Result<Vec<String>> {
        let mut cluster_path = Vec::new();

        for cluster_ref in clusters.iter() {
            let cluster = cluster_ref.value();
            if context.query.contains(&cluster.name) {
                cluster_path.push(cluster.id.clone());
            }
        }

        Ok(cluster_path)
    }

    pub fn mark_visited(&mut self, cluster_id: String) {
        self.visited_clusters.push(cluster_id);
    }
}

impl Default for ClusterNavigator {
    fn default() -> Self {
        Self::new()
    }
}
