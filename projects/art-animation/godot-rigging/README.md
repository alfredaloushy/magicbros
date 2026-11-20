# Godot cape and character lab

Godot 4.7.2 is installed at `~/.local/share/godot/4.7.2/`, with a
`~/.local/bin/godot` launcher and **Godot Editor** applications-menu entry.
The official Linux archive's SHA-512 is pinned in the installer. The launcher
includes the desktop/graphics library paths needed on this NixOS machine.
The silent fabric lab uses Godot's Dummy audio driver.
Engine downloads and import caches are excluded from Git.

## Open and play

From the repository root:

```sh
./projects/art-animation/install-godot.sh  # one-time setup on Linux x86_64
./projects/art-animation/godot.sh          # open mage_with_cape.tscn in the editor
./projects/art-animation/godot.sh --play   # run the combined character preview
./projects/art-animation/godot.sh --cape   # edit the reusable cape asset separately
./projects/art-animation/godot.sh --play-cape  # run only the cape study
```

Press **F5** in Godot for the resting character with its attached cape.
**Play animation** pauses/resumes the body and cloth together, **Resting idle**
switches breathing on/off,
**Breeze** changes wind, **Stretch resistance** changes stiffness, and
**Reset character** restarts the simulation. **Open cape-only study** switches
to the separate asset preview; **Back to character** returns. Use F5 for these controls; F6 runs
only the selected scene.

Select `CapeFabric` in `cape_world.tscn` to inspect the actual SoftBody3D mesh,
material, pinned points, mass, stiffness and simulation precision. That asset
and its physics controller remain independent of the character scene.

## Attached character

`mage_with_cape.tscn` instances `cape_world.tscn` and adds the original head,
torso/arms and legs as unfiltered pixel layers. The cape is not copied into
the character, so its mesh/material/cloth settings remain editable in one place.
The collar's 55 pinned points follow `ShoulderAttachment`, synchronized at
physics frequency so attachment also works during headless baking. Free fabric
vertices remain under the cape's existing solver. The flat pixel layers provide
visual overlap. Placeholder torso/shin collision volumes were removed after
they caused the cape to bunch up; the cape keeps the standalone asset's response.

`character_preview.tscn` renders at native 128 × 128 and displays at 4× through
the existing palette pass. Its camera shows the whole character. The original
pixel textures, hard transparency and palette remain in use.

The character's `AnimationPlayer` autoplays the editable **resting** animation:
a 3.6-second loop with a quiet inhale, longer exhale and delayed one-pixel head
rise. The torso expands vertically by at most 4% about the belt; the waist and
both feet stay planted. The shoulder follows that chest motion, while the shared
cape continues to simulate independently. Select `AnimationPlayer` and choose
`resting` in Godot's Animation panel to adjust its tracks. `RESET` restores the
original pose. Running/casting animations and separate hat deformation remain future work.

## Current cloth study

The cape uses native Jolt soft-body physics: 799 shared vertices, 55 collar pins,
12 solver iterations and zero internal pressure. The upper, middle and lower
surface respond to wind and gravity. Wind acts as physical force on the fabric;
no bottom-only bone animation or mesh scaling drives this version.

The original Aseprite PNG supplies all detail. The material uses nearest
filtering, unshaded colour, hard transparency and two-sided rendering. An
orthographic camera renders to 72 × 72, displayed at 8× for the close-up. No
detailed 3D character model or stretched cape texture was created.

The final 2D pass restores the source palette after Godot's 3D colour pipeline.
Automatic 3D texture compression and mipmaps are disabled. A rendered native
cape capture was checked: zero off-palette colours and no partial alpha.

This is a reusable fabric experiment. The default
settings pass attachment/motion/stretch regression bounds; those checks do not
establish that it looks natural. The user requested keeping it as a separate
asset and attaching it to the character. The combined presentation still needs
visual review; final looping and baking frames for Bevy remain pending.

## Preserved character rig

`red_mage.tscn` remains an editable Skeleton2D character: ten Bone2D nodes and
four textured Polygon2D meshes. Six short links control the cape. The older
`main.tscn` preview uses a damped-spring bone wave and separate hat flutter;
the user rejected that motion as insufficient, so it is not the default preview
or an approved animation. `AnimationPlayer/RESET` records its rest pose.

The four 128 × 128 PNGs are unchanged first-frame layers from the
[Aseprite source](../source/characters/red-mage-idle-v4.aseprite): cape, torso/arms,
legs/boots, and composited head/hat/tip. Aseprite remains the paint master.
Textures are lossless and have no mipmaps. The character rest render was compared
to the original atlas frame: zero RGBA differences.

Bevy remains the gameplay engine. Godot rigs do not load directly into the
current Bevy renderer; final PNG/timing export is a future handoff. The earlier
`wild-magic-academy/` Godot gameplay experiment is separate.

## Rebuild and validate

```sh
./projects/art-animation/godot.sh --build-rig
./projects/art-animation/godot.sh --build-cape
./projects/art-animation/godot.sh --build-character
./projects/art-animation/godot.sh --check
```

The builders replace their generated `.tscn` files, so save manual rig edits
before rebuilding. The cape builder derives its mesh and UVs from the unchanged
cape polygon. The character builder saves the composition and its reference to
the shared cape. Normal editing does not need a rebuild. Keep `.tscn`, `.gd`, `.uid`,
PNGs and `.import` settings in Git; exclude `.godot/`.

Checks cover character weights/rest poses and 600 native cloth steps: motion in
all three fabric regions, collar error below 0.001px and edge strain below 15%
at default settings. A further 900-step moving-shoulder check covers more than
two breathing cycles: shared cape reference, attachment error below 0.05px,
fixed boots/belt, one-pixel head travel, continuous loop motion, pause and
breathing toggle. These are regression limits, not artistic approval.
Editor startup and GPU-rendered character/cape previews were visually inspected.
User interaction and animation acceptance remain separate checks.

References: [Godot download](https://godotengine.org/download/linux/),
[2D skeleton workflow](https://docs.godotengine.org/en/stable/tutorials/animation/2d_skeletons.html)
and [native cloak simulation](https://docs.godotengine.org/en/stable/tutorials/physics/soft_body.html#cloak-simulation).
