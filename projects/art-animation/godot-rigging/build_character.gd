extends SceneTree
## Compose the existing pixel character with a shared cape scene instance.

var scene_root: Node3D

func _initialize() -> void:
	call_deferred("build")

func pixel_position(point: Vector2, depth: float) -> Vector3:
	return Vector3((point.x - 48.0) * 0.02, (96.0 - point.y) * 0.02, depth)

func layer(label: String, texture_name: String, depth: float) -> void:
	var sprite := Sprite3D.new()
	sprite.name = label
	sprite.texture = load("res://textures/%s.png" % texture_name)
	sprite.pixel_size = 0.02
	sprite.position = pixel_position(Vector2(64,64), depth)
	sprite.texture_filter = BaseMaterial3D.TEXTURE_FILTER_NEAREST
	sprite.alpha_cut = SpriteBase3D.ALPHA_CUT_DISCARD
	sprite.shaded = false
	scene_root.add_child(sprite)
	sprite.owner = scene_root

func build() -> void:
	scene_root = Node3D.new()
	scene_root.name = "MageWithCape"
	scene_root.set_script(load("res://character_attachment.gd"))
	root.add_child(scene_root)
	layer("Legs", "legs", -0.035)
	layer("Body", "body", 0.045)
	layer("Head", "head", 0.06)
	var shoulder := Marker3D.new()
	shoulder.name = "ShoulderAttachment"
	shoulder.position = pixel_position(Vector2(62,79), 0)
	scene_root.add_child(shoulder)
	shoulder.owner = scene_root
	# Retain the source scene state so packing saves only actual overrides.
	# In particular, collar metadata must keep inheriting future asset edits.
	var cape := load("res://cape_world.tscn").instantiate(PackedScene.GEN_EDIT_STATE_INSTANCE) as Node3D
	cape.name = "Cape"
	scene_root.add_child(cape)
	cape.owner = scene_root
	var camera := Camera3D.new()
	camera.name = "PixelCamera"
	camera.projection = Camera3D.PROJECTION_ORTHOGONAL
	camera.size = 2.56
	camera.position = pixel_position(Vector2(64,64), 3)
	scene_root.add_child(camera)
	camera.owner = scene_root
	camera.current = true
	var packed := PackedScene.new()
	assert(packed.pack(scene_root) == OK)
	assert(ResourceSaver.save(packed, "res://mage_with_cape.tscn") == OK)
	print("Saved pixel character with shared cape instance and shoulder attachment")
	quit()
