pub struct DualPathValidator;

impl DualPathValidator {
    pub fn new() -> Self {
        Self
    }

    pub fn validate(&self, node_path: &[String], cluster_path: &[String]) -> anyhow::Result<String> {
        if node_path.is_empty() && cluster_path.is_empty() {
            return Ok("INVALID: No paths provided".to_string());
        }

        if node_path.len() >= cluster_path.len() {
            Ok("VALID: Dual path consensus established".to_string())
        } else {
            Ok("WARNING: Cluster path exceeds node path".to_string())
        }
    }

    pub fn cross_validate(&self, path1: &[String], path2: &[String]) -> bool {
        if path1.is_empty() || path2.is_empty() {
            return false;
        }

        let set1: std::collections::HashSet<_> = path1.iter().collect();
        let set2: std::collections::HashSet<_> = path2.iter().collect();

        !set1.is_disjoint(&set2)
    }
}
