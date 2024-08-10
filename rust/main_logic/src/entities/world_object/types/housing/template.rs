use super::state::HousingState;
use crate::entities::world_object::{
    building_category::BuildingCategory, id::WorldObjectId, state::WorldObjectStateTrait,
    template::WorldObjectTemplateTrait,
};
use godot::prelude::*;
use std::rc::Rc;

#[derive(GodotClass, Default, Clone)]
#[class(tool, init, base=Resource)]
pub struct HousingWorldObjectTemplate {
    #[export]
    id: WorldObjectId,
    #[export]
    building_category: BuildingCategory,
    #[export]
    display_name: StringName,
    #[export]
    default_state: Gd<HousingState>,
}

impl WorldObjectTemplateTrait for HousingWorldObjectTemplate {
    fn create_state(&self) -> impl WorldObjectStateTrait {
        let mut state = self.default_state.bind().clone();
        state.template = Rc::new(self.clone());
        state
    }

    fn get_id(&self) -> WorldObjectId {
        self.id
    }

    fn get_display_name(&self) -> StringName {
        self.display_name.clone()
    }

    fn get_building_category(&self) -> BuildingCategory {
        self.building_category
    }
}
