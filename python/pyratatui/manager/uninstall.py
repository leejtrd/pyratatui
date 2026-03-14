"""
uninstall.py — Remove an installed pyratatui app.

Usage (via CLI)::

    pyratatui uninstall repo
    pyratatui uninstall repo --yes   # skip confirmation prompt
"""

from __future__ import annotations

import os
import shutil
import stat
import sys
from pathlib import Path

import typer

from .utils import app_dir, list_installed, remove_wrapper


def _force_rmtree(path: Path) -> None:
    """
    Nuke *path* completely, no matter what.

    Strategy (works on Windows with read-only .git pack files AND on every
    Python version from 3.10 to 3.14+):

    1. Walk the tree bottom-up and chmod every file/dir to 0o777 before
       attempting deletion.  This removes the read-only bit that Windows git
       sets on .git/objects/pack/* files (PermissionError WinError 5).

    2. Then call shutil.rmtree with an error callback that strips the
       read-only bit and retries on any remaining locked file.
       - Python <3.12  → ``onerror`` parameter
       - Python >=3.12 → ``onexc``  parameter  (renamed in 3.12, onerror
         silently ignored in 3.14)

    3. If rmtree still fails after all that (e.g. a process has the dir
       open), fall back to ``rd /s /q`` on Windows or ``rm -rf`` on Unix
       as a last resort.
    """
    if not path.exists():
        return

    # ── Pass 1: strip read-only from every file/dir in the tree ──────────────
    for root, dirs, files in os.walk(str(path)):
        for name in files + dirs:
            try:
                full = os.path.join(root, name)
                os.chmod(full, stat.S_IWRITE | stat.S_IREAD | stat.S_IEXEC)
            except OSError:
                pass

    # ── Pass 2: rmtree with version-aware error callback ─────────────────────
    def _on_error(func, fpath, exc_info):  # type: ignore[no-untyped-def]
        """onerror callback (Python < 3.12)."""
        try:
            os.chmod(fpath, stat.S_IWRITE | stat.S_IREAD)
            func(fpath)
        except OSError:
            pass

    def _on_exc(func, fpath, exc):  # type: ignore[no-untyped-def]
        """onexc callback (Python >= 3.12)."""
        try:
            os.chmod(fpath, stat.S_IWRITE | stat.S_IREAD)
            func(fpath)
        except OSError:
            pass

    if sys.version_info >= (3, 12):
        shutil.rmtree(str(path), onexc=_on_exc)
    else:
        shutil.rmtree(str(path), onerror=_on_error)

    # ── Pass 3: last-resort OS command if the directory still exists ──────────
    if path.exists():
        try:
            if sys.platform == "win32":
                os.system(f'rd /s /q "{path}"')
            else:
                os.system(f'rm -rf "{path}"')
        except Exception:  # noqa: BLE001
            pass


def uninstall_app(repo: str, *, yes: bool = False) -> None:
    """
    Uninstall an app that was previously installed with ``pyratatui install``.

    Args:
        repo: Repository name (the short name, not ``user/repo``).
        yes:  Skip the confirmation prompt.
    """
    # ── Find the installed app ────────────────────────────────────────────────
    all_apps = list_installed()
    matches = [a for a in all_apps if a.get("repo") == repo]

    if not matches:
        typer.echo(
            typer.style(f"Error: '{repo}' is not installed.", fg=typer.colors.RED),
            err=True,
        )
        raise typer.Exit(1)

    meta = matches[0]
    user: str = meta["user"]
    src_dir = app_dir(user, repo)

    # ── Confirm ───────────────────────────────────────────────────────────────
    if not yes:
        confirmed = typer.confirm(
            f"Uninstall '{repo}' (from {user}/{repo})? This cannot be undone."
        )
        if not confirmed:
            typer.echo("Aborted.")
            raise typer.Exit(0)

    # ── Remove source directory ───────────────────────────────────────────────
    if src_dir.exists():
        _force_rmtree(src_dir)
        typer.echo(f"  Removed {src_dir}")

    # ── Remove parent user directory if now empty ─────────────────────────────
    user_dir = src_dir.parent
    try:
        if user_dir.exists() and not any(user_dir.iterdir()):
            user_dir.rmdir()
    except OSError:
        pass

    # ── Remove executable wrapper ─────────────────────────────────────────────
    remove_wrapper(repo)

    # ── Remove metadata ───────────────────────────────────────────────────────
    from .utils import meta_file as _meta_file

    mf = _meta_file(user, repo)
    if mf.exists():
        mf.unlink()

    typer.echo(typer.style(f"\n✓ Uninstalled '{repo}'.", fg=typer.colors.GREEN))
