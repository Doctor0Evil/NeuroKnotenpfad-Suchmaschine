use neuro_node_path_engine::{
    EngineConfig, NeuroNodePathEngine, KnotenlexikonStore,
    repository::CodeRepository,
};
use std::path::PathBuf;
use tracing_subscriber;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = EngineConfig {
        max_depth: 32,
        enable_clustering: true,
        enable_audit: true,
        audit_retention_days: 365,
        supported_languages: vec!["en".to_string(), "de".to_string(), "es".to_string()],
        parallel_traversal: true,
    };

    let mut engine = NeuroNodePathEngine::new(config)?;
    let knotenlexikon = KnotenlexikonStore::default();
    engine.set_lemma_store(knotenlexikon);

    let repo_path = PathBuf::from("./target_repository");
    let repository = CodeRepository::new(repo_path)?;
    
    engine.index_repository(&repository).await?;

    let query = "neural clustering patterns in async module";
    let result = engine.query(query).await?;
    
    println!("Query: {}", query);
    println!("Result: {:#?}", result);

    Ok(())
}
