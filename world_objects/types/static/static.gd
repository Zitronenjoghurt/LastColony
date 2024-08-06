class_name StaticObject
extends WorldObject

@export var tile: WorldObjectTile

func _init() -> void:
	type = WorldObject.TYPE.STATIC

func push_tiles(tileset: TileSet) -> TileSet:
	tileset = push_tile(tile, tileset)
	return tileset

func new_state_from_dict(_data: Dictionary, _id: int) -> StaticObjectState:
	return StaticObjectState.from_dict(_data, _id)
