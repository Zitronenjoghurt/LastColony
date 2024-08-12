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
    Up = 2,
    Ground = 3,
    Active = 4,
    Inactive = 5,
    ActiveUp = 6,
    ActiveGround = 7,
    InactiveUp = 8,
    InactiveGround = 9,
}

impl Default for TileType {
    fn default() -> Self {
        Self::None
    }
}
