class_name Config
extends Object

const VERSION: int = 1
const CAMERA_MAX_ZOOM: float = 6.0
const VIEWPORT_WIDTH: int = 640
const VIEWPORT_HEIGHT: int = 360

const SECONDS_PER_MONTH: int = 120
const MONTHS_PER_YEAR: int = 4
const RANDOM_ADULT_MIN_AGE_YEARS: int = 18
const RANDOM_ADULT_MAX_AGE_YEARS: int = 30

static func get_seconds_per_year() -> int:
	return SECONDS_PER_MONTH * MONTHS_PER_YEAR
