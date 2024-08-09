# 1 if you can place buildings at the given index
# 0 if you can't
class_name BuildableMap
extends DataMap

func update(state: GameStateDepracated) -> void:
	for index: int in state.get_object_state_indices():
		var coords: Vector2i = state.index_to_coords(index)
		var object_state: WorldObjectState = state.get_object_state_by_index(index)
		if not object_state is WorldObjectState:
			push_error("An unexpected error occured while updating BuildableMap at index '%s' %s: State at index is not a WorldObjectState" % [index, coords])
			continue
		var object: WorldObject = object_state.get_world_object()
		if not object is WorldObject:
			push_error("An unexpected error occured while updating BuildableMap at index '%s' %s: WorldObject of State at index is not a WorldObject" % [index, coords])
			continue
		update_single(coords, object, state)

func update_single(coords: Vector2i, object: WorldObject, state: GameStateDepracated) -> void:
	if coords.y == 0:
		return
	if object.supports_buildings:
		var buildable_index: int = state.coords_to_index(Vector2i(coords.x, coords.y - 1))
		set_at_index(1, buildable_index)
