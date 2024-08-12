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
pub enum TileType {
    None = 0,
    Persistent = 1,
    Active = 2,
    Inactive = 3,
    ActiveUp = 4,
    ActiveGround = 5,
    InactiveUp = 6,
    InactiveGround = 7,
}

impl Default for TileType {
    fn default() -> Self {
        Self::None
    }
}
