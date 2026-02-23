use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuralChannel {
    pub id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub weight: f64,
    pub bandwidth: f64,
    pub latency_ms: f64,
    pub signal_type: SignalType,
    pub metadata: ChannelMetadata,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SignalType {
    DataFlow,
    ControlFlow,
    DependencyLink,
    CallGraph,
    Bidirectional,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub signal_count: u64,
    pub properties: HashMap<String, serde_json::Value>,
}

impl NeuralChannel {
    pub fn new(
        from_node_id: String,
        to_node_id: String,
        signal_type: SignalType,
    ) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            from_node_id,
            to_node_id,
            weight: 1.0,
            bandwidth: 100.0,
            latency_ms: 1.0,
            signal_type,
            metadata: ChannelMetadata {
                created_at: chrono::Utc::now(),
                signal_count: 0,
                properties: HashMap::new(),
            },
            active: true,
        }
    }

    pub fn transmit_signal(&mut self, signal_strength: f64) -> f64 {
        if !self.active {
            return 0.0;
        }
        self.metadata.signal_count += 1;
        signal_strength * self.weight
    }

    pub fn set_weight(&mut self, weight: f64) {
        self.weight = weight.max(0.0).min(1.0);
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }

    pub fn reactivate(&mut self) {
        self.active = true;
    }
}
