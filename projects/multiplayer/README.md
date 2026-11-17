# Wild Magic Academy — two-player LAN prototype

The Bevy app supports one authoritative host and one joining player, using the
accepted movement, single-charge air dash, Fireball, Gust returns/launch/slow,
health and respawn mechanics. Solo practice remains available.

## Same computer

From the repository root, use two terminals (start the host first):

```sh
# Terminal 1
./projects/multiplayer/run.sh host 127.0.0.1:24560
```

```sh
# Terminal 2
./projects/multiplayer/run.sh join 127.0.0.1:24560
```

Each opens a Bevy window. Violet is Player 1 (host), coral is Player 2 (client).
Each window marks its own actor YOU and shows its local health/spell/dash HUD.
Click a window to control that player. A/D moves, Space double-jumps, Shift with
WASD dashes, mouse aims, click/F casts, and 1/2 or Q/right-click selects spells.
The match continues when a window loses focus, with that player's inputs cleared.
R/T/G affect solo practice only. Escape leaves; either peer leaving ends the session.

## Two computers on the same Wi-Fi

1. Clone/update the same revision on both computers and build using
   `projects/core-gameplay/bevy-playtest.sh`.
2. On the host run `./projects/multiplayer/run.sh host`. Default bind is all local
   interfaces on TCP port **24560**; simulation waits for a joining player.
3. Find the host Wi-Fi IPv4 address in network settings or `ip -4 address`.
   Use an address such as `192.168.1.42`; `0.0.0.0` is not a join address.
4. On the other computer run `./projects/multiplayer/run.sh join 192.168.1.42`.
   If the firewall blocks it, allow TCP 24560 for this LAN test. Same-network play
   needs no router port forwarding. The scripts do not alter firewall settings.
5. Confirm both HUDs show CONNECTED and follow [PLAYTEST.md](PLAYTEST.md).

The scripts delegate to the core Bevy launcher and reuse its NixOS graphics
library paths and project-local Cargo cache.

## Architecture

- Host: 120 simulation ticks/second, existing core systems and tuning.
- Client: up to 60 control messages/second; host snapshots: 30/second.
- Nonblocking TCP with TCP_NODELAY and bounded, length-prefixed JSON frames.
- Host decides all motion, casts, collisions, reflection, damage and respawns.
  Clients send controls only, never positions/health/hit outcomes.
- Stable player slots and projectile IDs, input sequence acknowledgements and
  respawn generations. ECS Entity values never leave the host.
- Stalled input neutralizes after 250ms. Disconnect/five-second timeout ends play.
- Third players and incompatible/unexpected protocol messages are rejected.

See [PROTOCOL.md](PROTOCOL.md) for packet and lifecycle details.

## Verification

```sh
./projects/multiplayer/check.sh
```

Formatting, fourteen tests and strict Clippy pass. Six tests use real loopback
TCP sockets and need network execution permission in managed environments. They
verify movement, snapshot agreement, Fireball damage/defeat/respawn, Gust launch/
slow/returns, focus behavior, disconnects, third-player rejection, protocol mismatch
and fragmented/oversized frames. Other tests cover replay/generation protection,
stalled controls, two-player simulation and serialization.

Core checks and full Bevy tests/build pass too. Two actual Bevy windows connected
on loopback using NVIDIA GTX 1650/Vulkan; captured host/client views were inspected.
Optional `--capture` on the Bevy app writes its GPU screenshot under core `.cache`
after connecting. These captures are excluded from Git.

## Current limits

Desktop loopback is verified. Two physical computers on Wi-Fi, hotspot/no-internet
operation, Android and iOS remain manual/device checks.

Client presentation uses authoritative 30Hz snapshots. Prediction, interpolation,
rollback and latency compensation are deferred; assess client feel on real Wi-Fi.
Only two players and one arena are supported. Discovery, lobby, scoring, reconnect
grace, host migration and online services remain future work. Restart both peers
after a session ends.

This is for a trusted local network: no transport encryption or authenticated
player identity is implemented; the first accepted peer receives slot 1.
