#!/usr/bin/env bash
set -euo pipefail

# Publish the finance-query-core crate to crates.io.
# Usage: ./publish.sh
# Optional: set DRY_RUN_ONLY=1 to stop after cargo publish --dry-run.

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CRATE_NAME="finance-query-core"

cd "${ROOT_DIR}"

echo "==> Verifying CARGO_REGISTRY_TOKEN is set"
if [[ -z "${CARGO_REGISTRY_TOKEN:-}" ]]; then
  echo "Error: CARGO_REGISTRY_TOKEN is not set. Export it before running."
  exit 1
fi

echo "==> Running tests for ${CRATE_NAME}"
cargo test -p "${CRATE_NAME}"

echo "==> Dry-run publish for ${CRATE_NAME}"
cargo publish -p "${CRATE_NAME}" --dry-run

if [[ "${DRY_RUN_ONLY:-0}" == "1" ]]; then
  echo "DRY_RUN_ONLY=1 set; skipping real publish."
  exit 0
fi

echo "==> Publishing ${CRATE_NAME} to crates.io"
cargo publish -p "${CRATE_NAME}"

echo "==> Done. Note: crates.io can take a few minutes to index the new version."
