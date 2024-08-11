use serde::{Deserialize, Serialize};

use super::behavior::WorldObjectBehaviorType;
use std::collections::HashMap;

/// A collection that keeps track of all spawned behavior instances.
/// Indexing starts at the top left of the map.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct WorldObjectBehaviorCollection {
    behaviors: HashMap<u32, WorldObjectBehaviorType>,
}

impl WorldObjectBehaviorCollection {
    pub fn add_at_index(&mut self, index: u32, state: WorldObjectBehaviorType) {
        self.behaviors.insert(index, state);
    }

    pub fn is_empty(&self) -> bool {
        self.behaviors.is_empty()
    }
}
