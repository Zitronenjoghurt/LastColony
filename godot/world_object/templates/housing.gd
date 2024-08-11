class_name HousingWorldObjectTemplate
extends WorldObjectTemplate

@export var capacity: int

func apply_template(game_state: GameState, location: Vector2i) -> void:
	BehaviorFactory.push_housing(game_state, location, common_data, capacity)
