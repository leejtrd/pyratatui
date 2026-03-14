"""
run.py — Launch an installed pyratatui app.

Usage (via CLI)::

    pyratatui run <app>
    pyratatui run <app> -- --some-app-arg
"""

from __future__ import annotations

import os
import subprocess
from pathlib import Path

import typer

from .utils import list_installed, parse_entrypoint, require_python


def run_app(app_name: str, extra_args: list[str] | None = None) -> None:
    """
    Run an app that was installed with ``pyratatui install``.

    Flat-layout apps (``main.py`` at repo root) are launched as
    ``python main.py`` rather than ``python -m __main__`` to avoid the
    ``ValueError: __main__.__spec__ is None`` error that Python raises when
    ``-m`` is given a bare ``__main__`` with no enclosing package.

    Package-style apps (``<pkg>/__main__.py`` or ``<pkg>/main.py``) are still
    launched with ``python -m <module>`` as normal.

    Args:
        app_name:   Short repo name (e.g. ``my-tui``).
        extra_args: Additional CLI arguments forwarded verbatim to the app.
    """
    extra_args = extra_args or []

    # ── Look up metadata ──────────────────────────────────────────────────────
    all_apps = list_installed()
    matches = [a for a in all_apps if a.get("repo") == app_name]

    if not matches:
        typer.echo(
            typer.style(
                f"Error: '{app_name}' is not installed.",
                fg=typer.colors.RED,
            ),
            err=True,
        )
        available = [a.get("repo", "") for a in all_apps]
        if available:
            typer.echo(f"  Installed apps: {', '.join(available)}", err=True)
        else:
            typer.echo(
                "  No apps installed yet. Try: pyratatui install user/repo",
                err=True,
            )
        raise typer.Exit(1)

    meta = matches[0]
    src_dir = Path(meta["app_dir"])
    entrypoint: str = meta.get("entrypoint", "script:main.py")

    if not src_dir.exists():
        typer.echo(
            typer.style(
                f"Error: source directory '{src_dir}' is missing. "
                f"Try: pyratatui install {meta.get('slug', app_name)} --force",
                fg=typer.colors.RED,
            ),
            err=True,
        )
        raise typer.Exit(1)

    # ── Decode entrypoint ─────────────────────────────────────────────────────
    kind, target = parse_entrypoint(entrypoint)

    python = require_python()

    if kind == "script":
        # Flat-layout: run the script file directly.
        # "python main.py" works correctly; "python -m __main__" does not
        # because __main__.__spec__ is None outside a package context.
        script_path = src_dir / target
        cmd = [python, str(script_path)] + extra_args
    else:
        # Package layout: "python -m myapp" or "python -m myapp.main"
        cmd = [python, "-m", target] + extra_args

    # ── PYTHONPATH so the app's local imports resolve ─────────────────────────
    env = os.environ.copy()
    existing = env.get("PYTHONPATH", "")
    env["PYTHONPATH"] = f"{src_dir}{os.pathsep}{existing}" if existing else str(src_dir)

    # ── Launch ────────────────────────────────────────────────────────────────
    try:
        result = subprocess.run(cmd, cwd=src_dir, env=env)
        raise typer.Exit(result.returncode)
    except KeyboardInterrupt:
        raise typer.Exit(0) from None
