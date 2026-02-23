use serde_json::Value;
use std::path::Path;

pub struct ModelLoader;

impl ModelLoader {
    pub fn load_json_model(path: &Path) -> anyhow::Result<Value> {
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content)?)
    }

    pub fn load_protocol(path: &Path) -> anyhow::Result<String> {
        std::fs::read_to_string(path).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn validate_model(model: &Value) -> bool {
        model.is_object() || model.is_array()
    }
}
