use std::collections::HashMap;

pub struct Translator {
    translations: HashMap<(String, String), String>,
}

impl Translator {
    pub fn new() -> Self {
        let mut translator = Self {
            translations: HashMap::new(),
        };
        translator.populate_translations();
        translator
    }

    fn populate_translations(&mut self) {
        self.add_translation("en", "de", "neural", "neural");
        self.add_translation("en", "de", "cluster", "Cluster");
        self.add_translation("en", "de", "node", "Knoten");
        self.add_translation("en", "de", "path", "Pfad");
        self.add_translation("en", "de", "interface", "Schnittstelle");
        self.add_translation("en", "de", "audit", "Überprüfung");
        self.add_translation("en", "de", "validation", "Validierung");
        self.add_translation("en", "de", "channel", "Kanal");
        self.add_translation("en", "de", "repository", "Quelle");
    }

    pub fn add_translation(&mut self, from: &str, to: &str, source: &str, target: &str) {
        self.translations
            .insert((from.to_string(), to.to_string(), source.to_string()), target.to_string());
    }

    pub fn translate(&self, from: &str, to: &str, term: &str) -> Option<String> {
        self.translations
            .get(&(from.to_string(), to.to_string(), term.to_string()))
            .cloned()
    }
}

impl Default for Translator {
    fn default() -> Self {
        Self::new()
    }
}
