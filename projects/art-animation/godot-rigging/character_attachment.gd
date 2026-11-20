extends Node3D
## Instance-local attachment: the independently editable cape asset is untouched.

var cape: Node3D
var cloth: SoftBody3D
var shoulder: Marker3D
var pin_offsets: Dictionary = {}

func _ready() -> void:
	cape = get_node_or_null("Cape")
	shoulder = get_node_or_null("ShoulderAttachment")
	if not cape or not shoulder:
		return  # The scene builder adds the children later.
	cloth = cape.get_node("CapeFabric")
	# Leave the physical fabric in world space. Only its pinned collar follows
	# the shoulder; moving the character must not teleport every cloth vertex.
	cloth.top_level = true
	# Pixel layers supply visual overlap. Broad placeholder body volumes crush
	# the flat cape and must not change the independent asset's cloth response.
	for index in cape.pin_indices:
		var point: Vector3 = cloth.global_transform * cape.rest[index]
		pin_offsets[index] = shoulder.to_local(point)
		cloth.set_point_pinned(index, false)
		cloth.set_point_pinned(index, true, cloth.get_path_to(shoulder))
	cape.get_node("PixelCamera").current = false
	get_node("PixelCamera").make_current()

func _physics_process(_delta: float) -> void:
	if not cloth:
		return
	# Synchronize attachments at physics frequency, including headless baking.
	# Free vertices remain entirely under the shared cape's soft-body solver.
	for index in pin_offsets:
		PhysicsServer3D.soft_body_move_point(cloth.get_physics_rid(), index, shoulder.to_global(pin_offsets[index]))

func attachment_error_pixels() -> float:
	var error := 0.0
	for index in pin_offsets:
		var point: Vector3 = cloth.global_transform * cloth.get_point_transform(index)
		var target: Vector3 = shoulder.to_global(pin_offsets[index])
		error = maxf(error, point.distance_to(target) / 0.02)
	return error
