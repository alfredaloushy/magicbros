# Wild Magic Academy development roadmap

Starting point: accepted desktop combat baseline and implemented two-player LAN
prototype. The last implementation checkpoint passed 75 automated tests and
verified two connected Bevy windows on one computer. Two-machine Wi-Fi, phones,
production assets and full matches are still ahead.

This is the proposed sequence from that checkpoint. Complete each pass condition
before expanding dependent work; revise the plan using recorded playtest results.

## 1. Validate real LAN play

Owners: multiplayer and core gameplay.

- [ ] Play on two physical computers using the same Wi-Fi and repository revision.
- [ ] Check responsive movement, aiming, Fireball, Gust returns/launch/slow and
  repeated defeat/respawn agreement using the multiplayer playtest checklist.
- [ ] Record client delay/jitter, disconnect behavior and focus-loss behavior.
- [ ] Add interpolation/prediction or revise transport only where measured issues
  justify it; retain host authority over combat outcomes.
- [ ] Separately test hotspot/no-internet operation and record the arrangement.

**Pass condition:** two players can repeatedly fight and respawn with consistent
outcomes and acceptable responsiveness on the actual network. Capture evidence
per network/device arrangement rather than extending the loopback claim.

## 2. Establish the mobile foundation

Owners: mobile builds, core presentation and UI/audio.

- [ ] Confirm available Android/iOS toolchains and physical test devices early.
- [ ] Replace the temporary full-frame CPU preview with native Bevy sprites,
  camera, terrain and minimal HUD; placeholder art is sufficient for this step.
- [ ] Get a small playable build running on Android and establish iOS build/device
  feasibility in the same milestone.
- [ ] Add touch movement, aim/cast, jump, directional dash and spell switching
  through the existing input contract.
- [ ] Handle screen sizes, safe areas, app pause/resume and network permissions.
- [ ] Repeat the two-player loop on phones, including Android/iOS cross-play when
  both builds are available; record performance and input usability.

**Pass condition:** the existing two-spell prototype is playable on the target
devices with usable controls and agreed measured performance targets. No mobile
support is considered verified solely from a desktop build.

## 3. Produce one polished combat slice

Owners: art/animation and UI/audio, with core integration.

- [ ] Agree sprite scale, pivots and the storybook/pixel art treatment in motion.
- [ ] Build one wizard with idle/run/jump/dash/cast/hit/defeat animation.
- [ ] Replace Fireball/Gust placeholders with readable spell and impact effects.
- [ ] Add Gust recoil/tumble, contact burst and launch trail while preserving its
  accepted force. Connect presentation to confirmed gameplay events.
- [ ] Add spell/impact sounds and restrained optional camera shake/vibration.
- [ ] Give the simple arena a coherent visual pass and check phone readability.

**Pass condition:** one arena, one wizard and two spells feel coherent on phones
and remain readable during networked combat. This establishes the asset pipeline
before producing the full roster. Hit pause remains a separate core/network
timing decision if later playtests show it is needed.

## 4. Make a complete friend-playable alpha

Owners: UI/audio, multiplayer and core gameplay.

- [ ] Replace command-line-only connection with create/join screens and a lobby.
- [ ] Add ready state, room settings, match timer, kills/deaths, results and rematch.
- [ ] Specify tie handling, reflected-kill credit, fall credit and simultaneous
  defeat rules before implementing their scoring outcomes.
- [ ] Keep the initial alpha on Fireball/Gust while testing the whole match loop.
- [ ] Give friends an installable test build and record reproducible feedback.

**Pass condition:** two friends can open the app, join, finish several matches and
rematch without developer assistance. Network failure is understandable and does
not leave a player silently playing a different match.

## 5. Complete base combat and resources

Owner: core gameplay, with networking and presentation updates for every feature.

- [ ] Add Stone Fist and Frost Bolt individually, testing each before the next.
- [ ] Review Frost's identity alongside Gust's accepted slow so their roles differ.
- [ ] Add quick/charged casts and the documented overcast consequences; resolve
  remaining open spell behavior rather than inventing it implicitly in code.
- [ ] Add mana/regeneration, health recovery and separate supply pickups.
- [ ] Add signature-spell selection and the replaceable arena spell slot.
- [ ] Tune damage, resource costs, cooldowns and counterplay through playtests.

**Pass condition:** all four base spells are useful, charging is controllable on
touch screens, and resources/loadouts agree across players and respawns.

## 6. Add spell fusion one pairing at a time

Owners: core gameplay and art/animation, with authoritative network support.

- [ ] Define fusion input, wind-up, mana debit/refund and cooldown behavior first.
- [ ] Start with Fireball + Gust: Flame Tornado.
- [ ] Add Magma Bomb, Steam Burst, Orbiting Stones, Ice Jet and Glacier Wall.
- [ ] Verify terrain placement, projectile defense, visibility, escape movement
  and interactions with Gust/reflections for each pairing.
- [ ] Prevent loadout swaps from bypassing fusion cooldowns.

**Pass condition:** all six pairings work predictably on both devices, have clear
counterplay and readable effects, and do not introduce combat-state disagreements.

## 7. Build the intended private-room experience

Owners: all workstreams.

- [ ] Expand to 4–6 players and test host performance, bandwidth and fairness.
- [ ] Build Training Courtyard routes, supply locations and lethal pit.
- [ ] Add the breakable bridge and its rebuilding warning; replicate terrain state.
- [ ] Add nearby-room discovery alongside the manual-address fallback.
- [ ] Complete names/player colors, starter appearance choices and combat HUD.
- [ ] Finalize late join, disconnect/reconnect and safe-spawn behavior from the
  design; keep host departure ending the room until migration is separately built.

**Pass condition:** a group can reliably play complete private LAN matches on
the intended arena, with readable action and consistent shared terrain/combat.

## 8. Harden and prepare the first release

Owners: mobile builds and all workstreams.

- [ ] Run extended Android/iOS and mixed-device sessions on supported networks.
- [ ] Profile frame time, memory, bandwidth, battery and host load on real devices.
- [ ] Fix balance, crashes, reconnect issues and touch usability findings.
- [ ] Add onboarding and settings for sound, visual feedback and accessibility.
- [ ] Verify asset provenance, signing/build reproducibility and release material.
- [ ] Distribute a closed beta, resolve findings, then prepare store submissions
  for explicit publication approval.

**Pass condition:** agreed device/performance targets and acceptance checklists
pass, with no unresolved release-blocking defects in normal play.

## Work that can overlap

Art style exploration and one-character animation can proceed while real LAN and
mobile feasibility checks run. UI prototypes can proceed alongside touch tests.
Full asset production depends on an accepted in-motion style; additional spells
depend on usable mobile controls and reliable combat replication.

Online rooms, host migration, order-dependent fusions, magical species, achievement
cosmetics and additional arenas remain later scope rather than blockers for the
first private-LAN release.

## Progress discipline

Keep outcomes in PROJECT.md and the relevant workstream checklist. At each
meaningful milestone: run appropriate checks, record actual device/playtest
evidence, review changes, commit and push to magicbros. Keep credentials, caches,
build output and signing material out of Git.

Immediate next action: review the first Aseprite wizard concept, then animate
idle/run and explore a courtyard sample. The user paused milestone 1's physical
Wi-Fi test until a second computer is ready; its pass condition remains pending.
