use godot::prelude::*;

#[derive(GodotClass, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(tool, init, base = Resource)]
pub struct WorldObjectHousingData {
    pub apply: bool,
    #[export]
    pub capacity: u32,
}

#[godot_api]
impl WorldObjectHousingData {
    #[func]
    fn new_disabled() -> Gd<WorldObjectHousingData> {
        let data = WorldObjectHousingData {
            apply: false,
            ..Default::default()
        };
        Gd::from_object(data)
    }

    #[func]
    fn apply(&mut self) {
        self.apply = true;
    }
}
