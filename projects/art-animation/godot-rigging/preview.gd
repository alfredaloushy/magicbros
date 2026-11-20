extends Control

const WindPreview = preload("res://wind_preview.gd")
var wind: Node

func _ready() -> void:
	wind = WindPreview.new()
	$PixelView/Canvas/RedMage.add_child(wind)
	$Controls/Wind.value_changed.connect(func(value: float): wind.breeze = value)
	$Controls/Play.toggled.connect(func(value: bool): wind.playing = value)
	$Controls/Reset.pressed.connect(func():
		wind.reset_pose()
		wind.playing = false
		$Controls/Play.button_pressed = false)
	if "--capture-rest" in OS.get_cmdline_user_args():
		wind.playing = false
		wind.reset_pose()
		await RenderingServer.frame_post_draw
		await RenderingServer.frame_post_draw
		var image: Image = $PixelView/Canvas.get_texture().get_image()
		var path := ProjectSettings.globalize_path("res://../.cache/godot-rig-rest.png")
		assert(image.save_png(path) == OK)
		print("Captured native rest pose: ", path)
		get_tree().quit()
	elif "--capture-wind" in OS.get_cmdline_user_args():
		wind.playing = false
		wind.reset_pose()
		for i in 180:
			wind.step_wind(1.0 / 120.0)
		await RenderingServer.frame_post_draw
		await RenderingServer.frame_post_draw
		var image: Image = get_viewport().get_texture().get_image()
		var path := ProjectSettings.globalize_path("res://../.cache/godot-wind-preview.png")
		assert(image.save_png(path) == OK)
		print("Captured wind preview: ", path)
		get_tree().quit()
