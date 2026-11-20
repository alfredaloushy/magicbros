"""Validate the actual Blender/Aseprite handoff using Python's standard library."""

from pathlib import Path
import hashlib
import json
import struct
import zlib

ROOT = Path(__file__).resolve().parents[1]


def png(path):
    raw = path.read_bytes()
    assert raw[:8] == b'\x89PNG\r\n\x1a\n'
    pos, compressed, palette, alpha = 8, b'', None, None
    while pos < len(raw):
        length = struct.unpack_from('>I', raw, pos)[0]
        kind, data = raw[pos+4:pos+8], raw[pos+8:pos+8+length]
        pos += length + 12
        if kind == b'IHDR':
            w, h, depth, mode, _, _, interlace = struct.unpack('>IIBBBBB', data)
        elif kind == b'IDAT':
            compressed += data
        elif kind == b'PLTE':
            palette = [tuple(data[i:i+3]) for i in range(0, len(data), 3)]
        elif kind == b'tRNS':
            alpha = data
    assert depth == 8 and interlace == 0
    channels = {6: 4, 2: 3, 3: 1}[mode]
    stride = w * channels
    data, previous, output = zlib.decompress(compressed), bytearray(stride), []
    for y in range(h):
        offset = y * (stride + 1)
        filter_type = data[offset]
        row = bytearray(data[offset+1:offset+1+stride])
        for i in range(stride):
            a = row[i-channels] if i >= channels else 0
            b = previous[i]
            c = previous[i-channels] if i >= channels else 0
            p = a + b - c
            pa, pb, pc = abs(p-a), abs(p-b), abs(p-c)
            predictor = a if pa <= pb and pa <= pc else b if pb <= pc else c
            row[i] = (row[i] + [0, a, b, (a+b)//2, predictor][filter_type]) & 255
        for x in range(w):
            pixel = tuple(row[x*channels:(x+1)*channels])
            if mode == 2:
                pixel += (255,)
            elif mode == 3:
                index = pixel[0]
                pixel = palette[index] + (alpha[index] if alpha and index < len(alpha) else 255,)
            output.append(pixel if pixel[3] else (0, 0, 0, 0))
        previous = row
    return w, h, output


sw, _, source = png(ROOT / 'exports/characters/red-mage-idle-v4.png')
expected = [source[y*sw+x] for y in range(128) for x in range(128)]
aw, ah, atlas = png(ROOT / 'exports/characters/red-mage-pixel-rig-v1/idle.png')
metadata = json.loads((ROOT / 'exports/characters/red-mage-pixel-rig-v1/idle.json').read_text())
assert len(metadata['frames']) == 12 and (aw, ah) == (512, 384)
frames = []
for frame in metadata['frames']:
    rect = frame['frame']
    assert rect['w'] == rect['h'] == 128 and not frame['trimmed'] and not frame['rotated']
    frames.append([atlas[(rect['y']+y)*aw+rect['x']+x] for y in range(128) for x in range(128)])
assert frames[0] == expected, 'Neutral pose changed the reference pixels'
palette = {p for p in expected if p[3]}
for frame in frames:
    assert all(p[3] in (0, 255) for p in frame), 'Soft alpha'
    assert {p for p in frame if p[3]} <= palette, 'Off-palette colors'
    assert frame[108*128:] == expected[108*128:], 'Boot drift'
rig_report = json.loads((ROOT / 'exports/characters/red-mage-pixel-rig-v1/rig.json').read_text())
for part, check in rig_report['deformation'].items():
    assert check['shared_vertices'], f'{part} is disconnected pixel tiles'
    assert check['max_edge_length_change_px'] > .0001, f'{part} does not deform'
    assert check['anchor_error_px'] < .0001, f'{part} detached from its anchor'
    assert check['loop_error_px'] < .0001, f'{part} does not return to rest at frame 13'
assert sum(f['duration'] for f in metadata['frames']) == 1000
assert metadata['meta']['slices'][0]['keys'][0]['pivot'] == {'x': 70, 'y': 119}
raw_frames = sorted((ROOT / '.cache/pixel-rig-v1').glob('frame_*.png'))
if raw_frames:
    assert len(raw_frames) == 12
    for path, frame in zip(raw_frames, frames):
        assert png(path)[2] == frame, 'Atlas differs from Blender render'
    assert png(ROOT / '.cache/pixel-rig-v1/loop.png')[2] == frames[0], 'Loop closure differs'
unique = len({hashlib.sha256(bytes(c for p in frame for c in p)).digest() for frame in frames})
assert unique > 1, 'No motion'
print(json.dumps({'frames': 12, 'unique_poses': unique, 'duration_ms': 1000,
                  'neutral_rgba_differences': 0, 'palette_colors': len(palette),
                  'hard_alpha': True, 'boots_fixed': True, 'loop_returns_to_rest': True,
                  'render_comparison': bool(raw_frames)}))
