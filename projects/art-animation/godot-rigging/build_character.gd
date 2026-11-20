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

func breath(time: float, peak: float, start: float = 0.0) -> float:
	# A quiet inhale followed by a longer exhale, with no jump at the loop seam.
	if time <= start:
		return 0.0
	if time < peak:
		return 0.5 - 0.5 * cos(PI * (time - start) / (peak - start))
	return 0.5 + 0.5 * cos(PI * (time - peak) / (3.6 - peak))

func add_resting_animation() -> void:
	var player := AnimationPlayer.new()
	player.name = "AnimationPlayer"
	# Apply the pose before the collar pins are synchronized each physics tick.
	player.callback_mode_process = AnimationMixer.ANIMATION_CALLBACK_MODE_PROCESS_PHYSICS
	player.process_physics_priority = -10
	var idle := Animation.new()
	idle.length = 3.6
	idle.loop_mode = Animation.LOOP_LINEAR
	var paths := ["Body:scale", "Body:position", "Head:position", "ShoulderAttachment:position"]
	var reset := Animation.new()
	reset.length = 0.0
	for path in paths:
		var track := idle.add_track(Animation.TYPE_VALUE)
		idle.track_set_path(track, NodePath(path))
		reset.add_track(Animation.TYPE_VALUE)
		reset.track_set_path(track, NodePath(path))
		for sample in 73:
			var time := sample * 0.05
			var chest := breath(time, 1.4)
			var head := breath(time, 1.65, 0.15)
			var values := [
				Vector3(1, 1 + 0.04 * chest, 1),
				# Scale about the belt (source y=92), keeping the waist planted.
				pixel_position(Vector2(64, 64 - 28 * 0.04 * chest), 0.045),
				pixel_position(Vector2(64, 64 - head), 0.06),
				pixel_position(Vector2(62, 79 - 13 * 0.04 * chest), 0)]
			idle.track_insert_key(track, time, values[track])
			if sample == 0:
				reset.track_insert_key(track, 0.0, values[track])
	var library := AnimationLibrary.new()
	library.add_animation("RESET", reset)
	library.add_animation("resting", idle)
	player.add_animation_library("", library)
	player.autoplay = "resting"
	scene_root.add_child(player)
	player.owner = scene_root

func build() -> void:
	scene_root = Node3D.new()
	scene_root.name = "MageWithCape"
	scene_root.process_mode = Node.PROCESS_MODE_PAUSABLE
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
	add_resting_animation()
	var packed := PackedScene.new()
	assert(packed.pack(scene_root) == OK)
	assert(ResourceSaver.save(packed, "res://mage_with_cape.tscn") == OK)
	print("Saved pixel character with editable resting animation and shared cape")
	quit()
