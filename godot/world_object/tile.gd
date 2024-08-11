class_name WorldObjectTile
extends WorldObjectTileBase

func create_source() -> TileSetAtlasSource:
	return TileSetAtlasSource.new()

func get_atlas_coords() -> Vector2i:
	return Vector2i.ZERO
