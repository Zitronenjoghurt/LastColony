class_name TitleScreen
extends Node2D

var _test_preset: WorldPreset = load(Paths.DEFAULT_WORLD_PRESET)

@onready var main_menu: MainMenu = %MainMenu

func _ready() -> void:
	main_menu.start_game.connect(_start_game)

func _start_game() -> void:
	#GameManager.start_existing_game()
	GameManager.start_new_game(_test_preset)
	GameManager.save_game()
