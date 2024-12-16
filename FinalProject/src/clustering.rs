use std::collections::{HashMap, HashSet};

pub fn clustering_coefficient(graph: &HashMap<String, HashSet<String>>, node: &str) -> Option<f64> {
    let neighbors = graph.get(node)?;
    let degree = neighbors.len();

    if degree < 2 {
        return Some(0.0);
    }

    let mut links = 0;
    for neighbor in neighbors {
        if let Some(neighbor_neighbors) = graph.get(neighbor) {
            for mutual in neighbor_neighbors {
                if neighbors.contains(mutual) {
                    links += 1;
                }
            }
        }
    }

    Some(links as f64 / (degree * (degree - 1)) as f64)
}

pub fn global_clustering_coefficient(graph: &HashMap<String, HashSet<String>>) -> f64 {
    let mut triangles = 0;
    let mut triplets = 0;

    for (node, neighbors) in graph.iter() {
        for neighbor in neighbors {
            if let Some(neighbor_neighbors) = graph.get(neighbor) {
                for mutual in neighbor_neighbors {
                    if neighbors.contains(mutual) {
                        triangles += 1;
                    }
                }
            }
        }

        let degree = neighbors.len();
        if degree >= 2 {
            triplets += degree * (degree - 1);
        }
    }

    if triplets == 0 {
        0.0
    } else {
        triangles as f64 / triplets as f64
    }
}

pub fn average_clustering_coefficient(graph: &HashMap<String, HashSet<String>>) -> f64 {
    let mut total_coefficient = 0.0;
    let mut node_count = 0;

    for node in graph.keys() {
        if let Some(coefficient) = clustering_coefficient(graph, node) {
            total_coefficient += coefficient;
            node_count += 1;
        }
    }

    if node_count == 0 {
        0.0
    } else {
        total_coefficient / node_count as f64
    }
}
