use std::collections::HashMap;

use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{display_tile::DisplayTile, tile_data::TileDataMap},
    enums::world_object_id::WorldObjectId,
    gamestate::GameState,
};

use super::{
    base::BaseWorldObject,
    behavior::{Behavior, BehaviorType},
    data::{common::WorldObjectCommonData, housing::WorldObjectHousingData},
};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct WorldObject {
    #[serde(flatten)]
    base: BaseWorldObject,
    #[serde(default, skip_serializing_if = "HashMap::is_empty", rename = "bhvs")]
    behaviors: HashMap<BehaviorType, Behavior>,
}

impl WorldObject {
    pub fn get_id(&self) -> WorldObjectId {
        self.base.id
    }

    pub fn get_display_tile(&self, location: Vector2i, gamestate: &GameState) -> DisplayTile {
        DisplayTile {
            object_id: self.get_id(),
            tile_type: self.base.get_current_tile_type(location, gamestate),
            location,
        }
    }

    pub fn get_display_tile_gd(
        &self,
        location: Vector2i,
        gamestate: &GameState,
    ) -> Gd<DisplayTile> {
        Gd::from_object(self.get_display_tile(location, gamestate))
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

    pub fn write_tile_data(&self, self_index: u64, tile_data: &mut TileDataMap, map_width: u32) {
        self.base.write_tile_data(self_index, tile_data, map_width);
    }
}
