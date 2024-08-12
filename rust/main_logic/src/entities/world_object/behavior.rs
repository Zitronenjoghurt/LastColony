use godot::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    entities::display_tile::DisplayTile,
    enums::{job_type::JobType, tile_type::TileType, world_object_id::WorldObjectId},
};

use super::{
    behaviors::{housing::HousingBehavior, stable::StableBehavior},
    common_data::WorldObjectCommonData,
};

/// A trait that has to be implemented by all behaviors.
pub trait WorldObjectBehavior {
    fn get_id(&self) -> WorldObjectId;
    fn set_id(&mut self, id: WorldObjectId);

    fn get_housing_capacity(&self) -> u32 {
        0
    }

    fn set_housing_capacity(&mut self, _capacity: u32) {}

    fn get_job_type(&self) -> JobType {
        JobType::None
    }

    fn set_job_type(&mut self, _job_type: JobType) {}

    fn get_job_production(&self) -> u32 {
        0
    }

    fn set_job_production(&mut self, _job_production: u32) {}

    fn get_current_tile_type(&self) -> TileType {
        TileType::None
    }

    fn get_display_tile(&self, location: Vector2i) -> DisplayTile {
        DisplayTile {
            object_id: self.get_id(),
            tile_type: self.get_current_tile_type(),
            location,
        }
    }

    fn get_display_tile_gd(&self, location: Vector2i) -> Gd<DisplayTile> {
        Gd::from_object(self.get_display_tile(location))
    }

    fn apply_common_data(&mut self, common_data: Gd<WorldObjectCommonData>) {
        self.set_id(common_data.bind().id);
        self.set_job_type(common_data.bind().job_type)
    }

    fn provides_job(&self) -> bool {
        self.get_job_type() != JobType::None
    }
}

/// All behaviors share an enum so their instances can be stored together in a hashmap
/// besides their different properties and behaviors.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WorldObjectBehaviorType {
    Stable(StableBehavior),
    Housing(HousingBehavior),
}

impl WorldObjectBehaviorType {
    pub fn get_behavior(&self) -> &dyn WorldObjectBehavior {
        match self {
            WorldObjectBehaviorType::Stable(state) => state,
            WorldObjectBehaviorType::Housing(state) => state,
        }
    }

    pub fn get_id(&self) -> WorldObjectId {
        let behavior = self.get_behavior();
        behavior.get_id()
    }

    pub fn get_job_type(&self) -> JobType {
        let behavior = self.get_behavior();
        behavior.get_job_type()
    }

    pub fn get_display_tile_gd(&self, location: Vector2i) -> Gd<DisplayTile> {
        let behavior = self.get_behavior();
        behavior.get_display_tile_gd(location)
    }

    pub fn provides_job(&self) -> bool {
        let behavior = self.get_behavior();
        behavior.provides_job()
    }
}
