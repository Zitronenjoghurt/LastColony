use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::tile_data::TileDataMap,
    enums::{
        display_behavior::DisplayBehavior, tile_type::TileType, world_object_id::WorldObjectId,
    },
    gamestate::{map_coords_to_index, map_index_to_coords, GameState},
};

use super::data::common::WorldObjectCommonData;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct BaseWorldObject {
    pub id: WorldObjectId,
    #[serde(rename = "dp_bhvr")]
    pub display_behavior: DisplayBehavior,
    #[serde(rename = "is_g")]
    pub is_ground: bool,
    #[serde(rename = "sup_bld")]
    pub supports_buildings: bool,
}

impl BaseWorldObject {
    pub fn apply_common_data(&mut self, common_data: &Gd<WorldObjectCommonData>) {
        let data = common_data.bind();
        self.id = data.id;
        self.display_behavior = data.display_behavior;
        self.is_ground = data.is_ground;
        self.supports_buildings = data.supports_buildings;
    }

    pub fn get_current_tile_type(&self, location: Vector2i, gamestate: &GameState) -> TileType {
        match self.display_behavior {
            DisplayBehavior::Persistent => TileType::Persistent,
            DisplayBehavior::Grounded => get_current_tile_grounded(location, gamestate),
            _ => TileType::None,
        }
    }

    pub fn write_tile_data(&self, self_index: u64, tile_data: &mut TileDataMap, map_width: u32) {
        let coords = map_index_to_coords(self_index, map_width);
        if coords.y > 0 && (self.is_ground || self.supports_buildings) {
            let up_index = map_coords_to_index(coords.x as u32, coords.y as u32 - 1, map_width);
            if self.is_ground {
                tile_data.set_on_ground(up_index, true);
            }
            if self.supports_buildings {
                tile_data.set_buildable(up_index, true);
            }
        }
    }
}

fn get_current_tile_grounded(location: Vector2i, gamestate: &GameState) -> TileType {
    let index = gamestate.coords_vec_to_index(location);
    let on_ground = gamestate.tile_data.is_on_ground(index);
    if on_ground {
        TileType::Ground
    } else {
        TileType::Up
    }
}
