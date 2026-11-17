# Wild Magic Academy development projects

Each directory owns a development area, its brief, and its deliverables. The core
and desktop LAN prototype are implemented; device playtests remain. These are local projects;
no new Codex conversations or remote repositories have been created.

| Project | Owns | Depends on |
| --- | --- | --- |
| [Core gameplay](core-gameplay/README.md) | Simulation, movement, collisions, quick-cast combat, health and respawn | Shared design |
| [Art and animation](art-animation/README.md) | Sprites, animation, spell effects, arena visuals | Stable gameplay dimensions and in-motion style test |
| [Multiplayer](multiplayer/README.md) | Local Wi-Fi host, commands, state synchronization and reconnect behavior | Core playtest gate and stable command contract |
| [UI and audio](ui-audio/README.md) | HUD, controls, menus, sound and feedback | Core state and cast/impact/defeat signals |
| [Mobile builds](mobile-builds/README.md) | Android/iOS builds, device testing and performance | Validated Bevy renderer and input adapter |

## Shared reference and authority

Read `../documents/Wild_Magic_Academy_Design_Draft.docx` and its illustrated PDF.
Later confirmed decisions supersede that draft: Bevy/Rust is the engine direction;
Android and iOS are targets, with desktop used for fast early testing.
Balance values are provisional. Changes to shared contracts must be documented
in [core integration notes](core-gameplay/INTEGRATION.md).

## Sequence

1. Verify local movement and quick-cast combat in the core lab.
2. Playtest and tune before expanding mechanics.
3. Connect two players over local Wi-Fi and validate outcomes on physical devices.
4. Integrate production assets, UI/audio and mobile packaging through their projects.

Art exploration and mobile build feasibility may proceed independently, but do not
block the core on final artwork or expand this milestone into the full release.
