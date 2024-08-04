extends Node

var world_scene: PackedScene = load(Paths.WORLD_SCENE)
var objects_tileset: TileSet
var state: GameState
var global: GlobalState

signal load_game_started()
signal load_game_finished()

func _ready() -> void:
	global = GlobalState.load_state()
	ObjectManager.load_objects()
	objects_tileset = ObjectManager.generate_tileset()
	
	if OS.is_debug_build():
		_debug_startup_checks()
	
func _debug_startup_checks() -> void:
	Paths.check()

func load_game(index: int = 0) -> void:
	load_game_started.emit()
	state = GameState.load_state(index)
	
	# Add an object and save state for testing purposes
	var object_state: WorldObjectState = ObjectManager.new_object_state(WorldObject.ID.HUT)
	state.add_object_state(0, object_state)
	state.add_object_state(1814, object_state)
	state.save()
	
	state.update_buildable_map()
	
	get_tree().change_scene_to_packed(world_scene)

func build_world_finished() -> void:
	load_game_finished.emit()
