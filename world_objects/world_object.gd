class_name WorldObject
extends Resource

enum ID {
	TREES,
	HUT,
	DIRT
}

enum TYPE {
	ENVIRONMENT,
	BUILDING,
	STATIC
}

enum BuildingCategory {
	NONE,
	HOUSING
}

enum JobType {
	NONE
}

@export var id: ID
@export var type: TYPE
# Determines in which building menu this object will show up
@export var building_category: BuildingCategory = BuildingCategory.NONE
# Determines the type of job this building offers
@export var job_type: JobType = JobType.NONE
@export var display_name: String
@export var default_state: WorldObjectState
@export var icon: Texture2D
@export var supports_buildings: bool = true
@export var is_ground: bool = false
@export var vertical_speed_multiplier: float = 1.0
@export var horizontal_speed_multiplier: float = 1.0

func push_tile(tile: WorldObjectTile, tileset: TileSet) -> TileSet:
	if not tile is WorldObjectTile:
		return tileset
	var source_id: int = tileset.add_source(tile.create_source())
	tile.source_id = source_id
	return tileset
	
func push_tiles(tileset: TileSet) -> TileSet:
	return tileset

func new_state() -> WorldObjectState:
	var state: WorldObjectState = default_state.duplicate(true)
	state.id = id
	return state

static func get_id_name(_id: ID) -> String:
	return ID.keys()[_id]
