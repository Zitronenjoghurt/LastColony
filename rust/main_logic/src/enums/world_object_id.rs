use godot::prelude::*;
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(
    GodotConvert,
    Var,
    Export,
    Debug,
    Clone,
    Copy,
    Serialize,
    Deserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[godot(via = u32)]
pub enum WorldObjectId {
    None = 0,
    DirtBlock = 1,
    DirtTop = 2,
    Hut = 3,
}

impl Default for WorldObjectId {
    fn default() -> Self {
        Self::None
    }
}
