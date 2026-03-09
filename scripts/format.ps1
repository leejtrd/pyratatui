$ErrorActionPreference = "Stop"

Write-Host "Installing/updating Python tooling..."
python -m pip install --upgrade pip
python -m pip install --upgrade ruff black isort pytest

Write-Host "Running Ruff (auto-fix)..."
ruff check . --fix

Write-Host "Running isort..."
python -m isort .

Write-Host "Running Black..."
python -m black .

if (Get-Command cargo -ErrorAction SilentlyContinue) {

    Write-Host "Running cargo fmt..."
    cargo fmt

    Write-Host "Checking Rust formatting..."
    cargo fmt --check

    Write-Host "Running Clippy..."
    cargo clippy --all-targets --all-features -- -D warnings

    Write-Host "Running Rust tests..."
    cargo test

} else {
    Write-Warning "Cargo not found. Skipping Rust checks."
}

if ((Get-Command cargo -ErrorAction SilentlyContinue) -and (Get-Command maturin -ErrorAction SilentlyContinue)) {
    Write-Host "Syncing pyratatui build for pytest..."
    & "$PSScriptRoot\build.ps1" --dev
} else {
    Write-Warning "Cargo or maturin not found. pytest may use a stale installed pyratatui build."
}

Write-Host "Running pytest..."
pytest -q

Write-Host "All formatting, linting, and tests passed."
