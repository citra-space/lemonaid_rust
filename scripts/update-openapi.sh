#!/usr/bin/env bash
# Refresh the vendored OpenAPI specification from dev.api.citra.space.
#
# The dev and prod APIs run from the same source and are at most a day out
# of sync, so we always pull from dev. The fetched document is committed
# verbatim — `build.rs` performs the OpenAPI 3.1 -> 3.0 downconversion at
# build time, so the on-disk spec stays byte-identical to upstream.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
SPEC_URL="${SPEC_URL:-https://dev.api.citra.space/openapi.json}"
OUT="$REPO_ROOT/openapi/citra.json"

mkdir -p "$(dirname "$OUT")"
TMP="$(mktemp)"
trap 'rm -f "$TMP"' EXIT

echo "Fetching $SPEC_URL"
curl --fail --silent --show-error "$SPEC_URL" -o "$TMP"

# Validate it parses as JSON before clobbering the vendored copy.
python3 -m json.tool "$TMP" > /dev/null

# Pretty-print so diffs are readable in code review.
python3 -m json.tool --no-ensure-ascii "$TMP" > "$OUT"

echo "Wrote $OUT"
