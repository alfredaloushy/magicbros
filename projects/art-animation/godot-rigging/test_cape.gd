extends SceneTree

func _initialize() -> void:
	call_deferred("check")

func check() -> void:
	var world := load("res://cape_world.tscn").instantiate() as Node3D
	root.add_child(world)
	await physics_frame
	var peak_upper := 0.0
	var peak_middle := 0.0
	var peak_lower := 0.0
	var worst_strain := 0.0
	var pin_error := 0.0
	for frame in 600:
		await physics_frame
		if frame % 20 == 0:
			var result: Dictionary = world.measurements()
			peak_upper = maxf(peak_upper, result.upper_motion_px)
			peak_middle = maxf(peak_middle, result.middle_motion_px)
			peak_lower = maxf(peak_lower, result.lower_motion_px)
			worst_strain = maxf(worst_strain, result.max_edge_strain)
			pin_error = maxf(pin_error, result.pin_error_px)
	var whole_surface_moves := peak_upper > 0.1 and peak_middle > 0.1 and peak_lower > 0.1
	var passed := whole_surface_moves and worst_strain < 0.15 and pin_error < 0.001
	print(JSON.stringify({"whole_surface_moves":whole_surface_moves,"upper_px":peak_upper,
		"middle_px":peak_middle,"lower_px":peak_lower,"max_strain":worst_strain,
		"pin_error_px":pin_error}))
	if not passed:
		push_error("Cloth motion, stretch or attachment check failed")
	quit(0 if passed else 1)
