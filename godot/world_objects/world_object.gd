class_name WorldObject
extends Resource

enum ID {
	TREES,
	HUT,
	DIRT_TOP,
	DIRT_BLOCK
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
@export var default_state: WorldObjectStateDeprecated
@export var icon: Texture2D
@export var supports_buildings: bool = true
# Determines if buildings placed on this object will get is_grounded = true
@export var is_ground: bool = false
@export var vertical_speed_multiplier: float = 1.0
@export var horizontal_speed_multiplier: float = 1.0

func push_tile(tile: WorldObjectTileDeprecated, tileset: TileSet) -> TileSet:
	if not tile is WorldObjectTileDeprecated:
		return tileset
	var source_id: int = tileset.add_source(tile.create_source())
	tile.source_id = source_id
	return tileset
	
func push_tiles(tileset: TileSet) -> TileSet:
	return tileset

func new_state() -> WorldObjectStateDeprecated:
	var state: WorldObjectStateDeprecated = default_state.duplicate(true)
	state.id = id
	return state

func new_state_from_dict(_data: Dictionary, _id: int) -> WorldObjectStateDeprecated:
	push_error("WorldObject new_state_from_dict: Unimplemented for WorldObject of id '%s'" % _id)
	return null

static func get_id_name(_id: ID) -> String:
	if _id >= len(ID.keys()):
		return str(_id)
	return ID.keys()[_id]
