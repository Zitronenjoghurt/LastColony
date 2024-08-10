use crate::entities::world_object::{
    state::WorldObjectStateTrait, template::WorldObjectTemplateTrait,
};
use godot::prelude::*;
use serde::{Deserialize, Serialize};
use std::rc::Rc;

use super::template::HousingWorldObjectTemplate;

#[derive(GodotClass, Default, Clone, Serialize, Deserialize)]
#[class(tool, init, base=Resource)]
pub struct HousingState {
    #[serde(skip)]
    pub template: Rc<HousingWorldObjectTemplate>,
    #[export]
    capacity: u32,
}

impl<'de> WorldObjectStateTrait<'de> for HousingState {
    fn get_template(&self) -> Rc<impl WorldObjectTemplateTrait> {
        self.template.clone()
    }
}
