use std::collections::HashMap;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entities::display_tile::DisplayTile, enums::world_object_id::WorldObjectId};

use super::{
    base::BaseWorldObject,
    behavior::{Behavior, BehaviorType},
    data::{common::WorldObjectCommonData, housing::WorldObjectHousingData},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorldObject {
    base: BaseWorldObject,
    #[serde(default, skip_serializing_if = "HashMap::is_empty", rename = "bhvs")]
    behaviors: HashMap<BehaviorType, Behavior>,
}

impl WorldObject {
    pub fn get_id(&self) -> WorldObjectId {
        self.base.id
    }

    pub fn get_display_tile(&self, location: Vector2i) -> DisplayTile {
        DisplayTile {
            object_id: self.get_id(),
            tile_type: self.base.get_current_tile_type(),
            location,
        }
    }

    pub fn get_display_tile_gd(&self, location: Vector2i) -> Gd<DisplayTile> {
        Gd::from_object(self.get_display_tile(location))
    }

    pub fn apply_common_data(&mut self, common_data: &Gd<WorldObjectCommonData>) {
        self.base.apply_common_data(common_data);
    }

    pub fn apply_housing_data(&mut self, housing_data: &Gd<WorldObjectHousingData>) {
        if housing_data.bind().apply {
            let behavior = Behavior::new_housing(housing_data);
            self.behaviors.insert(BehaviorType::Housing, behavior);
        }
    }
}
