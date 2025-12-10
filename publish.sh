#!/usr/bin/env bash
set -euo pipefail

# Publish the finance-query-core crate to crates.io.
# Usage:
#   ./publish.sh            # test -> dry-run -> publish
#   ./publish.sh --dry-only # test -> dry-run (no publish)

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_NAME="finance-query-core"

DRY_ONLY=0
if [[ "${1:-}" == "--dry-only" ]]; then
  DRY_ONLY=1
fi

cd "${ROOT_DIR}"

echo "==> Using existing Cargo credentials (set CARGO_REGISTRY_TOKEN to override)"

echo "==> Running tests for ${CRATE_NAME}"
cargo test -p "${CRATE_NAME}"

echo "==> Dry-run publish for ${CRATE_NAME}"
cargo publish -p "${CRATE_NAME}" --dry-run

if [[ "${DRY_ONLY}" == "1" ]]; then
  echo "Dry-run complete; skipping real publish (--dry-only)."
  exit 0
fi

echo "==> Dry-run succeeded; publishing ${CRATE_NAME} to crates.io"
cargo publish -p "${CRATE_NAME}"

echo "==> Done. crates.io may take a few minutes to index the new version."
