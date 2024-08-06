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
var buildable_map: BuildableMap = BuildableMap.new()
var grounded_map: GroundedMap = GroundedMap.new()

var lock_object_states: Mutex = Mutex.new()
var lock_pawns: Mutex = Mutex.new()

func initialize() -> void:
	buildable_map.initialize(map_height, map_width, 0)
	grounded_map.initialize(map_height, map_width, 0)
	full_update()
	
func full_update() -> void:
	update_all_maps_full()
	update_all_objects()

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
	var state: WorldObjectState = ObjectManager.new_object_state(object_id)
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
	
func update_all_objects() -> void:
	for index: int in get_object_state_indices():
		var object_state: WorldObjectState = get_object_state_by_index(index)
		if not object_state is WorldObjectState:
			continue
		object_state.update(self, index)
	
func update_buildable_map_full() -> void:
	buildable_map.update(self)
	
func update_grounded_map_full() -> void:
	grounded_map.update(self)

# Better for updating all data maps at once since you only iterate over every world object once
func update_all_maps_full() -> void:
	for index: int in get_object_state_indices():
		update_all_maps_at_index(index)

func update_all_maps_at_index(index: int) -> void:
	var coords: Vector2i = index_to_coords(index)
	var object_state: WorldObjectState = get_object_state_by_index(index)
	if not object_state is WorldObjectState:
		push_error("An unexpected error occured while updating all data maps at index '%s' %s: State at index is not a WorldObjectState" % [index, coords])
		return
	var object: WorldObject = object_state.get_world_object()
	if not object is WorldObject:
		push_error("An unexpected error occured while updating all data maps at index '%s' %s: WorldObject of State at index is not a WorldObject" % [index, coords])
		return
	buildable_map.update_single(coords, object, self)
	grounded_map.update_single(coords, object, self)

static func create_new(index: int) -> GameState:
	var state: GameState = GameState.new()
	state.file_index = index
	return state

static func get_save_path(index: int) -> String:
	return "user://savegame_"+ str(index) +".save"

static func save_file_exists(index: int) -> bool:
	return FileAccess.file_exists(get_save_path(index))

static func load_state(index: int) -> GameState:
	if not save_file_exists(index):
		return GameState.create_new(index)
	var path: String = get_save_path(index)
	var file: FileAccess = FileAccess.open(path, FileAccess.READ)
	var json_string: String = file.get_as_text()
	var json: JSON = JSON.new()
	var parse_result: int = json.parse(json_string)
	if not parse_result == OK:
		push_error("An error occured while loading existing game state at index '%s': %s on line %s" % [index, json.get_error_message(), json.get_error_line()])
		return
	var data: Dictionary = json.get_data()
	if not data is Dictionary:
		push_error("An error occured while loading existing game state at index '%s': Retrieved data is not a dictionary" % index)
		return
	var state: GameState = GameState.from_dict(data)
	if not state is GameState:
		push_error("An error occured while loading existing game state at index '%s'" % index)
		return
	state.file_index = index
	return state

func save() -> void:
	var path: String = GameState.get_save_path(file_index)
	var save_data: Dictionary = to_dict()
	
	var file: FileAccess = FileAccess.open(path, FileAccess.WRITE)
	if file == null:
		push_error("An error occured while writing save file of index '%s': %s" % [file_index, FileAccess.get_open_error()])
		return
	
	var json_string: String = JSON.stringify(save_data)
	file.store_string(json_string)

static func from_dict(data: Dictionary) -> Variant:
	var context: String = "GameState"
	var state: GameState = GameState.new()
	
	var _game_version: int = Deserialize.process_int(data, "game_version", context, true, 1, 1)
	var _map_height: int = Deserialize.process_int(data, "map_height", context, true, 0, 1)
	var _map_width: int = Deserialize.process_int(data, "map_width", context, true, 0, 1)
	var _new_pawn_index: int = Deserialize.process_int(data, "new_pawn_index", context, true, 0, 0)
	
	var failed: bool = Deserialize.check_for_null([_game_version, _map_height, _map_width, _new_pawn_index])
	if failed:
		return
	
	var _object_states_data: Dictionary = data.get("object_states", {})
	var _object_states: Dictionary = {}
	for index: String in _object_states_data:
		if not index.is_valid_int():
			push_error("GameState Deserialization: Skipped object state of unknown index, index was not an integer")
			continue
		var object_data: Dictionary = _object_states_data[index]
		if not object_data is Dictionary:
			push_error("GameState Deserialization: Skipped object state of index '%s', data is not a dictionary" % index)
			continue
		var object_state: WorldObjectState = ObjectManager.object_state_from_dict(object_data)
		if not object_state is WorldObjectState:
			push_error("GameState Deserialization: Skipped object state of index '%s', failed to create object state from data" % index)
			continue
		_object_states[index.to_int()] = object_state
	
	state.game_version = _game_version
	state.map_height = _map_height
	state.map_width = _map_width
	state.object_states = _object_states
	state.new_pawn_index = _new_pawn_index
	
	return state

func to_dict() -> Dictionary:
	var data: Dictionary = {}
	data.game_version = game_version
	data.map_height = map_height
	data.map_width = map_width
	data.new_pawn_index = new_pawn_index
	
	var _object_states: Dictionary = {}
	for index: int in get_object_state_indices():
		var state: WorldObjectState = get_object_state_by_index(index)
		if not state is WorldObjectState:
			push_error("GameState Serialization: Skipped object state of index '%s', state is not a valid WorldObjectState" % index)
			continue
		_object_states[index] = state.to_dict()
	
	data.object_states = _object_states
	
	return data
