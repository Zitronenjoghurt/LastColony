use crate::entities::world_object::data::housing::WorldObjectHousingData;
use godot::prelude::*;

pub trait HousingBehaviorTrait {
    fn get_housing_capacity(&self) -> u32;
    fn set_housing_capacity(&mut self, capacity: u32);
    fn apply_housing_data(&mut self, housing_data: &Gd<WorldObjectHousingData>) {
        let data = housing_data.bind();
        self.set_housing_capacity(data.get_capacity());
    }
}
