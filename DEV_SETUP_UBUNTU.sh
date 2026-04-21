#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export DEBIAN_FRONTEND=noninteractive

need_sudo=0
if [[ "${EUID}" -ne 0 ]]; then
    need_sudo=1
fi

run_root() {
    if [[ "${need_sudo}" -eq 1 ]]; then
        sudo "$@"
    else
        "$@"
    fi
}

echo "[1/6] Installing Ubuntu packages needed for Rust + native deps..."
run_root apt-get update
run_root apt-get install -y \
    build-essential \
    pkg-config \
    curl \
    git \
    ca-certificates \
    cmake \
    ninja-build \
    python3 \
    libasound2-dev \
    libudev-dev \
    libx11-dev \
    libxext-dev \
    libxrandr-dev \
    libxi-dev \
    libxcursor-dev \
    libxinerama-dev \
    libwayland-dev \
    wayland-protocols \
    libgl1-mesa-dev

echo "[2/6] Installing Rust toolchain (rustup) if needed..."
if ! command -v cargo >/dev/null 2>&1; then
    curl https://sh.rustup.rs -sSf | sh -s -- -y --profile default --default-toolchain stable
fi

# Make cargo/rustc available in this shell even if rustup was just installed.
if [[ -f "${HOME}/.cargo/env" ]]; then
    # shellcheck disable=SC1090
    source "${HOME}/.cargo/env"
fi

echo "[3/6] Verifying Rust toolchain..."
command -v rustc >/dev/null 2>&1
command -v cargo >/dev/null 2>&1
rustc --version
cargo --version

echo "[4/6] Fetching Rust dependencies..."
cd "${SCRIPT_DIR}"
cargo fetch

echo "[5/6] Building release binary..."
cargo build --release