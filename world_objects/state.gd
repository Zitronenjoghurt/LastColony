class_name WorldObjectState
extends Resource

var id: WorldObject.ID

func get_display_name() -> String:
	if not id is WorldObject.ID:
		push_error("Error while trying to retrieve WorldObjectState display name: WorldObject id for WorldObjectState '%s' has not been set" % resource_name)
		return "missing name"
	var object: WorldObject = ObjectManager.get_object(id)
	if not object is WorldObject:
		push_error("Error while trying to retrieve WorldObjectState display name: WorldObject for id '%s' does exist" % WorldObject.ID.keys()[id])
		return "missing name"
	return object.display_name
