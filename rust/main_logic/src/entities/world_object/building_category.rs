use godot::prelude::*;
use serde::{Deserialize, Serialize};

/// Determines in which building menu a given world object will appear
#[derive(
    GodotConvert, Var, Export, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord,
)]
#[godot(via = GString)]
pub enum BuildingCategory {
    NONE,
    HOUSING,
}

impl Default for BuildingCategory {
    fn default() -> Self {
        Self::NONE
    }
}
