#!/usr/bin/env bash
# build.sh — pyratatui build helper
# Usage:
#   ./scripts/build.sh           # release wheel + pip install
#   ./scripts/build.sh --dev     # editable maturin develop
#   ./scripts/build.sh --sdist   # source distribution only
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

OS="$(uname -s)"
ARCH="$(uname -m)"

echo "[pyratatui] build script"
echo "   OS:   $OS"
echo "   Arch: $ARCH"

command -v maturin >/dev/null 2>&1 || { echo "[ERROR] maturin not found. Run: pip install maturin"; exit 1; }
command -v cargo   >/dev/null 2>&1 || { echo "[ERROR] cargo not found. Install Rust: https://rustup.rs"; exit 1; }

echo "[OK] $(maturin --version)"
echo "[OK] $(cargo --version)"

MODE="${1:-}"

has_editable_env() {
    [[ -n "${VIRTUAL_ENV:-}" || -n "${CONDA_PREFIX:-}" || -d "$PROJECT_DIR/.venv" ]]
}

install_release_wheel() {
    echo "[BUILD] Building release wheel..."
    maturin build --release --strip --out dist/

    WHEEL="$(ls -t dist/*.whl 2>/dev/null | head -n 1 || true)"

    if [[ -z "$WHEEL" ]]; then
        echo "[ERROR] No wheel produced in dist/"
        exit 1
    fi

    echo "[INSTALL] Installing $(basename "$WHEEL")..."
    python -m pip install --upgrade "$WHEEL" --force-reinstall

    echo "[OK] Installed successfully."
}

case "$MODE" in
    --dev)
        if has_editable_env; then
            echo "[DEV] Development mode (editable install)..."
            maturin develop --release
            echo "[OK] Installed (editable)."
        else
            echo "[WARN] No virtualenv or conda environment detected. Falling back to wheel build + install."
            install_release_wheel
        fi
        ;;
    --sdist)
        echo "[SDIST] Building source distribution..."
        maturin sdist --out dist/
        echo "[OK] sdist created in dist/"
        ;;
    *)
        install_release_wheel
        ;;
esac
