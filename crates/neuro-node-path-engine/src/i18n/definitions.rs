use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LemmaDefinition {
    pub canonical_id: String,
    pub term_en: String,
    pub term_de: String,
    pub definition_en: String,
    pub definition_de: String,
    pub category: String,
    pub examples: Vec<String>,
}

impl LemmaDefinition {
    pub fn new(canonical_id: String, term_en: String, term_de: String) -> Self {
        Self {
            canonical_id,
            term_en,
            term_de,
            definition_en: String::new(),
            definition_de: String::new(),
            category: String::new(),
            examples: Vec::new(),
        }
    }

    pub fn with_definitions(mut self, def_en: String, def_de: String) -> Self {
        self.definition_en = def_en;
        self.definition_de = def_de;
        self
    }

    pub fn with_category(mut self, category: String) -> Self {
        self.category = category;
        self
    }

    pub fn add_example(&mut self, example: String) {
        self.examples.push(example);
    }
}
