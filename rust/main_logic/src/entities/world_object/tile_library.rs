use std::collections::HashMap;

use godot::prelude::*;

use crate::{
    entities::display_tile::DisplayTile,
    enums::{tile_type::TileType, world_object_id::WorldObjectId},
};

#[derive(Debug, Clone, Default)]
pub struct TileLibraryEntry {
    tiles: HashMap<TileType, (u32, Vector2i)>,
}

impl TileLibraryEntry {
    fn add_source_id_and_atlas_coords(
        &mut self,
        tile_type: TileType,
        source_id: u32,
        atlas_coords: Vector2i,
    ) {
        self.tiles.insert(tile_type, (source_id, atlas_coords));
    }

    fn has_tile_type(&self, tile_type: TileType) -> bool {
        self.tiles.contains_key(&tile_type)
    }
}

/// Will keep track of the source id of the tile for a
/// specified world object id and tile type
#[derive(GodotClass, Debug, Default, Clone)]
#[class(init)]
pub struct TileLibrary {
    entries: HashMap<WorldObjectId, TileLibraryEntry>,
}

#[godot_api]
impl TileLibrary {
    #[func]
    fn get_display_tile_source_id(&self, display_tile: Gd<DisplayTile>) -> u32 {
        let object_id = display_tile.bind().object_id;
        let tile_type = display_tile.bind().tile_type;
        self.get_source_id(object_id, tile_type)
    }

    #[func]
    fn get_source_id(&self, object_id: WorldObjectId, tile_type: TileType) -> u32 {
        self.get_source_id_and_atlas_coords(object_id, tile_type).0
    }

    #[func]
    fn get_atlas_coords(&self, object_id: WorldObjectId, tile_type: TileType) -> Vector2i {
        self.get_source_id_and_atlas_coords(object_id, tile_type).1
    }

    #[func]
    fn add_source_id_and_atlas_coords(
        &mut self,
        object_id: WorldObjectId,
        tile_type: TileType,
        source_id: u32,
        atlas_coords: Vector2i,
    ) -> bool {
        if self.has_object_tile_type(object_id, tile_type) {
            return false;
        }
        let entry = self.entries.entry(object_id).or_default();
        entry.add_source_id_and_atlas_coords(tile_type, source_id, atlas_coords);
        true
    }

    #[func]
    fn has_object_tile_type(&self, object_id: WorldObjectId, tile_type: TileType) -> bool {
        if let Some(entry) = self.entries.get(&object_id) {
            entry.has_tile_type(tile_type)
        } else {
            false
        }
    }

    fn get_source_id_and_atlas_coords(
        &self,
        object_id: WorldObjectId,
        tile_type: TileType,
    ) -> (u32, Vector2i) {
        self.entries
            .get(&object_id)
            .and_then(|entry| entry.tiles.get(&tile_type))
            .copied()
            .unwrap_or_default()
    }
}
