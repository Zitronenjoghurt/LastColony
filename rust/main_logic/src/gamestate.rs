use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{pop::collection::PopCollection, tick_result::TickResult},
    traits::serde::{FromJsonString, ToJsonString},
};

#[derive(GodotClass, Debug, Serialize, Deserialize, Default)]
#[class(no_init)]
pub struct GameState {
    file_index: u64,
    game_version: u64,
    #[serde(default)]
    map_height: u32,
    #[serde(default)]
    map_width: u32,
    #[serde(default, skip_serializing_if = "PopCollection::is_empty")]
    pop_collection: PopCollection,
}

#[godot_api]
impl GameState {
    #[func]
    fn create(file_index: u64, game_version: u64) -> Gd<Self> {
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
    fn tick(&mut self, tps: u64) -> Gd<TickResult> {
        let pop_result = self.pop_collection.tick(tps);

        let tick_result = TickResult { pop_result };
        Gd::from_object(tick_result)
    }

    fn new(file_index: u64, game_version: u64) -> Self {
        GameState {
            file_index,
            game_version,
            ..Default::default()
        }
    }
}
