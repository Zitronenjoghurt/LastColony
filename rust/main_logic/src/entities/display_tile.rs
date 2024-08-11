use godot::prelude::*;

use crate::enums::{tile_type::TileType, world_object_id::WorldObjectId};

#[derive(GodotClass, Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[class(no_init)]
pub struct DisplayTile {
    pub object_id: WorldObjectId,
    pub tile_type: TileType,
    pub location: Vector2i,
}

#[godot_api]
impl DisplayTile {
    #[func]
    fn get_object_id(&self) -> WorldObjectId {
        self.object_id
    }

    #[func]
    fn get_tile_type(&self) -> TileType {
        self.tile_type
    }

    #[func]
    fn get_location(&self) -> Vector2i {
        self.location
    }
}
