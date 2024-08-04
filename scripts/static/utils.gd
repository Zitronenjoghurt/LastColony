class_name Utils
extends Object

static func populate_new_array_int(size: int, value: int) -> Array[int]:
	var array: Array[int] = []
	array.resize(size)
	array.fill(value)
	return array
