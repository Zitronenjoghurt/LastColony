class_name WorldObject
extends Resource

enum ID {
	TREES,
	HUT
}

enum TYPE {
	ENVIRONMENT,
	BUILDING
}

@export var id: ID
@export var type: TYPE
@export var display_name: String
@export var static_tile: WorldObjectTile

func push_tile(tile: WorldObjectTile, tileset: TileSet) -> TileSet:
	if not tile is WorldObjectTile:
		return tileset
	var source_id: int = tileset.add_source(tile.create_source())
	tile.source_id = source_id
	return tileset
	
func push_tiles(tileset: TileSet) -> TileSet:
	tileset = push_tile(static_tile, tileset)
	return tileset
