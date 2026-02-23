use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interface {
    pub id: String,
    pub name: String,
    pub interface_type: InterfaceType,
    pub exposed_methods: Vec<MethodSignature>,
    pub input_schema: serde_json::Value,
    pub output_schema: serde_json::Value,
    pub metadata: InterfaceMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum InterfaceType {
    QueryInterface,
    AuditInterface,
    ValidationInterface,
    ClusterInterface,
    ChannelInterface,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MethodSignature {
    pub name: String,
    pub parameters: Vec<(String, String)>,
    pub return_type: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InterfaceMetadata {
    pub version: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub properties: HashMap<String, serde_json::Value>,
}

impl Interface {
    pub fn new(name: String, interface_type: InterfaceType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            interface_type,
            exposed_methods: Vec::new(),
            input_schema: serde_json::json!({}),
            output_schema: serde_json::json!({}),
            metadata: InterfaceMetadata {
                version: "1.0.0".to_string(),
                created_at: chrono::Utc::now(),
                properties: HashMap::new(),
            },
        }
    }

    pub fn add_method(&mut self, method: MethodSignature) {
        self.exposed_methods.push(method);
    }

    pub fn with_schemas(
        mut self,
        input: serde_json::Value,
        output: serde_json::Value,
    ) -> Self {
        self.input_schema = input;
        self.output_schema = output;
        self
    }
}
