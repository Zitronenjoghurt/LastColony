class_name ClickableIcon
extends PanelContainer

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
@export var background_texture_margin: int = 0
@export var background_content_margin: int = 2
@export var color: BackgroundColor = BackgroundColor.NONE
@export var hoverable: bool = true

@onready var hover_overlay: Panel = %HoverOverlay
@onready var icon_rect: TextureRect = %Icon

signal pressed(button_index: int)
signal released(button_index: int)

func _ready() -> void:
	display_background()

func display(icon_texture: Texture2D) -> void:
	icon_rect.texture = icon_texture
	display_background()

func display_background() -> void:
	if color == BackgroundColor.NONE:
		remove_theme_stylebox_override("panel")
	else:
		var atlas_texture: AtlasTexture = AtlasTexture.new()
		atlas_texture.atlas = background_tiles
		var atlas_rect: Rect2i = Rect2i((color - 1) * 16, 0, 16, 16)
		atlas_texture.region = atlas_rect
		var stylebox: StyleBoxTexture = StyleBoxTexture.new()
		stylebox.texture = atlas_texture
		stylebox.content_margin_bottom = background_content_margin
		stylebox.content_margin_left = background_content_margin
		stylebox.content_margin_right = background_content_margin
		stylebox.content_margin_top = background_content_margin
		stylebox.texture_margin_bottom = background_texture_margin
		stylebox.texture_margin_left = background_texture_margin
		stylebox.texture_margin_right = background_texture_margin
		stylebox.texture_margin_top = background_texture_margin
		add_theme_stylebox_override("panel", stylebox)

func _on_gui_input(event: InputEvent) -> void:
	if event is InputEventMouseButton:
		if event.pressed:
			pressed.emit(event.button_index)
		else:
			released.emit(event.button_index)

func _on_mouse_entered() -> void:
	hover_overlay.show()

func _on_mouse_exited() -> void:
	hover_overlay.hide()
