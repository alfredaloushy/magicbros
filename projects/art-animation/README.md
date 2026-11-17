# Wild Magic Academy — art and animation project

Status: active. A first character concept and an editable Aseprite template are
prepared for user review. Animation, map artwork and engine integration are pending.

## Current work

The user supplies art direction and Codex draws in the live Aseprite window.
The first requested style reference is ANNO: Mutationem. Start with one original
apprentice mock-up, settle the design, then animate it and test a courtyard sample.

- [Concept brief](CONCEPT-01.md): design intent, reference and review gate.
- [Editable wizard concept](source/characters/wizard-concept-01.aseprite).
- [Blank character template](source/characters/wizard-template.aseprite).
- [Transparent sprite](exports/characters/wizard-concept-01.png),
  [atlas metadata](exports/characters/wizard-concept-01.json) and
  [enlarged preview](exports/characters/wizard-concept-01-preview.png).
- [Aseprite workflow](ASEPRITE-WORKFLOW.md): source/export conventions and the
  planned character, animation and map handoff to Bevy.

This is one static 64 × 64 design study. It has not been approved as the final
character style, animated or imported into the current renderer.

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
