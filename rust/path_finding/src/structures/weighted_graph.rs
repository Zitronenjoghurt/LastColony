use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Edge((u32, u32));

impl Edge {
    fn new(source_node: u32, target_node: u32) -> Self {
        if source_node < target_node {
            Edge((source_node, target_node))
        } else {
            Edge((target_node, source_node))
        }
    }

    fn contains(&self, node: u32) -> bool {
        self.0 .0 == node || self.0 .1 == node
    }
}

/// An undirected weighted graph using an adjacency list.
#[derive(Debug, Default)]
pub struct WeightedGraph {
    adj_list: HashMap<u32, HashSet<u32>>,
    weights: HashMap<Edge, u64>,
}

impl WeightedGraph {
    fn add_node(&mut self, node: u32) {
        self.adj_list.entry(node).or_default();
    }

    fn remove_node(&mut self, node: u32) {
        // Remove node from its neighbors adjacency lists
        if let Some(neighbors) = self.adj_list.get(&node) {
            let neighbors = neighbors.clone();
            for neighbor in neighbors {
                if let Some(neighbor_set) = self.adj_list.get_mut(&neighbor) {
                    neighbor_set.remove(&node);
                }
            }
        }

        // Remove edges containing the node from weights
        let edges_to_remove: Vec<Edge> = self
            .weights
            .keys()
            .filter(|&edge| edge.contains(node))
            .cloned()
            .collect();

        for edge in edges_to_remove {
            self.weights.remove(&edge);
        }

        // Remove the node itself
        self.adj_list.remove(&node);
    }

    fn add_edge(&mut self, source_node: u32, target_node: u32, weight: u64) {
        self.adj_list
            .entry(source_node)
            .or_default()
            .insert(target_node);

        if source_node != target_node {
            self.adj_list
                .entry(target_node)
                .or_default()
                .insert(source_node);
        }

        self.weights
            .insert(Edge::new(source_node, target_node), weight);
    }

    fn remove_edge(&mut self, source_node: u32, target_node: u32) {
        if let Some(neighbors) = self.adj_list.get_mut(&source_node) {
            neighbors.remove(&target_node);
        }

        if source_node != target_node {
            if let Some(neighbors) = self.adj_list.get_mut(&target_node) {
                neighbors.remove(&source_node);
            }
        }

        self.weights.remove(&Edge::new(source_node, target_node));

        // Clean up nodes without neighbors
        if self
            .get_neighbors(source_node)
            .map_or(false, |neighbors| neighbors.is_empty())
        {
            self.adj_list.remove(&source_node);
        }

        if self
            .get_neighbors(target_node)
            .map_or(false, |neighbors| neighbors.is_empty())
        {
            self.adj_list.remove(&target_node);
        }
    }

    fn has_node(&self, node: u32) -> bool {
        self.adj_list.contains_key(&node)
    }

    fn has_edge(&self, source_node: u32, target_node: u32) -> bool {
        self.weights
            .contains_key(&Edge::new(source_node, target_node))
    }

    fn get_neighbors(&self, node: u32) -> Option<&HashSet<u32>> {
        self.adj_list.get(&node)
    }

    fn get_weight(&self, source_node: u32, target_node: u32) -> Option<u64> {
        self.weights
            .get(&Edge::new(source_node, target_node))
            .copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_node() {
        let mut graph = WeightedGraph::default();
        graph.add_node(1);
        assert!(graph.has_node(1));
    }

    #[test]
    fn test_add_edge() {
        let mut graph = WeightedGraph::default();
        graph.add_edge(1, 1, 5);
        graph.add_edge(1, 2, 50);
        graph.add_edge(2, 3, 40);

        assert!(graph.has_node(1));
        assert!(graph.has_node(2));
        assert!(graph.has_node(3));

        assert!(graph.has_edge(1, 1));
        assert!(graph.has_edge(1, 2));
        assert!(graph.has_edge(2, 3));

        assert_eq!(graph.get_weight(1, 1), Some(5));
        assert_eq!(graph.get_weight(1, 2), Some(50));
        assert_eq!(graph.get_weight(2, 3), Some(40));
    }

    #[test]
    fn test_remove_node() {
        let mut graph = WeightedGraph::default();
        graph.add_edge(1, 1, 5);
        graph.add_edge(1, 2, 50);
        graph.add_edge(2, 3, 40);

        graph.remove_node(1);

        assert!(!graph.has_node(1));
        assert!(graph.has_node(2));
        assert!(graph.has_node(3));

        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(1, 2));
        assert!(graph.has_edge(2, 3));

        assert_eq!(graph.get_weight(1, 1), None);
        assert_eq!(graph.get_weight(1, 2), None);
        assert_eq!(graph.get_weight(2, 3), Some(40));
    }

    #[test]
    fn test_remove_edge() {
        let mut graph = WeightedGraph::default();
        graph.add_edge(1, 1, 5);
        graph.add_edge(1, 2, 50);
        graph.add_edge(2, 3, 40);

        graph.remove_edge(1, 1);
        graph.remove_edge(1, 2);

        assert!(!graph.has_node(1));
        assert!(graph.has_node(2));
        assert!(graph.has_node(3));

        assert!(!graph.has_edge(1, 1));
        assert!(!graph.has_edge(1, 2));
        assert!(graph.has_edge(2, 3));

        assert_eq!(graph.get_weight(1, 1), None);
        assert_eq!(graph.get_weight(1, 2), None);
        assert_eq!(graph.get_weight(2, 3), Some(40));
    }
}
