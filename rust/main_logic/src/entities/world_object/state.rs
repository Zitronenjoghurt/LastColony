use super::{
    building_category::BuildingCategory,
    id::WorldObjectId,
    template::{WorldObjectTemplate, WorldObjectTemplateTrait},
};
use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

pub trait WorldObjectStateTrait<'de>: Default + Serialize + Deserialize<'de> {
    fn get_template(&self) -> Rc<impl WorldObjectTemplateTrait>;
    fn get_id(&self) -> WorldObjectId {
        self.get_template().get_id()
    }
    fn get_building_category(&self) -> BuildingCategory {
        self.get_template().get_building_category()
    }
    fn get_display_name(&self) -> StringName {
        self.get_template().get_display_name()
    }
    fn is_buildable(&self) -> bool {
        self.get_building_category() != BuildingCategory::NONE
    }
}

#[derive(GodotClass, Default, Clone, Serialize, Deserialize)]
#[class(init, base=Resource)]
pub struct WorldObjectState {
    #[serde(skip)]
    pub template: Rc<WorldObjectTemplate>,
}

impl<'de> WorldObjectStateTrait<'de> for WorldObjectState {
    fn get_template(&self) -> Rc<impl WorldObjectTemplateTrait> {
        self.template.clone()
    }
}
