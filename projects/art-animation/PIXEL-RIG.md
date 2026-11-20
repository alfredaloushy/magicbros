# Red mage deformable pixel mesh study

The user requested a custom character with the pixel appearance of the existing
side-view exports. This study uses our own artwork, without downloaded model
geometry. Blender animates flat pixel meshes; Aseprite supplies the original
layered artwork and packages the rendered frames. The user rejected separate
cloth tiles and a height-only idle; the revised cape and hat bend as continuous
weighted surfaces, and the chest expands through shape keys. Bevy receives a PNG atlas and
timing/pivot metadata. No engine integration is included yet.

## Review files

- [Blender source](source/characters/red-mage-pixel-rig-v1.blend).
- [Baked Aseprite animation](source/characters/red-mage-pixel-rig-v1.aseprite).
- [Idle GIF](exports/characters/red-mage-pixel-rig-v1/idle.gif).
- [4x GIF preview](exports/characters/red-mage-pixel-rig-v1/idle-preview.gif).
- [128px-frame atlas](exports/characters/red-mage-pixel-rig-v1/idle.png) and
  [JSON metadata](exports/characters/red-mage-pixel-rig-v1/idle.json).
- Original layered source: [v4 Aseprite](source/characters/red-mage-idle-v4.aseprite).

Twelve bones control the root, body, near/far arms, near/far legs, head/hat,
hat middle/tip and cape root/middle/tip. Adjacent pixel faces share vertices:
the cape is one connected mesh rather than separate upper/lower tiles, and
the original hat layers are composited into one surface. Overlapping weights
produce a continuous bend while the cape collar and main hat base stay attached.
Chest shape keys expand the torso around its waist, with coordinated shoulder
and head movement; both boots remain planted.

The first pose preserves the original v4 frame exactly. Palette-coloured
emission materials, an orthographic camera, one render sample, zero filtering
across pixel boundaries and zero dithering keep the native output crisp. Mesh
deformations are rasterized into the 128px grid; contours still need visual
review for pixel shimmer. The one-second idle has 12 distinct frames, using 80/80/90ms
timing repeated four times so GIF and atlas timing agree. Feet pivot is `(70,119)`.

This is an idle style test, awaiting user approval. The hat uses deform bones
within the head surface; the face and brim stay attached to the head. Limb
partitions contain the original visible pixels; hidden joint
surfaces are not reconstructed. Broad arm swings and a complete running cycle
need additional pixel drawing and joint cleanup. This is controlled weighted
deformation, not a simulated physical cloth material. Artwork is not yet tested
in Bevy or at phone scale; collision and combat behaviour remain unchanged.

## Rebuild and validate

In Blender's Python console or its MCP execute endpoint, execute:

```python
p = '/home/abe/minidupe/projects/art-animation/scripts/build_pixel_rig.py'
exec(compile(open(p).read(), p, 'exec'), {'__file__': p, '__name__': '__main__'})
```

This retains existing scenes in memory and makes a new pixel study active. It
renders all frames to the ignored `.cache/pixel-rig-v1/` folder and writes a
standalone `.blend` containing only the new scene and its dependencies.
Package the frames from the repository root:

```sh
aseprite --batch \
  --script-param root="$PWD/projects/art-animation" \
  --script projects/art-animation/scripts/export_pixel_rig.lua
aseprite --batch projects/art-animation/source/characters/red-mage-pixel-rig-v1.aseprite \
  --format json-array --list-tags --list-slices \
  --sheet-type rows --sheet-columns 4 \
  --sheet projects/art-animation/exports/characters/red-mage-pixel-rig-v1/idle.png \
  --data projects/art-animation/exports/characters/red-mage-pixel-rig-v1/idle.json \
  --save-as projects/art-animation/exports/characters/red-mage-pixel-rig-v1/idle.gif
aseprite --batch projects/art-animation/source/characters/red-mage-pixel-rig-v1.aseprite \
  --scale 4 --save-as projects/art-animation/exports/characters/red-mage-pixel-rig-v1/idle-preview.gif
python3 projects/art-animation/scripts/check_pixel_rig.py
```

Validation checks actual exported pixels: neutral pose equals the original,
27 opaque colours only, hard alpha, fixed boots, frame-13 loop closure, frame
dimensions/timing/pivot and atlas agreement with cached Blender renders when
those are present. The [mesh report](exports/characters/red-mage-pixel-rig-v1/rig.json)
records non-rigid shape change, shared vertices, pinned-anchor errors and loop
closure from evaluated geometry. The rendered poses were visually reviewed. These checks
do not establish that full movement/casting animation or gameplay is complete.
