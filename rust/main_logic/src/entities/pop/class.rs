use crate::traits::serde::{FromJsonString, ToJsonString};
use godot::prelude::*;
use serde::{Deserialize, Serialize};

use super::tick::PopTickResult;

#[derive(GodotClass, Debug, Serialize, Deserialize)]
#[class(no_init)]
pub struct Pop {
    #[serde(default)]
    name: String,
    #[serde(default)]
    delta_ticks: u64,
    #[serde(default)]
    age_seconds: u64,
    #[serde(default)]
    current_location: Vector2i,
    #[serde(default)]
    home_location: Vector2i,
    #[serde(default)]
    work_location: Vector2i,
}

impl Default for Pop {
    fn default() -> Self {
        Self {
            name: "no_name".to_string(),
            delta_ticks: Default::default(),
            age_seconds: Default::default(),
            current_location: Vector2i { x: -1, y: -1 },
            home_location: Vector2i { x: -1, y: -1 },
            work_location: Vector2i { x: -1, y: -1 },
        }
    }
}

#[godot_api]
impl Pop {
    #[func]
    fn create(age_years: u64, seconds_per_year: u64) -> Gd<Self> {
        Gd::from_object(Pop::new_from_age(age_years, seconds_per_year))
    }

    #[func]
    fn from_json(json_string: String) -> Gd<Self> {
        let pop: Pop = Pop::from_json_string(&json_string);
        Gd::from_object(pop)
    }

    #[func]
    fn to_json(&self) -> String {
        self.to_json_string()
    }

    fn new_from_age(age_years: u64, seconds_per_year: u64) -> Self {
        Pop {
            age_seconds: age_years * seconds_per_year,
            ..Default::default()
        }
    }

    pub fn tick(&mut self, tps: u64) -> PopTickResult {
        self.delta_ticks += 1;
        if self.delta_ticks >= tps {
            self.delta_ticks = 0;
            self.age_second();
        };
        PopTickResult::default()
    }

    fn age_second(&mut self) {
        self.age_seconds += 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_from_age() {
        let pop = Pop::new_from_age(10, 120);
        assert_eq!(pop.age_seconds, 1200)
    }
}
