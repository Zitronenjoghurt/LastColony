class_name WorldObjectState
extends Resource

@export var id: WorldObject.ID
@export var is_active: bool = false
@export var is_grounded: bool = false

func get_world_object() -> WorldObject:
	if not id is WorldObject.ID:
		push_error("Error while trying to retrieve WorldObject: WorldObject id for WorldObjectState '%s' has not been set" % resource_name)
		return null
	var object: WorldObject = ObjectManager.get_object(id)
	if not object is WorldObject:
		push_error("Error while trying to retrieve WorldObject: WorldObject for id '%s' does not exist" % WorldObject.ID.keys()[id])
		return null
	return object

# Returns the source ID of the tile to be displayed by this object
func get_current_tile() -> WorldObjectTile:
	var object: WorldObject = get_world_object()
	if not object is WorldObject:
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
