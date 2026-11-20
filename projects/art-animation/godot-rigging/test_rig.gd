extends SceneTree

func _initialize() -> void:
	call_deferred("check")

func check() -> void:
	var rig := load("res://red_mage.tscn").instantiate() as Node2D
	root.add_child(rig)
	var skeleton := rig.get_node("Skeleton2D") as Skeleton2D
	assert(skeleton.get_bone_count() == 10)
	for part in ["Cape", "Legs", "Body", "Head"]:
		var mesh := rig.get_node(part) as Polygon2D
		assert(mesh.texture_filter == CanvasItem.TEXTURE_FILTER_NEAREST)
		assert(mesh.polygon.size() == mesh.uv.size())
		for i in mesh.polygon.size():
			var total := 0.0
			for j in mesh.get_bone_count():
				total += mesh.get_bone_weights(j)[i]
			assert(absf(total - 1.0) < 0.00001, "Unbound or overweight mesh vertex")
	for i in skeleton.get_bone_count():
		var link := skeleton.get_bone(i)
		assert(link.transform == link.rest)
		assert(link.scale == Vector2.ONE)
	print("PASS: editable character mesh bindings and rest poses")
	quit()
