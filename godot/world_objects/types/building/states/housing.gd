class_name HousingBuildingState
extends BuildingObjectState

@export var capacity: int
@export var habitant_ids: Array[int] = []

static func from_dict(data: Dictionary, _id: int) -> HousingBuildingState:
	var context: String = "HousingBuildingState"
	var _capacity: int = Deserialize.process_int(data, "cpty", context, true, 1)
	var _habitant_ids: Array[int] = Deserialize.process_array_int(data, "hb_ids", context)
	
	var failed: bool = Deserialize.check_for_null([_capacity, _habitant_ids])
	if failed:
		push_error("HousingBuildingState Deserialization: Unable to deserialize data")
		return null
	
	var state: HousingBuildingState = HousingBuildingState.new()
	state.id = _id
	state.capacity = _capacity
	state.habitant_ids = _habitant_ids
	return state
	
func to_dict() -> Dictionary:
	var data: Dictionary = {}
	data.id = id
	data.s_id = "housing"
	data.cpty = capacity
	data.hb_ids = habitant_ids
	return data
