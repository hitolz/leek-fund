#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

ensure_target() {
  local target="$1"
  if ! rustup target list --installed | grep -q "^${target}$"; then
    echo "Installing Rust target: ${target}"
    rustup target add "${target}"
  fi
}

build_target() {
  local target="$1"
  echo "Building Tauri bundle for ${target}..."
  (cd "${ROOT_DIR}" && npm run tauri:build -- --target "${target}")
}

ensure_target "aarch64-apple-darwin"
ensure_target "x86_64-apple-darwin"

build_target "aarch64-apple-darwin"
build_target "x86_64-apple-darwin"

echo "Done. Bundles are under:"
echo "  src-tauri/target/aarch64-apple-darwin/release/bundle"
echo "  src-tauri/target/x86_64-apple-darwin/release/bundle"
