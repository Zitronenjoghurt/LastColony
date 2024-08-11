class_name World
extends Node2D

@onready var grid_map: BackgroundGridMap = %GridMap
@onready var object_map: TileMap = %ObjectMap
@onready var camera: Camera2D = %Camera

func _ready() -> void:
	var state: GameState = GameState.create(1, 1)
	var result: TickResult = state.tick(1)
	
	add_to_group("world")
	object_map.tile_set = GameManager.objects_tileset
	GameManager.state.limit_camera(camera)
	_init_grid_map()
	_draw_all_world_objects()
	GameManager.build_world_finished()
	
func _init_grid_map() -> void:
	grid_map.initialize(GameManager.state.get_map_height(), GameManager.state.get_map_width())
	#grid_map.draw_from_bitmap(GameManager.state.buildable_map.get_data())

func _draw_all_world_objects() -> void:
	var game_state: GameStateDepracated = GameManager.state as GameStateDepracated
	for index: int in game_state.get_object_state_indices():
		var object_state: WorldObjectStateDeprecated = game_state.get_object_state_by_index(index)
		if not object_state is WorldObjectStateDeprecated:
			continue
		var object_tile: WorldObjectTileDeprecated = object_state.get_current_tile()
		if not object_tile is WorldObjectTileDeprecated:
			object_tile = WorldObjectTileDeprecated.create_placeholder(object_state)
		var coords: Vector2i = game_state.index_to_coords(index)
		object_tile.draw_at_coords(object_map, coords)
