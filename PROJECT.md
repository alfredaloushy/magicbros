# Wild Magic Academy — live project board

This file is updated as work progresses. Open it alongside the conversation to follow the build.

## Current milestone: desktop two-player LAN prototype

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
| Core baseline | Accepted by user after local playtesting; tuning retained |
| Formatting and static checks | Pass: rustfmt, Clippy with warnings denied, shell syntax |
| Input timing, jump/dash and counter-return fixes | Verified by regression tests |
| Repeatable setup | `run.sh` and `check.sh` use a persistent project-local Cargo cache |
| Airborne dash charge | One per flight; landing restores charge; cooldown retained |
| LAN host/join | Implemented in Bevy; manual IP, TCP 24560; exactly two players |
| Shared combat state | Host-authoritative movement, spells, Gust status, health and respawn |
| Protocol/input correctness | Stable IDs, sequencing, respawn generations, bounded framing and timeouts |
| LAN verification | Real loopback combat tests pass; host/client GPU screenshots inspected |
| Physical network/device gate | Two computers on Wi-Fi and Android/iOS playtests still pending |

## Development projects

- `projects/core-gameplay/` — active: movement, combat, collision and respawn.
- `projects/art-animation/` — character sprites, spell effects, arena artwork and animation.
- `projects/multiplayer/` — implemented authoritative host, input transport and synchronization.
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
- Fourteen LAN tests pass, including six real TCP loopback cases and stalled-input
  handling. Total: 75 tests across core/harness, Bevy adapter and LAN package.
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
- User accepted the locally tested core baseline and requested the LAN milestone.
- Two actual Bevy windows connected on loopback. GPU captures were inspected:
  each has its correct YOU label, own HUD, peer actor and connected status.
- Loopback tests confirm shared Fireball damage/defeat/respawn, Gust launch/slow/
  reflection, disconnects, extra-player rejection and continued unfocused hosting.

## Next concrete action

The forward plan is in [ROADMAP.md](ROADMAP.md), from real Wi-Fi validation through
mobile controls, a polished combat slice, complete matches, expanded combat and
release hardening. Each milestone has an explicit pass condition.

Try the open host/client windows: click one to control that player. A/D moves,
Space jumps, Shift dashes, mouse + click aims/casts, 1/2 switches. LAN gameplay
continues while unfocused. Leaving either window ends the session.

To host, run `projects/multiplayer/run.sh host`; join with
`projects/multiplayer/run.sh join HOST_IP`. Use 127.0.0.1 for two windows locally.
Follow `projects/multiplayer/PLAYTEST.md` on two physical computers next and record
client responsiveness. Client prediction/interpolation are not yet implemented.

For solo practice, run `projects/core-gameplay/run.sh --bevy`. For the complete
download/build/check/launch flow, use `bevy-playtest.sh`; its output is logged to
`.cache/logs/bevy-playtest.log` inside the core project.

Code fixes are detailed in [FIXES.md](projects/core-gameplay/FIXES.md).

Phones, hotspot/no-internet validation and the broader release features remain
separate milestones. Loopback evidence does not mark those gates complete.
