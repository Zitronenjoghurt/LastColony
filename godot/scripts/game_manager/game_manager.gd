extends Node

var world_scene: PackedScene = load(Paths.WORLD_SCENE)
var objects_tileset: TileSet
var state: GameState
var global: GlobalState

signal load_game_started()
signal load_game_finished()

func _ready() -> void:
	global = GlobalState.load_state()
	objects_tileset = TemplateManager.load_templates()
	
	if OS.is_debug_build():
		_debug_startup_checks()
	
func _debug_startup_checks() -> void:
	Paths.check()

func save_game() -> void:
	if state is GameState:
		SaveManager.save_state(state)
	
func start_new_game(preset: WorldPreset, index: int = 0) -> void:
	load_game_started.emit()
	state = preset.create_gamestate(index)
	_start_game()

func start_existing_game(index: int = 0) -> void:
	load_game_started.emit()
	state = SaveManager.load_state(index)
	_start_game()

func _start_game() -> void:
	state.initialize()
	get_tree().change_scene_to_packed(world_scene)

func build_world_finished() -> void:
	load_game_finished.emit()
