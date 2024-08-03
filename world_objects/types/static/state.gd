class_name StaticObjectState
extends WorldObjectState

func get_current_tile() -> WorldObjectTile:
	var object: StaticObject = get_world_object()
	if not object is StaticObject:
		push_error("An error occured while retrieving StaticObject for StaticObjectState with id '%s': Retrieved object is not of type StaticObject" % WorldObject.get_id_name(id))
		return null
	return object.tile