#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
DEST_DIR="${1:-$ROOT_DIR/external}"
mkdir -p "$DEST_DIR"

clone_or_update() {
  local repo_url="$1"
  local folder_name="$2"

  if [ -d "$DEST_DIR/$folder_name/.git" ]; then
    echo "[update] $folder_name"
    git -C "$DEST_DIR/$folder_name" pull --ff-only
  else
    echo "[clone] $repo_url -> $DEST_DIR/$folder_name"
    git clone "$repo_url" "$DEST_DIR/$folder_name"
  fi
}

# Codex skill source repository.
clone_or_update "https://github.com/openai/skills.git" "codex-skills"

# Antigravity skill/tooling repository.
clone_or_update "https://github.com/antigravity-ai/antigravity.git" "antigravity"

echo "Done. Repos are available under: $DEST_DIR"
