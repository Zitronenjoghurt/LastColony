class_name BackgroundGridMap
extends TileMap

var height: int = 0
var width: int = 0

func initialize(map_height: int, map_width: int) -> void:
	height = map_height
	width = map_width

func draw() -> void:
	for w: int in range(width):
		for h: int in range(height):
			draw_cell(w, h)

func reset() -> void:
	for w: int in range(width):
		for h: int in range(height):
			reset_cell(w, h)

func draw_from_bitmap(map: Array[bool]) -> void:
	if len(map) < height * width:
		push_error("An error occured while drawind BackgroundGridMap from bitmap: Map size (%s) is too small for given height (%s) and width (%s)" % [len(map), height, width])
		return
	
	for w: int in range(width):
		for h: int in range(height):
			var index: int = h * width + w
			if not map[index]:
				continue
			print(w, h)
			draw_cell(w, h)

func draw_cell(x: int, y: int) -> void:
	set_cell(0, Vector2i(x, y), 0, Vector2i(0, 0))

func reset_cell(x: int, y: int) -> void:
	erase_cell(0, Vector2i(x, y))
