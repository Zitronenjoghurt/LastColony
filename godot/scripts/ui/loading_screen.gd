extends CanvasLayer

@onready var animation_player: AnimationPlayer = %AnimationPlayer

func _ready() -> void:
	GameManager.load_game_started.connect(_fade_out)
	GameManager.load_game_finished.connect(_fade_in)

func _fade_out() -> void:
	visible = true
	animation_player.play("fade_out")

func _fade_in() -> void:
	animation_player.play("fade_in")
	visible = false
