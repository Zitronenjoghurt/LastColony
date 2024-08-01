class_name WorldObject
extends Resource

enum ID {
	TREES,
	HUT
}

enum TYPE {
	ENVIRONMENT,
	BUILDING
}

enum BuildingCategory {
	NONE,
	HOUSING
}

@export var id: ID
@export var type: TYPE
@export var building_category: BuildingCategory = BuildingCategory.NONE
@export var display_name: String
@export var default_state: WorldObjectState
@export var icon: Texture2D
@export var tile_ground_inactive: WorldObjectTile
@export var tile_ground_active: WorldObjectTile
@export var tile_up_inactive: WorldObjectTile
@export var tile_up_active: WorldObjectTile

func push_tile(tile: WorldObjectTile, tileset: TileSet) -> TileSet:
	if not tile is WorldObjectTile:
		return tileset
	var source_id: int = tileset.add_source(tile.create_source())
	tile.source_id = source_id
	return tileset
	
func push_tiles(tileset: TileSet) -> TileSet:
	tileset = push_tile(tile_ground_active, tileset)
	tileset = push_tile(tile_ground_inactive, tileset)
	tileset = push_tile(tile_up_active, tileset)
	tileset = push_tile(tile_up_inactive, tileset)
	return tileset

func new_state() -> WorldObjectState:
	var state: WorldObjectState = default_state.duplicate(true)
	state.id = id
	return state
