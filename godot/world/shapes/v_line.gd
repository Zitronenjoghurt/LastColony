class_name VLinePresetShape
extends WorldPresetShape

@export var x: int = 0
@export var start_y: int = 0
@export var stop_y: int = 0

func apply(state: GameState) -> void:
	for y: int in range(start_y, stop_y + 1):
		var coords: Vector2i = Vector2i(x, y)
		template.apply_template(state, coords)
		#var success: bool = state.add_new_object_state_at_coords(coords, object_id)
		#if not success:
		#	push_warning("WorldPresetShape v_line was unable to place object of id '%s' at coords %s" % [WorldObject.get_id_name(object_id), coords])
