class_name NavigationMap
extends Resource

@export var h_map: DataMap = DataMap.new()
@export var v_map: DataMap = DataMap.new()

enum Direction {H, V}

func initialize(map_height: int, map_width: int) -> void:
	h_map.initialize(map_height, map_width, 0)
	v_map.initialize(map_height, map_width, 0)

func update(state: GameStateDepracated) -> void:
	for index: int in state.get_object_state_indices():
		var coords: Vector2i = state.index_to_coords(index)
		var object_state: WorldObjectState = state.get_object_state_by_index(index)
		if not object_state is WorldObjectState:
			push_error("An unexpected error occured while updating NavigationMap at index '%s' %s: State at index is not a WorldObjectState" % [index, coords])
			continue
		var object: WorldObject = object_state.get_world_object()
		if not object is WorldObject:
			push_error("An unexpected error occured while updating NavigationMap at index '%s' %s: WorldObject of State at index is not a WorldObject" % [index, coords])
			continue
		update_single(coords, index, object, state)

func update_single(coords: Vector2i, index: int, object: WorldObject, state: GameStateDepracated) -> void:
	set_at_index_v(object.vertical_speed_multiplier, index)
	
	# So pawns are able to walk above ground tiles
	if object.is_ground and coords.y > 0:
		var above_index: int = state.coords_to_index(Vector2i(coords.x, coords.y - 1))
		set_at_index_h(object.horizontal_speed_multiplier, above_index)
	else:
		set_at_index_h(object.horizontal_speed_multiplier, index)

func reset() -> void:
	h_map.reset()
	v_map.reset()

func reset_h() -> void:
	h_map.reset()

func reset_v() -> void:
	v_map.reset()

func get_value_h(index: int) -> float:
	return h_map.get_value(index)

func get_value_v(index: int) -> float:
	return v_map.get_value(index)

func set_at_index_h(value: float, index: int) -> void:
	var existing_value: float = get_value_h(index)
	if existing_value != 0:
		value *= existing_value
	h_map.set_at_index(value, index)

func set_at_index_v(value: float, index: int) -> void:
	var existing_value: float = get_value_v(index)
	if existing_value != 0:
		value *= existing_value
	v_map.set_at_index(value, index)
