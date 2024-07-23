extends Node

var buildings_tileset: TileSet

func _ready() -> void:
	ObjectManager.load_objects()
	buildings_tileset = ObjectManager.generate_tileset()
	