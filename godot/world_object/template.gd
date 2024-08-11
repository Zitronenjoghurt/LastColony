class_name WorldObjectTemplate
extends Resource

@export var common_data: WorldObjectCommonData

func apply_template(_game_state: GameState, _location: Vector2i) -> void:
	return

func get_id() -> int:
	return common_data.id
