use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::pop_collection::PopCollection,
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
    #[serde(default)]
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

    fn new(file_index: u64, game_version: u64) -> Self {
        GameState {
            file_index,
            game_version,
            ..Default::default()
        }
    }
}
