#[cfg(test)]
mod tests {
    use neuro_node_path_engine::{
        core::{Neuron, Node, Cluster, NeuralChannel, Interface},
        utils::Hasher,
    };

    #[test]
    fn test_neuron_creation() {
        let neuron = Neuron::new(
            "test_neuron".to_string(),
            "processing".to_string(),
            0.8,
        );
        assert_eq!(neuron.metadata.label, "test_neuron");
        assert_eq!(neuron.activation_level, 0.0);
    }

    #[test]
    fn test_neuron_firing() {
        let mut neuron = Neuron::new(
            "test".to_string(),
            "test".to_string(),
            1.0,
        );
        let output = neuron.fire(0.5);
        assert!(output > 0.0 && output < 1.0);
    }

    #[test]
    fn test_node_creation() {
        let node = Node::new(
            "test_node".to_string(),
            neuro_node_path_engine::core::node::NodeType::Module,
            "/path/to/node".to_string(),
        );
        assert_eq!(node.name, "test_node");
        assert_eq!(node.depth, 0);
    }

    #[test]
    fn test_cluster_creation() {
        let cluster = Cluster::new(
            "test_cluster".to_string(),
            neuro_node_path_engine::core::cluster::ClusterType::Functional,
        );
        assert_eq!(cluster.name, "test_cluster");
        assert_eq!(cluster.node_ids.len(), 0);
    }

    #[test]
    fn test_neural_channel_creation() {
        let channel = NeuralChannel::new(
            "node_1".to_string(),
            "node_2".to_string(),
            neuro_node_path_engine::core::channel::SignalType::DataFlow,
        );
        assert!(channel.active);
        assert_eq!(channel.weight, 1.0);
    }

    #[test]
    fn test_hasher() {
        let hash1 = Hasher::hash_string("test_data");
        let hash2 = Hasher::hash_string("test_data");
        assert_eq!(hash1, hash2);

        let is_valid = Hasher::verify_hash("test_data", &hash1);
        assert!(is_valid);
    }

    #[test]
    fn test_interface_creation() {
        let interface = Interface::new(
            "query_interface".to_string(),
            neuro_node_path_engine::core::interface::InterfaceType::QueryInterface,
        );
        assert_eq!(interface.name, "query_interface");
    }
}
