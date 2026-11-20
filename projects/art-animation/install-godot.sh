#!/usr/bin/env bash
set -euo pipefail
rig_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
rig_version=4.7.2
rig_archive="Godot_v${rig_version}-stable_linux.x86_64.zip"
rig_digest=9aa00f7a605200940bce3027a567b782f49bd8e940dd06ae9e987bd65aee1b1467edd56ed84fcdcbdd44354bf613bdbb4e5d2913e925850368e150c59ed54c65
rig_cache="$rig_root/.cache/godot-$rig_version"
rig_install="$HOME/.local/share/godot/$rig_version"
if [[ "$(uname -sm)" != "Linux x86_64" ]]; then
  echo 'This installer targets Linux x86_64. Use the official Godot download for other systems.' >&2
  exit 1
fi
mkdir -p "$rig_cache" "$rig_install" "$HOME/.local/bin" "$HOME/.local/share/applications"
if [[ ! -f "$rig_cache/$rig_archive" ]]; then
  curl --fail --location --show-error \
    "https://github.com/godotengine/godot/releases/download/$rig_version-stable/$rig_archive" \
    --output "$rig_cache/$rig_archive.part"
  mv "$rig_cache/$rig_archive.part" "$rig_cache/$rig_archive"
fi
printf '%s  %s\n' "$rig_digest" "$rig_cache/$rig_archive" | sha512sum --check --status
if [[ ! -x "$rig_install/Godot_v$rig_version-stable_linux.x86_64" ]]; then
  unzip -oq "$rig_cache/$rig_archive" -d "$rig_install"
fi
chmod +x "$rig_install/Godot_v$rig_version-stable_linux.x86_64"
cat > "$HOME/.local/bin/godot" <<'LAUNCHER'
#!/usr/bin/env bash
set -euo pipefail
# Supplement NixOS's loader path for Godot's dynamically loaded desktop libraries.
rig_libs="${LD_LIBRARY_PATH:-${NIX_LD_LIBRARY_PATH:-}}"
for rig_libdir in /run/opengl-driver/lib /nix/store/*-libx11-*/lib \
  /nix/store/*-libxcursor-*/lib /nix/store/*-libxinerama-*/lib \
  /nix/store/*-libxi-*/lib /nix/store/*-libxrandr-*/lib \
  /nix/store/*-libxrender-*/lib /nix/store/*-libxcb-*/lib \
  /nix/store/*-libxext-*/lib /nix/store/*-libdecor-*/lib /nix/store/*-dbus-*/lib \
  /nix/store/*-libxkbcommon-*/lib /nix/store/*-wayland-*/lib \
  /nix/store/*-libglvnd-*/lib /nix/store/*-vulkan-loader-*/lib; do
  if [[ -d "$rig_libdir" ]]; then rig_libs="${rig_libs:+$rig_libs:}$rig_libdir"; fi
done
export LD_LIBRARY_PATH="$rig_libs"
exec "$HOME/.local/share/godot/4.7.2/Godot_v4.7.2-stable_linux.x86_64" "$@"
LAUNCHER
chmod +x "$HOME/.local/bin/godot"
cat > "$HOME/.local/share/applications/godot-editor.desktop" <<DESKTOP
[Desktop Entry]
Type=Application
Name=Godot Editor
Comment=Godot 4.7.2 game and 2D rig editor
Exec="$HOME/.local/bin/godot" --project-manager
Icon=applications-development
Terminal=false
Categories=Development;IDE;
DESKTOP
"$HOME/.local/bin/godot" --version
echo 'Installed Godot Editor and the applications-menu entry.'
