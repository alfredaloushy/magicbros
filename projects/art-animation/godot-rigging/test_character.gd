extends SceneTree

func _initialize() -> void:
	call_deferred("check")

func check() -> void:
	var character := load("res://mage_with_cape.tscn").instantiate() as Node3D
	root.add_child(character)
	await physics_frame
	var cloth: SoftBody3D = character.cloth
	var cape: Node3D = character.cape
	var passed := cape.scene_file_path == "res://cape_world.tscn" and cloth.collision_mask == 0
	var error := 0.0
	for frame in 300:
		# Follow a moving shoulder instead of merely sharing a stationary origin.
		character.position = Vector3(0.012 * sin(frame * 0.025), 0.008 * sin(frame * 0.04), 0)
		await physics_frame
		error = maxf(error, character.attachment_error_pixels())
		passed = passed and character.get_node("Legs").scale == Vector3.ONE
	passed = passed and error < 0.05
	print(JSON.stringify({"shared_cape_instance":cape.scene_file_path,"attachment_error_px":error,"passed":passed}))
	if not passed:
		push_error("Character/cape attachment check failed")
	quit(0 if passed else 1)
