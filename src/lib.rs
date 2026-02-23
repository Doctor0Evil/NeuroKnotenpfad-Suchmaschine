//! CADSP Core: Cybernetic AI Design Synthesis Platform
//! NeuroNodePath Engine for biophysical code discovery and synthesis
//!
//! This crate provides:
//! - Safe repository scanning (GitHub API v3)
//! - Biophysical pattern detection in code
//! - NeuroNodePath traversal and validation
//! - ALN schema definition generation
//! - Cross-language compliance validation

pub mod ingestion;
pub mod biophysical_patterns;
pub mod neuro_node_path;
pub mod aln_schema;
pub mod compliance;
pub mod errors;

// Re-export main types
pub use ingestion::{RepositoryScanMetadata, RepositoryScanner};
pub use biophysical_patterns::{DiscoveredObject, BiophysicalIndicators};
pub use neuro_node_path::{NeuroNodePath, NodeCluster, NeurochanelInterface};
pub use aln_schema::{ALNObjectDefinition, ObjectDef};
pub use errors::{CADSPError, Result};

/// CADSP version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
