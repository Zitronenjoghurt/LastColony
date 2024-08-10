use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    GodotConvert, Var, Export, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
#[godot(via = GString)]
pub enum WorldObjectId {
    NONE,
    HUT,
}

impl Default for WorldObjectId {
    fn default() -> Self {
        Self::NONE
    }
}
