class_name StaticWorldObjectTile
extends WorldObjectTile

@export var texture: Texture2D
@export var y_offset: int = 0

func create_source() -> TileSetAtlasSource:
	var source: TileSetAtlasSource = TileSetAtlasSource.new()
	source.texture = texture
	source.create_tile(Vector2i(0, y_offset))
	return source

func get_atlas_coords() -> Vector2i:
	return Vector2i(0, y_offset)
