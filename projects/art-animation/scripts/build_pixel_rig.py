"""Build the red mage's deformable pixel mesh study in Blender.

Run in Blender's Python console or through its MCP execute_code endpoint:
exec(compile(open('/absolute/path/build_pixel_rig.py').read(), 'pixel-rig', 'exec'))
The existing scenes are retained; the new study becomes the active scene.
"""

from pathlib import Path
import json
import math
import struct
import zlib

import bpy


ROOT = Path(__file__).resolve().parents[1]
SOURCE = ROOT / 'source/characters/red-mage-idle-v4.aseprite'
DEST = ROOT / 'source/characters/red-mage-pixel-rig-v1.blend'
EXPORT = ROOT / 'exports/characters/red-mage-pixel-rig-v1'
CACHE = ROOT / '.cache/pixel-rig-v1'
PROMPT = 'Original red mage deformable pixel mesh study; no downloaded geometry.'


def first_frame_layers(path):
    """Read visible, full-opacity, normal RGBA layers in this Aseprite source."""
    layers, cels = [], {}
    with path.open('rb') as stream:
        header = stream.read(128)
        _, magic, _, width, height, depth = struct.unpack_from('<I5H', header)
        assert magic == 0xA5E0 and depth == 32
        frame_header = stream.read(16)
        chunks = struct.unpack_from('<H', frame_header, 6)[0]
        for _ in range(chunks):
            size, kind = struct.unpack('<IH', stream.read(6))
            data = stream.read(size - 6)
            if kind == 0x2004:
                flags, layer_type, _, _, _, blend = struct.unpack_from('<6H', data)
                assert layer_type == 0 and blend == 0 and data[12] == 255
                length = struct.unpack_from('<H', data, 16)[0]
                layers.append((data[18:18 + length].decode(), bool(flags & 1)))
            elif kind == 0x2005:
                layer, x, y, opacity, cel_type = struct.unpack_from('<HhhBH', data)
                assert opacity == 255 and cel_type == 2
                w, h = struct.unpack_from('<HH', data, 16)
                raw = zlib.decompress(data[20:])
                assert len(raw) == w * h * 4
                cels[layer] = [(x + i % w, y + i // w, tuple(raw[4*i:4*i+4]))
                               for i in range(w*h) if raw[4*i+3]]
    return width, height, [(name, cels[i]) for i, (name, visible) in enumerate(layers)
                           if visible and i in cels]


def linear(value):
    value /= 255
    return value / 12.92 if value <= 0.04045 else ((value + 0.055) / 1.055) ** 2.4


def part_name(layer, x, y):
    if layer == 'Cape':
        return 'cape'
    if layer == 'Legs and boots':
        return 'leg_far' if x < 69 else 'leg_near'
    if layer == 'Torso and arms':
        if x >= 83 and y >= 77:
            return 'arm_near'
        if x < 63 and y < 80:
            return 'arm_far'
        return 'body'
    return {'Head and hat': 'head_hat'}[layer]


def clamp(value):
    return max(0, min(1, value))


def cloth_weights(name, x, y):
    if name == 'cape':
        # The collar stays on the body; influence increases towards the hem.
        t = clamp(((63-x) + (y-80)*.65)/48) * clamp((y-83)/15)
        bones = ('cape', 'cape_mid', 'cape_tip')
    elif name == 'head_hat':
        # The face/brim remain on the head; the soft crown bends into its tip.
        t = clamp((54-x)/28) * clamp((52-y)/10)
        bones = ('head_hat', 'hat_mid', 'hat_tip')
    else:
        return {name: 1}
    if t < .5:
        return {bones[0]: 1-2*t, bones[1]: 2*t}
    return {bones[1]: 2-2*t, bones[2]: 2*t-1}


width, height, layers = first_frame_layers(SOURCE)
# Composite the two original hat layers into one connected surface, preserving
# the visible pixels while removing the old overlapping cutout boundary.
head = {(x, y): c for name, pixels in layers if name == 'Head and hat' for x, y, c in pixels}
head.update({(x, y): c for name, pixels in layers if name == 'Hat tip' for x, y, c in pixels})
layers = [(name, [(x, y, c) for (x, y), c in head.items()] if name == 'Head and hat' else pixels)
          for name, pixels in layers if name != 'Hat tip']
assert (width, height) == (128, 128)
for _, pixels in layers:
    assert all(c[3] == 255 for _, _, c in pixels), 'Source must use hard alpha'
scene = bpy.data.scenes.new('Red Mage | Deformable Pixel Cloth v1')
bpy.context.window.scene = scene
scene['description'] = PROMPT
scene['source'] = str(SOURCE.relative_to(ROOT))
scene['pixel_size'] = 0.01
scene['feet_pivot'] = [70, 119]
scene['limitations'] = ('Front arm and legs are partitioned visible pixels; hidden joint '
                       'art is not reconstructed. Hat and cape use weighted mesh bending. '
                       'Idle study only, not a finished run/cast rig.')

rig_data = bpy.data.armatures.new('RedMage_PixelBones')
rig = bpy.data.objects.new('RedMage_PixelRig', rig_data)
scene.collection.objects.link(rig)
bpy.context.view_layer.objects.active = rig
rig.select_set(True)
rig.show_in_front = True
bpy.ops.object.mode_set(mode='EDIT')
pivots = {
    'root': (70, 119, None), 'body': (71, 96, 'root'),
    'head_hat': (74, 66, 'body'), 'hat_mid': (48, 28, 'head_hat'),
    'hat_tip': (33, 37, 'hat_mid'),
    'cape': (62, 79, 'body'), 'cape_mid': (51, 91, 'cape'),
    'cape_tip': (39, 100, 'cape_mid'),
    'arm_far': (62, 74, 'body'), 'arm_near': (84, 79, 'body'),
    'leg_far': (62, 99, 'root'), 'leg_near': (76, 99, 'root'),
}
for name, (x, y, parent) in pivots.items():
    bone = rig_data.edit_bones.new(name)
    bone.head = ((x-70)/100, (119-y)/100, 0)
    bone.tail = (bone.head.x, bone.head.y + 0.09, 0)
    if parent:
        bone.parent = rig_data.edit_bones[parent]
bpy.ops.object.mode_set(mode='OBJECT')

palette = sorted({c for _, pixels in layers for _, _, c in pixels})
materials = []
for rgba in palette:
    mat = bpy.data.materials.new('Pixel_' + ''.join(f'{c:02X}' for c in rgba[:3]))
    mat.use_nodes = True
    nodes = mat.node_tree.nodes
    nodes.clear()
    emission = nodes.new('ShaderNodeEmission')
    emission.inputs[0].default_value = (*[linear(v) for v in rgba[:3]], 1)
    output = nodes.new('ShaderNodeOutputMaterial')
    mat.node_tree.links.new(emission.outputs[0], output.inputs['Surface'])
    materials.append(mat)
index = {color: i for i, color in enumerate(palette)}
parts = {}
for depth, (layer, pixels) in enumerate(layers):
    for x, y, color in pixels:
        parts.setdefault(part_name(layer, x, y), []).append((x, y, color, depth))
objects = {}
for name, pixels in parts.items():
    verts, faces, colors = [], [], []
    vertex_index = {}
    for x, y, color, depth in pixels:
        face = []
        for px, py in [(x,y), (x,y+1), (x+1,y+1), (x+1,y)]:
            key = (px, py, depth)
            if key not in vertex_index:
                vertex_index[key] = len(verts)
                verts.append(((px-70)/100, (119-py)/100, depth*.001))
            face.append(vertex_index[key])
        faces.append(tuple(face))
        colors.append(index[color])
    mesh = bpy.data.meshes.new('Pixels_' + name)
    mesh.from_pydata(verts, [], faces)
    mesh.update()
    obj = bpy.data.objects.new('Art_' + name, mesh)
    objects[name] = obj
    obj['part'] = name
    scene.collection.objects.link(obj)
    for mat in materials:
        mesh.materials.append(mat)
    for poly, color in zip(mesh.polygons, colors):
        poly.material_index = color
    for vertex in mesh.vertices:
        x, y = vertex.co.x*100+70, 119-vertex.co.y*100
        for bone, weight in cloth_weights(name, x, y).items():
            if weight > 0:
                group = obj.vertex_groups.get(bone) or obj.vertex_groups.new(name=bone)
                group.add([vertex.index], weight, 'REPLACE')
    # Chest expansion changes the silhouette around a fixed waist. Apply the
    # same deformation to all adjoining torso pieces to keep their seams closed.
    if name in ('body', 'arm_near', 'arm_far'):
        obj.shape_key_add(name='Basis')
        inhale = obj.shape_key_add(name='Chest expansion')
        for point in inhale.data:
            y = 119-point.co.y*100
            influence = clamp((98-y)/28)
            point.co.x += (point.co.x-.01) * .024 * influence
            point.co.y += .006 * influence
    modifier = obj.modifiers.new('Continuous weighted bending', 'ARMATURE')
    modifier.object = rig
    obj.parent = rig
    obj['source_pixel_count'] = len(pixels)

# Animate continuous surfaces. The cape collar and hat base stay attached while
# overlapping bone influences create a changing bend along the cloth.
for frame in range(1, 14):
    phase = math.tau * ((frame-1) % 12) / 12
    breath = (1-math.cos(phase))/2
    angles = {
        'body': -.007*breath, 'head_hat': .013*breath,
        'cape_mid': .10*math.sin(phase),
        'cape_tip': .13*(math.sin(phase-.65)+math.sin(.65)),
        'hat_mid': .045*(math.sin(phase-.35)+math.sin(.35)),
        'hat_tip': .09*(math.sin(phase-.8)+math.sin(.8)),
    }
    for name, bone in rig.pose.bones.items():
        bone.rotation_mode = 'XYZ'
        bone.rotation_euler.z = angles.get(name, 0)
        bone.location = (0, .006*breath if name == 'head_hat' else .004*breath if name == 'cape' else 0, 0)
        bone.keyframe_insert('location', frame=frame, group=name)
        bone.keyframe_insert('rotation_euler', frame=frame, group=name)
    for name in ('body', 'arm_near', 'arm_far'):
        key = objects[name].data.shape_keys.key_blocks['Chest expansion']
        key.value = breath
        key.keyframe_insert('value', frame=frame)
action = rig.animation_data.action
action.name = 'Idle_PixelStudy_12fps'
for layer in action.layers:
    for strip in layer.strips:
        for bag in strip.channelbags:
            for curve in bag.fcurves:
                for key in curve.keyframe_points:
                    key.interpolation = 'BEZIER'
                curve.modifiers.new('CYCLES')

camera_data = bpy.data.cameras.new('PixelCamera')
camera = bpy.data.objects.new('PixelCamera', camera_data)
scene.collection.objects.link(camera)
camera.location = (-.06, .55, 4)
camera_data.type = 'ORTHO'
camera_data.ortho_scale = 1.28
scene.camera = camera
scene.render.resolution_x = 128
scene.render.resolution_y = 128
scene.render.resolution_percentage = 100
scene.render.film_transparent = True
scene.render.image_settings.file_format = 'PNG'
scene.render.image_settings.color_mode = 'RGBA'
scene.render.image_settings.color_depth = '8'
scene.render.engine = 'CYCLES'
scene.cycles.samples = 1
scene.cycles.use_denoising = False
scene.cycles.pixel_filter_type = 'BOX'
scene.render.filter_size = 0.01
scene.render.dither_intensity = 0
scene.view_settings.view_transform = 'Standard'
scene.view_settings.look = 'None'
scene.view_settings.exposure = 0
scene.view_settings.gamma = 1
scene.render.fps = 12
scene.frame_start, scene.frame_end = 1, 12
scene.frame_set(1)
rig.show_in_front = False
for area in bpy.context.screen.areas:
    if area.type == 'VIEW_3D':
        area.spaces.active.region_3d.view_perspective = 'CAMERA'
        area.spaces.active.shading.type = 'MATERIAL'
for directory in [DEST.parent, EXPORT, CACHE]:
    directory.mkdir(parents=True, exist_ok=True)

# Measure the evaluated surfaces, so the handoff demonstrates actual bending
# rather than separate tiles or a translated silhouette.
deformation = {}
for name, anchor in [('cape', 'cape'), ('head_hat', 'head_hat'), ('body', 'body')]:
    obj = objects[name]
    samples, anchor_error = [], 0
    for frame in (1, 4, 7, 10, 13):
        scene.frame_set(frame)
        evaluated = obj.evaluated_get(bpy.context.evaluated_depsgraph_get())
        coordinates = [v.co.copy() for v in evaluated.data.vertices]
        samples.append(coordinates)
        if name != 'body':
            transform = rig.pose.bones[anchor].matrix @ rig_data.bones[anchor].matrix_local.inverted()
            for vertex, coordinate in zip(obj.data.vertices, coordinates):
                influences = {obj.vertex_groups[g.group].name: g.weight for g in vertex.groups}
                if influences.get(anchor, 0) > .999999:
                    anchor_error = max(anchor_error, (coordinate-transform @ vertex.co).length)
    delta = max(abs((coordinates[e.vertices[0]]-coordinates[e.vertices[1]]).length -
                    (samples[0][e.vertices[0]]-samples[0][e.vertices[1]]).length)
                for coordinates in samples[1:] for e in obj.data.edges)
    loop_error = max((a-b).length for a, b in zip(samples[0], samples[-1]))
    assert delta > 1e-6, f'{name} has no actual shape change'
    assert anchor_error < 1e-6 and loop_error < 1e-6
    deformation[name] = {'vertices': len(obj.data.vertices), 'faces': len(obj.data.polygons),
                         'shared_vertices': len(obj.data.vertices) < 4*len(obj.data.polygons),
                         'max_edge_length_change_px': delta*100,
                         'anchor_error_px': anchor_error*100, 'loop_error_px': loop_error*100}
report = {'bones': len(rig_data.bones), 'deformation': deformation,
          'motion': 'weighted cloth bending and chest shape keys', 'frames': 12}
(EXPORT / 'rig.json').write_text(json.dumps(report, indent=2) + '\n')
for frame in range(1, 13):
    scene.frame_set(frame)
    scene.render.filepath = str(CACHE / f'frame_{frame:02d}.png')
    bpy.ops.render.render(write_still=True)
scene.frame_set(13)
scene.render.filepath = str(CACHE / 'loop.png')
bpy.ops.render.render(write_still=True)
scene.frame_set(1)
scene.render.filepath = str(CACHE / 'frame_01.png')
# Save only this scene and its dependencies. Other open scenes (including the
# downloaded-model review) stay in memory and are not bundled in our source.
bpy.data.libraries.write(str(DEST), {scene}, fake_user=True, compress=True)
print(json.dumps({'scene': scene.name, 'bones': len(rig_data.bones),
                  'parts': {name: len(p) for name, p in parts.items()},
                  'colors': len(palette), 'blend': str(DEST)}))
