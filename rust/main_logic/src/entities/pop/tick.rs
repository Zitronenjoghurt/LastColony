use godot::prelude::*;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PopTickResult {
    pub food_eaten: u64,
}

#[derive(GodotClass, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(no_init)]
pub struct PopTickResultAccumulated {
    pub total_food_eaten: u64,
}

#[godot_api]
impl PopTickResultAccumulated {
    #[func]
    fn get_total_food_eaten(&self) -> u64 {
        self.total_food_eaten
    }

    pub fn combine(&mut self, other: PopTickResultAccumulated) {
        self.total_food_eaten += other.total_food_eaten
    }
}
