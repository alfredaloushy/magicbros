# Wild Magic Academy — UI and audio project

Status: separate workstream prepared; the core lab uses a temporary HUD only.

## Brief

Create readable combat feedback and desktop/mobile input adapters without
duplicating mechanics. Begin with local health, spell selection, aim, cast/dash
readiness and respawn feedback. Test touch movement, aiming and switching early.

## Owns

HUD and menu layouts, touch controls, accessibility/feedback intensity options,
spell/impact/defeat audio and eventual room flow. Production audio and polished
menus are outside the current core milestone.

## Inputs and deliverables

Use [the core contract](../core-gameplay/INTEGRATION.md) and output `PlayerCommand`.
Opponent health fades after damage; do not persistently expose opponent loadouts.
Establish a stable gameplay event stream with core before attaching sound effects.
Deliver UI layouts, mapped inputs, audio assets with provenance and a phone-size
usability checklist. Keep engine implementation details out of player-facing UI.

First gate: aim, jump/dash, cast and spell switching can be performed reliably
without accidental actions or unreadable combat information.
