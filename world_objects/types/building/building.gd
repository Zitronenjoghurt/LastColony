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
