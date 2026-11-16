#!/usr/bin/env bash
# Sourced by launch/check scripts. Keep all Cargo writes inside this project.
wma_project_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
if [[ -z "${CARGO_HOME:-}" ]]; then
  export CARGO_HOME="$wma_project_dir/.cache/cargo"
  if [[ ! -d "$CARGO_HOME/registry/index" ]]; then
    mkdir -p -- "$CARGO_HOME/registry"
    for wma_registry_part in index cache; do
      if [[ -d "$HOME/.cargo/registry/$wma_registry_part" ]]; then
        printf 'Seeding project Cargo %s from the local cache...\n' "$wma_registry_part" >&2
        cp -a -- "$HOME/.cargo/registry/$wma_registry_part" "$CARGO_HOME/registry/"
      fi
    done
  fi
fi

# Prefer the configured Nix runtime path; supplement dynamically loaded window libs.
if [[ -d /nix/store ]]; then
  wma_runtime_libs="${LD_LIBRARY_PATH:-${NIX_LD_LIBRARY_PATH:-}}"
  for wma_libdir in /nix/store/*-libx11-*/lib /nix/store/*-libxcb-*/lib \
    /nix/store/*-libxi-*/lib /nix/store/*-libxcursor-*/lib \
    /nix/store/*-libxrandr-*/lib /nix/store/*-libxkbcommon-*/lib \
    /nix/store/*-wayland-*/lib /nix/store/*-vulkan-loader-*/lib \
    /nix/store/*-libglvnd-*/lib /run/opengl-driver/lib; do
    if [[ -d "$wma_libdir" ]]; then
      wma_runtime_libs="${wma_runtime_libs:+$wma_runtime_libs:}$wma_libdir"
    fi
  done
  export LD_LIBRARY_PATH="$wma_runtime_libs"
fi
