use godot::prelude::*;

struct PathFinding;

#[gdextension]
unsafe impl ExtensionLibrary for PathFinding {}

pub mod gamestate;
pub mod pathfinder;

pub mod entities {
    pub mod pop {
        pub mod class;
        pub mod collection;
        pub mod tick;
    }
    pub mod world_object {
        pub mod behaviors {
            pub mod housing;
        }
        pub mod data {
            pub mod common;
            pub mod housing;
        }
        pub mod base;
        pub mod behavior;
        pub mod class;
        pub mod collection;
        pub mod factory;
        pub mod tile;
    }
    pub mod display_tile;
    pub mod housing_registry;
    pub mod tick_result;
    pub mod tile_data;
    pub mod tile_library;
}

pub mod enums {
    pub mod display_behavior;
    pub mod job_type;
    pub mod tile_type;
    pub mod world_object_id;
}

pub mod structures {
    pub mod weighted_graph;
}

pub mod traits {
    pub mod serde;
    pub mod to_godot_array;
}
