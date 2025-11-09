#!/usr/bin/env bash
set -Eeuo pipefail

# Build Linux wheels via the official manylinux container so the result is
# widely installable from PyPI (abi3 for CPython >= 3.8).
PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
cd "$PROJECT_ROOT"

DOCKER_IMAGE=${DOCKER_IMAGE:-ghcr.io/pyo3/maturin}
TARGET_ARCH=${TARGET_ARCH:-x86_64-unknown-linux-gnu}
COMPAT_LEVEL=${COMPAT_LEVEL:-manylinux_2_17}
OUTPUT_DIR=${OUTPUT_DIR:-dist}
EXTRA_ARGS=(${EXTRA_ARGS:-})
SKIP_PULL=${SKIP_PULL:-0}

if ! command -v docker >/dev/null 2>&1; then
  echo "[ERROR] docker command not found. Install Docker or switch to another build flow." >&2
  exit 1
fi

if [[ "$SKIP_PULL" != "1" ]]; then
  echo "[INFO] Pulling $DOCKER_IMAGE ..."
  docker pull "$DOCKER_IMAGE"
fi

docker run --rm -v "$PWD":/io "$DOCKER_IMAGE" build \
  --release \
  --features python \
  --strip \
  --target "$TARGET_ARCH" \
  --compatibility "$COMPAT_LEVEL" \
  --out "$OUTPUT_DIR" \
  "${EXTRA_ARGS[@]}"
