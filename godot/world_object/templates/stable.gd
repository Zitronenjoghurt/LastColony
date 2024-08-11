class_name StableWorldObjectTemplate
extends WorldObjectTemplate

func apply_template(game_state: GameState, location: Vector2i) -> void:
	BehaviorFactory.push_stable(game_state, location, common_data)
