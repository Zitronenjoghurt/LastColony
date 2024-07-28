class_name TitleScreen
extends Node2D

@onready var main_menu: MainMenu = %MainMenu

func _ready() -> void:
	main_menu.start_game.connect(_start_game)

func _start_game() -> void:
	GameManager.load_game()
	
