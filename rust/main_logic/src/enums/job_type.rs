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
pub enum JobType {
    None = 0,
    Wood = 1,
}

impl Default for JobType {
    fn default() -> Self {
        Self::None
    }
}
