# LAN protocol v1

TCP carries UTF-8 JSON packets preceded by a four-byte unsigned big-endian payload
length. Maximum payload is 64 KiB; receive/send queues are bounded to 256 KiB.
Each poll processes at most 64 messages and bounded accepts. Partial reads/writes
and coalesced messages are handled. Invalid frames/messages or socket errors end play.

## Handshake and direction

1. Client sends `Hello { version: 1 }`.
2. Host assigns slot 1 and returns `Welcome { version: 1, slot: 1 }`.
3. Host sends initial `State`, then one after every four ticks (30Hz).
4. Client sends `Input` at up to 60Hz.

Host owns slot 0 and waits to simulate until the handshake. Queued host actions
are cleared at join. The client cannot choose a player ID; extra peers are closed.
Both sides validate version/message direction. Bump VERSION for incompatible changes.

## Controls

`Input { sequence, generation, command }` carries movement, world aim, jump/dash/
switch edges, selected spell and cast. The host accepts monotonic sequences and
reports the last received sequence in `State.acknowledged_input`.

Respawn generation must match the host actor. Old-generation commands/held actions
are discarded, and movement/aim is sanitized. No input variant supplies position,
health, tuning, projectile spawns or hit results. Edges execute once; held controls
persist between packets, and short cast presses survive coalesced press/release.

After 250ms without valid input the host clears held controls and pending edges.
Five seconds without peer messages, a socket failure, or either app closing ends
the session. LAN apps continue polling/simulating unfocused, with neutral local
controls; solo practice retains pause-on-focus-loss.

## Authoritative state

`State` includes tick, input acknowledgement, tuning, both Actor/Body records,
projectiles, terrain, transient effects and diagnostic counters. Complete state
replaces the previous client view; clients do not run the combat simulation.

Slots 0/1 identify players. Projectile IDs are monotonic u64 values, with current
owner/original caster encoded as player slots. No ECS Entity bits cross the wire.
Reflection changes current owner while preserving original caster, following the
existing core counter-return and self-damage rules.

Health, velocity, ground/jump/air-dash state, cooldowns, protection, Gust slow/
launch timers and respawn generation all come from the host. Client state accepts
two players with valid tuning and bounded object counts. Older/equal ticks are
ignored after the first state. Entity fields in client presentation snapshots are
placeholders: never feed them back to a simulation or treat them as network IDs.

No lockstep determinism, replay, prediction, deltas or stable audio-event stream is
implemented. Visual effects use the latest full state; audio/event IDs are future work.

## Lifecycle

Joining uses a timeout-bounded worker thread so rendering cannot block on connect.
Established sockets are nonblocking with TCP_NODELAY. Protocol/queue/socket/timeout
failures freeze the last view with SESSION ENDED in the HUD and a terminal error;
the client never substitutes local gameplay. Reconnect/host migration are deferred.
