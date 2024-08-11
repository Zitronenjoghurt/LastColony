class_name World
extends Node2D

@onready var grid_map: BackgroundGridMap = %GridMap
@onready var object_map: TileMap = %ObjectMap
@onready var camera: Camera2D = %Camera

func _ready() -> void:
	var state: GameState = GameState.create(1, 1)
	var _result: TickResult = state.tick(1)
	
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
	var game_state: GameState = GameManager.state as GameState
	for display_tile: DisplayTile in game_state.get_current_display_tiles():
		var object_id: int = display_tile.get_object_id()
		var tile_type: int = display_tile.get_tile_type()
		var source_id: int = TemplateManager.get_source_id(object_id, tile_type)
		var atlas_coords: Vector2i = TemplateManager.get_atlas_coords(object_id, tile_type)
		var location: Vector2i = display_tile.get_location()
		object_map.set_cell(0, location, source_id, atlas_coords)
