use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        pop::collection::PopCollection,
        tick_result::TickResult,
        world_object::{
            behavior::WorldObjectBehaviorType, collection::WorldObjectBehaviorCollection,
        },
    },
    traits::serde::{FromJsonString, ToJsonString},
};

#[derive(GodotClass, Debug, Serialize, Deserialize, Default)]
#[class(no_init)]
pub struct GameState {
    file_index: u32,
    game_version: u32,
    map_height: u32,
    map_width: u32,
    #[serde(default, skip_serializing_if = "PopCollection::is_empty")]
    pop_collection: PopCollection,
    #[serde(
        default,
        skip_serializing_if = "WorldObjectBehaviorCollection::is_empty"
    )]
    object_collection: WorldObjectBehaviorCollection,
}

#[godot_api]
impl GameState {
    #[func]
    fn tick(&mut self, tps: u64) -> Gd<TickResult> {
        let pop_result = self.pop_collection.tick(tps);

        let tick_result = TickResult { pop_result };
        Gd::from_object(tick_result)
    }

    #[func]
    fn create(file_index: u32, game_version: u32) -> Gd<Self> {
        let state = GameState::new(file_index, game_version);
        Gd::from_object(state)
    }

    #[func]
    fn from_json(json_string: String) -> Gd<Self> {
        let state = GameState::from_json_string(&json_string);
        Gd::from_object(state)
    }

    #[func]
    fn to_json(&self) -> String {
        self.to_json_string()
    }

    #[func]
    fn get_file_index(&self) -> u32 {
        self.file_index
    }

    #[func]
    fn get_map_height(&self) -> u32 {
        self.map_height
    }

    #[func]
    fn get_map_width(&self) -> u32 {
        self.map_width
    }

    pub fn push_object(&mut self, location: Vector2i, state: WorldObjectBehaviorType) {
        let index = self.coords_to_index(location);
        self.object_collection.add_at_index(index, state);
    }

    pub fn coords_to_index(&self, coords: Vector2i) -> u32 {
        (coords.y * self.map_width as i32 + coords.x) as u32
    }

    fn new(file_index: u32, game_version: u32) -> Self {
        GameState {
            file_index,
            game_version,
            ..Default::default()
        }
    }
}
