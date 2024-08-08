use std::collections::{HashMap, HashSet};

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct Edge {
    a: u32,
    b: u32,
}

impl Edge {
    fn new(source_node: u32, target_node: u32) -> Self {
        if source_node < target_node {
            Edge {
                a: source_node,
                b: target_node,
            }
        } else {
            Edge {
                a: target_node,
                b: source_node,
            }
        }
    }

    fn contains(&self, node: u32) -> bool {
        self.a == node || self.b == node
    }
}

/// An undirected weighted graph using an adjacency list.
#[derive(Debug, Default)]
pub struct WeightedGraph {
    adj_list: HashMap<u32, HashSet<u32>>,
    weights: HashMap<Edge, u64>,
}

impl WeightedGraph {
    pub fn add_node(&mut self, node: u32) {
        self.adj_list.entry(node).or_default();
    }

    pub fn remove_node(&mut self, node: u32) {
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

    pub fn add_edge(&mut self, source_node: u32, target_node: u32, weight: u64) {
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

    pub fn remove_edge(&mut self, source_node: u32, target_node: u32) {
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

    pub fn has_node(&self, node: u32) -> bool {
        self.adj_list.contains_key(&node)
    }

    pub fn has_edge(&self, source_node: u32, target_node: u32) -> bool {
        self.weights
            .contains_key(&Edge::new(source_node, target_node))
    }

    pub fn get_nodes(&self) -> Vec<u32> {
        self.adj_list.keys().cloned().collect()
    }

    pub fn get_edges(&self) -> Vec<Edge> {
        self.weights.keys().cloned().collect()
    }

    pub fn get_neighbors(&self, node: u32) -> Option<&HashSet<u32>> {
        self.adj_list.get(&node)
    }

    pub fn get_weight(&self, source_node: u32, target_node: u32) -> Option<u64> {
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

    #[test]
    fn test_get_nodes() {
        let mut graph = WeightedGraph::default();

        assert_eq!(graph.get_nodes(), vec![]);
        graph.add_node(1);
        graph.add_node(2);
        graph.add_node(3);
        graph.add_node(7);

        let nodes = graph.get_nodes();
        assert!(nodes.contains(&1));
        assert!(nodes.contains(&2));
        assert!(nodes.contains(&3));
        assert!(!nodes.contains(&4));
        assert!(nodes.contains(&7));
    }

    #[test]
    fn test_get_edges() {
        let mut graph = WeightedGraph::default();

        assert_eq!(graph.get_edges(), vec![]);
        graph.add_edge(1, 2, 50);
        graph.add_edge(2, 3, 50);

        let edges = graph.get_edges();
        assert!(edges.contains(&Edge::new(1, 2)));
        assert!(edges.contains(&Edge::new(2, 3)));
    }
}
