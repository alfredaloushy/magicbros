# Wild Magic Academy — live project board

This file is updated as work progresses. Open it alongside the conversation to follow the build.

## Current milestone: core gameplay prototype

| Part | Status |
| --- | --- |
| Read design document and earlier Godot prototype | Complete |
| Separate development projects | Complete: five project directories with scope/handoff briefs |
| Bevy ECS simulation | Implemented; 48 gameplay tests pass |
| Run, double jump, directional dash, solid platforms | Implemented; regression tests pass |
| World-space aiming and spell switching | Implemented |
| Fireball, explosions, damage, knockback | Implemented; regression tests pass |
| Gust push and projectile return | Implemented; regression tests pass |
| Gust knockback tuning | Fixed 850 launch, minimum 500 upward lift, 0.2s forced motion; 190-unit range |
| Gust movement slow | In-range hits reduce run/dash speed and dash reach by 30% for 1 second; timer shown |
| Defeat, protection and automatic respawn | Implemented; regression tests pass |
| Offline desktop test window and basic HUD | Builds; CLI/render/input regressions pass; snapshot reviewed |
| Full Bevy renderer adapter | Bevy 0.18.1 downloaded; build, three adapter tests and strict Clippy pass |
| Bevy window startup | Running on NVIDIA GTX 1650 / Vulkan after graphics library fix |
| Playtest checklist and integration notes | Complete; human playtest remains pending |
| Formatting and static checks | Pass: rustfmt, Clippy with warnings denied, shell syntax |
| Input timing, jump/dash and counter-return fixes | Verified by regression tests |
| Repeatable setup | `run.sh` and `check.sh` use a persistent project-local Cargo cache |
| Airborne dash charge | One per flight; landing restores charge; cooldown retained |

## Development projects

- `projects/core-gameplay/` — active: movement, combat, collision and respawn.
- `projects/art-animation/` — character sprites, spell effects, arena artwork and animation.
- `projects/multiplayer/` — authoritative host, input transport and synchronization.
- `projects/ui-audio/` — HUD, menus, feedback, audio and eventual touch controls.
- `projects/mobile-builds/` — Android/iOS packaging, device testing and performance.

Shared design reference: `documents/Wild_Magic_Academy_Design_Draft.pdf`.
Later decisions in the conversation supersede the draft: Bevy is selected; Android and iOS are targets.

## Current constraints

Downloads and desktop startup now work through approved escalation. Sandboxed
network/display access remains restricted; additional downloads or GUI launches
may require approval. Full Bevy 0.18.1 and its locked dependencies are cached.
The core uses Bevy ECS; both the full Bevy adapter and the winit/softbuffer harness
exercise that same simulation with placeholder graphics.
The existing Godot project remains available as a reference.

## Latest verification

- 58 tests pass: 48 gameplay, five shared input/clock, five desktop CLI/render.
- Three additional full-Bevy adapter tests pass; its strict Clippy check passes.
- Includes a 12,000-tick combat soak and regressions for missed clicks, thin-wall
  shots, dash/jump consumption, dash knockback, counter-returns and resize aiming.
- `projects/core-gameplay/check.sh` passes from the project-local Cargo cache.
- Offline desktop binary builds and generates a 1280 × 720 preview successfully.
- Preview: [core gameplay lab](projects/core-gameplay/preview.png).
- Preview inspected: arena, player/dummy, health, spell selection, controls and training status are legible.
- Full Bevy compiled successfully and created the playtest window using NVIDIA
  GeForce GTX 1650, driver 595.71.05, Vulkan. Process remains open for playtesting.
- Fixed Wayland build-time library discovery with runtime loading, and fixed
  NixOS GPU discovery by exposing the Vulkan loader and graphics driver libraries.
- User reported the prototype seems in order and dash cooldown feels good.
  New requested rule: one airborne dash per flight, restored on landing.
  New charge behavior awaits a follow-up playtest; individual checklist results
  have not been recorded as completed.

## Next concrete action

Try the open Bevy window: A/D to move, Space to jump, Shift to dash, mouse + click
to aim/cast, 1/2 to switch, and T to enable dummy fire. Follow the core project's
`PLAYTEST.md` and report any failures for fixes before beginning local Wi-Fi work.

To relaunch, run `projects/core-gameplay/run.sh --bevy`. For the complete
download/build/check/launch flow, use `bevy-playtest.sh`; its output is logged to
`.cache/logs/bevy-playtest.log` inside the core project.

Code fixes are detailed in [FIXES.md](projects/core-gameplay/FIXES.md).

Begin the local Wi-Fi milestone after the local core playtest gate passes.
