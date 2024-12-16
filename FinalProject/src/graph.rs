use std::collections::{HashMap, HashSet};

pub struct Graph {
    pub adjacency_list: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Graph {
            adjacency_list: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, node1: String, node2: String) {
        self.adjacency_list
            .entry(node1.clone())
            .or_insert_with(HashSet::new)
            .insert(node2.clone());
        self.adjacency_list
            .entry(node2)
            .or_insert_with(HashSet::new)
            .insert(node1);
    }

    pub fn shortest_path(&self, start: &str, end: &str) -> Option<Vec<String>> {
        if !self.adjacency_list.contains_key(start) || !self.adjacency_list.contains_key(end) {
            return None;
        }

        let mut visited = HashSet::new();
        let mut queue = std::collections::VecDeque::new();
        let mut predecessors: HashMap<String, String> = HashMap::new();

        visited.insert(start.to_string());
        queue.push_back(start.to_string());

        while let Some(current) = queue.pop_front() {
            if current == end {
                let mut path = vec![end.to_string()];
                let mut current_node = end;
                while let Some(predecessor) = predecessors.get(current_node) {
                    path.push(predecessor.clone());
                    current_node = predecessor;
                }
                path.reverse();
                return Some(path);
            }

            if let Some(neighbors) = self.adjacency_list.get(&current) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor) {
                        visited.insert(neighbor.clone());
                        queue.push_back(neighbor.clone());
                        predecessors.insert(neighbor.clone(), current.clone());
                    }
                }
            }
        }

        None
    }
}
