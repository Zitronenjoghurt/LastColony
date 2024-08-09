use godot::prelude::*;

struct PathFinding;

#[gdextension]
unsafe impl ExtensionLibrary for PathFinding {}

pub mod gamestate;
pub mod pathfinder;

pub mod entities {
    pub mod pop;
    pub mod pop_collection;
}

pub mod structures {
    pub mod weighted_graph;
}

pub mod traits {
    pub mod serde;
    pub mod to_godot_array;
}
