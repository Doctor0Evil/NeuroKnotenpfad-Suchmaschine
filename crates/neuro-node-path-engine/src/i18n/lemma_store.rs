use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LemmaEntry {
    pub canonical_id: String,
    pub german_label: String,
    pub english_label: String,
    pub german_definition: String,
    pub english_definition: String,
    pub pronunciation_de: String,
    pub word_type: String,
    pub related_concepts: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct KnotenlexikonStore {
    entries: HashMap<String, LemmaEntry>,
}

impl KnotenlexikonStore {
    pub fn new() -> Self {
        let mut store = Self {
            entries: HashMap::new(),
        };
        store.populate_default_lemmas();
        store
    }

    fn populate_default_lemmas(&mut self) {
        self.add_entry(LemmaEntry {
            canonical_id: "neuro_node_path_engine".to_string(),
            german_label: "NeuroKnotenpfad-Suchmaschine".to_string(),
            english_label: "NeuroNodePath Engine".to_string(),
            german_definition: "Ein computergestütztes System, das Code-Repositorien, Modelle und Protokolle als Organismusnetz interpretiert".to_string(),
            english_definition: "A system that treats code repositories, models, and logs as an organism-like network of nodes, neurochannels, clusters, and interfaces".to_string(),
            pronunciation_de: "NOY-ro-KNOH-ten-pfaat ZOOKH-ma-SHEE-nuh".to_string(),
            word_type: "feminine, die NeuroKnotenpfad-Suchmaschine".to_string(),
            related_concepts: vec![
                "neuro_node_path".to_string(),
                "cluster_path".to_string(),
                "neuro_channel_interface".to_string(),
                "organism_network".to_string(),
                "dual_path_validation".to_string(),
            ],
        });

        self.add_entry(LemmaEntry {
            canonical_id: "neuro_node_path".to_string(),
            german_label: "Neuroknotenpfad".to_string(),
            english_label: "NeuroNodePath".to_string(),
            german_definition: "Ein nachvollziehbarer Pfad durch ein Netzwerk von miteinander verbundenen Knoten".to_string(),
            english_definition: "A traceable path through an interconnected network of nodes".to_string(),
            pronunciation_de: "NOY-ro-KNOH-ten-pfaat".to_string(),
            word_type: "masculine, der Neuroknotenpfad".to_string(),
            related_concepts: vec![
                "cluster_path".to_string(),
                "audit_path".to_string(),
            ],
        });

        self.add_entry(LemmaEntry {
            canonical_id: "cluster_path".to_string(),
            german_label: "Clusterpfad".to_string(),
            english_label: "ClusterPath".to_string(),
            german_definition: "Ein Pfad durch thematisch zusammenhängende Cluster".to_string(),
            english_definition: "A path through thematically related clusters".to_string(),
            pronunciation_de: "KLUS-ter-pfaat".to_string(),
            word_type: "masculine, der Clusterpfad".to_string(),
            related_concepts: vec!["cluster".to_string(), "neuro_node_path".to_string()],
        });

        self.add_entry(LemmaEntry {
            canonical_id: "neuro_channel_interface".to_string(),
            german_label: "NeurokanalSchnittstelle".to_string(),
            english_label: "NeuralChannelInterface".to_string(),
            german_definition: "Eine Schnittstelle für die Datenübertragung zwischen Neurokanälen".to_string(),
            english_definition: "An interface for data transmission between neural channels".to_string(),
            pronunciation_de: "NOY-ro-kah-NAHL-shhn-it-shteh-luh".to_string(),
            word_type: "feminine, die NeurokanalSchnittstelle".to_string(),
            related_concepts: vec!["neuro_channel".to_string(), "interface".to_string()],
        });
    }

    pub fn add_entry(&mut self, entry: LemmaEntry) {
        self.entries.insert(entry.canonical_id.clone(), entry);
    }

    pub fn get_entry(&self, canonical_id: &str) -> Option<&LemmaEntry> {
        self.entries.get(canonical_id)
    }

    pub fn search_by_german(&self, term: &str) -> Vec<&LemmaEntry> {
        self.entries
            .values()
            .filter(|e| e.german_label.contains(term) || e.german_definition.contains(term))
            .collect()
    }

    pub fn search_by_english(&self, term: &str) -> Vec<&LemmaEntry> {
        self.entries
            .values()
            .filter(|e| e.english_label.contains(term) || e.english_definition.contains(term))
            .collect()
    }

    pub fn get_all_entries(&self) -> Vec<&LemmaEntry> {
        self.entries.values().collect()
    }
}
