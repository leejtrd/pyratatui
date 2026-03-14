"""
pyratatui.core — Core application runtime helpers.

Exports:
    AsyncTerminal   — asyncio-compatible terminal driver
    run_app         — synchronous application loop helper
    run_app_async   — async application loop helper
"""

from __future__ import annotations

from ..async_terminal import AsyncTerminal
from ..helpers import run_app, run_app_async

__all__ = [
    "AsyncTerminal",
    "run_app",
    "run_app_async",
]
