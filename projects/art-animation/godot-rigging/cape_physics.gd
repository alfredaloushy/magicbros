extends Node3D
## Native Jolt soft-body dynamics on the whole cape, with a small pinned collar.

@export_range(0.0, 1.0, 0.05) var breeze: float = 0.55
var elapsed: float = 0.0
var cloth: SoftBody3D
var rest := PackedVector3Array()
var edges: Array[Vector2i] = []
var pin_indices := PackedInt32Array()
var max_strain: float = 0.0

func _ready() -> void:
	process_mode = Node.PROCESS_MODE_PAUSABLE
	cloth = get_node_or_null("CapeFabric")
	if not cloth:
		return  # The builder adds children after the root.
	rest = cloth.mesh.surface_get_arrays(0)[Mesh.ARRAY_VERTEX]
	pin_indices = get_meta("pin_indices")
	var indices: PackedInt32Array = cloth.mesh.surface_get_arrays(0)[Mesh.ARRAY_INDEX]
	var seen: Dictionary = {}
	for i in range(0, indices.size(), 3):
		for pair in [Vector2i(indices[i],indices[i+1]), Vector2i(indices[i+1],indices[i+2]), Vector2i(indices[i+2],indices[i])]:
			var edge := Vector2i(mini(pair.x,pair.y), maxi(pair.x,pair.y))
			if not seen.has(edge):
				edges.append(edge)
				seen[edge] = true

func _physics_process(delta: float) -> void:
	if not cloth:
		return
	elapsed += delta
	# Force is applied to the physical surface, not to a hand-picked bottom row.
	var gust := 0.65 + 0.35 * sin(elapsed * 1.4)
	cloth.apply_central_force(breeze * gust * Vector3(-0.08,0.012,0.012 * sin(elapsed * 2.4)))

func measurements() -> Dictionary:
	var upper := 0.0
	var middle := 0.0
	var lower := 0.0
	var pin_error := 0.0
	var positions := PackedVector3Array()
	for i in rest.size():
		var point := cloth.get_point_transform(i)
		positions.append(point)
		var motion := point.distance_to(rest[i])
		if i in pin_indices:
			pin_error = maxf(pin_error, motion)
		elif rest[i].y > 0.26:
			upper = maxf(upper, motion)
		elif rest[i].y > 0.05:
			middle = maxf(middle, motion)
		else:
			lower = maxf(lower, motion)
	var strain := 0.0
	for edge in edges:
		var original := rest[edge.x].distance_to(rest[edge.y])
		strain = maxf(strain, absf(positions[edge.x].distance_to(positions[edge.y]) / original - 1.0))
	return {"upper_motion_px":upper/0.02,"middle_motion_px":middle/0.02,
		"lower_motion_px":lower/0.02,"pin_error_px":pin_error/0.02,
		"max_edge_strain":strain}
