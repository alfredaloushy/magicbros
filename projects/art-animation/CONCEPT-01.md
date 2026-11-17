# First wizard concept

Status: draft for visual review, not an approved character or animation set.

The user directs the design and Codex draws in the live Aseprite window. The
first requested reference is **ANNO: Mutationem**. Borrow its readable human
proportions, sharp costume shapes and contrasting light, while designing an
original apprentice for Wild Magic Academy. Reference:
[official game page and screenshots](https://store.steampowered.com/app/1368030/ANNO_Mutationem/).
Reference screenshots are kept in the ignored local cache, outside deliverables.

## First-pass art spec

- One silver-haired apprentice, facing right in a relaxed three-quarter side view.
- Fitted violet academy coat, pale inner tunic, rose scarf, tall dark boots,
  restrained brass trim and a small cyan spell over the raised hand.
- A 64 × 64 transparent RGB canvas, with roughly 54 pixels of character height.
  Selective dark outlines, light from upper left and a small cyan rim near magic.
- Separate head, torso, near/far arms and legs, scarf and magic layers. The
  construction sketch stays hidden; a ground shadow can be added later.
- Fixed feet pivot `(32, 60)`, measured from the frame's top-left boundary.
  A future scale test starts at one world unit per art pixel. The gameplay body
  remains 32 × 54 world units; the pose and costume never define collision.
- One static concept pose. Costume, palette, anatomy and resolution remain
  provisional until the user reviews the mock-up. Idle/run animation comes next.

## Reference interpretation

Use a slim, readable silhouette, long legs, asymmetric coat folds, cool shadows
and small warm highlights. Keep the face economical at this size. A luminous
hand spell and academy clothing communicate a mage without adding a weapon.
Do not trace or redistribute the reference game's sprites. Its 3D environments
are a visual reference; this prototype still uses a 2D arena.

## Optional concept-reference prompt

If a generated design sheet is useful for a later revision, use this prompt:

```text
Original pixel-art student mage concept for Wild Magic Academy. A confident
young adult apprentice with short silver hair, a fitted violet academy coat,
pale inner tunic, rose scarf, tall dark boots, restrained brass trim and a cyan
spell floating over one raised hand. Slim human proportions and long legs;
sharp costume shapes and cinematic warm/cool contrast inspired by ANNO:
Mutationem. Three-quarter side view, facing right, upper-left light, selective
dark outlines, hard pixel edges, no anti-aliasing. Design for a 64x64 frame with
a character about 54 pixels tall. One row of three equal panels: resting pose,
casting pose, back view. Same scale and ground line in every panel. Flat neutral
background, no scenery, text, labels, weapons, borders or watermark.
```

## Review gate

Review the silhouette and costume at 1× and enlarged, then against a small
academy courtyard sample. Agree the design and pixel density before producing
the movement/casting animation set or a large map tileset. Export source and
transparent PNG together; engine integration is a separate presentation task.
