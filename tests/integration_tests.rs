//! Integration tests for CADSP

#[cfg(test)]
mod tests {
    use cadsp_core::*;

    #[test]
    fn test_biophysical_pattern_detection() {
        let code = r#"
            struct Reservoir {
                state_vector: Vec<f32>,
                weight_matrix: Vec<Vec<f32>>,
            }
            impl Reservoir {
                fn process(&self, input: &[f32]) -> Vec<f32> { vec![] }
                fn learn(&mut self, signal: &[f32]) { }
            }
        "#;

        let discoveries =
            biophysical_patterns::PatternDetector::detect(code).expect("Pattern detection failed");

        assert!(!discoveries.is_empty());
        assert!(discoveries
            .iter()
            .any(|d| d.pattern_family == "reservoir_computing"));
        assert!(discoveries.confidence_score > 0.85);
    }

    #[test]
    fn test_neuro_node_path_computation() {
        let objects = vec![
            ("soma_1".to_string(), 0.9),
            ("synapse_2".to_string(), 0.85),
            ("reservoir_3".to_string(), 0.92),
        ];

        let path = neuro_node_path::NeuroNodePathEngine::compute_path("test_query", &objects)
            .expect("Path computation failed");

        assert_eq!(path.nodes.len(), 3);
        assert!(!path.clusters.is_empty());
        assert!(path.dual_path_validation.convergence_score > 0.7);
    }

    #[test]
    fn test_aln_definition_generation() {
        let indicators = BiophysicalIndicators {
            soma_like: Some(biophysical_patterns::SomaIndicators {
                state_field_names: vec!["state_vector".to_string()],
                spike_threshold_logic: true,
                central_accumulation: true,
                confidence: 0.9,
            }),
            synapse_like: None,
            axon_like: None,
            dendrite_like: None,
        };

        let def = aln_schema::ALNDefinitionGenerator::generate("TestObject", "test_repo", &indicators)
            .expect("Definition generation failed");

        assert_eq!(def.object_definition.canonical_name, "TestObject");
        assert!(!def.object_definition.properties.is_empty());
    }

    #[test]
    fn test_compliance_validation() {
        let implementations = vec![
            ("rust".to_string(), r#"pub struct Node { state: Vec<f32> }"#.to_string()),
            ("kotlin".to_string(), r#"class Node { var state: FloatArray }"#.to_string()),
        ];

        let report = compliance::ComplianceValidator::validate("test_obj", &implementations)
            .expect("Compliance check failed");

        assert!(!report.language_pairs.is_empty());
        assert!(matches!(
            report.overall_verdict.as_str(),
            "APPROVED_FOR_PRODUCTION" | "APPROVED_WITH_CAVEATS" | "REQUIRES_REVIEW"
        ));
    }
}
