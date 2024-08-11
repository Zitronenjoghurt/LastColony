use godot::prelude::*;
use serde::{Deserialize, Serialize};

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
    None,
    Hut,
}

impl Default for WorldObjectId {
    fn default() -> Self {
        Self::None
    }
}
