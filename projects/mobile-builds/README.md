# Wild Magic Academy — mobile builds project

Status: separate workstream prepared; no Android/iOS build has been made.

## Brief

Package and validate the Bevy game on Android and iOS. Begin after the full Bevy
renderer builds and runs; the software desktop runner is only a test harness.
Confirm device/toolchain access before committing to platform-specific steps.

## Owns

Platform build configurations, packaging/signing setup, device launch, app
lifecycle, safe areas/aspect ratios, touch integration checks, local network
permissions and performance measurements on real hardware.

## Inputs and deliverables

Read [the core contract](../core-gameplay/INTEGRATION.md). Keep gameplay rules
shared, integrate native sprites/VFX instead of the temporary CPU preview, and
record tested device/OS/build combinations. Document launch/build steps and
measured performance. Signing credentials must remain outside source control.

First gate: launch on a physical device, traverse the arena, aim/switch/cast,
pause/resume cleanly and maintain acceptable measured performance. Two-device
Wi-Fi verification is shared with the multiplayer project. Store publication
is a later action, outside the current milestone.
