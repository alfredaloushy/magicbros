# Red mage — original concept study

The user requested this character be drawn solely from the project's original
[concept sheet](../../documents/.build/concept-art.png), replacing the earlier
ANNO: Mutationem direction. The specific reference is the red wizard in the
lower-left lineup (source region x=55, y=614, width=146, height=174).

## Design and files

The sprite follows the oversized floppy red hat with a gold star and brim,
brown fringe, small pale face, short red cape with gold edging, dark tunic,
square gold belt buckle and brown boots. It uses the reference's compact
proportions and resting pose. Decorative stitching and fine hat texture are
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
