class_name GlobalState
extends Resource

var zoom_sensitivity: float = 5.0

static func load_state() -> GlobalState:
	if not ResourceLoader.exists(Paths.GLOBAL_STATE):
		return GlobalState.new()
	return load(Paths.GLOBAL_STATE)

func save() -> void:
	ResourceSaver.save(self, Paths.GLOBAL_STATE)
