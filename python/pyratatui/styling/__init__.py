"""
pyratatui.styling — Style, color, and text primitives.

All names are re-exported from the compiled Rust extension.

    from pyratatui.styling import Color, Style, Modifier, Span, Line, Text
"""

from __future__ import annotations

from .._pyratatui import Color, Line, Modifier, Span, Style, Text  # noqa: F401

__all__ = [
    "Color",
    "Line",
    "Modifier",
    "Span",
    "Style",
    "Text",
]
