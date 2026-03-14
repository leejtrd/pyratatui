"""
install.py — Clone and install a pyratatui app from GitHub.

Usage (via CLI)::

    pyratatui install user/repo
    pyratatui install user/repo --branch dev
    pyratatui install user/repo --force

Dependency installation priority
---------------------------------
1. ``pyproject.toml`` present → ``pip install .``
   Installs the declared dependencies *and* registers any console scripts.
   This is the expected path for projects scaffolded with ``pyratatui init``.

2. ``requirements.txt`` present (no pyproject.toml) → ``pip install -r requirements.txt``
   Legacy / minimal fallback for simpler repos.

3. Neither present → skip dep installation silently.
"""

from __future__ import annotations

import shutil
import sys
import time
from pathlib import Path

import typer

from .utils import (
    app_dir,
    bin_dir_in_path,
    create_wrapper,
    detect_entrypoint,
    path_advisory,
    read_meta,
    require_git,
    run_cmd,
    write_meta,
)


def _install_deps(dest: Path, quiet: bool) -> None:
    """
    Install the app's Python dependencies in priority order.

    Args:
        dest:  Cloned source directory.
        quiet: Suppress non-error output when True.
    """
    pyproject = dest / "pyproject.toml"
    req_file = dest / "requirements.txt"

    if pyproject.exists():
        # Full PEP 517 install: pulls declared deps + registers entry points.
        if not quiet:
            typer.echo("  Installing via pyproject.toml  (pip install .) …")
        try:
            run_cmd(
                [sys.executable, "-m", "pip", "install", ".", "--quiet"],
                cwd=dest,
            )
        except Exception as exc:  # noqa: BLE001
            typer.echo(
                typer.style(
                    f"Warning: pip install . failed — {exc}",
                    fg=typer.colors.YELLOW,
                ),
                err=True,
            )

    elif req_file.exists():
        # Fallback for repos without a pyproject.toml.
        if not quiet:
            typer.echo("  Installing via requirements.txt …")
        try:
            run_cmd(
                [
                    sys.executable,
                    "-m",
                    "pip",
                    "install",
                    "-r",
                    str(req_file),
                    "--quiet",
                ],
                cwd=dest,
            )
        except Exception as exc:  # noqa: BLE001
            typer.echo(
                typer.style(
                    f"Warning: pip install -r requirements.txt failed — {exc}",
                    fg=typer.colors.YELLOW,
                ),
                err=True,
            )

    # If neither file exists we proceed without installing deps.


def install_app(
    repo_slug: str,
    *,
    branch: str | None = None,
    force: bool = False,
    quiet: bool = False,
) -> None:
    """
    Clone *repo_slug* (``user/repo``) from GitHub and register it as an app.

    Args:
        repo_slug: GitHub path in ``user/repo`` format.
        branch:    Optional branch / tag / commit to checkout.
        force:     If True, remove any existing installation first.
        quiet:     Suppress non-error output.
    """
    # ── Validate slug ─────────────────────────────────────────────────────────
    parts = repo_slug.split("/")
    if len(parts) != 2 or not all(parts):
        typer.echo(
            typer.style(
                f"Error: '{repo_slug}' is not a valid user/repo slug.",
                fg=typer.colors.RED,
            ),
            err=True,
        )
        raise typer.Exit(1)

    user, repo = parts
    git = require_git()
    dest = app_dir(user, repo)

    # ── Already installed? ────────────────────────────────────────────────────
    existing = read_meta(user, repo)
    if existing and not force:
        typer.echo(
            typer.style(
                f"'{repo}' is already installed. Use --force to reinstall.",
                fg=typer.colors.YELLOW,
            )
        )
        raise typer.Exit(0)

    if force and dest.exists():
        if not quiet:
            typer.echo(f"  Removing existing installation of '{repo}' …")
        shutil.rmtree(dest)

    # ── Clone ─────────────────────────────────────────────────────────────────
    clone_url = f"https://github.com/{user}/{repo}.git"
    dest.parent.mkdir(parents=True, exist_ok=True)

    if not quiet:
        typer.echo(f"  Cloning {clone_url} …")

    clone_args = [git, "clone", "--depth=1"]
    if branch:
        clone_args += ["--branch", branch]
    clone_args += [clone_url, str(dest)]

    try:
        run_cmd(clone_args)
    except Exception as exc:  # noqa: BLE001
        typer.echo(
            typer.style(f"Error: clone failed — {exc}", fg=typer.colors.RED),
            err=True,
        )
        if dest.exists():
            shutil.rmtree(dest)
        raise typer.Exit(1) from exc

    # ── Install Python dependencies ───────────────────────────────────────────
    _install_deps(dest, quiet=quiet)

    # ── Detect entrypoint & create wrapper ────────────────────────────────────
    entrypoint = detect_entrypoint(dest)
    wrapper = create_wrapper(user, repo, entrypoint)

    # ── Persist metadata ──────────────────────────────────────────────────────
    has_pyproject = (dest / "pyproject.toml").exists()
    meta: dict = {  # type: ignore[type-arg]
        "user": user,
        "repo": repo,
        "slug": repo_slug,
        "url": clone_url,
        "branch": branch or "default",
        "entrypoint": entrypoint,
        "installed_at": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "app_dir": str(dest),
        "wrapper": str(wrapper),
        "has_pyproject": has_pyproject,
    }
    write_meta(user, repo, meta)

    # ── Done ──────────────────────────────────────────────────────────────────
    if not quiet:
        typer.echo(
            typer.style(
                f"\n✓ Installed '{repo}' from {user}/{repo}",
                fg=typer.colors.GREEN,
            )
        )
        typer.echo(f"  Location : {dest}")
        typer.echo(f"  Wrapper  : {wrapper}")
        if has_pyproject:
            typer.echo("  Deps     : installed via pyproject.toml")
        typer.echo(f"\nRun it with:  pyratatui run {repo}")

        if not bin_dir_in_path():
            typer.echo(typer.style(path_advisory(), fg=typer.colors.YELLOW))
