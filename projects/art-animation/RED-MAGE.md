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

Inspected at 1× and 8× and compared side by side with the source concept.
Palette analysis reports no off-palette colours. Validation reports no errors;
partial outline warnings remain on overlapping body-part edges and gold trim.
The source, PNG dimensions, atlas rectangle and feet pivot are checked.

This is a static pixel interpretation awaiting the user's visual review.
Animation, movement scale, background contrast and Bevy integration remain
pending. Gameplay collision remains 32 × 54 world units; the visual hat and
cape do not define collision. The prior apprentice study remains in Git history
and its existing files, but is no longer the current design direction.
