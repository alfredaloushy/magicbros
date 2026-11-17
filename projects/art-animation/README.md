# Wild Magic Academy — art and animation project

Status: active. The generated red mage idle sheet is imported as a six-frame
Aseprite animation and previewed in the live editor. Style approval, native
pixel cleanup, map artwork and engine integration remain pending.

## Current work

Current: [six-frame idle sprite sheet](exports/characters/red-mage-idle-sheet-v3.png),
a 1536 × 1024 transparent PNG with six equal 512 × 512 cells, clearer cape
flow and delayed hat-tip movement. The [editable Aseprite animation](source/characters/red-mage-idle-sheet-v3.aseprite)
plays a 1.3-second loop. [GIF preview](exports/characters/red-mage-idle-sheet-v3.gif)
and [frame metadata](exports/characters/red-mage-idle-sheet-v3.json) are saved.
Frame alignment was corrected in the live editor, and the planted boots are
pixel-identical in all six frames. The generated source's soft alpha and
large color range remain; native pixel cleanup is a separate step.

Earlier: the [four-frame idle animation](source/characters/red-mage-idle-sheet-v2.aseprite)
plays in Aseprite as a 1.2-second loop, with four 627 × 627 frames and timings
of 400/200/400/200ms. [GIF preview](exports/characters/red-mage-idle-aseprite-v2.gif),
[horizontal atlas](exports/characters/red-mage-idle-aseprite-v2.png) and
[frame metadata](exports/characters/red-mage-idle-aseprite-v2.json) are saved.
This uses the earlier side-facing sheet; the single magenta image below is a
separate source for the manual external-video workflow.

For the manual Pixelate workflow: [single front-facing red mage source](references/red-mage-idle-source-magenta-v1.png),
fully opaque with an exact `#FF00FF` background. The square source is styled as
256 × 256 pixel art and retained at its actual 1254 × 1254 raster size.

New: [generated four-frame red mage idle sheet](exports/characters/red-mage-idle-sheet-v2.png)
with [frame coordinates and timing](exports/characters/red-mage-idle-sheet-v2.json).
This preserves the generated RGBA image at 1254 × 1254, arranged as four
627 × 627 cells in reading order. It is reference-resolution animation art for
manual processing/review, not an integrated native-resolution game sprite.

The user supplies art direction and Codex draws in the live Aseprite window.
The current direction is to follow our original concept art exclusively. The red
mage is drawn from the lower-left character lineup in
[the concept sheet](../../documents/.build/concept-art.png). The earlier ANNO-inspired
apprentice is superseded and retained only as an earlier study.

- [Red mage brief](RED-MAGE.md): current reference, source, exports and review.
- [Generated red mage reference](references/red-mage-reference-v1.png):
  right-facing character reference for the next Aseprite recreation.
- [Exact Aseprite copy](source/characters/red-mage-reference-v1.aseprite):
  original reference pixels and transparency preserved at 1293 × 1217.
- [Red mage source](source/characters/red-mage.aseprite) and
  [preview](exports/characters/red-mage-preview.png).
- [Earlier concept brief](CONCEPT-01.md): superseded study.
- [Editable wizard concept](source/characters/wizard-concept-01.aseprite).
- [Blank character template](source/characters/wizard-template.aseprite).
- [Transparent sprite](exports/characters/wizard-concept-01.png),
  [atlas metadata](exports/characters/wizard-concept-01.json) and
  [enlarged preview](exports/characters/wizard-concept-01-preview.png).
- [Aseprite workflow](ASEPRITE-WORKFLOW.md): source/export conventions and the
  planned character, animation and map handoff to Bevy.

The earlier 64 × 64 character files are static design studies. The new idle
animation retains the generated sheet's reference resolution and soft alpha;
it has not been approved as the final character style or imported into Bevy.

## Brief

Create readable wizard sprites, animation, spell VFX and arena art for the Bevy
game. Begin with a small in-motion style test using one wizard, one platform,
Fireball and Gust. The storybook-background/pixel-character mixture is provisional.
Use the shared design PDF and illustrated pages 17–21 for exploration.

## Owns

Wizard silhouette/color variants, movement/casting/hit/defeat animations, spell
sprites/effects, terrain and background artwork, export metadata and asset manifests.
Use Aseprite and its connected MCP when asset creation begins.

## Inputs and deliverables

Read [core integration notes](../core-gameplay/INTEGRATION.md) for dimensions,
state and effect semantics. Agree pixel density/pivots before large asset batches.
Deliver editable sources, PNG sprite sheets, animation names/timings and a preview
integrated with the core lab. Do not change collision sizes or spell behavior.

First gate: readable movement, aiming, Fireball and Gust on desktop and a phone-size
preview, with player silhouettes visible during effects.
