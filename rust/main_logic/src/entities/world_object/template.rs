use std::rc::Rc;

use super::{
    building_category::BuildingCategory,
    id::WorldObjectId,
    state::{WorldObjectState, WorldObjectStateTrait},
};
use godot::prelude::*;

pub trait WorldObjectTemplateTrait: Default + Clone {
    fn create_state(&self) -> impl WorldObjectStateTrait;
    fn get_id(&self) -> WorldObjectId;
    fn get_display_name(&self) -> StringName;
    fn get_building_category(&self) -> BuildingCategory;
    fn is_buildable(&self) -> bool {
        self.get_building_category() != BuildingCategory::NONE
    }
}

#[derive(GodotClass, Default, Clone)]
#[class(tool, init, base=Resource)]
pub struct WorldObjectTemplate {
    #[export]
    id: WorldObjectId,
    #[export]
    building_category: BuildingCategory,
    #[export]
    display_name: StringName,
    #[export]
    default_state: Gd<WorldObjectState>,
}

impl WorldObjectTemplateTrait for WorldObjectTemplate {
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
