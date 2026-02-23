use std::collections::HashMap;

pub struct ConsensusValidator {
    threshold: f64,
}

impl ConsensusValidator {
    pub fn new(threshold: f64) -> Self {
        Self { threshold }
    }

    pub fn validate_paths(&self, paths: Vec<Vec<String>>) -> bool {
        if paths.is_empty() {
            return false;
        }

        let mut counts: HashMap<String, usize> = HashMap::new();
        let mut total = 0;

        for path in paths {
            for node in path {
                *counts.entry(node).or_insert(0) += 1;
                total += 1;
            }
        }

        let agreement_ratio = counts.values().max().copied().unwrap_or(0) as f64 / total as f64;
        agreement_ratio >= self.threshold
    }
}

impl Default for ConsensusValidator {
    fn default() -> Self {
        Self::new(0.66)
    }
}
