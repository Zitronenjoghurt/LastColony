class_name WorldObjectTile
extends Resource

@export var texture: Texture2D

# Will be configured when generating the tileset
var source_id: int = -1

func create_source() -> TileSetAtlasSource:
	var source: TileSetAtlasSource = TileSetAtlasSource.new()
	source.texture = texture
	source.create_tile(Vector2i(0,0))
	return source
