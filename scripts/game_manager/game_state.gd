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
@export var buildable_map: Array[int] = []

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
		return null
	return object_states[index]
	
func add_object_state(index: int, object_state: WorldObjectState) -> bool:
	if index in object_states:
		return false
	lock_object_states.lock()
	object_states[index] = object_state
	lock_object_states.unlock()
	return true
	
func get_pawn_by_index(index: int) -> Pawn:
	if index not in pawns:
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
