extends Control

@onready var character: Node3D = $PixelView/Canvas/Physics/MageWithCape
var frame_count: int = 0
var capture: bool = false

func _ready() -> void:
	get_tree().paused = false
	$PixelView/Canvas/Output.texture = $PixelView/Canvas/Physics.get_texture()
	capture = "--capture-character" in OS.get_cmdline_user_args()
	$Controls/Wind.value_changed.connect(func(value: float): character.cape.breeze = value)
	$Controls/Stiffness.value_changed.connect(func(value: float): character.cloth.linear_stiffness = value)
	$Controls/Play.toggled.connect(func(value: bool): get_tree().paused = not value)
	$Controls/Reset.pressed.connect(func():
		get_tree().paused = false
		get_tree().reload_current_scene())
	$Controls/CapeOnly.pressed.connect(func():
		get_tree().paused = false
		get_tree().change_scene_to_file("res://cape_only.tscn"))

func _process(_delta: float) -> void:
	frame_count += 1
	if frame_count % 30 == 0:
		$Controls/Metrics.text = "Cape attached at shoulder\nStretch: %.1f%%" % (character.cape.measurements().max_edge_strain * 100.0)
	if capture and frame_count == 300:
		await RenderingServer.frame_post_draw
		get_viewport().get_texture().get_image().save_png(ProjectSettings.globalize_path("res://../.cache/godot-character-cape.png"))
		$PixelView/Canvas.get_texture().get_image().save_png(ProjectSettings.globalize_path("res://../.cache/godot-character-native.png"))
		print("Character attachment error px: ", character.attachment_error_pixels())
		print("Cape response: ", JSON.stringify(character.cape.measurements()))
		get_tree().quit()
