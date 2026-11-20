#!/usr/bin/env bash
set -euo pipefail
rig_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
rig_godot="${GODOT_BIN:-$HOME/.local/bin/godot}"
if [[ ! -x "$rig_godot" ]]; then
  echo 'Install Godot first: ./projects/art-animation/install-godot.sh' >&2
  exit 1
fi
case "${1:---editor}" in
  --editor) exec "$rig_godot" --editor --audio-driver Dummy --path "$rig_root/godot-rigging" res://cape_world.tscn ;;
  --play) exec "$rig_godot" --audio-driver Dummy --path "$rig_root/godot-rigging" ;;
  --check)
    "$rig_godot" --headless --editor --path "$rig_root/godot-rigging" --import --quit
    "$rig_godot" --headless --path "$rig_root/godot-rigging" --script res://test_rig.gd
    exec "$rig_godot" --headless --fixed-fps 120 --path "$rig_root/godot-rigging" --script res://test_cape.gd ;;
  --build-rig) exec "$rig_godot" --headless --path "$rig_root/godot-rigging" --script res://build_rig.gd ;;
  --build-cape) exec "$rig_godot" --headless --path "$rig_root/godot-rigging" --script res://build_cape.gd ;;
  *) echo 'Usage: godot.sh [--editor|--play|--check|--build-rig|--build-cape]' >&2; exit 2 ;;
esac
