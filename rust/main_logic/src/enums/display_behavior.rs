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
)]
#[repr(u32)]
#[godot(via = u32)]
pub enum DisplayBehavior {
    None = 0,
    Persistent = 1,
    Grounded = 2,
    Activity = 3,
    GroundedActivity = 4,
}

impl Default for DisplayBehavior {
    fn default() -> Self {
        Self::None
    }
}
