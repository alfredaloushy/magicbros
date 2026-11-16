# Wild Magic Academy

Repository: **magicbros**.

A 2D magic arena brawler for friends, built with Rust and Bevy. Android and iOS
are the intended targets; the current prototype runs on desktop.

## Current prototype

- Running, double jump, directional dash and solid platforms.
- One airborne dash per flight, recharged on landing, with a separate cooldown.
- Quick-cast Fireball and Gust, damage, protection, defeat and automatic respawn.
- Gust projectile returns, strong airborne launch and a temporary movement slow.
- A practice dummy with switchable Fireball/Gust for local playtesting.

Gust currently uses a fixed launch with upward lift and a 0.2-second forced
motion phase, followed by a one-second 30% reduction to run/dash speed. All balance
values are provisional and centralized in the core project's tuning resource.

## Run and verify

Requires Rust 1.89+ and a supported desktop graphics driver. From a normal desktop
terminal, download dependencies, compile, check and launch:

```sh
cd projects/core-gameplay
./bevy-playtest.sh
```

After setup, use `./run.sh --bevy` to launch and `./check.sh` for the offline
core/harness checks. The launcher handles the local Cargo cache and installed
window/graphics library paths on NixOS.

See the [core README](projects/core-gameplay/README.md) for controls, limitations,
and separate Bevy adapter checks. Follow [PLAYTEST.md](projects/core-gameplay/PLAYTEST.md)
for manual validation.

## Project layout

| Directory | Purpose |
| --- | --- |
| `projects/core-gameplay/` | Active Bevy ECS simulation and desktop prototypes |
| `projects/multiplayer/` | Next milestone: two-player local Wi-Fi |
| `projects/art-animation/` | Asset and impact animation workstream |
| `projects/ui-audio/` | UI, feedback, sound and touch-control workstream |
| `projects/mobile-builds/` | Android/iOS builds and device validation |
| `documents/` | Design documents, source illustrations and generation scripts |
| `wild-magic-academy/` | Earlier Godot experiment, preserved for reference |

[PROJECT.md](PROJECT.md) tracks progress and [projects/README.md](projects/README.md)
describes ownership and handoffs. The design draft predates the Bevy selection;
later confirmed decisions and implemented behavior are recorded in project notes.

## Version control workflow

Keep code, tests, project briefs, Cargo lockfiles, design documents and source
artwork together in this repository. Exclude build targets, dependency caches,
downloaded engine binaries, document review exports and credentials.

At each meaningful milestone, run the relevant checks, review the staged changes,
commit with a descriptive message and push to the configured remote branch.
The user has requested continuing Git progress updates. Do not force-push or
overwrite remote history; inspect and reconcile existing work first.

No license or distribution rights are assigned by this repository setup.
