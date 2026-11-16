# Core prototype fixes

## Corrected behavior

- Quick click/release between ticks is retained and casts once. Held casting still
  repeats at cooldown. Both desktop adapters use the same buffer and fixed clock.
- Dash no longer spends a jump that it immediately overrides. External vertical
  knockback survives subsequent dash ticks.
- Fireball starts at the caster and sweeps its entire path, avoiding shots that
  begin beyond a close thin obstacle or overlapping target.
- A projectile's original caster can counter-return it toward its previous owner,
  rather than accidentally aiming it back at themself. This is documented as a
  provisional prototype interaction, without adding scoring rules.
- Nonfinite/oversized local inputs cannot poison aim or movement. Invalid initial
  tuning is rejected, and client catch-up work is bounded.
- Window resize recalculates aim from the current viewport and stored pointer
  position. Raster output and input use identical integer letterbox offsets.
  The Bevy adapter uses its camera transform for pointer-to-world conversion.
- HUD health bars and tick labels use configured values. Rendering a temporarily
  missing local actor no longer indexes a nonexistent first actor.
- Desktop redraw requests are paced. Smoke tests count actual presentations,
  reject invalid options and fail on early interruption or presentation errors.
- Launch/check scripts use a persistent writable project-local Cargo cache;
  initial setup no longer depends on `/tmp/wma-cargo` surviving.

## Validation

`./check.sh` passes: 58 tests, strict Clippy, formatting and a display-free preview.
The tests cover the regressions above and a 12,000-tick combat soak.

## Gust movement slow

In-range Gust hits now slow running and dashing by 30% for one second. Repeated hits refresh
the timer without multiplying strength or adding durations. Gust retains its
0.65-second casting cooldown; no extra immunity window is added. Dash remains
available with reduced speed/reach and its existing duration, cooldown and air
charge rules. The slow updates during active dashes and normal speed returns on
expiry. The slow itself does not change jump strength. Protected/dead targets ignore the effect;
defeat and respawn clear it. A visible timer shows recovery, and G switches the
dummy spell so the local player can try being hit by Gust. Seven additional cases
cover movement recovery, refresh, jump/dash escape, Fireball exclusion, respawn,
dummy Gust testing and tuning validation; existing range/protection cases also
verify slow eligibility.
Three dash follow-up cases verify reach in all directions, recovery after expiry,
and applying/removing the slow during an active dash.

## Strong, consistent airborne Gust launch

Gust now replaces incoming velocity/old impulses instead of adding a push that
opposing run or dash velocity could cancel. Launch strength is 850 along aim,
with at least 500 upward speed. A 0.2-second ballistic phase prevents movement,
jump or dash inputs from negating the hit; an active dash is canceled, casting
remains available, and terrain still blocks travel. Normal controls return with
the remaining slow. HUD labels this initial phase as LAUNCHED. Six additional
regressions cover incoming motion from both sides, forced lift/displacement,
control recovery, protection, terrain collision and defeat/respawn cleanup.

## Single airborne dash charge

User playtest feedback confirmed the existing controls/cooldown feel good and
requested one airborne dash per flight. Added a charge restored on floor/platform
contact and respawn; cooldown remains independent. Wall/ceiling contact and the
timer cannot recharge it. Ground-launch dashes consume the charge for that flight.
Horizontal ground dashes preserve support and remain reusable after cooldown.
The HUD indicates when landing is required. Eight regressions cover cooldown
expiry, platform landing, cooldown retention, ground dashes, ground launch,
wall/ceiling contact and respawn.

## Bevy compilation and startup follow-up

- Approved escalation enabled dependency downloads and desktop launch. Full Bevy
  0.18.1 is cached and its lockfile is retained.
- Fixed Wayland compilation by enabling winit runtime loading.
- Fixed GPU discovery by adding installed Vulkan loaders, GL dispatch and
  `/run/opengl-driver/lib` to the NixOS runtime library search path.
- Fixed native adapter Clippy findings with a named view parameter type and
  fixed-size pixel chunks.
- Full Bevy build, three adapter tests and strict Clippy pass. It opened the
  playtest window on NVIDIA GTX 1650 / Vulkan, driver 595.71.05.
- Human playtesting remains pending; the window is available for feedback.

Ordinary sandbox network/display restrictions still apply. Use approved
escalation or run `./run.sh --bevy` from a normal desktop terminal.
