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
    None,
    Persistent,
    Active,
    Inactive,
    ActiveUp,
    ActiveGround,
    InactiveUp,
    InactiveGround,
}

impl Default for TileType {
    fn default() -> Self {
        Self::None
    }
}
