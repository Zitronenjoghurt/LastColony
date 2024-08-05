class_name Deserialize
extends Object

static func process(data: Dictionary, property: String, context: String, required: bool, default: Variant) -> Variant:
	var has_value: bool = data.has(property)
	if not has_value:
		if required:
			error_required(context, property)
			return null
		else:
			return default
	return data.get(property, default)

static func process_int(data: Dictionary, property: String, context: String, required: bool = false, default: int = 0, min_value: int = Constants.INT32_MIN, max_value: int = Constants.INT32_MIN) -> Variant:
	var value: int = process(data, property, context, required, default)
	if value == default:
		return value
	if not value is int:
		error_typing(context, property, "int")
		return null
	if min_value != Constants.INT32_MIN and value < min_value:
		error_min(context, property, str(value), str(min_value))
		return null
	if max_value != Constants.INT32_MIN and value > max_value:
		error_max(context, property, str(value), str(max_value))
		return null
	return value

static func error_required(context: String, property: String) -> void:
	push_error("Deserialization error (context: %s): Missing required property '%s'" % [context, property])

static func error_typing(context: String, property: String, type: String) -> void:
	push_error("Deserialization error (context: %s): Property '%s' has to be of type '%s'" % [context, property, type])

static func error_min(context: String, property: String, value: String, min_value: String) -> void:
	push_error("Deserialization error (context: %s): Property '%s' of value '%s' is lower than required minimum '%s'" % [context, property, value, min_value])

static func error_max(context: String, property: String, value: String, max_value: String) -> void:
	push_error("Deserialization error (context: %s): Property '%s' of value '%s' is greater than required maximum '%s'" % [context, property, value, max_value])
