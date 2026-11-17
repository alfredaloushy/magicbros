# Red mage — original concept study

The user requested this character be drawn solely from the project's original
[concept sheet](../../documents/.build/concept-art.png), replacing the earlier
ANNO: Mutationem direction. The specific reference is the red wizard in the
lower-left lineup (source region x=55, y=614, width=146, height=174).

## Design and files

The user requested an imagegen reference to guide a subsequent faithful Aseprite
recreation. The generated [reference PNG](references/red-mage-reference-v1.png)
uses the original concept sheet as its sole visual input, emphasizing a
right-facing profile and consistent compact proportions. Created with built-in
imagegen; the [full prompt](references/red-mage-reference-v1-prompt.txt) is retained.
An [exact Aseprite copy](source/characters/red-mage-reference-v1.aseprite) is now
available and open in the editor. The PNG was opened directly and saved as
Aseprite source, preserving its original 1293 × 1217 resolution, RGB colours
and alpha on one editable layer named `Original reference pixels`. A PNG
export from this document was decoded and compared against the original:
zero RGBA pixel differences. No resizing, palette reduction or redrawing was
applied. This is a faithful full-resolution image copy; limb separation,
native gameplay resolution and animation remain future work.

The smaller Aseprite source below is the earlier hand-drawn interpretation.

The sprite follows the oversized floppy red hat with a gold star and brim,
brown fringe, small pale face, short red cape with gold edging, dark tunic,
square gold belt buckle and brown boots. It uses the reference's compact
proportions and a right-facing resting pose. The revised profile has one visible
eye, a nose/chin contour, a narrower shoulder/chest silhouette and one dominant
near arm. The head, collar and waist share a connected body axis; the rear arm
is partly hidden, the cape attaches at the back shoulder, and both boots point
right with staggered feet. Decorative stitching and fine hat texture are
simplified to fit the pixel grid. No spell is included in the resting pose.

- Source: [red-mage.aseprite](source/characters/red-mage.aseprite).
- Game image: [red-mage.png](exports/characters/red-mage.png).
- Frame metadata: [red-mage.json](exports/characters/red-mage.json).
- Review image: [red-mage-preview.png](exports/characters/red-mage-preview.png).

One transparent 64 × 64 frame, 18 palette slots, feet pivot `(32, 60)`.
The preview is enlarged 8× with nearest-neighbour sampling. Nine editable
layers separate cape, legs, arms, torso, head, collar and hat. The reference
was imported temporarily for comparison and removed before saving exports.

## Review and limits

### Aseprite playback of the generated idle sheet

The user requested trying the sprite sheet as an animation in Aseprite.
The live editor imported the four equal panels of
`exports/characters/red-mage-idle-sheet-v2.png` without scaling, repositioning
or palette conversion. The saved [editable source](source/characters/red-mage-idle-sheet-v2.aseprite)
has four 627 × 627 RGB frames on `Idle artwork`, an infinitely looping `idle`
tag and a shared feet pivot `(345, 582)`. Timings are 400/200/400/200ms, totaling
1.2 seconds. The existing generated poses provide the motion; no new poses or
body-part rig were created. Live playback was started in the user's window,
and three successive state reads showed frames 1, 2 and 3 advancing.

Validation found 5,106 isolated pixels with alpha exactly 1/255 in the input.
Those pixels were cleared through four undoable edits in the live document.
Independent RGBA comparisons verified that every other pixel is identical
to its corresponding source-sheet panel, including the visible artwork.
All four foot baselines remain at `y=581` using alpha ≥128. The source PNG is
unchanged. Final live validation reports zero errors, nine warnings and one
note: the generated image retains off-palette colors, soft alpha and uneven
outline colors. These are retained to preserve the supplied artwork; this
playback test is not a completed native-pixel cleanup or Bevy integration.

Saved exports: [GIF](exports/characters/red-mage-idle-aseprite-v2.gif),
[2508 × 627 horizontal PNG atlas](exports/characters/red-mage-idle-aseprite-v2.png),
and [JSON](exports/characters/red-mage-idle-aseprite-v2.json). Atlas rectangles,
frame/tag/pivot metadata, the editable file header and all four GIF frame
durations were independently checked against the live document.

### Single magenta source for manual animation

The user's next prompt requested one complete, centered, front-facing subject
on an opaque, uniform `#FF00FF` background, visually simulating 256 × 256 pixel
art. Built-in imagegen used the original red mage reference to create the
[single source image](references/red-mage-idle-source-magenta-v1.png), retaining
the actual 1254 × 1254 output resolution. This is a static starting frame for
external idle-animation generation, rather than an animation or native 256px
export. It does not change the side-facing gameplay-art convention.

The generated background contained near-magenta variation. After explicit
user approval, a deterministic pixel pass replaced only pixels in the narrow
near-magenta background band with exact RGB `(255, 0, 255)`. The character was
not resized, repositioned or recolored outside that mask. The
[untouched generated original](references/red-mage-idle-source-magenta-v1-generated.png),
[generation prompt](references/red-mage-idle-source-magenta-v1-prompt.txt) and
[validation metadata](references/red-mage-idle-source-magenta-v1.json) are retained.
Independent lossless PNG decoding verified full opacity, exact key color on
every border/background-mask pixel and unchanged values outside the mask.
The subject bounds are `(324, 174)` through `(929, 1080)`, centered with more
than 10% empty margin on every side.

### Generated idle sheet v2

At the user's request, built-in imagegen produced a new four-frame idle sheet
using `references/red-mage-reference-v1.png` as its sole visual reference.
The [complete generation prompt](references/red-mage-idle-sheet-v2-prompt.txt)
records the design, centering and planted-feet requirements. The resulting
[transparent RGBA sheet](exports/characters/red-mage-idle-sheet-v2.png) is retained
without resizing, recoloring, alpha replacement or other post-processing.

Actual output dimensions are 1254 × 1254, with four equal 627 × 627 cells,
ordered left-to-right then top-to-bottom. This differs from the requested
1024 × 1024 size and is documented using the actual file dimensions.
[Atlas metadata](exports/characters/red-mage-idle-sheet-v2.json) supplies exact
rectangles, an `idle` tag, provisional 400ms timing and a shared feet pivot
`(345, 582)`. All four sole baselines measure `y=581` at alpha ≥128; visible
horizontal bounds differ by at most one pixel. The sheet was visually checked
against the reference for facing, silhouette, costume, complete limbs and
separation between cells. Soft alpha and faint near-transparent background
pixels remain in the generated raster; this is not a validated native pixel
grid. User review and any Pixelate cleanup are pending. It has not been loaded
into Bevy, and this generation does not replace the editable Aseprite drafts.

### Earlier 64 × 64 static study

Inspected at 1× and 8× and compared side by side with the source concept.
Palette analysis reports no off-palette colours. Validation reports no errors;
partial outline warnings remain on overlapping body-part edges and gold trim.
The source, PNG dimensions, atlas rectangle and feet pivot are checked.

This is a static pixel interpretation awaiting the user's visual review.
Animation, movement scale, background contrast and Bevy integration remain
pending. Gameplay collision remains 32 × 54 world units; the visual hat and
cape do not define collision. The prior apprentice study remains in Git history
and its existing files, but is no longer the current design direction.
