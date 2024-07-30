class_name Camera
extends Camera2D

var target_position: Vector2 = Vector2.ZERO

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
	var viewport_size: Vector2 = get_viewport().size
	var width: float = viewport_size.x / zoom.x
	var height: float = viewport_size.y / zoom.y
	
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
	
	zoom += Vector2.ONE * zoom_delta
	if zoom.x > Config.CAMERA_MAX_ZOOM:
		zoom = Vector2.ONE * Config.CAMERA_MAX_ZOOM
	elif zoom.x < Config.CAMERA_MIN_ZOOM:
		zoom = Vector2.ONE * Config.CAMERA_MIN_ZOOM
