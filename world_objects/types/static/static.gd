class_name StaticObject
extends WorldObject

@export var tile: WorldObjectTile

func _init() -> void:
	type = WorldObject.TYPE.STATIC

func push_tiles(tileset: TileSet) -> TileSet:
	tileset = push_tile(tile, tileset)
	return tileset
