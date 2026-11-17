# Two-player LAN playtest

Automated TCP combat tests and two Bevy windows have passed connection/render
checks. This checklist still needs two people on separate physical computers.

- [ ] Same repository revision on both machines; host starts before client.
- [ ] Join using host Wi-Fi IP/TCP 24560; both HUDs show connected.
- [ ] Each player controls only their own actor; YOU and health/spell HUD are local.
- [ ] A third client cannot join an active two-player session.
- [ ] Both can run, double-jump, land and aim while the other moves.
- [ ] One airborne dash per flight; landing restores charge, cooldown still applies.
- [ ] Rapid jump/switch/cast taps execute once, without packet-driven repetition.
- [ ] Fireball impacts, damage and explosions agree on both screens.
- [ ] Gust returns incoming Fireballs toward their caster on both screens.
- [ ] Gust launch overrides incoming motion, lifts the target and forces 0.2s of
  motion, followed by the one-second slow and shorter/slower dash behavior.
- [ ] Defeat/respawn resets both views, health, spells and protection. Repeat at
  least five cycles without stale controls, duplicate actors or lingering shots.
- [ ] Unfocus either window: its controls stop, but the match continues.
- [ ] Compare aiming/dodging/interception between host and client; record client
  input delay/jitter separately from combat-rule mismatches.
- [ ] Host exits: client shows SESSION ENDED, never continued local combat.
- [ ] Client exits/unplugs: host ends play; no stuck movement/auto-casting.
- [ ] A wrong host address gives a connection error. Restart both peers after failure.
- [ ] Test two machines on Wi-Fi, recording device/OS and observed responsiveness.
- [ ] Separately test hotspot without internet before claiming support.
- [ ] Later repeat on physical Android/iOS devices with touch controls.

For failures record machines, host/client role, network arrangement, inputs,
expected/observed outcome and which view disagreed. Loopback evidence does not
establish two-machine, phone, hotspot or no-internet support.
