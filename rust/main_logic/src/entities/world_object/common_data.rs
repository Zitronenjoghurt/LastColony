use godot::prelude::*;

use crate::enums::{job_type::JobType, world_object_id::WorldObjectId};

/// These are fields that all WorldObjectTemplates have in common
/// and will be passed to all new behavior instances
#[derive(GodotClass, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(tool, init, base = Resource)]
pub struct WorldObjectCommonData {
    #[export]
    pub id: WorldObjectId,
    #[export]
    pub job_type: JobType,
}
