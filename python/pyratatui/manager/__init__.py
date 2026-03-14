"""
pyratatui.manager — App manager subpackage.

Provides programmatic access to the same operations exposed by the CLI:

    from pyratatui.manager import install_app, uninstall_app, list_apps, run_app, app_info

All four functions mirror the CLI commands 1-to-1.
"""

from __future__ import annotations

from .install import install_app
from .list import list_apps
from .run import run_app
from .uninstall import uninstall_app
from .utils import app_dir, apps_root, bin_dir, list_installed, read_meta

__all__ = [
    "install_app",
    "uninstall_app",
    "list_apps",
    "run_app",
    "app_dir",
    "apps_root",
    "bin_dir",
    "list_installed",
    "read_meta",
]
