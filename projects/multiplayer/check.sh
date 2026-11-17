#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"
source ../core-gameplay/scripts/environment.sh
cargo fmt -- --check
cargo test --locked --offline -j 4
cargo clippy --locked --offline --all-targets -j 4 -- -D warnings
printf 'LAN checks passed, including real TCP loopback tests.\n'
