extends Node
## Gentle fixed-step damped springs; bones retain their rest lengths and scales.
## This is a controlled 2D wind rig, not a full fabric collision solver.

@export_range(0.0, 1.0, 0.05) var breeze: float = 0.55
@export var playing: bool = true
var elapsed: float = 0.0
var velocity := PackedFloat32Array([0,0,0,0,0,0])
var angles := PackedFloat32Array([0,0,0,0,0,0])
var cape: Array[Bone2D] = []
var tip: Bone2D

func _ready() -> void:
	var skeleton: Skeleton2D = get_parent().get_node("Skeleton2D")
	for i in skeleton.get_bone_count():
		var link := skeleton.get_bone(i)
		if link.name.begins_with("Cape"):
			cape.append(link)
		if link.name == "HatTip":
			tip = link

func reset_pose() -> void:
	elapsed = 0.0
	for i in cape.size():
		angles[i] = 0.0
		velocity[i] = 0.0
		cape[i].apply_rest()
	if tip:
		tip.apply_rest()

func step_wind(delta: float) -> void:
	elapsed += delta
	for i in cape.size():
		# The pinned collar is immobile. A delayed wave grows gently at the hem.
		if i == 0:
			continue
		var target := breeze * 0.055 * (float(i) / 5.0) * sin(elapsed * 2.2 - i * 0.75)
		var acceleration := 90.0 * (target - angles[i]) - 16.0 * velocity[i]
		velocity[i] += acceleration * delta
		angles[i] = clampf(angles[i] + velocity[i] * delta, -0.045, 0.045)
		cape[i].rotation = angles[i]
	if tip:
		tip.rotation = breeze * 0.008 * sin(elapsed * 3.1 - 1.2)

func _physics_process(delta: float) -> void:
	if playing:
		step_wind(delta)
