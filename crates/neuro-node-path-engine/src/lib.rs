pub mod core;
pub mod engine;
pub mod repository;
pub mod i18n;
pub mod validation;
pub mod utils;

pub use core::{node::Node, cluster::Cluster, channel::NeuralChannel, interface::Interface};
pub use engine::{NeuroNodePathEngine, QueryContext};
pub use i18n::KnotenlexikonStore;
pub use validation::DualPathValidator;

#[derive(Debug, Clone)]
pub struct EngineConfig {
    pub max_depth: usize,
    pub enable_clustering: bool,
    pub enable_audit: bool,
    pub audit_retention_days: u32,
    pub supported_languages: Vec<String>,
    pub parallel_traversal: bool,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            max_depth: 32,
            enable_clustering: true,
            enable_audit: true,
            audit_retention_days: 365,
            supported_languages: vec!["en".to_string(), "de".to_string()],
            parallel_traversal: true,
        }
    }
}
