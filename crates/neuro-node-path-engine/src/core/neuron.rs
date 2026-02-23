use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Neuron {
    pub id: String,
    pub activation_level: f64,
    pub metadata: NeuronMetadata,
    pub created_at: DateTime<Utc>,
    pub last_fired: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeuronMetadata {
    pub label: String,
    pub category: String,
    pub weight: f64,
    pub bias: f64,
    pub properties: std::collections::HashMap<String, String>,
}

impl Neuron {
    pub fn new(label: String, category: String, weight: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            activation_level: 0.0,
            metadata: NeuronMetadata {
                label,
                category,
                weight,
                bias: 0.5,
                properties: std::collections::HashMap::new(),
            },
            created_at: Utc::now(),
            last_fired: None,
        }
    }

    pub fn fire(&mut self, signal_strength: f64) -> f64 {
        self.activation_level = (self.activation_level + signal_strength).min(1.0).max(0.0);
        self.last_fired = Some(Utc::now());
        self.apply_activation_function()
    }

    fn apply_activation_function(&self) -> f64 {
        1.0 / (1.0 + (-self.activation_level).exp())
    }

    pub fn reset(&mut self) {
        self.activation_level = 0.0;
    }
}
