-- Package Blender renders without changing their pixels.
local root = assert(app.params['root'], 'Pass --script-param root=/path/to/art-animation')
local sprite = Sprite(128, 128, ColorMode.RGB)
sprite.layers[1].name = 'Blender pixel cutout render'
for i = 1, 12 do
  if i > 1 then sprite:newEmptyFrame() end
  local image = Image{fromFile=string.format('%s/.cache/pixel-rig-v1/frame_%02d.png', root, i)}
  sprite:newCel(sprite.layers[1], i, image, Point(0, 0))
  -- GIF-compatible centisecond timing; three frames total exactly 250 ms.
  sprite.frames[i].duration = (i % 3 == 0 and 0.090 or 0.080)
end
local tag = sprite:newTag(1, 12)
tag.name = 'idle'
local feet = sprite:newSlice(Rectangle(0, 0, 128, 128))
feet.name = 'feet'
feet.pivot = Point(70, 119)
sprite:saveAs(root .. '/source/characters/red-mage-pixel-rig-v1.aseprite')
app.activeSprite = sprite
