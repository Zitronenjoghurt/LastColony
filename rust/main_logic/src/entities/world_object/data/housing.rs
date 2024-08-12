use godot::prelude::*;

#[derive(GodotClass, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(tool, init, base = Resource)]
pub struct WorldObjectHousingData {
    #[export]
    capacity: u32,
}
