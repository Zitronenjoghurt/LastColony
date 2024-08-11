class_name StaticObjectState
extends WorldObjectStateDeprecated

func get_current_tile() -> WorldObjectTileDeprecated:
	var object: StaticObject = get_world_object()
	if not object is StaticObject:
		push_error("An error occured while retrieving StaticObject for StaticObjectState with id '%s': Retrieved object is not of type StaticObject" % WorldObject.get_id_name(id))
		return null
	return object.tile

static func from_dict(_data: Dictionary, _id: WorldObject.ID) -> StaticObjectState:
	var state: StaticObjectState = StaticObjectState.new()
	state.id = _id
	return state

func to_dict() -> Dictionary:
	return {"id": id}
