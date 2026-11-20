extends SceneTree
## Builds a normal, editable Skeleton2D/Polygon2D scene from Aseprite layers.

var scene_root: Node2D
var skeleton: Skeleton2D
var bones: Dictionary = {}

func _initialize() -> void:
	call_deferred("build")

func bone(label: String, at: Vector2, parent: Node2D) -> Bone2D:
	var result := Bone2D.new()
	result.name = label
	result.set_autocalculate_length_and_angle(false)
	result.set_length(5.0)
	parent.add_child(result)
	result.owner = scene_root
	result.position = at
	result.rest = result.transform
	bones[label] = result
	return result

func weights_for(part: String, point: Vector2) -> Dictionary:
	if part == "cape":
		# Smoothly distribute the visible fabric across six short links.
		var distance := clampf((point.y - 81.0) / 24.0, 0.0, 1.0) * 5.0
		var first := mini(int(distance), 4)
		var blend := distance - first
		return {"Cape%d" % first: 1.0 - blend, "Cape%d" % (first + 1): blend}
	if part == "head":
		var amount := clampf((54.0 - point.x) / 30.0, 0.0, 1.0) * clampf((50.0 - point.y) / 10.0, 0.0, 1.0)
		return {"Head": 1.0 - amount, "HatTip": amount}
	return {"Legs": 1.0} if part == "legs" else {"Body": 1.0}

func make_mesh(part: String, depth: int) -> void:
	var texture := load("res://textures/%s.png" % part) as Texture2D
	var image := texture.get_image()
	assert(image != null and image.get_size() == Vector2i(128, 128))
	var points := PackedVector2Array()
	var cells: Array[PackedInt32Array] = []
	var lookup: Dictionary = {}
	for y in range(128):
		for x in range(128):
			if image.get_pixel(x, y).a < 0.5:
				continue
			var cell := PackedInt32Array()
			for corner in [Vector2i(x,y), Vector2i(x+1,y), Vector2i(x+1,y+1), Vector2i(x,y+1)]:
				if not lookup.has(corner):
					lookup[corner] = points.size()
					points.append(Vector2(corner))
				cell.append(lookup[corner])
			cells.append(cell)
	var mesh := Polygon2D.new()
	mesh.name = part.capitalize()
	mesh.texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
	mesh.texture = texture
	mesh.polygon = points
	mesh.uv = points
	mesh.polygons = cells
	mesh.antialiased = false
	mesh.z_index = depth
	scene_root.add_child(mesh)
	mesh.owner = scene_root
	mesh.skeleton = mesh.get_path_to(skeleton)
	var binding: Dictionary = {}
	for label in bones:
		var values := PackedFloat32Array()
		values.resize(points.size())
		binding[label] = values
	for i in points.size():
		for label in weights_for(part, points[i]):
			binding[label][i] = weights_for(part, points[i])[label]
	for label in binding:
		mesh.add_bone(skeleton.get_path_to(bones[label]), binding[label])

func build() -> void:
	scene_root = Node2D.new()
	scene_root.name = "RedMage"
	scene_root.set_script(load("res://rig_authoring.gd"))
	scene_root.texture_filter = CanvasItem.TEXTURE_FILTER_NEAREST
	root.add_child(scene_root)
	skeleton = Skeleton2D.new()
	skeleton.name = "Skeleton2D"
	scene_root.add_child(skeleton)
	skeleton.owner = scene_root
	bone("Body", Vector2(71, 96), skeleton)
	bone("Legs", Vector2(70, 119), skeleton)
	var head := bone("Head", Vector2(74, 66), skeleton)
	bone("HatTip", Vector2(-26, -30), head)
	var previous: Node2D = skeleton
	for i in range(6):
		previous = bone("Cape%d" % i, Vector2(62,81) if i == 0 else Vector2(-2.5, 4.8), previous)
	make_mesh("cape", 0)
	make_mesh("legs", 1)
	make_mesh("body", 2)
	make_mesh("head", 3)
	var animation_player := AnimationPlayer.new()
	animation_player.name = "AnimationPlayer"
	scene_root.add_child(animation_player)
	animation_player.owner = scene_root
	var rest := Animation.new()
	rest.length = 0.1
	for label in bones:
		var track := rest.add_track(Animation.TYPE_VALUE)
		rest.track_set_path(track, NodePath("%s:rotation" % scene_root.get_path_to(bones[label])))
		rest.track_insert_key(track, 0.0, 0.0)
	var library := AnimationLibrary.new()
	library.add_animation("RESET", rest)
	animation_player.add_animation_library("", library)
	var packed := PackedScene.new()
	assert(packed.pack(scene_root) == OK)
	assert(ResourceSaver.save(packed, "res://red_mage.tscn") == OK)
	print("Saved editable pixel rig: %d bones, four textured meshes" % bones.size())
	quit()
