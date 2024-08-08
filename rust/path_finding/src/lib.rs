use godot::prelude::*;

struct PathFinding;

#[gdextension]
unsafe impl ExtensionLibrary for PathFinding {}

#[derive(GodotClass)]
struct AStarPathFinder {
    base: Base<RefCounted>,
}

#[godot_api]
impl IRefCounted for AStarPathFinder {
    fn init(base: Base<RefCounted>) -> Self {
        AStarPathFinder { base }
    }
}
