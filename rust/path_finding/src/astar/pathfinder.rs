// Godot docs can be used from Godot 4.3, upgrade game to 4.3 after first stable release
//#![cfg(feature = "register-docs")]
use godot::prelude::*;

use crate::structures::weighted_graph::WeightedGraph;

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
