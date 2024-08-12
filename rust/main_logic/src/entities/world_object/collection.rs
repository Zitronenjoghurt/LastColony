use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::{display_tile::DisplayTile, tile_data::TileDataMap},
    gamestate::GameState,
};

use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

use super::class::WorldObject;

/// A collection that keeps track of all spawned behavior instances.
/// Indexing starts at the top left of the map.
#[derive(Default, Debug, Clone)]
pub struct WorldObjectCollection {
    objects: Arc<RwLock<HashMap<u64, WorldObject>>>,
}

impl WorldObjectCollection {
    pub fn get_current_display_tiles(&self, gamestate: &GameState) -> Array<Gd<DisplayTile>> {
        let mut array = Array::new();
        let objects = self.objects.read().unwrap();
        objects.iter().for_each(|(index, object)| {
            let location = gamestate.index_to_coords(*index);
            let display_tile = object.get_display_tile_gd(location, gamestate);
            array.push(display_tile);
        });

        array
    }

    pub fn add_at_index(&mut self, index: u64, state: WorldObject) {
        let mut objects = self.objects.write().unwrap();
        objects.insert(index, state);
    }

    pub fn is_empty(&self) -> bool {
        self.objects.read().unwrap().is_empty()
    }

    pub fn push_tile_data_full(&self, tile_data: &mut TileDataMap, map_width: u32) {
        let objects = self.objects.write().unwrap();
        objects
            .iter()
            .for_each(|(index, object)| object.write_tile_data(*index, tile_data, map_width));
    }
}

impl Serialize for WorldObjectCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.objects.read().unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WorldObjectCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let objects = HashMap::deserialize(deserializer)?;
        Ok(WorldObjectCollection {
            objects: Arc::new(RwLock::new(objects)),
        })
    }
}
