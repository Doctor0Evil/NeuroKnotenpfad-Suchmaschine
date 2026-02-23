use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};
use hex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub timestamp: DateTime<Utc>,
    pub query: String,
    pub node_path: Vec<String>,
    pub cluster_path: Vec<String>,
    pub hash: String,
    pub previous_hash: Option<String>,
}

pub struct AuditTrail {
    entries: Vec<AuditEntry>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn log_query(
        &mut self,
        context: &crate::engine::QueryContext,
        node_path: &[String],
        cluster_path: &[String],
    ) -> anyhow::Result<Vec<String>> {
        let previous_hash = self.entries.last().map(|e| e.hash.clone());

        let entry_data = format!(
            "{}:{}:{}",
            context.timestamp,
            node_path.join(","),
            cluster_path.join(",")
        );

        let mut hasher = Sha256::new();
        hasher.update(entry_data);
        let hash = hex::encode(hasher.finalize());

        let entry = AuditEntry {
            timestamp: context.timestamp,
            query: context.query.clone(),
            node_path: node_path.to_vec(),
            cluster_path: cluster_path.to_vec(),
            hash: hash.clone(),
            previous_hash,
        };

        self.entries.push(entry);

        let audit_path: Vec<String> = self.entries.iter().map(|e| e.hash.clone()).collect();
        Ok(audit_path)
    }

    pub fn verify_integrity(&self) -> bool {
        for i in 1..self.entries.len() {
            if self.entries[i].previous_hash != Some(self.entries[i - 1].hash.clone()) {
                return false;
            }
        }
        true
    }

    pub fn get_entries(&self) -> &[AuditEntry] {
        &self.entries
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}
