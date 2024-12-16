use std::collections::{HashMap, HashSet};

pub fn degree_distribution(graph: &HashMap<String, HashSet<String>>) -> HashMap<usize, usize> {
    let mut degree_count = HashMap::new();

    for neighbors in graph.values() {
        let degree = neighbors.len();
        *degree_count.entry(degree).or_insert(0) += 1;
    }

    degree_count
}

pub fn average_degree(graph: &HashMap<String, HashSet<String>>) -> f64 {
    let total_degrees: usize = graph.values().map(|neighbors| neighbors.len()).sum();
    let total_nodes = graph.len();

    if total_nodes == 0 {
        0.0
    } else {
        total_degrees as f64 / total_nodes as f64
    }
}

pub fn densest_subgraph(
    graph: &HashMap<String, HashSet<String>>,
) -> (HashSet<String>, f64) {
    let mut max_density = 0.0;
    let mut best_subgraph = HashSet::new();

    for (node, neighbors) in graph.iter() {
        let mut subgraph = HashSet::new();
        let mut edge_count = 0;

        subgraph.insert(node.clone());
        for neighbor in neighbors {
            subgraph.insert(neighbor.clone());
        }

        for sub_node in &subgraph {
            if let Some(sub_neighbors) = graph.get(sub_node) {
                for sub_neighbor in sub_neighbors {
                    if subgraph.contains(sub_neighbor) {
                        edge_count += 1;
                    }
                }
            }
        }

        let edge_count = edge_count / 2;
        let density = edge_count as f64 / subgraph.len() as f64;

        if density > max_density {
            max_density = density;
            best_subgraph = subgraph;
        }
    }

    (best_subgraph, max_density)
}
