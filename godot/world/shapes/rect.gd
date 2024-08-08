class_name RectPresetShape
extends WorldPresetShape

@export var top_left: Vector2i = Vector2i(0, 0)
@export var bottom_right: Vector2i = Vector2i(0, 0)

func apply(state: GameState) -> void:
	for y: int in range(top_left.y, bottom_right.y + 1):
		for x: int in range(top_left.x, bottom_right.x + 1):
			var coords: Vector2i = Vector2i(x, y)
			var success: bool = state.add_new_object_state_at_coords(coords, object_id)
			if not success:
				push_warning("WorldPresetShape rect was unable to place object of id '%s' at coords %s" % [WorldObject.get_id_name(object_id), coords])
