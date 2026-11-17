# Aseprite source and Bevy handoff

## Files

Keep editable `.aseprite` documents under `source/characters/` and `source/maps/`.
Keep the corresponding game PNGs and JSON metadata under `exports/characters/`
and `exports/maps/`. Commit both source and game exports. Put outside reference
images and temporary review reports in the repository's ignored `.cache/`.

Work through the live Aseprite connection so the user sees the changes. Read the
open document before editing, preserve its palette and layers, preview the
result, validate it, save the source and export after the final edit. An export
does not save the document.

## First character convention

These values are provisional until the first in-motion scale test:

| Item | Starting value |
| --- | --- |
| Canvas | 64 × 64, RGB, transparent background |
| Facing | Right; left-facing treatment is still to be reviewed |
| Feet pivot | `(32, 60)`, from the top-left boundary of every frame |
| Initial art scale | One world unit per art pixel |
| Gameplay body | Existing 32 × 54 world units; feet 27 units below actor centre |
| Frame alignment | Constant canvas and pivot; no per-frame trimming or rotation |
| Filtering | Nearest neighbour |
| Colour/alpha | Fixed palette per character; hard transparent or opaque pixels |

The rig separates head, torso, near/far arms and legs. Scarf, hair, magic and
other independently moving parts get their own layers when the design needs
them. Hide sketch/reference layers in game exports. The blank template supplies
structure; the concept document supplies the current provisional palette.

The sprite's pivot is not its collision box. The future renderer positions the
feet at `Body.position.y - 27`, using the exported pivot and its agreed art scale.
Cosmetic poses and spell effects must not move the physics body or extend range.

## Animation after design review

Start with idle and run, then jump/fall/land, dash, Fireball cast, Gust cast,
hit/launch and defeat/respawn. Tag each completed sequence, record its per-frame
milliseconds, and specify whether it loops. Do not label a single concept pose
as a completed movement animation.

Plan casting to match the simulation's existing timing. Gust's 0.2-second forced
launch and one-second slow remain gameplay rules; animation follows their state.
Visible recoil, tumble and spell impact can be added without changing the force.

## Exports

For the current static concept, export one untrimmed 64 × 64 PNG plus its JSON
atlas with **zero padding**. Verify the atlas rectangle and source size agree
with the actual PNG. There is no adjacent frame to bleed into in this export.
An 8× nearest-neighbour PNG is for visual review only.

For animation, use fixed-size untrimmed frames and preserve frame durations,
tags and the feet slice. Set padding and atlas handling together when the Bevy
importer is implemented; do not assume an exported padded rectangle is the
logical frame size. Verify the files rather than relying on the tool summary.

The Bevy adapter currently displays a CPU-rendered placeholder canvas. Loading
these assets, using native sprites/atlases and selecting animations from actor
state still require a presentation change. The PNG and JSON are a portable
handoff, not evidence that import or gameplay presentation has been tested.

## Map design after the character study

Begin with a small academy courtyard sample beside the approved wizard. A
32 × 32 tile grid is a starting proposal; confirm it in the scale test. Keep
background, distant architecture, playable platforms, foreground decoration and
collision guides separate. Make repeatable terrain into a tileset instead of
painting every platform into one image.

Draw map mock-ups in the game's side-on 2D view, following the original project
concept sheet. The user superseded the earlier ANNO: Mutationem direction. A map PNG provides appearance, not playable collision, spawns or pit
rules. Those need separate level data and core validation when implemented.

## Earlier concept 01 review (superseded)

The visible sprite was inspected enlarged and as a flat silhouette. The face
and spell clusters were corrected during review. Aseprite validation passes
with no errors and eight outline style warnings. These are retained because
the rig's overlapping joints use partial outlines and the luminous spell uses
cyan edges instead of a dark contour. The hidden construction sketch is kept
for iteration and excluded from game exports.

The draft still needs user review, a background contrast test and movement
testing. The small face and simplified cloth shading are deliberate limits of
this 64 × 64 study; it is not a finished production character.
