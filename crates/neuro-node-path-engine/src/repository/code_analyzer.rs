use regex::Regex;
use std::collections::HashMap;

pub struct CodeAnalyzer {
    patterns: Vec<(Regex, String)>,
}

impl CodeAnalyzer {
    pub fn new() -> Self {
        let patterns = vec![
            (
                Regex::new(r"(?m)^(pub\s+)?async\s+fn\s+(\w+)").unwrap(),
                "async_function".to_string(),
            ),
            (
                Regex::new(r"(?m)^(pub\s+)?fn\s+(\w+)").unwrap(),
                "function".to_string(),
            ),
            (
                Regex::new(r"(?m)^(pub\s+)?(struct|enum|trait)\s+(\w+)").unwrap(),
                "data_structure".to_string(),
            ),
            (
                Regex::new(r"(?m)^mod\s+(\w+)").unwrap(),
                "module".to_string(),
            ),
        ];

        Self { patterns }
    }

    pub fn analyze_code(&self, code: &str) -> HashMap<String, Vec<String>> {
        let mut entities = HashMap::new();

        for (pattern, entity_type) in &self.patterns {
            for caps in pattern.captures_iter(code) {
                let name = caps
                    .get(caps.len() - 1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                if !name.is_empty() {
                    entities
                        .entry(entity_type.clone())
                        .or_insert_with(Vec::new)
                        .push(name);
                }
            }
        }

        entities
    }
}

impl Default for CodeAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
