use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{entities::display_tile::DisplayTile, gamestate::GameState};

use super::behavior::WorldObjectBehaviorType;
use std::collections::HashMap;

/// A collection that keeps track of all spawned behavior instances.
/// Indexing starts at the top left of the map.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WorldObjectBehaviorCollection {
    behaviors: HashMap<u32, WorldObjectBehaviorType>,
}

impl WorldObjectBehaviorCollection {
    pub fn get_current_display_tiles(&self, gamestate: &GameState) -> Array<Gd<DisplayTile>> {
        let mut array = Array::new();

        self.behaviors.iter().for_each(|(index, behavior)| {
            let location = gamestate.index_to_coords(*index);
            let display_tile = behavior.get_display_tile_gd(location);
            array.push(display_tile);
        });

        array
    }

    pub fn add_at_index(&mut self, index: u32, state: WorldObjectBehaviorType) {
        self.behaviors.insert(index, state);
    }

    pub fn is_empty(&self) -> bool {
        self.behaviors.is_empty()
    }
}
