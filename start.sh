#!/bin/bash
set -e

cd "$(dirname "$0")"

NODE_VERSION=$(node -v 2>/dev/null || echo "none")

if [[ "$NODE_VERSION" != v22* ]]; then
  echo "Current node: $NODE_VERSION, switching to v22..."
  source ~/.nvm/nvm.sh
  nvm use 22
fi

echo "Using node: $(node -v)"
npm run tauri:dev
