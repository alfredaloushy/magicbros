@tool
extends Node2D

func _ready() -> void:
	if Engine.is_editor_hint():
		call_deferred("show_2d_workspace")

func show_2d_workspace() -> void:
	await get_tree().process_frame
	await get_tree().process_frame
	EditorInterface.set_main_screen_editor("2D")
