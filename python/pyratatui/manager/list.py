"""
list.py — List all installed pyratatui apps.

Usage (via CLI)::

    pyratatui list
    pyratatui list --json
"""

from __future__ import annotations

import json
from pathlib import Path

import typer

from .utils import list_installed


def list_apps(*, as_json: bool = False) -> None:
    """
    Print a table of all apps installed under ``~/.pyratatui/apps``.

    Args:
        as_json: Output raw JSON instead of a human-readable table.
    """
    apps = list_installed()

    if as_json:
        typer.echo(json.dumps(apps, indent=2))
        return

    if not apps:
        typer.echo("No apps installed yet.")
        typer.echo("  Install one with:  pyratatui install user/repo")
        return

    # ── Pretty table ──────────────────────────────────────────────────────────
    col_w = {"repo": 22, "slug": 30, "branch": 14, "installed_at": 22}
    header = (
        f"{'App':<{col_w['repo']}} "
        f"{'Source':<{col_w['slug']}} "
        f"{'Branch':<{col_w['branch']}} "
        f"{'Installed':<{col_w['installed_at']}}"
    )

    typer.echo(typer.style(header, bold=True))
    typer.echo("─" * (sum(col_w.values()) + len(col_w) - 1))

    for app in apps:
        repo = app.get("repo", "?")
        slug = app.get("slug", "?")
        branch = app.get("branch", "?")
        installed_at = app.get("installed_at", "?")

        src_dir = Path(app.get("app_dir", ""))
        present = src_dir.exists() if src_dir != Path("") else False
        status = (
            typer.style("✓", fg=typer.colors.GREEN)
            if present
            else typer.style("✗", fg=typer.colors.RED)
        )

        row = (
            f"{status} "
            f"{repo:<{col_w['repo'] - 2}} "
            f"{slug:<{col_w['slug']}} "
            f"{branch:<{col_w['branch']}} "
            f"{installed_at:<{col_w['installed_at']}}"
        )
        typer.echo(row)

    typer.echo(f"\n{len(apps)} app(s) installed.")
    typer.echo("Run an app with:  pyratatui run <app>")
