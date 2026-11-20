extends SceneTree

func _initialize() -> void:
	call_deferred("build")

func build() -> void:
	var character := load("res://red_mage.tscn").instantiate() as Node2D
	var original := character.get_node("Cape") as Polygon2D
	var vertices := PackedVector3Array()
	var normals := PackedVector3Array()
	var uvs := PackedVector2Array()
	var indices := PackedInt32Array()
	for point in original.polygon:
		vertices.append(Vector3((point.x - 48.0) * 0.02, (96.0 - point.y) * 0.02, 0))
		normals.append(Vector3(0,0,1))
		uvs.append(point / 128.0)
	for cell in original.polygons:
		indices.append_array(PackedInt32Array([cell[0],cell[1],cell[2],cell[0],cell[2],cell[3]]))
	var arrays: Array = []
	arrays.resize(Mesh.ARRAY_MAX)
	arrays[Mesh.ARRAY_VERTEX] = vertices
	arrays[Mesh.ARRAY_NORMAL] = normals
	arrays[Mesh.ARRAY_TEX_UV] = uvs
	arrays[Mesh.ARRAY_INDEX] = indices
	var mesh := ArrayMesh.new()
	mesh.add_surface_from_arrays(Mesh.PRIMITIVE_TRIANGLES, arrays)
	var material := StandardMaterial3D.new()
	material.shading_mode = BaseMaterial3D.SHADING_MODE_UNSHADED
	material.transparency = BaseMaterial3D.TRANSPARENCY_ALPHA_SCISSOR
	material.alpha_scissor_threshold = 0.5
	material.cull_mode = BaseMaterial3D.CULL_DISABLED
	material.texture_filter = BaseMaterial3D.TEXTURE_FILTER_NEAREST
	material.albedo_texture = load("res://textures/cape.png")
	mesh.surface_set_material(0, material)
	var world := Node3D.new()
	world.name = "CapeWorld"
	world.set_script(load("res://cape_physics.gd"))
	root.add_child(world)
	var cloth := SoftBody3D.new()
	cloth.name = "CapeFabric"
	cloth.mesh = mesh
	cloth.total_mass = 0.06
	cloth.linear_stiffness = 0.98
	cloth.simulation_precision = 12
	cloth.damping_coefficient = 0.08
	cloth.drag_coefficient = 0.15
	cloth.pressure_coefficient = 0
	cloth.collision_layer = 0
	cloth.collision_mask = 0
	world.add_child(cloth)
	cloth.owner = world
	var pins := PackedInt32Array()
	for i in original.polygon.size():
		if original.polygon[i].distance_to(Vector2(62,79)) < 4.5:
			cloth.set_point_pinned(i, true)
			pins.append(i)
	world.set_meta("pin_indices", pins)
	var camera := Camera3D.new()
	camera.name = "PixelCamera"
	camera.projection = Camera3D.PROJECTION_ORTHOGONAL
	camera.size = 1.44
	camera.position = Vector3(0,0,3)
	world.add_child(camera)
	camera.owner = world
	camera.current = true
	var packed := PackedScene.new()
	assert(packed.pack(world) == OK)
	assert(ResourceSaver.save(packed, "res://cape_world.tscn") == OK)
	print("Saved native SoftBody3D cape: %d vertices, %d collar pins" % [vertices.size(), pins.size()])
	character.free()
	quit()
