class_name MainMenu
extends PanelContainer

signal start_game()

func _on_start_button_pressed() -> void:
	start_game.emit()
