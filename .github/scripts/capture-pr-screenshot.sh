#!/usr/bin/env bash
# Capture Playwright smoke screenshots for PR Screenshot/Capture.
# Requires the app already serving at http://127.0.0.1:8080 (e.g. docker compose up).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"

BASE_URL="${BASE_URL:-http://127.0.0.1:8080}"
ARTIFACT_DIR="e2e/artifacts"
OUT_DIR="docs/assets/pr-screenshots"

if ! curl -sf "${BASE_URL}/" >/dev/null; then
  echo "error: app is not reachable at ${BASE_URL}/" >&2
  echo "start it first, e.g.: docker compose up --build" >&2
  exit 1
fi

if [[ ! -d node_modules ]]; then
  npm ci
fi

# Prefer system Chrome (playwright.config channel: chrome). Bundled Chromium
# may be unavailable on older macOS; install is best-effort only.
npx playwright install chromium 2>/dev/null || true

npm run e2e:screenshot

BRANCH="$(git branch --show-current 2>/dev/null || echo unknown)"
SLUG="$(printf '%s' "$BRANCH" | tr '/' '-')"

mkdir -p "$OUT_DIR"

shopt -s nullglob
pngs=("$ARTIFACT_DIR"/*.png)
if [[ ${#pngs[@]} -eq 0 ]]; then
  echo "error: no png files in ${ARTIFACT_DIR}/" >&2
  exit 1
fi

REMOTE_URL="$(git remote get-url origin 2>/dev/null || true)"
OWNER_REPO=""
if [[ "$REMOTE_URL" =~ github.com[:/]([^/]+)/([^/.]+)(\.git)?$ ]]; then
  OWNER_REPO="${BASH_REMATCH[1]}/${BASH_REMATCH[2]}"
fi

echo ""
echo "### Screenshot/Capture (paste into PR body)"
echo ""

for src in "${pngs[@]}"; do
  base="$(basename "$src" .png)"
  dest="${OUT_DIR}/${SLUG}-${base}.png"
  cp "$src" "$dest"
  echo "wrote ${dest}" >&2
  if [[ -n "$OWNER_REPO" && -n "$BRANCH" && "$BRANCH" != "unknown" ]]; then
    echo "![${base}](https://github.com/${OWNER_REPO}/raw/${BRANCH}/${dest})"
  else
    echo "![${base}](${dest})"
  fi
done
