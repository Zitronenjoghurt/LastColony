class_name SaveManager
extends Object

static func create_new(index: int) -> GameState:
	var state: GameState = GameState.create(index, Config.VERSION)
	return state

static func get_save_path(index: int) -> String:
	return "user://savegame_" + str(index) + ".save"

static func save_file_exists(index: int) -> bool:
	return FileAccess.file_exists(get_save_path(index))

static func load_state(index: int) -> GameState:
	if not save_file_exists(index):
		return create_new(index)
	var path: String = get_save_path(index)
	var file: FileAccess = FileAccess.open(path, FileAccess.READ)
	var json_string: String = file.get_as_text()
	var state: GameState = GameState.from_json(json_string)
	return state

static func save_state(state: GameState) -> void:
	var file_index: int = state.get_file_index()
	var path: String = get_save_path(file_index)
	
	var file: FileAccess = FileAccess.open(path, FileAccess.WRITE)
	if file == null:
		push_error("An error occured while writing save file of index '%s': %s" % [file_index, FileAccess.get_open_error()])
		return
		
	file.store_string(state.to_json())
