class_name WorldObjectTile
extends Resource

@export var texture: Texture2D
@export var y_offset: int = 0

# Will be configured when generating the tileset
var source_id: int = 0

# The world object state this tile is referring to if it was created as a placeholder
# Important for debugging information
var placeholder_object_state: WorldObjectState

static func create_placeholder(object_state: WorldObjectState) -> WorldObjectTile:
	var tile: WorldObjectTile = WorldObjectTile.new()
	tile.placeholder_object_state = object_state
	return tile

func create_source() -> TileSetAtlasSource:
	var source: TileSetAtlasSource = TileSetAtlasSource.new()
	source.texture = texture
	source.create_tile(Vector2i(0, y_offset))
	return source

func draw_at_coords(tile_map: TileMap, coords: Vector2i) -> void:
	tile_map.set_cell(0, coords, source_id, Vector2i(0, y_offset))
	
	if placeholder_object_state is WorldObjectState:
		push_error("Missing tile texture at cell '%s' for world object id '%s'" % [coords, WorldObject.get_id_name(placeholder_object_state.id)])
