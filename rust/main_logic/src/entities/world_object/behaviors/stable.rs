use serde::{Deserialize, Serialize};

use crate::{
    entities::world_object::behavior::WorldObjectBehavior,
    enums::{tile_type::TileType, world_object_id::WorldObjectId},
};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct StableBehavior {
    id: WorldObjectId,
}

impl WorldObjectBehavior for StableBehavior {
    fn get_id(&self) -> WorldObjectId {
        self.id
    }

    fn set_id(&mut self, id: WorldObjectId) {
        self.id = id
    }

    fn get_current_tile_type(&self) -> TileType {
        TileType::Persistent
    }
}
