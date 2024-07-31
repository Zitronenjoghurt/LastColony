class_name ClickableIcon
extends Panel

enum BackgroundColor {
	NONE,
	WHITE,
	WHITE_YELLOW,
	BLUE,
	DARK_BLUE,
	GREEN,
	DARK_GREEN,
	YELLOW,
	ORANGE,
	RED,
	BLACK
}

@export var background_tiles: Texture2D
@export var color: BackgroundColor = BackgroundColor.NONE
@onready var icon_rect: TextureRect = %Icon

signal pressed(button_index: int)
signal released(button_index: int)

func display(icon_texture: Texture2D) -> void:
	icon_rect.texture = icon_texture
	
	if color == BackgroundColor.NONE:
		remove_theme_stylebox_override("panel")
	else:
		var atlas_texture: AtlasTexture = AtlasTexture.new()
		atlas_texture.atlas = background_tiles
		var atlas_rect: Rect2i = Rect2i(color - 1, 0, 1, 1)
		atlas_texture.region = atlas_rect
		var stylebox: StyleBoxTexture = StyleBoxTexture.new()
		stylebox.texture = atlas_texture
		add_theme_stylebox_override("panel", stylebox) 

func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			pressed.emit(event.button_index)
		else:
			released.emit(event.button_index)
