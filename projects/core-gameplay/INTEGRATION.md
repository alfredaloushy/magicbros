# Shared integration contract — prototype 0.1

Core gameplay owns authoritative mechanics. Other projects adapt input and
presentation; they should not reimplement damage, movement or spell outcomes.

## Inputs and state

`PlayerCommand` carries movement (X for run, Y for dash direction), an absolute
`aim_world` target, held `cast`, and single-tick `jump`, `dash`, `switch` and
optional absolute `select`. Both adapters use `BufferedInput`: write movement,
aim and edge actions into `command`, set `held_cast` from held controls, and call
`press_cast()` on a fresh cast press. `take_tick()` emits one command and clears
pending edges/presses while retaining held input. This preserves a press/release
that occurs between simulation ticks. Reset the buffer on focus loss or
disconnected control ownership.

`Simulation::step` runs one fixed 1/120 second tick by default. `Tuning.tick` is the
step duration; use the same duration for any adapter accumulator. Renderer
frequency does not change the step. Both adapters use `FixedClock`, which caps
frame stalls to 100ms and catch-up work to 240 ticks per frame, discarding excess
backlog. Reset the clock on pause. Invalid times are ignored. Initial tuning is
validated; local commands are sanitized before entering the schedule.

`Simulation::snapshot()` supplies actor state, body bounds and position, projectile
state, arena platforms, transient effects and bounded diagnostic counters. It
clones this small world for the prototype, not a production networking format.
Bevy `Entity` values are process-local and **must not be used as network IDs**.
`Actor.slot` currently identifies the local player (0) and dummy (1); `generation`
increments at respawn for stale-state rejection in a future protocol.

There is no replay, rollback, prediction, serialization or transport yet. The
simulation is fixed-step but cross-machine deterministic lockstep is not claimed.

## Art and animation

- World units: Y up, camera canvas 1280 × 720, origin at arena center.
- Player collision: width 32, height 54; position is the center, feet are Y minus 27.
- Platform colliders are solid `center`/`half` rectangles. Art may extend beyond
  collision only when readability remains clear.
- Fireball collision radius is currently 7; blast radius is configured in tuning.
- Use `Actor.aim`, body velocity/grounded, dash state, health, hit flash, protection
  and generation to select animation. Cosmetics must not modify collision.
- `Actor.air_dash_available` is a single charge restored by ground/platform contact
  or respawn. Dash cooldown is independent and still applies after landing. A dash
  that launches from the ground consumes the flight's charge; walls/ceilings and
  cooldown expiry do not refill it. Include this field in future replicated state.
- `Actor.slow_left` is the remaining Gust slow duration. While positive, run and
  dash speeds use `Tuning.gust_slow_speed_multiplier` (0.70). Dash duration stays
  fixed, reducing its reach by 30% for a fully slowed dash. The multiplier updates
  each tick, including mid-dash application/expiry. Outside the initial Gust launch,
  jump, dash cooldown and airborne-charge rules remain unchanged.
  In-range Gust hits carry `Hit.slow_duration` (1.0 second);
  protected/dead targets ignore it, repeats refresh without stacking, and defeat
  and respawn clear it. Replicate remaining duration with authoritative state.
- `Actor.gust_launch_left` is the initial forced-launch timer (0.2 seconds).
  Gust sets `Hit.launch` and carries an absolute launch velocity in `Hit.impulse`:
  aim X × 850, aim Y × 850 with a minimum upward speed of 500. Applying it replaces
  body velocity, clears previous impulses, cancels the active dash and clears
  grounded state. Movement is ballistic under gravity during this timer, ignoring
  steering/jump/dash inputs while still permitting casting. Terrain collision
  remains active. Both launch/slow timers clear on defeat/respawn; protection
  blocks application. Other hits retain additive impulse behavior. Replicate this
  timer alongside velocity to avoid clients steering out of the authoritative hit.
- `EffectKind` describes Fireball explosions and Gust direction/range. Replace
  the placeholder renderer, preserving those gameplay dimensions.
- Establish an in-motion pixel density test before producing full sprite sets.

## Multiplayer

- Initial design: one authoritative host over local Wi-Fi; host departure ends play.
- Introduce stable player/projectile IDs, tick/sequence numbers, validated commands,
  acknowledgement and state serialization before sending state between machines.
- Resolve all movement/collision, casting, reflection, damage and respawn on the host.
- Remote actors need their own input submissions; `Simulation::step` currently
  accepts only local-player input and the dummy has an automatic input system.
  Replace the dummy input source and extend command submission before networking.
- Keep original caster and current owner separately. Reflections preserve the
  former and change the latter. The first return targets the original caster.
  Provisional counter-return rule: when that original caster uses Gust, return to
  the previous owner so the spell flies away from the defender. Review this rule
  before finalizing multiplayer combat; kill attribution remains unresolved.
- A generation change should reset interpolation/prediction at respawn. Clear
  stale input, projectile visuals and old damage feedback on that transition.
- Existing Godot netcode may inform behavior but is not the Bevy network backend.

## UI and audio

- Local HUD reads health, selected spell, cast/dash cooldowns and respawn delay.
  Snapshot includes tuning so health-bar fractions and tick labels remain correct.
- Show the temporary slow timer on affected actors and local HUD. The G key
  changes the practice dummy spell only; it is not a release gameplay control.
- Opponent health is shown temporarily after damage (`health_reveal`); opponent
  selected spells are not continuously exposed.
- Future touch controls must output the same `PlayerCommand` in world coordinates.
- Current effects/counters support simple visuals/debugging. Add stable event IDs
  or a per-tick event stream before audio, network replay or prediction to avoid
  duplicated sounds and effects. Do not treat cumulative diagnostic counters as
  the final gameplay event API.

## Mobile builds

- Validate the full Bevy renderer adapter before Android/iOS work. The software
  runner is a desktop test aid and is not the mobile rendering architecture.
- Replace the CPU preview texture with native sprites/VFX before performance work.
- Keep the same gameplay unit scale and tick rules when adapting camera/viewport.
- Test touch ergonomics, app pause/resume, aspect ratios, safe areas and local
  network permissions on devices. No device performance claim exists yet.

## Change discipline

Document changes to this contract here and update the corresponding project brief.
Tune values in `src/tuning.rs`; avoid copying mechanics constants into adapters.
Current prototype decisions are listed in the core README and should be reviewed
before release features build on them.
