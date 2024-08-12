use godot::obj::Gd;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::{behaviors::housing::HousingBehavior, data::housing::WorldObjectHousingData};

#[derive(
    Debug, Serialize_repr, Deserialize_repr, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[repr(u32)]
pub enum BehaviorType {
    Housing,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Behavior {
    Housing(HousingBehavior),
}

impl Behavior {
    pub fn new_housing(housing_data: &Gd<WorldObjectHousingData>) -> Behavior {
        let data = housing_data.bind();
        let behavior = HousingBehavior {
            capacity: data.capacity,
        };
        Self::Housing(behavior)
    }
}
