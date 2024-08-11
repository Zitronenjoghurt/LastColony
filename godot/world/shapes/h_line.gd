class_name HLinePresetShape
extends WorldPresetShape

@export var y: int = 0
@export var start_x: int = 0
@export var stop_x: int = 0

func apply(state: GameState) -> void:
	for x: int in range(start_x, stop_x + 1):
		var coords: Vector2i = Vector2i(x, y)
		template.apply_template(state, coords)
		#var success: bool = state.add_new_object_state_at_coords(coords, object_id)
		#if not success:
		#	push_warning("WorldPresetShape h_line was unable to place object of id '%s' at coords %s" % [WorldObject.get_id_name(object_id), coords])
