class_name BuildingObjectState
extends WorldObjectStateDeprecated

var is_active: bool = false
var is_grounded: bool = false

func update(state: GameStateDepracated, index: int) -> void:
	var grounded_value: int = state.grounded_map.get_value(index)
	if grounded_value == 1:
		is_grounded = true
	else:
		is_grounded = false

func get_current_tile() -> WorldObjectTileDeprecated:
	var object: BuildingObject = get_world_object()
	if not object is BuildingObject:
		push_error("An error occured while retrieving BuildingObject for BuildingObjectState with id '%s': Retrieved object is not of type BuildingObject" % WorldObject.get_id_name(id))
		return null
	
	if is_active and is_grounded:
		return object.tile_ground_active
	if is_active and not is_grounded:
		return object.tile_up_active
	if not is_active and is_grounded:
		return object.tile_ground_inactive
	if not is_active and not is_grounded:
		return object.tile_up_inactive
	return null
