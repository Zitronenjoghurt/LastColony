class_name BackgroundGridMap
extends TileMap

func draw(height: int, width: int) -> void:
	for w: int in range(width):
		for h: int in range(height):
			var cell: Vector2i = Vector2i(w, h)
			set_cell(0, cell, 0, Vector2i(0, 0))
