#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

source scripts/environment.sh

if [[ "${1:-}" == --bevy ]]; then
  shift
  # Dependency resolution may need the network on the first full-renderer build.
  exec cargo run --manifest-path bevy-desktop/Cargo.toml -- "$@"
fi

exec cargo run --locked --offline -p wild-magic-desktop -- "$@"
