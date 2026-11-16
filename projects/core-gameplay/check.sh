#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
source scripts/environment.sh

cargo fmt --all -- --check
cargo fmt --manifest-path bevy-desktop/Cargo.toml -- --check
cargo test --locked --offline --workspace
cargo clippy --locked --offline --workspace --all-targets -- -D warnings
mkdir -p .cache
cargo run --locked --offline -p wild-magic-desktop -- --snapshot .cache/verified-preview.ppm
printf 'Offline checks passed. Window presentation and the full Bevy build are separate checks.\n'
