# Wild Magic Academy — multiplayer project

Status: separate workstream prepared; transport and netcode are not implemented.

## Brief

After the local core playtest gate passes, connect two players over local Wi-Fi.
Use an authoritative host, a simple create/join flow with manual address entry,
and shared movement, collision, Fireball, Gust, damage and respawn outcomes.

## Owns

Transport selection, stable network IDs, validated input commands, sequence/tick
tracking, host state snapshots, client presentation/prediction, disconnect handling
and multi-device tests. Host departure ends the prototype session.

## Inputs and deliverables

Use [the core contract](../core-gameplay/INTEGRATION.md). Replace the dummy input
source with a remote actor, extend input submission, and keep mechanics in core.
Document packet/state formats and reproducible two-device tests. The historical
Godot implementation is a behavioral reference, not a required transport choice.

First gate: two devices agree on hits, projectile returns, deaths and respawns.
Test hotspot/no-internet operation separately. Online rooms, host migration,
discovery, scoring and full room UI are outside this first networking milestone.
