use serde::{Deserialize, Serialize};

use crate::{
    entities::world_object::{
        behavior::WorldObjectBehavior, traits::housing::HousingBehaviorTrait,
    },
    enums::{job_type::JobType, tile_type::TileType, world_object_id::WorldObjectId},
};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct HousingBehavior {
    id: WorldObjectId,
    capacity: u32,
    job_type: JobType,
    job_production: u32,
}

impl WorldObjectBehavior for HousingBehavior {
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

impl HousingBehaviorTrait for HousingBehavior {
    fn get_housing_capacity(&self) -> u32 {
        self.capacity
    }

    fn set_housing_capacity(&mut self, capacity: u32) {
        self.capacity = capacity
    }
}
