class_name Pawn
extends Resource

enum PresetType {
	BABY,
	ADULT
}

@export var name: String = "no_name"
@export var current_location: Vector2i = Vector2i(-1, -1)
@export var home_location: Vector2i = Vector2i(-1, -1)
@export var work_location: Vector2i = Vector2i(-1, -1)
@export var delta_ticks: int = 0
@export var age_seconds: int = 0

static var RNG: RandomNumberGenerator = RandomNumberGenerator.new()

func tick() -> void:
	delta_ticks += 1
	if delta_ticks >= GameManager.global.ticks_per_second:
		delta_ticks = 0
		age_second()

func age_second() -> void:
	age_seconds += 1
	
func get_age_years() -> int:
	return int(age_seconds / Config.get_seconds_per_year())

static func create_from_preset(preset: PresetType) -> Pawn:
	match preset:
		PresetType.ADULT:
			return create_adult()
	return Pawn.new()

static func create_adult() -> Pawn:
	var random_year: int = RNG.randi_range(Config.RANDOM_ADULT_MIN_AGE_YEARS, Config.RANDOM_ADULT_MAX_AGE_YEARS)
	var pawn_age_seconds: int = random_year * Config.get_seconds_per_year()
	
	var pawn: Pawn = Pawn.new()
	pawn.age_seconds = pawn_age_seconds
	return pawn

func to_dict() -> Dictionary:
	var data: Dictionary = {}
	data.name = name
	data.clx = current_location.x
	data.cly = current_location.y
	data.hlx = home_location.x
	data.hly = home_location.y
	data.wlx = work_location.x
	data.wly = work_location.y
	data.d_ts = delta_ticks
	data.a_sec = age_seconds
	return data

static func from_dict(data: Dictionary) -> Pawn:
	var context: String = "Pawn"
	var _name: String = str(data.get("name", "no_name"))
	var _current_location_x: int = Deserialize.process_int(data, "clx", context, false, -1)
	var _current_location_y: int = Deserialize.process_int(data, "cly", context, false, -1)
	var _home_location_x: int = Deserialize.process_int(data, "hlx", context, false, -1)
	var _home_location_y: int = Deserialize.process_int(data, "hly", context, false, -1)
	var _work_location_x: int = Deserialize.process_int(data, "wlx", context, false, -1)
	var _work_location_y: int = Deserialize.process_int(data, "wly", context, false, -1)
	var _delta_ticks: int = Deserialize.process_int(data, "d_ts", context, false, 0)
	var _age_seconds: int = Deserialize.process_int(data, "a_sec", context, false, 0)
	
	var failed: bool = Deserialize.check_for_null([_current_location_x])
	if failed:
		push_error("Pawn Deserialization: Unable to create pawn from data")
		return null
	
	var pawn: Pawn = Pawn.new()
	pawn.name = _name
	pawn.current_location.x = _current_location_x
	pawn.current_location.y = _current_location_y
	pawn.home_location.x = _home_location_x
	pawn.home_location.y = _home_location_y
	pawn.work_location.x = _work_location_x
	pawn.work_location.y = _work_location_y
	pawn.delta_ticks = _delta_ticks
	pawn.age_seconds = _age_seconds
	return pawn
