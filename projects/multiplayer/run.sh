#!/usr/bin/env bash
set -euo pipefail
wma_core_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../core-gameplay" && pwd)"
case "${1:-}" in
  host) shift; exec "$wma_core_dir/run.sh" --bevy --host "$@" ;;
  join) shift; exec "$wma_core_dir/run.sh" --bevy --join "$@" ;;
  *) printf 'Usage: %s host [IP:PORT]\n       %s join HOST_IP[:PORT]\n' "$0" "$0" >&2; exit 2 ;;
esac
