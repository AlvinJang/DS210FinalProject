use std::collections::{HashMap, HashSet};
use crate::analysis::{degree_distribution, average_degree, densest_subgraph};
use crate::graph::Graph;
use crate::clustering::{clustering_coefficient, global_clustering_coefficient, average_clustering_coefficient};

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_degree_distribution() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::from(["B".to_string(), "C".to_string()]));
        graph.insert("B".to_string(), HashSet::from(["A".to_string()]));
        graph.insert("C".to_string(), HashSet::from(["A".to_string()]));

        let distribution = degree_distribution(&graph);
        assert_eq!(distribution.get(&1), Some(&2)); // Two nodes have degree 1
        assert_eq!(distribution.get(&2), Some(&1)); // One node has degree 2
    }

    #[test]
    fn test_average_degree() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::from(["B".to_string()]));
        graph.insert("B".to_string(), HashSet::from(["A".to_string(), "C".to_string()]));
        graph.insert("C".to_string(), HashSet::from(["B".to_string()]));

        let avg_degree = average_degree(&graph);
        assert!((avg_degree - 1.333).abs() < 0.001); // Approximate check for 4/3
    }

    #[test]
    fn test_densest_subgraph() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::from(["B".to_string(), "C".to_string()]));
        graph.insert("B".to_string(), HashSet::from(["A".to_string(), "C".to_string()]));
        graph.insert("C".to_string(), HashSet::from(["A".to_string(), "B".to_string()]));

        let (subgraph, density) = densest_subgraph(&graph);
        assert_eq!(subgraph.len(), 3); // All three nodes
        assert!((density - 1.0).abs() < 0.001); // Density is 1.0
    }

    #[test]
    fn test_shortest_path() {
        let mut graph = Graph::new();
        graph.add_edge("A".to_string(), "B".to_string());
        graph.add_edge("B".to_string(), "C".to_string());

        let path = graph.shortest_path("A", "C").unwrap();
        assert_eq!(path, vec!["A", "B", "C"]);
    }

    #[test]
    fn test_clustering_coefficient() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::from(["B".to_string(), "C".to_string()]));
        graph.insert("B".to_string(), HashSet::from(["A".to_string(), "C".to_string()]));
        graph.insert("C".to_string(), HashSet::from(["A".to_string(), "B".to_string()]));

        let coefficient = clustering_coefficient(&graph, "A").unwrap();
        assert!((coefficient - 1.0).abs() < 0.001); // Fully connected triangle
    }

    #[test]
    fn test_global_clustering_coefficient() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::from(["B".to_string(), "C".to_string()]));
        graph.insert("B".to_string(), HashSet::from(["A".to_string(), "C".to_string()]));
        graph.insert("C".to_string(), HashSet::from(["A".to_string(), "B".to_string()]));

        let coefficient = global_clustering_coefficient(&graph);
        assert!((coefficient - 1.0).abs() < 0.001); // Fully connected graph
    }

    #[test]
    fn test_average_clustering_coefficient() {
        let mut graph = HashMap::new();
        graph.insert("A".to_string(), HashSet::from(["B".to_string(), "C".to_string()]));
        graph.insert("B".to_string(), HashSet::from(["A".to_string(), "C".to_string()]));
        graph.insert("C".to_string(), HashSet::from(["A".to_string(), "B".to_string()]));

        let coefficient = average_clustering_coefficient(&graph);
        assert!((coefficient - 1.0).abs() < 0.001); // Fully connected graph
    }
}
