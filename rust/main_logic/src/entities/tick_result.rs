use godot::prelude::*;

use crate::entities::pop::tick::PopTickResultAccumulated;

#[derive(GodotClass, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(no_init)]
pub struct TickResult {
    pub pop_result: PopTickResultAccumulated,
}

#[godot_api]
impl TickResult {
    #[func]
    fn get_pop_result(&self) -> Gd<PopTickResultAccumulated> {
        Gd::from_object(self.pop_result)
    }
}
