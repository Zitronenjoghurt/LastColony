use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entities::display_tile::DisplayTile, gamestate::GameState};

use super::behavior::{WorldObjectBehavior, WorldObjectBehaviorType};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

/// A collection that keeps track of all spawned behavior instances.
/// Indexing starts at the top left of the map.
#[derive(Default, Debug)]
pub struct WorldObjectBehaviorCollection {
    behaviors: Arc<RwLock<HashMap<u32, WorldObjectBehaviorType>>>,
}

impl WorldObjectBehaviorCollection {
    pub fn get_current_display_tiles(&self, gamestate: &GameState) -> Array<Gd<DisplayTile>> {
        let mut array = Array::new();
        let behaviors = self.behaviors.read().unwrap();
        behaviors.iter().for_each(|(index, behavior)| {
            let location = gamestate.index_to_coords(*index);
            let display_tile = behavior.get_display_tile_gd(location);
            array.push(display_tile);
        });

        array
    }

    pub fn add_at_index(&mut self, index: u32, state: WorldObjectBehaviorType) {
        let mut behaviors = self.behaviors.write().unwrap();
        behaviors.insert(index, state);
    }

    pub fn is_empty(&self) -> bool {
        self.behaviors.read().unwrap().is_empty()
    }
}

impl Serialize for WorldObjectBehaviorCollection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.behaviors.read().unwrap().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for WorldObjectBehaviorCollection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let behaviors = HashMap::deserialize(deserializer)?;
        Ok(WorldObjectBehaviorCollection {
            behaviors: Arc::new(RwLock::new(behaviors)),
        })
    }
}
