class_name BuildingTypeSelection
extends GridContainer

@export var housing_icon: AtlasTexture

@onready var housing: ClickableIcon = %Housing

func _ready() -> void:
	housing.display(housing_icon)

func _on_housing_pressed(button_index: int) -> void:
	pass # Replace with function body.
