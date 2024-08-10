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
        pub mod building_category;
        pub mod id;
        pub mod state;
        pub mod template;
        pub mod types {
            pub mod housing {
                pub mod state;
                pub mod template;
            }
        }
    }
    pub mod tick_result;
}

pub mod structures {
    pub mod weighted_graph;
}

pub mod traits {
    pub mod serde;
    pub mod to_godot_array;
}
