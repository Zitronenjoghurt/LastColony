class_name WorldObjectTemplate
extends Resource

@export var common_data: WorldObjectCommonData
@export var housing_data: WorldObjectHousingData
@export var tiles: Array[WorldObjectTile]

func apply_template(game_state: GameState, location: Vector2i) -> void:
	var housing: WorldObjectHousingData = WorldObjectHousingData.new_disabled()
	if housing_data is WorldObjectHousingData:
		housing = housing_data
		housing.apply()
	BehaviorFactory.push(game_state, location, common_data, housing)

func get_id() -> int:
	return common_data.id

func build_tiles(tileset: TileSet, tile_library: TileLibrary) -> void:
	for tile: WorldObjectTile in tiles:
		var source: TileSetAtlasSource = tile.create_source()
		var atlas_coords: Vector2i = tile.get_atlas_coords()
		var source_id: int = tileset.add_source(source)
		tile_library.add_source_id_and_atlas_coords(get_id(), tile.tile_type, source_id, atlas_coords)
