use godot::prelude::*;

use crate::{entities::world_object::common_data::WorldObjectCommonData, gamestate::GameState};

use super::{
    behavior::{WorldObjectBehavior, WorldObjectBehaviorType},
    behaviors::{housing::HousingBehavior, stable::StableBehavior},
};

/// WorlObjectTemplates will use this to spawn behaviors in the game state
/// according to the properties specified in the template
#[derive(GodotClass, Debug, Default)]
#[class(no_init)]
pub struct BehaviorFactory {}

#[godot_api]
impl BehaviorFactory {
    #[func]
    fn push_stable(
        mut gamestate: Gd<GameState>,
        location: Vector2i,
        common_data: Gd<WorldObjectCommonData>,
    ) {
        let mut state = StableBehavior::default();
        state.apply_common_data(common_data);
        gamestate
            .bind_mut()
            .push_object(location, WorldObjectBehaviorType::Stable(state))
    }

    #[func]
    fn push_housing(
        mut gamestate: Gd<GameState>,
        location: Vector2i,
        common_data: Gd<WorldObjectCommonData>,
        capacity: u32,
    ) {
        let mut state = HousingBehavior::default();
        state.apply_common_data(common_data);
        state.capacity = capacity;
        gamestate
            .bind_mut()
            .push_object(location, WorldObjectBehaviorType::Housing(state))
    }
}
