use serde::Serialize;

pub struct Serializer;

impl Serializer {
    pub fn to_json<T: Serialize>(data: &T) -> anyhow::Result<String> {
        Ok(serde_json::to_string_pretty(data)?)
    }

    pub fn to_json_compact<T: Serialize>(data: &T) -> anyhow::Result<String> {
        Ok(serde_json::to_string(data)?)
    }

    pub fn from_json<'a, T: serde::de::DeserializeOwned>(json: &'a str) -> anyhow::Result<T> {
        Ok(serde_json::from_str(json)?)
    }
}
