use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{
        display_tile::DisplayTile,
        pop::collection::PopCollection,
        tick_result::TickResult,
        tile_data::TileDataMap,
        world_object::{class::WorldObject, collection::WorldObjectCollection},
    },
    traits::{
        serde::{FromJsonString, ToJsonString},
        to_godot_array::ToGodotArray,
    },
};

#[derive(GodotClass, Debug, Serialize, Deserialize, Default, Clone)]
#[class(no_init)]
pub struct GameState {
    file_index: u32,
    game_version: u32,
    map_height: u32,
    map_width: u32,
    #[serde(default, skip_serializing_if = "PopCollection::is_empty")]
    pop_collection: PopCollection,
    #[serde(default, skip_serializing_if = "WorldObjectCollection::is_empty")]
    object_collection: WorldObjectCollection,
    #[serde(skip)]
    pub tile_data: TileDataMap,
    #[serde(skip)]
    pub map_size: u64,
}

#[godot_api]
impl GameState {
    #[func]
    fn initialize(&mut self) {
        self.map_size = self.map_height as u64 * self.map_width as u64;
        self.tile_data_full_update();
    }

    #[func]
    fn limit_camera(&self, mut camera: Gd<Camera2D>) {
        camera.set_limit(Side::BOTTOM, self.map_height as i32 * 16);
        camera.set_limit(Side::RIGHT, self.map_width as i32 * 16);
    }

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
    fn get_current_display_tiles(&self) -> Array<Gd<DisplayTile>> {
        self.object_collection.get_current_display_tiles(self)
    }

    #[func]
    fn get_map_height(&self) -> u32 {
        self.map_height
    }

    #[func]
    fn set_map_height(&mut self, height: u32) {
        self.map_height = height
    }

    #[func]
    fn get_map_width(&self) -> u32 {
        self.map_width
    }

    #[func]
    fn set_map_width(&mut self, width: u32) {
        self.map_width = width
    }

    #[func]
    fn tile_data_full_update(&mut self) {
        self.object_collection
            .push_tile_data_full(&mut self.tile_data, self.map_width);
    }

    #[func]
    fn get_buildable_bitmap(&self) -> Array<bool> {
        self.tile_data
            .get_buildable_tiles(self.map_size)
            .to_godot_array()
    }

    pub fn push_object(&mut self, location: Vector2i, state: WorldObject) {
        let index = self.coords_vec_to_index(location);
        self.object_collection.add_at_index(index, state);
    }

    pub fn coords_to_index(&self, x: u32, y: u32) -> u64 {
        map_coords_to_index(x, y, self.map_width)
    }

    pub fn coords_vec_to_index(&self, coords: Vector2i) -> u64 {
        map_coords_vec_to_index(coords, self.map_width)
    }

    pub fn index_to_coords(&self, index: u64) -> Vector2i {
        map_index_to_coords(index, self.map_width)
    }

    fn new(file_index: u32, game_version: u32) -> Self {
        GameState {
            file_index,
            game_version,
            ..Default::default()
        }
    }
}

pub fn map_coords_to_index(x: u32, y: u32, map_width: u32) -> u64 {
    (y * map_width + x) as u64
}

pub fn map_coords_vec_to_index(coords: Vector2i, map_width: u32) -> u64 {
    (coords.y * map_width as i32 + coords.x) as u64
}

pub fn map_index_to_coords(index: u64, map_width: u32) -> Vector2i {
    let x = index % map_width as u64;
    let y = index / map_width as u64;
    Vector2i {
        x: x as i32,
        y: y as i32,
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::OnceCell, fs};

    use gamestate::GameState;
    use traits::serde::FromJsonString;

    use crate::*;

    const TEST_SAVE_FILE_PATH: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/test_save.json");

    thread_local! {
        static TEST_STATE: OnceCell<GameState> = const { OnceCell::new() };
    }

    fn get_test_state() -> GameState {
        TEST_STATE.with(|cell| {
            if let Some(state) = cell.get() {
                state.clone()
            } else {
                let json_string =
                    fs::read_to_string(TEST_SAVE_FILE_PATH).expect("Failed to read JSON file");
                let state = GameState::from_json_string(&json_string);
                cell.set(state.clone()).expect("Failed to set TEST_STATE");
                state
            }
        })
    }

    #[test]
    fn test_from_json_string() {
        let state = get_test_state();
        assert_eq!(state.game_version, 1);
        assert_eq!(state.map_height, 50);
        assert_eq!(state.map_width, 200);
    }

    #[test]
    fn test_initialization() {
        let mut state = get_test_state();
        state.initialize();

        let buildable_location = Vector2i { x: 0, y: 45 };
        let non_buildable_location = Vector2i { x: 0, y: 46 };
        let ground_location = Vector2i { x: 1, y: 45 };
        let non_ground_location = Vector2i { x: 5, y: 48 };

        assert!(state
            .tile_data
            .is_buildable(state.coords_vec_to_index(buildable_location)));
        assert!(!state
            .tile_data
            .is_buildable(state.coords_vec_to_index(non_buildable_location)));
        assert!(state
            .tile_data
            .is_on_ground(state.coords_vec_to_index(ground_location)));
        assert!(!state
            .tile_data
            .is_on_ground(state.coords_vec_to_index(non_ground_location)));
    }
}
