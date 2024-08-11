class_name Camera
extends Camera2D

var target_position: Vector2 = Vector2.ZERO
var min_map_zoom: float = 0.0

func _ready() -> void:
	# Depending on the map, limit the zoom-out maximum so you can't look out of map
	var min_map_zoom_x: float = Config.VIEWPORT_WIDTH / float(GameManager.state.get_map_width() * 16)
	var min_map_zoom_y: float = Config.VIEWPORT_HEIGHT / float(GameManager.state.get_map_height() * 16)
	min_map_zoom = max(min_map_zoom_x, min_map_zoom_y)

func _physics_process(delta: float) -> void:
	_process_grab()
	_limit_offset()
	_process_zoom(delta)

func _process_grab() -> void:
	if Input.is_action_just_pressed("camera_grab"):
		target_position = get_global_mouse_position()
	if Input.is_action_pressed("camera_grab"):
		var distance: Vector2 = target_position - get_global_mouse_position()
		offset += distance
		
func _limit_offset() -> void:
	var width: float = Config.VIEWPORT_WIDTH / zoom.x
	var height: float = Config.VIEWPORT_HEIGHT / zoom.y
	
	if offset.x < limit_left:
		offset.x = limit_left
	elif offset.x > limit_right - width:
		offset.x = max(limit_right - width, 0)
	
	if offset.y < limit_top:
		offset.y = limit_top
	elif offset.y > limit_bottom - height:
		offset.y = max(limit_bottom - height, 0)

func _process_zoom(delta: float) -> void:
	var zoom_delta: float = 0
	if Input.is_action_just_pressed("zoom_in"):
		zoom_delta = delta * GameManager.global.zoom_sensitivity
	elif Input.is_action_just_pressed("zoom_out"):
		zoom_delta = -delta * GameManager.global.zoom_sensitivity
	else:
		return
	
	var new_zoom: float = zoom.x + zoom_delta
	new_zoom = clamp(new_zoom, min_map_zoom, Config.CAMERA_MAX_ZOOM)
	zoom = Vector2(new_zoom, new_zoom)
