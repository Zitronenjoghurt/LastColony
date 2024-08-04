class_name GameState
extends Resource

@export var file_index: int
@export var game_version: int = Config.VERSION
@export var map_height: int = 50
@export var map_width: int = 200
@export var object_states: Dictionary = {}
@export var pawns: Dictionary = {}
@export var new_pawn_index: int = 0

# Maps
var buildable_map: Array[int] = []

var lock_object_states: Mutex = Mutex.new()
var lock_pawns: Mutex = Mutex.new()

func limit_camera(camera: Camera2D) -> void:
	camera.limit_bottom = map_height * 16
	camera.limit_right = map_width * 16
	
func coords_to_index(coords: Vector2i) -> int:
	return coords.y * map_width + coords.x
	
func index_to_coords(index: int) -> Vector2i:
	var x: int = index % map_width
	var y: int = int(index / map_width)
	return Vector2i(x, y)
	
func get_object_state_indices() -> Array:
	return object_states.keys()
	
func get_object_state_by_index(index: int) -> WorldObjectState:
	if not index in object_states:
		push_warning("Tried to access object state of index '%s' but it yielded null" % index)
		return null
	return object_states[index]
	
func add_new_object_state_at_coords(coords: Vector2i, object_id: WorldObject.ID) -> bool:
	var index: int = coords_to_index(coords)
	return add_new_object_state(index, object_id)

func add_new_object_state(index: int, object_id: WorldObject.ID) -> bool:
	var object: WorldObject = ObjectManager.get_object(object_id)
	if not object is WorldObject:
		push_error("An error occured while adding new object state of id '%s': Given ID did not yield any WorldObject" % WorldObject.get_id_name(object_id))
		return false
	var state: WorldObjectState = object.new_state()
	return add_object_state(index, state)
	
func add_object_state_at_coords(coords: Vector2i, object_state: WorldObjectState) -> bool:
	var index: int = coords_to_index(coords)
	return add_object_state(index, object_state)

func add_object_state(index: int, object_state: WorldObjectState) -> bool:
	if index in object_states:
		push_error("An error occured while adding object state of id '%s' at index '%s': There is already a state at the given index" % [WorldObject.get_id_name(object_state.id), index])
		return false
	lock_object_states.lock()
	object_states[index] = object_state
	lock_object_states.unlock()
	return true
	
func get_pawn_by_index(index: int) -> Pawn:
	if index not in pawns:
		push_warning("Tried to access pawn of index '%s' but it yielded null" % index)
		return null
	return pawns[index]
	
func add_pawn(pawn: Pawn) -> void:
	lock_pawns.lock()
	pawns[new_pawn_index] = pawn
	new_pawn_index += 1
	lock_pawns.unlock()
	
func update_buildable_map() -> void:
	buildable_map = Utils.populate_new_array_int(map_height * map_width, 0)
	for index: int in get_object_state_indices():
		var coords: Vector2i = index_to_coords(index)
		if coords.y == 0:
			continue
		var state: WorldObjectState = get_object_state_by_index(index)
		if not state is WorldObjectState:
			push_error("An unexpected error occured while updating buildable_map at index '%s' %s: State at index is not a WorldObjectState" % [index, coords])
			continue
		var object: WorldObject = state.get_world_object()
		if not object is WorldObject:
			push_error("An unexpected error occured while updating buildable_map at index '%s' %s: WorldObject of State at index is not a WorldObject" % [index, coords])
			continue
		if object.supports_buildings:
			var buildable_index: int = coords_to_index(Vector2i(coords.x, coords.y - 1))
			buildable_map[buildable_index] = 1

static func create_new(index: int) -> GameState:
	var state: GameState = GameState.new()
	state.file_index = index
	return state

static func get_save_path(index: int) -> String:
	return "user://savegame_"+ str(index) +".tres"

static func save_file_exists(index: int) -> bool:
	return ResourceLoader.exists(get_save_path(index))

static func load_state(index: int) -> GameState:
	if not save_file_exists(index):
		return GameState.create_new(index)
	var path: String = get_save_path(index)
	return load(path)

func save() -> void:
	var path: String = GameState.get_save_path(file_index)
	ResourceSaver.save(self, path)
