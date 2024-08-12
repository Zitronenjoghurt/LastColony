use godot::prelude::*;

use crate::enums::{display_behavior::DisplayBehavior, world_object_id::WorldObjectId};

/// These are fields that all WorldObjectTemplates have in common
/// and will be passed to all new behavior instances
#[derive(GodotClass, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(tool, init, base = Resource)]
pub struct WorldObjectCommonData {
    #[export]
    pub id: WorldObjectId,
    #[export]
    pub display_behavior: DisplayBehavior,
}
