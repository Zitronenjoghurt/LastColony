use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::enums::{
    display_behavior::DisplayBehavior, tile_type::TileType, world_object_id::WorldObjectId,
};

use super::data::common::WorldObjectCommonData;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseWorldObject {
    pub id: WorldObjectId,
    #[serde(rename = "dp_bhvr")]
    pub display_behavior: DisplayBehavior,
}

impl BaseWorldObject {
    pub fn apply_common_data(&mut self, common_data: &Gd<WorldObjectCommonData>) {
        let data = common_data.bind();
        self.id = data.id;
        self.display_behavior = data.display_behavior;
    }

    pub fn get_current_tile_type(&self) -> TileType {
        match self.display_behavior {
            DisplayBehavior::Persistent => TileType::Persistent,
            _ => TileType::None,
        }
    }
}
