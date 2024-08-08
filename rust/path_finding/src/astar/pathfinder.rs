// Godot docs can be used from Godot 4.3, upgrade game to 4.3 after first stable release
//#![cfg(feature = "register-docs")]
use crate::{structures::weighted_graph::WeightedGraph, traits::to_godot_array::ToGodotArray};
use godot::prelude::*;

/// Custom A* path finding algorithm with a path cache built on using signposts between nodes.
/// Optimized for quick access of the next optimal node on a path to a given target node.
#[derive(GodotClass)]
struct AStarPathFinder {
    nav_graph: WeightedGraph,
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for AStarPathFinder {
    fn init(base: Base<RefCounted>) -> Self {
        AStarPathFinder {
            nav_graph: WeightedGraph::default(),
            base,
        }
    }
}

#[godot_api]
impl AStarPathFinder {
    #[func]
    fn add_nav_node(&mut self, node: u32) {
        self.nav_graph.add_node(node);
    }

    #[func]
    fn add_connection(&mut self, source_node: u32, target_node: u32, weight: u64) {
        self.nav_graph.add_edge(source_node, target_node, weight);
    }

    #[func]
    fn get_nav_nodes(&self) -> Array<u32> {
        self.nav_graph.get_nodes().to_godot_array()
    }
}
