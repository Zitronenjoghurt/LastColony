class_name World
extends Node2D

@onready var tilemap: TileMap = %TileMap
@onready var camera: Camera2D = %Camera

func _ready() -> void:
	add_to_group("world")
	tilemap.tile_set = GameManager.buildings_tileset
	GameManager.state.limit_camera(camera)
	_draw_all_world_objects()
	GameManager.build_world_finished()

func _draw_all_world_objects() -> void:
	var game_state: GameState = GameManager.state as GameState
	for index: int in game_state.get_object_state_indices():
		var object_state: WorldObjectState = game_state.get_object_state_by_index(index)
		if not object_state is WorldObjectState:
			continue
		var object_tile: WorldObjectTile = object_state.get_current_tile()
		if not object_tile is WorldObjectTile or object_tile.source_id == -1:
			continue
		var coords: Vector2i = game_state.index_to_coords(index)
		tilemap.set_cell(0, coords, object_tile.source_id, Vector2i(0, 0))
