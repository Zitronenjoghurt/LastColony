class_name BuildingObject
extends WorldObject

@export var tile_ground_inactive: WorldObjectTile
@export var tile_ground_active: WorldObjectTile
@export var tile_up_inactive: WorldObjectTile
@export var tile_up_active: WorldObjectTile

func _init() -> void:
	type = WorldObject.TYPE.BUILDING

func push_tiles(tileset: TileSet) -> TileSet:
	tileset = push_tile(tile_ground_inactive, tileset)
	tileset = push_tile(tile_ground_active, tileset)
	tileset = push_tile(tile_up_inactive, tileset)
	tileset = push_tile(tile_ground_active, tileset)
	return tileset

func new_state_from_dict(data: Dictionary, _id: int) -> WorldObjectState:
	var serde_id: String = data.get("serde_id")
	if not serde_id is String:
		push_error("BuldingObjectState Deserialization: Missing serde_id for object state of id '%s'" % _id)
		return null
	
	match serde_id:
		"housing":
			return HousingBuildingState.from_dict(data, _id)
	
	push_error("BuildingObjectState Deserialization: Invalid serde_id '%s' for object state of id '%s'" % [serde_id, _id])
	return null
