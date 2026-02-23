use crate::engine::QueryContext;
use regex::Regex;
use std::collections::HashMap;

pub struct QueryProcessor {
    query_patterns: Vec<(Regex, String)>,
}

impl QueryProcessor {
    pub fn new() -> Self {
        let patterns = vec![
            (
                Regex::new(r"(?i)(neural|neuro)").unwrap(),
                "neural_keyword".to_string(),
            ),
            (
                Regex::new(r"(?i)(cluster|clust)").unwrap(),
                "cluster_keyword".to_string(),
            ),
            (
                Regex::new(r"(?i)(path|route)").unwrap(),
                "path_keyword".to_string(),
            ),
            (
                Regex::new(r"(?i)(channel|interface)").unwrap(),
                "channel_keyword".to_string(),
            ),
        ];

        Self {
            query_patterns: patterns,
        }
    }

    pub fn parse_query(&self, context: &QueryContext) -> HashMap<String, Vec<String>> {
        let mut tokens = HashMap::new();

        for (pattern, key) in &self.query_patterns {
            if pattern.is_match(&context.query) {
                tokens.entry(key.clone())
                    .or_insert_with(Vec::new)
                    .push(context.query.clone());
            }
        }

        tokens
    }

    pub fn extract_entities(&self, query: &str) -> Vec<String> {
        query
            .split_whitespace()
            .map(|s| s.to_lowercase())
            .filter(|s| s.len() > 3)
            .collect()
    }
}

impl Default for QueryProcessor {
    fn default() -> Self {
        Self::new()
    }
}
