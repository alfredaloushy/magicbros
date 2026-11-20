extends Control

@onready var world: Node3D = $PixelView/Canvas/Physics/CapeWorld
var frame_count: int = 0
var capture: bool = false

func _ready() -> void:
	get_tree().paused = false
	$PixelView/Canvas/Output.texture = $PixelView/Canvas/Physics.get_texture()
	capture = "--capture-cape" in OS.get_cmdline_user_args()
	$Controls/Wind.value_changed.connect(func(value: float): world.breeze = value)
	$Controls/Stiffness.value_changed.connect(func(value: float): world.cloth.linear_stiffness = value)
	$Controls/Play.toggled.connect(func(value: bool): get_tree().paused = not value)
	$Controls/Reset.pressed.connect(func():
		get_tree().paused = false
		get_tree().reload_current_scene())

func _process(_delta: float) -> void:
	frame_count += 1
	if frame_count % 30 == 0:
		var measure: Dictionary = world.measurements()
		$Controls/Metrics.text = "Shoulder pinned\nStretch: %.1f%%" % (measure.max_edge_strain * 100.0)
	if capture and frame_count == 300:
		await RenderingServer.frame_post_draw
		var path := ProjectSettings.globalize_path("res://../.cache/godot-cape-only.png")
		get_viewport().get_texture().get_image().save_png(path)
		$PixelView/Canvas.get_texture().get_image().save_png(ProjectSettings.globalize_path("res://../.cache/godot-cape-native.png"))
		print("Cape measurements: ", JSON.stringify(world.measurements()))
		get_tree().quit()
