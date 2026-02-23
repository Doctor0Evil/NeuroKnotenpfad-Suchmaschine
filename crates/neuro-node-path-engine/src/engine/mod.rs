pub mod query_processor;
pub mod path_resolver;
pub mod audit_trail;
pub mod cluster_navigator;

pub use query_processor::QueryProcessor;
pub use path_resolver::PathResolver;
pub use audit_trail::AuditTrail;
pub use cluster_navigator::ClusterNavigator;

use crate::{
    core::{Node, Cluster, NeuralChannel, Interface},
    i18n::KnotenlexikonStore,
    validation::DualPathValidator,
    EngineConfig,
};
use dashmap::DashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct QueryContext {
    pub query: String,
    pub language: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub request_id: String,
}

pub struct NeuroNodePathEngine {
    config: EngineConfig,
    nodes: Arc<DashMap<String, Node>>,
    clusters: Arc<DashMap<String, Cluster>>,
    channels: Arc<DashMap<String, NeuralChannel>>,
    interfaces: Arc<DashMap<String, Interface>>,
    audit_trail: Arc<RwLock<AuditTrail>>,
    path_resolver: PathResolver,
    query_processor: QueryProcessor,
    cluster_navigator: ClusterNavigator,
    knotenlexikon: Arc<RwLock<KnotenlexikonStore>>,
    dual_path_validator: DualPathValidator,
}

#[derive(Debug, serde::Serialize)]
pub struct QueryResult {
    pub request_id: String,
    pub query: String,
    pub node_path: Vec<String>,
    pub cluster_path: Vec<String>,
    pub channel_interfaces: Vec<String>,
    pub audit_path: Vec<String>,
    pub explanation_en: String,
    pub explanation_de: String,
    pub validation_status: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl NeuroNodePathEngine {
    pub fn new(config: EngineConfig) -> anyhow::Result<Self> {
        Ok(Self {
            config,
            nodes: Arc::new(DashMap::new()),
            clusters: Arc::new(DashMap::new()),
            channels: Arc::new(DashMap::new()),
            interfaces: Arc::new(DashMap::new()),
            audit_trail: Arc::new(RwLock::new(AuditTrail::new())),
            path_resolver: PathResolver::new(),
            query_processor: QueryProcessor::new(),
            cluster_navigator: ClusterNavigator::new(),
            knotenlexikon: Arc::new(RwLock::new(KnotenlexikonStore::default())),
            dual_path_validator: DualPathValidator::new(),
        })
    }

    pub fn set_lemma_store(&mut self, store: KnotenlexikonStore) {
        self.knotenlexikon = Arc::new(RwLock::new(store));
    }

    pub async fn index_repository(
        &mut self,
        repository: &crate::repository::CodeRepository,
    ) -> anyhow::Result<()> {
        let files = repository.scan_files().await?;
        
        for file in files {
            let node = Node::new(
                file.clone(),
                crate::core::node::NodeType::File,
                file,
            );
            self.nodes.insert(node.id.clone(), node);
        }

        Ok(())
    }

    pub async fn query(&self, query_str: &str) -> anyhow::Result<QueryResult> {
        let context = QueryContext {
            query: query_str.to_string(),
            language: "en".to_string(),
            timestamp: chrono::Utc::now(),
            request_id: uuid::Uuid::new_v4().to_string(),
        };

        let nodes_snapshot: Vec<_> = self.nodes
            .iter()
            .map(|ref_multi| ref_multi.clone())
            .collect();

        let node_path = self.path_resolver.resolve(&context, &nodes_snapshot)?;
        let cluster_path = self.cluster_navigator.navigate(&context, &self.clusters)?;
        
        let channel_interfaces: Vec<String> = self.channels
            .iter()
            .map(|ref_multi| ref_multi.id.clone())
            .collect();

        let mut audit_trail = self.audit_trail.write().await;
        let audit_path = audit_trail.log_query(&context, &node_path, &cluster_path)?;

        let (explanation_en, explanation_de) = self.generate_explanations(&node_path)?;

        let validation = self.dual_path_validator.validate(&node_path, &cluster_path)?;

        Ok(QueryResult {
            request_id: context.request_id,
            query: context.query,
            node_path,
            cluster_path,
            channel_interfaces,
            audit_path,
            explanation_en,
            explanation_de,
            validation_status: validation,
            timestamp: chrono::Utc::now(),
        })
    }

    fn generate_explanations(&self, _node_path: &[String]) -> anyhow::Result<(String, String)> {
        let en = "Query execution completed through neural node paths with cluster traversal and channel validation.".to_string();
        let de = "Abfrageausführung über Neuroknotenpfade mit Clusterdurchquerung und Kanalvalidierung abgeschlossen.".to_string();
        Ok((en, de))
    }
}
