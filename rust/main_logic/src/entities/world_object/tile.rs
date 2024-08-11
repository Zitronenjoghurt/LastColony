use godot::prelude::*;

use crate::enums::tile_type::TileType;

#[derive(GodotClass, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(tool, init, base = Resource)]
pub struct WorldObjectTileBase {
    #[export]
    pub tile_type: TileType,
}
