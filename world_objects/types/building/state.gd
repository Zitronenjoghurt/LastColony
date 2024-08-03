class_name BuildingObjectState
extends WorldObjectState

@export var is_active: bool = false
@export var is_grounded: bool = false

func get_current_tile() -> WorldObjectTile:
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
