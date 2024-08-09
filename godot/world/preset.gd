class_name WorldPreset
extends Resource

@export var height: int = 50
@export var width: int = 50
@export var shapes: Array[WorldPresetShape] = []

func create_gamestate(index: int) -> GameStateDepracated:
	var game_state: GameStateDepracated = GameStateDepracated.create_new(index)
	for shape: WorldPresetShape in shapes:
		shape.apply(game_state)
	return game_state
