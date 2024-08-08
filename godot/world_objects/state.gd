class_name WorldObjectState
extends Resource

@export var id: WorldObject.ID

func update(_state: GameState, _index: int) -> void:
	return

# What happens when you place this object
func place() -> void:
	return

func get_world_object() -> WorldObject:
	if not id is WorldObject.ID:
		push_error("Error while trying to retrieve WorldObject: WorldObject id for WorldObjectState '%s' has not been set" % resource_name)
		return null
	var object: WorldObject = ObjectManager.get_object(id)
	if not object is WorldObject:
		push_error("Error while trying to retrieve WorldObject: WorldObject for id '%s' does not exist" % WorldObject.get_id_name(id))
		return null
	return object

# Returns the source ID of the tile to be displayed by this object
func get_current_tile() -> WorldObjectTile:
	push_error("WorldObjectState get_current_tile: Unimplemented for state of id '%s'" % id)
	return null

static func from_dict(_data: Dictionary, _id: int) -> WorldObjectState:
	push_error("WorldObjectState Deserialization: Unimplemented for state of id '%s'" % _id)
	return null

func to_dict() -> Dictionary:
	push_error("WorldObjectState Serialization: Unimplemented for state of id '%s'" % id)
	return {}