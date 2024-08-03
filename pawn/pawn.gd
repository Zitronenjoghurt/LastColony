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
