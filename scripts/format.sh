#!/usr/bin/env bash
set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "Installing/updating Python tooling..."
python -m pip install --upgrade pip
python -m pip install --upgrade ruff black isort pytest

echo "Running Ruff (auto-fix)..."
ruff check . --fix

echo "Running isort..."
python -m isort .

echo "Running Black..."
python -m black .

if command -v cargo >/dev/null 2>&1; then
    echo "Running cargo fmt..."
    cargo fmt

    echo "Checking Rust formatting..."
    cargo fmt --check

    echo "Running Clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

    echo "Running Rust tests..."
    cargo test
else
    echo "Cargo not found. Skipping Rust checks."
fi

if command -v cargo >/dev/null 2>&1 && command -v maturin >/dev/null 2>&1; then
    echo "Syncing pyratatui build for pytest..."
    "$SCRIPT_DIR/build.sh" --dev
else
    echo "Cargo or maturin not found. pytest may use a stale installed pyratatui build."
fi

echo "Running pytest..."
pytest -q

echo "All formatting, linting, and tests passed."
