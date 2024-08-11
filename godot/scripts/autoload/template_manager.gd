extends Node

var placeholder_tile_texture: Texture2D = load(Paths.PLACEHOLDER_TILE)
var templates_by_id: Dictionary = {}
var tile_library: TileLibrary = TileLibrary.new()

func load_templates() -> void:
	var templates_dir: DirAccess = DirAccess.open(Paths.WORLD_OBJECT_TEMPLATES)
	if templates_dir == null:
		push_error("Failed to open templates directory at path %s" % Paths.WORLD_OBJECT_TEMPLATES)
		return
	
	for path: String in templates_dir.get_files():
		var template: WorldObjectTemplate = load(Paths.WORLD_OBJECT_TEMPLATES + path)
		if not template is WorldObjectTemplate:
			push_error("Invalid world object template '%s'" % path)
			continue
		var id: int = template.get_id()
		if id in templates_by_id:
			push_error("Duplicate world object template id '%s'" % id)
			continue
		templates_by_id[id] = template
