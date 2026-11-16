extends CharacterBody2D
const T = preload("res://game/tuning.gd")
var peer_id := 1
var tint := Color("f17870")
var health := T.MAX_HEALTH
var dead_time := 0.0
var protection := 0.0
var hit_flash := 0.0
var health_reveal := 0.0
var spell := 0
var cast_cooldown := 0.0
var jumps := 0
var dash_left := 0.0
var dash_cooldown := 0.0
var dash_direction := Vector2.RIGHT
var aim := Vector2.RIGHT
var facing := 1.0
var anim := 0.0
var target_position := Vector2.ZERO
var received_state := false
var respawn_generation := 0
var last_jump := 0
var last_dash := 0
var last_switch := 0
var last_cast := 0
var accepted_sequence := -1

func _ready() -> void:
 collision_layer = 2
 collision_mask = 1
 var shape := CollisionShape2D.new()
 var capsule := CapsuleShape2D.new()
 capsule.radius = 15.0
 capsule.height = 54.0
 shape.shape = capsule
 shape.position.y = -26
 add_child(shape)

func alive() -> bool:
 return health > 0.0

func simulate(delta: float, command: Dictionary) -> void:
 anim += delta
 hit_flash = maxf(0.0, hit_flash - delta)
 health_reveal = maxf(0.0, health_reveal - delta)
 protection = maxf(0.0, protection - delta)
 cast_cooldown = maxf(0.0, cast_cooldown - delta)
 dash_cooldown = maxf(0.0, dash_cooldown - delta)
 if not alive():
  dead_time -= delta
  queue_redraw()
  return
 var move: float = float(command.get("move", 0.0))
 var desired: Vector2 = command.get("aim", aim)
 if desired.length_squared() > 0.01:
  aim = desired.normalized()
 if absf(aim.x) > 0.1:
  facing = signf(aim.x)
 if is_on_floor():
  jumps = 0
 if int(command.get("jump", 0)) > last_jump:
  last_jump = int(command.jump)
  if jumps < 2:
   velocity.y = -T.JUMP_SPEED
   jumps += 1
 if int(command.get("dash", 0)) > last_dash:
  last_dash = int(command.dash)
  if dash_cooldown <= 0.0:
   dash_direction = Vector2(move, float(command.get("vertical", 0.0)))
   if dash_direction.length() < 0.2:
    dash_direction = aim
   dash_direction = dash_direction.normalized()
   dash_left = T.DASH_TIME
   dash_cooldown = T.DASH_COOLDOWN
 if dash_left > 0.0:
  dash_left -= delta
  velocity = dash_direction * T.DASH_SPEED
 else:
  velocity.x = move_toward(velocity.x, move * T.RUN_SPEED, T.ACCEL * delta)
  velocity.y = minf(velocity.y + T.GRAVITY * delta, 1100.0)
 move_and_slide()
 queue_redraw()

func visual_tick(delta: float) -> void:
 anim += delta
 health_reveal = maxf(0.0, health_reveal - delta)
 hit_flash = maxf(0.0, hit_flash - delta)
 if received_state:
  position = position.lerp(target_position, 1.0 - exp(-18.0 * delta))
 queue_redraw()

func respawn(at: Vector2) -> void:
 position = at
 target_position = at
 velocity = Vector2.ZERO
 health = T.MAX_HEALTH
 dead_time = 0.0
 protection = T.SPAWN_PROTECTION
 cast_cooldown = 0.0
 dash_left = 0.0
 dash_cooldown = 0.0
 jumps = 0
 respawn_generation += 1
 queue_redraw()

func state() -> Dictionary:
 return {"id":peer_id, "p":position, "v":velocity, "hp":health, "dead":dead_time,
 "shield":protection, "spell":spell, "aim":aim, "dash_cd":dash_cooldown,
 "gen":respawn_generation, "hit":health_reveal, "ack":accepted_sequence}

func apply_state(data: Dictionary, local: bool) -> void:
 var changed_generation := int(data.gen) != respawn_generation
 if float(data.hp) < health:
  hit_flash = 0.16
 health = float(data.hp)
 dead_time = float(data.dead)
 protection = float(data.shield)
 spell = int(data.spell)
 health_reveal = maxf(health_reveal, float(data.hit))
 respawn_generation = int(data.gen)
 target_position = data.p
 if not local:
  aim = data.aim
  facing = signf(aim.x) if absf(aim.x) > 0.1 else facing
  velocity = data.v
 if not received_state or changed_generation or not alive():
  position = target_position
  velocity = data.v
 elif local:
  # Small corrections preserve touch responsiveness; large drift snaps safely.
  var error := position.distance_to(target_position)
  if error > 150.0:
   position = target_position
   velocity = data.v
  elif error > 10.0:
   position = position.lerp(target_position, 0.22)
 received_state = true
 queue_redraw()

func _draw() -> void:
 if not alive():
  draw_ellipse_shadow()
  draw_rect(Rect2(-23, -13, 46, 11), tint.darkened(0.4))
  return
 var bob := sin(anim * 8.0) * (1.4 if is_on_floor() else 0.0)
 if dash_left > 0.0:
  draw_line(-dash_direction * 60 + Vector2(0,-26), Vector2(0,-26), Color(tint,0.3), 17)
 draw_ellipse_shadow()
 var cloth := tint.lightened(0.5) if hit_flash > 0.0 else tint
 draw_polygon(PackedVector2Array([Vector2(-20,-8+bob),Vector2(-12,-35+bob),Vector2(10,-35+bob),Vector2(21,-8+bob)]),cloth.darkened(0.35))
 draw_rect(Rect2(-13,-31+bob,26,21),cloth)
 draw_rect(Rect2(-14,-13+bob,28,5),Color("382c47"))
 draw_rect(Rect2(-3,-13+bob,6,5),Color("f4c77b"))
 draw_rect(Rect2(-13,-8,10,8),Color("22263e"));draw_rect(Rect2(5,-8,10,8),Color("22263e"))
 draw_rect(Rect2(-12,-51+bob,24,20),Color("edbf99"))
 draw_rect(Rect2(1 if facing > 0 else -8,-44+bob,5,5),Color("242338"))
 draw_polygon(PackedVector2Array([Vector2(-23,-50+bob),Vector2(1,-81+bob),Vector2(21,-50+bob)]),cloth)
 draw_rect(Rect2(-27,-53+bob,54,7),cloth.darkened(0.28))
 draw_rect(Rect2(-12,-58+bob,25,5),Color("edbe69"))
 var hand := Vector2(aim.x * 24,-29 + aim.y * 15 + bob)
 draw_circle(hand,5,Color("edbf99"))
 if protection > 0.0:
  draw_arc(Vector2(0,-32),43,0,TAU,32,Color(0.6,0.9,1.0,0.5),2)
 if health_reveal > 0.0:
  draw_rect(Rect2(-26,-94,52,5),Color("182035"))
  draw_rect(Rect2(-26,-94,52*health/T.MAX_HEALTH,5),tint)

func draw_ellipse_shadow() -> void:
 draw_set_transform(Vector2.ZERO,0,Vector2(1,0.27))
 draw_circle(Vector2(0,-3),25,Color(0.02,0.03,0.07,0.30))
 draw_set_transform(Vector2.ZERO)
