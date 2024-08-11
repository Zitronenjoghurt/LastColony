class_name WorldPreset
extends Resource

@export var height: int = 50
@export var width: int = 50
@export var shapes: Array[WorldPresetShape] = []

func create_gamestate(index: int) -> GameState:
	var game_state: GameState = SaveManager.create_new(index)
	game_state.set_map_height(height)
	game_state.set_map_width(width)
	for shape: WorldPresetShape in shapes:
		shape.apply(game_state)
	return game_state
