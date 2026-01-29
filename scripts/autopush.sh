#!/bin/bash
set -e

# This script builds (if needed) and runs the repo-watch autopush tool.

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT" || { echo "Repository root not found! Exiting."; exit 1; }

BIN="$REPO_ROOT/target/release/repo-watch"

# Build if binary does not exist or is not executable
if [ ! -x "$BIN" ]; then
  echo "[repo-watch] Building binary..."
  cargo build --release || { echo "Build failed! Exiting."; exit 1; }
fi

echo "[repo-watch] $(date): Running..."
"$BIN"