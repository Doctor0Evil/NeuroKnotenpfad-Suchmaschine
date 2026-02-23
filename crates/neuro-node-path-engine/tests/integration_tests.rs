#[cfg(test)]
mod integration_tests {
    use neuro_node_path_engine::{
        EngineConfig, NeuroNodePathEngine, KnotenlexikonStore,
        repository::CodeRepository,
    };
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_engine_initialization() {
        let config = EngineConfig::default();
        let engine = NeuroNodePathEngine::new(config);
        assert!(engine.is_ok());
    }

    #[tokio::test]
    async fn test_knotenlexikon_store() {
        let store = KnotenlexikonStore::new();
        let entry = store.get_entry("neuro_node_path_engine");
        assert!(entry.is_some());

        let entry = entry.unwrap();
        assert_eq!(entry.english_label, "NeuroNodePath Engine");
        assert_eq!(entry.german_label, "NeuroKnotenpfad-Suchmaschine");
    }

    #[test]
    fn test_lemma_search() {
        let store = KnotenlexikonStore::new();
        let results = store.search_by_german("Knoten");
        assert!(!results.is_empty());
    }
}
