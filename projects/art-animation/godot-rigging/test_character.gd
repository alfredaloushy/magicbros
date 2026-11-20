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
	var legs: Sprite3D = character.get_node("Legs")
	var body: Sprite3D = character.get_node("Body")
	var head: Sprite3D = character.get_node("Head")
	var player: AnimationPlayer = character.get_node("AnimationPlayer")
	var planted := legs.transform
	var head_low := head.position.y
	var head_high := head.position.y
	var last_head := head.position.y
	var max_head_step := 0.0
	for frame in 900:
		# Follow a moving shoulder instead of merely sharing a stationary origin.
		character.position = Vector3(0.012 * sin(frame * 0.025), 0.008 * sin(frame * 0.04), 0)
		await physics_frame
		error = maxf(error, character.attachment_error_pixels())
		passed = passed and legs.transform == planted
		# Chest expansion pivots at the belt instead of lifting the whole body.
		var belt: Vector3 = body.transform * Vector3(0, -0.56, 0)
		passed = passed and absf(belt.y - 0.08) < 0.00001
		head_low = minf(head_low, head.position.y)
		head_high = maxf(head_high, head.position.y)
		max_head_step = maxf(max_head_step, absf(head.position.y - last_head) / 0.02)
		last_head = head.position.y
	passed = passed and error < 0.05
	passed = passed and player.is_playing() and (head_high - head_low) / 0.02 > 0.9
	passed = passed and max_head_step < 0.03
	# Pausing the preview must stop both the body and the physical cloth.
	paused = true
	var held_body := body.transform
	var held_time: float = cape.elapsed
	for frame in 12:
		await physics_frame
	passed = passed and body.transform == held_body and cape.elapsed == held_time
	paused = false
	character.set_resting(false)
	await physics_frame
	passed = passed and body.scale.is_equal_approx(Vector3.ONE) and is_equal_approx(head.position.y, 0.64)
	character.set_resting(true)
	for frame in 60:
		await physics_frame
	passed = passed and body.scale.y > 1.005
	print(JSON.stringify({"shared_cape_instance":cape.scene_file_path,"attachment_error_px":error,
		"breathing_head_range_px":(head_high-head_low)/0.02,"max_head_step_px":max_head_step,
		"planted_feet_and_pause":passed,"passed":passed}))
	if not passed:
		push_error("Character/cape attachment check failed")
	quit(0 if passed else 1)
