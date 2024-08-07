class_name DataMap
extends Resource

@export var height: int = 1
@export var width: int = 1
@export var data: Array = []

var _size: int = 1
var _default: Variant = 0

func initialize(map_height: int, map_width: int, default: Variant = 0) -> void:
	height = map_height
	width = map_width
	_size = height * width
	_default = default
	reset()

func set_at_index(value: Variant, index: int) -> void:
	if index >= _size:
		push_error("An error occured while setting DataMap value: Index '%s' for value '%s' is out of range for size '%s'" % [index, value, _size])
		return
	data[index] = value

func reset() -> void:
	data = []
	data.resize(_size)
	data.fill(_default)

func get_data() -> Array:
	return data

func get_value(index: int) -> Variant:
	if index >= _size:
		push_error("An error occured while retrieving DataMap value: Index '%s' is out of range for size '%s'" % [index, _size])
		return -1
	return data[index]
