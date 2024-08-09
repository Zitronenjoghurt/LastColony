use godot::prelude::*;

struct PathFinding;

#[gdextension]
unsafe impl ExtensionLibrary for PathFinding {}

pub mod pathfinder;

pub mod entities {
    pub mod pop;
}

pub mod structures {
    pub mod weighted_graph;
}

pub mod traits {
    pub mod to_godot_array;
}
