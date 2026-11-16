#!/usr/bin/env bash
# Run in a terminal with internet access and access to the desktop session.
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
source scripts/environment.sh
mkdir -p .cache/logs
wma_log="$wma_project_dir/.cache/logs/bevy-playtest.log"
wma_stage="setup"
exec > >(tee "$wma_log") 2>&1
trap 'wma_status=$?; printf "\nBevy playtest stopped during %s (exit %s). Log: %s\n" "$wma_stage" "$wma_status" "$wma_log" >&2; exit "$wma_status"' ERR

wma_stage="dependency download"
printf '\n[1/5] Downloading Bevy dependencies\n'
cargo fetch --manifest-path bevy-desktop/Cargo.toml

wma_stage="Bevy compilation"
printf '\n[2/5] Compiling the Bevy desktop app\n'
cargo build --locked --manifest-path bevy-desktop/Cargo.toml

wma_stage="Bevy adapter tests"
printf '\n[3/5] Testing the Bevy presentation adapter\n'
cargo test --locked --manifest-path bevy-desktop/Cargo.toml

wma_stage="Bevy static checks"
printf '\n[4/5] Checking the Bevy adapter\n'
cargo clippy --locked --manifest-path bevy-desktop/Cargo.toml --all-targets -- -D warnings

wma_stage="interactive playtest"
printf '\n[5/5] Opening Wild Magic Academy\n'
printf 'Follow PLAYTEST.md. A successful launch does not by itself mark the human playtest as passed.\n'
cargo run --locked --manifest-path bevy-desktop/Cargo.toml
printf '\nBevy exited successfully. Record actual playtest findings in PLAYTEST.md.\n'
