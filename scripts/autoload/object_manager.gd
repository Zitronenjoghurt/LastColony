extends Node

var placeholder_tile_texture: Texture2D = load(Paths.PLACEHOLDER_TILE)
var objects_by_id: Array[WorldObject] = []
var objects_count: int = len(WorldObject.ID.values())

func load_objects() -> void:
	var objects_dir: DirAccess = DirAccess.open(Paths.OBJECTS)
	if objects_dir == null:
		push_error("Failed to open objects directory at path %s" % Paths.OBJECTS)
		return
		
	objects_by_id.resize(objects_count)
	
	for path: String in objects_dir.get_files():
		var object: WorldObject = load(Paths.OBJECTS + path)
		if not object is WorldObject:
			push_error("Invalid world object '%s'" % path)
			continue
		if not objects_by_id[object.id] == null:
			push_error("Duplicate world object '%s' with id '%s'" % [path, object.id])
			continue
		objects_by_id[object.id] = object
	
	if OS.is_debug_build():
		for i: int in range(objects_count):
			if objects_by_id[i] == null:
				var id_name: String = WorldObject.get_id_name(i)
				push_warning("Unused world object id '%s'" % id_name)

func generate_tileset() -> TileSet:
	var tileset: TileSet = TileSet.new()
	
	# Add placeholder tile at index 0 to tileset
	var placeholder_source: TileSetAtlasSource = TileSetAtlasSource.new()
	placeholder_source.texture = placeholder_tile_texture
	placeholder_source.create_tile(Vector2i(0, 0))
	tileset.add_source(placeholder_source)
	
	for object: WorldObject in objects_by_id:
		if not object is WorldObject:
			continue
		tileset = object.push_tiles(tileset)
	return tileset

func get_object(id: WorldObject.ID) -> WorldObject:
	if id >= objects_count:
		return null
	return objects_by_id[id]

func new_object_state(id: WorldObject.ID) -> WorldObjectState:
	var object: WorldObject = get_object(id)
	if not object is WorldObject:
		return null
	return object.new_state()
