class_name HousingWorldObjectTemplate
extends WorldObjectTemplate

@export var housing_data: WorldObjectHousingData

func apply_template(game_state: GameState, location: Vector2i) -> void:
	BehaviorFactory.push_housing(game_state, location, common_data, housing_data)
