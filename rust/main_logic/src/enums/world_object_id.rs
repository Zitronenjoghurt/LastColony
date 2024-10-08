use godot::prelude::*;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    GodotConvert,
    Var,
    Export,
    Debug,
    Clone,
    Copy,
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[repr(u32)]
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
