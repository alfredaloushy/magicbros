extends Node2D
# Graybox geometry with readable original placeholder art. No final-map commitment.
const PLATFORMS := [Rect2(100,1060,900,80),Rect2(1400,1060,900,80),
 Rect2(400,700,1600,48),Rect2(90,870,350,36),Rect2(1960,870,350,36),
 Rect2(260,470,420,40),Rect2(1720,470,420,40),Rect2(870,430,260,34),
 Rect2(1270,430,260,34),Rect2(1080,220,240,34)]
func _ready() -> void:
 for rect in PLATFORMS:
  var body := StaticBody2D.new()
  body.position = rect.position + rect.size/2.0
  body.collision_layer = 1
  body.collision_mask = 0
  var shape := CollisionShape2D.new()
  var rectangle := RectangleShape2D.new()
  rectangle.size = rect.size
  shape.shape = rectangle
  body.add_child(shape)
  add_child(body)
 queue_redraw()

func _draw() -> void:
 draw_rect(Rect2(-2000,-1800,6500,3600),Color("101a2c"))
 draw_circle(Vector2(1650,-100),140,Color("293856"))
 draw_circle(Vector2(1620,-130),125,Color("101a2c"))
 for i in range(70):
  var x := float((i * 347) % 3000) - 300
  var y := float((i * 193) % 900) - 650
  draw_rect(Rect2(x,y,2 if i%3 else 4,2 if i%3 else 4),Color("61718c"))
 for i in range(11):
  var x := float(i*260)-150
  var top := float(50+(i*151)%320)
  draw_rect(Rect2(x,top,145,1200),Color("18263c"))
  draw_polygon(PackedVector2Array([Vector2(x-20,top),Vector2(x+70,top-115),Vector2(x+165,top)]),Color("22334e"))
  for j in range(3):
   draw_rect(Rect2(x+28+j*37,top+50,15,28),Color("665841"))
 draw_rect(Rect2(-200,1140,2800,400),Color("211e38"))
 for rect in PLATFORMS:
  draw_rect(Rect2(rect.position+Vector2(0,6),rect.size),Color("26354a"))
  draw_rect(rect,Color("526279"))
  draw_rect(Rect2(rect.position,Vector2(rect.size.x,7)),Color("8b9a9d"))
  draw_rect(Rect2(rect.position+Vector2(0,7),Vector2(rect.size.x,4)),Color("6c8c80"))
  for x in range(int(rect.position.x)+20,int(rect.end.x)-10,56):
   draw_line(Vector2(x,rect.position.y+15),Vector2(x+32,rect.position.y+15),Color("3c4d62"),2)
 for x in [460,1900]:
  draw_rect(Rect2(x,700,7,95),Color("ad8b63"))
  draw_polygon(PackedVector2Array([Vector2(x+7,718),Vector2(x+70,718),Vector2(x+70,770),Vector2(x+38,791),Vector2(x+7,770)]),Color("775482"))
  draw_circle(Vector2(x+38,743),9,Color("d4b782"))
