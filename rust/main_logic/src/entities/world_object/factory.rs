use godot::prelude::*;

use crate::{entities::world_object::data::common::WorldObjectCommonData, gamestate::GameState};

use super::{class::WorldObject, data::housing::WorldObjectHousingData};

/// WorlObjectTemplates will use this to spawn behaviors in the game state
/// according to the properties specified in the template
#[derive(GodotClass, Debug, Default)]
#[class(no_init)]
pub struct BehaviorFactory {}

#[godot_api]
impl BehaviorFactory {
    #[func]
    fn push(
        mut gamestate: Gd<GameState>,
        location: Vector2i,
        common_data: Gd<WorldObjectCommonData>,
        housing_data: Gd<WorldObjectHousingData>,
    ) {
        let mut state = WorldObject::default();
        state.apply_common_data(&common_data);
        state.apply_housing_data(&housing_data);
        gamestate.bind_mut().push_object(location, state)
    }
}
