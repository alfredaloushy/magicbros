# Core lab playtest gate

Run `./run.sh`. These checks require a person playing the window. Record each
result as pass/fail, plus reproduction steps or balance preferences. No human
playtest has been claimed by the implementation checks.

- [ ] Move left/right, stop and reverse. Acceleration and braking feel responsive.
- [ ] Jump to both side platforms and the upper platform. Double jump works once
  in air; landing restores both jumps. No sticking, sinking or tunneling.
- [ ] Dash horizontally, diagonally and upward. Without movement input, dash uses
  aim. Terrain stops movement and rapid Shift presses cannot bypass cooldown.
- [ ] Dash once in the air, then try again after cooldown while still airborne:
  the second attempt is blocked. Land on the floor or a platform, then dash again.
- [ ] Landing restores the air charge without bypassing cooldown. Side-wall and
  ceiling contact do not recharge it. An upward dash from the ground consumes the
  flight's charge; horizontal ground dashes keep their cooldown behavior.
- [ ] Aim left/right/up/down while moving. Reticle and Fireball travel agree.
- [ ] Use 1/2, Q and right mouse to switch. Switching causes no unwanted casts.
- [ ] Hit the dummy with Fireball. Impact damages and pushes it; four ordinary
  direct hits defeat a full-health unprotected target.
- [ ] Fire into the ground nearby. Own blasts deal reduced damage and can push you.
- [ ] Stand roughly 120–180 world units from the dummy, press T, select Gust, aim
  at the incoming Fireball and cast before it hits. The shot visibly returns and
  can damage its original caster. Shots behind you or outside range are unaffected.
- [ ] Cast Gust near the dummy. It pushes without direct health damage.
- [ ] With dummy Gust enabled (G, T), approach within range while running toward
  it, running away and dashing toward it. Every hit produces a strong launch away
  with upward lift, rather than incoming motion reducing the knockback.
- [ ] During the first 0.2 seconds, movement/jump/dash cannot cancel the launch;
  casting still works. Control resumes afterward with the remaining one-second
  slow. A wall blocks the launch rather than allowing terrain penetration.
- [ ] Gust hit displays a one-second slow timer; targets behind/outside range are
  unaffected. Running returns to normal when the timer ends.
- [ ] Press G to change dummy spell to Gust, enable its casts with T, and approach
  within range. While slowed, running and dash speed drop by 30%, with a shorter
  dash distance. Jumping works after the initial launch phase. Repeated hits refresh
  the slow timer without stacking its strength.
- [ ] Compare a slowed dash against a normal dash in the same direction. Duration
  and cooldown stay the same. Speed and reach return to normal when the slow ends.
- [ ] Leave Gust range and let the slow expire. Defeat/respawn clears any slow.
- [ ] Let the dummy defeat you. During the delay you cannot move or cast; roughly
  two seconds later health and spells reset. Protection briefly blocks damage and
  ends immediately if you attack.
- [ ] Repeat several defeat/respawn cycles. No duplicate actors or stale shots.
- [ ] Resize the window. Aiming remains aligned with the letterboxed arena.
- [ ] Change window focus while holding movement/fire. Gameplay pauses; returning
  does not leave movement or firing stuck on.
- [ ] Tap and release cast quickly; it still casts when the press occurs between
  simulation ticks. Repeat while the window runs at a high refresh rate.
- [ ] Jump during an active dash. The ignored jump does not consume an air jump.
- [ ] Counter-return an already reflected projectile: the original caster's Gust
  sends it back toward its previous owner, away from the defending caster.
- [ ] Stand against a thin wall and fire toward it. Fireball cannot spawn beyond it.

## Gate for the multiplayer project

Do not call this a validated two-player prototype yet. Once local gameplay passes,
add host/client input and state adapters. Then test on two physical devices over
the same Wi-Fi: movement responsiveness, agreed hits, Gust returns, defeat and
respawn. Include a hotspot/no-internet session before claiming that arrangement.
