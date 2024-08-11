class_name Paths
extends Object

const DEFAULT_WORLD_PRESET: String = "res://world/presets/default.tres"
const GLOBAL_STATE: String = "user://global_state.tres"
const OBJECTS: String = "res://world_objects/objects/"
const PLACEHOLDER_TILE: String = "res://assets/ui/placeholder_tile.png"
const WORLD_OBJECT_TEMPLATES: String = "res://resources/world_object_templates/"
const WORLD_SCENE: String = "res://world/world.tscn"

const RESOURCE_PATHS: Array[String] = [DEFAULT_WORLD_PRESET, PLACEHOLDER_TILE, WORLD_SCENE]

static func check() -> void:
	check_resource_paths()

static func check_resource_paths() -> void:
	for path: String in RESOURCE_PATHS:
		if not ResourceLoader.exists(path):
			push_error("Resource path '%s' might have changed, revalidate the path constants" % path)
