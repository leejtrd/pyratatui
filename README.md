<div align="center">

<img src="https://github.com/pyratatui/pyratatui/raw/main/gallery/pyratatui.png" alt="pyratatui logo" width="110" />

# pyratatui

**Python bindings for [ratatui](https://ratatui.rs) — high-performance terminal UIs, from Python.**

[![PyPI](https://img.shields.io/pypi/v/pyratatui?style=for-the-badge&logo=pypi&logoColor=white&color=3775A9)](https://pypi.org/project/pyratatui/)
[![Python](https://img.shields.io/badge/Python-3.10%20→%203.13-3776AB?style=for-the-badge&logo=python&logoColor=white)](https://www.python.org)
[![ratatui](https://img.shields.io/badge/ratatui-0.30-E06C00?style=for-the-badge)](https://ratatui.rs)
[![CI](https://img.shields.io/github/actions/workflow/status/pyratatui/pyratatui/ci.yml?style=for-the-badge&logo=githubactions&logoColor=white&label=CI)](https://github.com/pyratatui/pyratatui/actions)
[![License](https://img.shields.io/github/license/pyratatui/pyratatui?style=for-the-badge&color=brightgreen)](LICENSE)

<br/>

> **Zero runtime dependencies.** The wheel ships a self-contained native extension  
> compiled with [PyO3](https://pyo3.rs) 🦀 and [Maturin](https://github.com/PyO3/maturin).

<br/>

[**Quick Start**](#-quick-start) · [**Features**](#-feature-overview) · [**Examples**](#-examples) · [**Docs**](docs/) · [**Contributing**](#-contributing)

</div>

---

## Gallery

<div align="center">

| | | |
|:---:|:---:|:---:|
| ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_1.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_2.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_3.png) |
| ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_4.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_5.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_6.png) |
| ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_7.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_8.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_9.png) |
| ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_10.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_11.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_12.png) |
| ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_13.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_14.png) | ![](https://github.com/pyratatui/pyratatui/raw/main/gallery/snip_15.png) |

</div>

---

## Overview

**pyratatui** wraps the full **ratatui 0.30** widget library — including the new Calendar widget — plus the **tachyonfx animation engine** and **five third-party widget integrations**, all exposed through a clean, typed, idiomatic Python API.

It targets Python developers who want the rendering power and performance of Rust's leading TUI framework without writing a single line of Rust.

- 🏎️ **Native performance** — PyO3/Maturin compiled extension, no Python overhead in the render path
- 🎨 **Full ratatui parity** — every widget, layout primitive, and style option from ratatui 0.30
- ⚡ **Async-first** — `AsyncTerminal` with `async for ev in term.events(fps=30)`
- 🔒 **Typed** — complete `.pyi` stubs for IDE autocompletion and mypy

---

## ✨ What's New in 0.2.2

| Feature | Type | Description |
|:---|:---|:---|
| 🎨 **Canvas** | Widget | Draw lines, points, and rectangles with ratatui canvas primitives |
| 🗺️ **Map** | Widget | Render world map with `MapResolution.Low` / `MapResolution.High` |
| 🔘 **Button** | Widget | Focus-aware button rendering with keyboard handling |
| 🚀 **CLI Tool** | Tool | `pyratatui init <project>` for scaffolding new projects |
| 📝 **3 New Examples** | Examples | Canvas, Map, and Button widget demonstrations |

### CLI Quick Start

```bash
# Create a new project
pyratatui init my_tui_app
cd my_tui_app

# Install and run
pip install -r requirements.txt
python main.py
```

The generated project includes:
- `main.py` — Entry point with a working example
- `app.py` — Application state management pattern
- `requirements.txt` — Minimal dependencies
- `README.md` — Project documentation

---

## ✨ What's New in 0.2.1

| Widget | Source | Description |
|:---|:---|:---|
| 📊 **BarGraph** | `tui-bar-graph` | Colorful gradient bar graphs |
| 🌲 **Tree** | `tui-tree-widget` | Interactive hierarchical tree view |
| 📝 **Markdown** | `tui-markdown` | `markdown_to_text()` → styled `Text` |
| 📜 **Logger** | `tui-logger` | Real-time log viewer widget |
| 🖼️ **Image** | `ratatui-image` | Sixel / kitty / halfblock image display |
| 📅 **Calendar** | ratatui core | `Monthly`, `CalendarDate`, `CalendarEventStore` |
| 🔢 **30 Examples** | — | `01_hello_world.py` → `30_image_view.py` |

---

## 📦 Installation

```bash
pip install pyratatui
```

Pre-built wheels on PyPI for:

| Platform | Architectures |
|:---|:---|
| **Linux** | x86_64 |
| **macOS** | x86_64 (starting from v0.2.2), arm64 (Apple Silicon) |
| **Windows** | x86_64 |

**Python 3.10 – 3.13** supported.

<details>
<summary><strong>🔧 Build from source</strong> (requires Rust stable + Maturin)</summary>

```bash
pip install maturin
git clone https://github.com/pyratatui/pyratatui.git
cd pyratatui
maturin develop --release
```

</details>

---

## 🚀 Quick Start

```python
from pyratatui import Terminal, Paragraph, Block, Style, Color

with Terminal() as term:
    while True:
        def ui(frame):
            frame.render_widget(
                Paragraph.from_string("Hello, pyratatui! 🐀  Press q to quit.")
                    .block(Block().bordered().title("Hello World"))
                    .style(Style().fg(Color.cyan())),
                frame.area,
            )
        term.draw(ui)
        ev = term.poll_event(timeout_ms=100)
        if ev and ev.code == "q":
            break
```

---

## 🗺️ Feature Overview

| Category | Details |
|:---|:---|
| **Widgets** | `Block`, `Paragraph`, `List`, `Table`, `Gauge`, `LineGauge`, `BarChart`, `Sparkline`, `Tabs`, `Scrollbar`, `Clear`, `Monthly` |
| **Third-party** | `BarGraph`, `Tree`, `Logger`, `Image` (sixel/kitty/halfblocks), Markdown renderer |
| **Layout** | Constraint-based splits (length, %, fill, min, max, ratio), flex modes, margins, spacing |
| **Styling** | 16 named + 256-indexed + RGB true-colour, 9 text modifiers, immutable builder pattern |
| **Text** | `Span` → `Line` → `Text` hierarchy with per-span styling and alignment |
| **Async** | `AsyncTerminal` with `async for ev in term.events(fps=30)` |
| **Effects** | tachyonfx — fade, dissolve, coalesce, slide, sweep, sequence, parallel, ping-pong |
| **Prompts** | `TextPrompt`, `PasswordPrompt` with live validation |
| **Popups** | `Popup`, `PopupState` — floating centered dialogs, draggable |
| **TextArea** | Full multi-line editor (Emacs bindings, undo/redo, search) |
| **ScrollView** | `ScrollView`, `ScrollViewState` — scrollable content viewport |
| **QR Codes** | `QrCodeWidget`, `QrColors` — terminal QR via Unicode half-blocks |
| **Calendar** | `Monthly`, `CalendarDate`, `CalendarEventStore` |
| **Type stubs** | Complete `.pyi` for IDE completion and mypy |

---

## 📐 Layout & Widgets

```python
from pyratatui import (
    Terminal, Layout, Constraint, Direction,
    Block, Paragraph, List, ListItem, ListState,
)

with Terminal() as term:
    list_state = ListState()
    while True:
        def ui(frame):
            chunks = (
                Layout()
                .direction(Direction.Vertical)
                .constraints([
                    Constraint.length(3),
                    Constraint.fill(1),
                    Constraint.length(1),
                ])
                .split(frame.area)
            )
            frame.render_widget(
                Paragraph.from_string("My App").centered()
                    .block(Block().bordered().title(" Header ")),
                chunks[0],
            )
            frame.render_stateful_list(
                List([ListItem("Item A"), ListItem("Item B"), ListItem("Item C")]),
                chunks[1],
                list_state,
            )
            frame.render_widget(
                Paragraph.from_string("  q: quit  ↑/↓: navigate"),
                chunks[2],
            )
        term.draw(ui)
        ev = term.poll_event(timeout_ms=100)
        if ev:
            if ev.code == "q":      break
            elif ev.code == "Down": list_state.select_next()
            elif ev.code == "Up":   list_state.select_previous()
```

---

## ⚡ Async

```python
import asyncio
from pyratatui import AsyncTerminal, Paragraph, Block, Style, Color

async def main():
    tick = 0
    async with AsyncTerminal() as term:
        async for ev in term.events(fps=30, stop_on_quit=True):
            tick += 1
            def ui(frame, t=tick):
                frame.render_widget(
                    Paragraph.from_string(f"Tick {t} — press q to quit")
                        .block(Block().bordered())
                        .style(Style().fg(Color.green())),
                    frame.area,
                )
            term.draw(ui)

asyncio.run(main())
```

---

## 🎬 TachyonFX Animations

```python
from pyratatui import Terminal, Paragraph, Block, EffectManager, Motion, Interpolation

manager = EffectManager()
manager.add(
    "fade",
    "fade_from_fg #000000 300ms; then sweep_in_from_left 500ms",
)

with Terminal() as term:
    start = __import__("time").time()
    while True:
        def ui(frame):
            elapsed = int((__import__("time").time() - start) * 1000)
            frame.render_widget(
                Paragraph.from_string("Animated!").block(Block().bordered()),
                frame.area,
            )
            frame.apply_effect_manager(manager, elapsed, frame.area)
        term.draw(ui)
        ev = term.poll_event(timeout_ms=16)
        if ev and ev.code == "q":
            break
```

<details>
<summary><strong>Available effects</strong></summary>

| Effect | DSL keyword |
|:---|:---|
| Fade in/out | `fade_from_fg`, `fade_to_fg` |
| Dissolve | `dissolve` |
| Coalesce | `coalesce` |
| Slide | `slide_in`, `slide_out` |
| Sweep | `sweep_in_from_left`, `sweep_in_from_right`, … |
| Sequence | `effect_a; then effect_b` |
| Parallel | `effect_a | effect_b` |
| Ping-pong | `ping_pong(effect)` |

</details>

---

## 📅 Calendar Widget

```python
from pyratatui import CalendarDate, CalendarEventStore, Monthly, Block, Style, Color, Terminal

store = CalendarEventStore.today_highlighted(Style().fg(Color.green()).bold())
store.add(CalendarDate.from_ymd(2024, 12, 25), Style().fg(Color.red()).bold())

cal = (
    Monthly(CalendarDate.today(), store)
    .block(Block().bordered().title(" December 2024 "))
    .show_month_header(Style().bold().fg(Color.cyan()))
    .show_weekdays_header(Style().italic())
    .show_surrounding(Style().dim())
    .default_style(Style().fg(Color.white()))
)

with Terminal() as term:
    term.draw(lambda frame: frame.render_widget(cal, frame.area))
    term.poll_event(timeout_ms=10_000)
```

```bash
python examples/25_calendar.py
# ←/→: prev/next month   ↑/↓: prev/next year   t: today   q: quit
```

---

## 🖼️ QR Code Widget

```python
from pyratatui import QrCodeWidget, QrColors, Block, Terminal

qr = QrCodeWidget("https://ratatui.rs").colors(QrColors.Inverted)

with Terminal() as term:
    while True:
        def ui(frame, _qr=qr):
            blk = Block().bordered().title(" QR Code ")
            inner = blk.inner(frame.area)
            frame.render_widget(blk, frame.area)
            frame.render_qrcode(_qr, inner)
        term.draw(ui)
        ev = term.poll_event(timeout_ms=30_000)
        if ev and ev.code in ("q", "Esc"):
            break
```

---

## ✏️ TextArea Editor

```python
from pyratatui import TextArea, Block, Terminal

ta = TextArea.from_lines(["Edit this text...", "Line two"])
ta.set_block(Block().bordered().title(" Editor "))

with Terminal() as term:
    while True:
        def ui(frame, _ta=ta):
            frame.render_textarea(_ta, frame.area)
        term.draw(ui)
        ev = term.poll_event(timeout_ms=100)
        if ev:
            if ev.ctrl and ev.code == "c":
                break
            ta.input_key(ev.code, ev.ctrl, ev.alt, ev.shift)
```

> Supports Emacs key bindings, undo/redo, and incremental search out of the box.

---

## 📜 ScrollView

```python
from pyratatui import ScrollView, ScrollViewState, Terminal

lines = [f"Line {i:>4}:  " + "█" * (i % 40) for i in range(200)]
state = ScrollViewState()

with Terminal() as term:
    while True:
        def ui(frame, _lines=lines):
            sv = ScrollView.from_lines(_lines, content_width=80)
            frame.render_stateful_scrollview(sv, frame.area, state)
        term.draw(ui)
        ev = term.poll_event(timeout_ms=100)
        if ev:
            if ev.code == "q":      break
            elif ev.code == "Down": state.scroll_down(1)
            elif ev.code == "Up":   state.scroll_up(1)
```

---

## 📋 Cheat Sheet

<details>
<summary><strong>Expand API quick reference</strong></summary>

```python
# ── Style ──────────────────────────────────────────────────────────────────
Style().fg(Color.cyan()).bg(Color.black()).bold().italic().dim()

# ── Layout ─────────────────────────────────────────────────────────────────
Layout().direction(Direction.Vertical).constraints([
    Constraint.length(3),
    Constraint.fill(1),
    Constraint.percentage(30),
    Constraint.min(5),
    Constraint.max(20),
]).split(frame.area)

# ── Block with inner area ──────────────────────────────────────────────────
blk   = Block().bordered().title(" Panel ")
inner = blk.inner(area)           # Rect minus borders
frame.render_widget(blk, area)
frame.render_widget(content, inner)

# ── Table ──────────────────────────────────────────────────────────────────
Table([Row([Cell("A"), Cell("B")])]).column_widths([
    Constraint.fill(1), Constraint.length(8),
]).header(Row([Cell("Name"), Cell("Value")]))

# ── Calendar ───────────────────────────────────────────────────────────────
Monthly(CalendarDate.today(), CalendarEventStore.today_highlighted(
    Style().fg(Color.green()).bold()
)).show_month_header(Style().bold())

# ── QR Code ────────────────────────────────────────────────────────────────
frame.render_qrcode(QrCodeWidget("https://example.com"), area)

# ── ScrollView ─────────────────────────────────────────────────────────────
frame.render_stateful_scrollview(ScrollView.from_lines(lines, 80), area, state)

# ── TextArea ───────────────────────────────────────────────────────────────
frame.render_textarea(ta, area)
```

</details>

---

## 📚 Examples

30 numbered, runnable examples covering every feature:

| # | File | Demonstrates |
|:---:|:---|:---|
| 01 | `01_hello_world.py` | `Terminal`, `Paragraph`, `Block`, `Style`, `Color` |
| 02 | `02_layout.py` | `Layout`, `Constraint`, `Direction` |
| 03 | `03_styled_text.py` | `Span`, `Line`, `Text`, `Modifier` |
| 04 | `04_list_navigation.py` | `List`, `ListState`, keyboard navigation |
| 05 | `05_progress_bar.py` | `Gauge`, `LineGauge`, live updates |
| 06 | `06_table_dynamic.py` | `Table`, `TableState`, dynamic rows |
| 07 | `07_async_reactive.py` | `AsyncTerminal`, asyncio, reactive data |
| 08 | `08_effects_fade.py` | `EffectManager`, fade-in/out |
| 09 | `09_effects_dsl.py` | `compile_effect` DSL |
| 10 | `10_full_app.py` | Multi-tab app with all widgets |
| 11 | `11_popup_basic.py` | `Popup` (stateless centered) |
| 12 | `12_popup_stateful.py` | `PopupState` (draggable) |
| 13 | `13_popup_scrollable.py` | Scrollable popup content |
| 14 | `14_textarea_basic.py` | `TextArea` basics (Emacs bindings) |
| 15 | `15_textarea_advanced.py` | `TextArea` Vim modal mode |
| 16 | `16_scrollview.py` | `ScrollView`, `ScrollViewState` |
| 17 | `17_qrcode.py` | `QrCodeWidget`, `QrColors` |
| 18 | `18_async_progress.py` | `AsyncTerminal` + async progress |
| 19 | `19_effects_glitch.py` | Glitch effects |
| 20 | `20_effects_matrix.py` | Matrix rain effect |
| 21 | `21_prompt_confirm.py` | `PasswordPrompt` |
| 22 | `22_prompt_select.py` | Select prompt |
| 23 | `23_prompt_text.py` | `TextPrompt` |
| 24 | `24_dashboard.py` | Full monitoring dashboard |
| **25** | **`25_calendar.py`** | **`Monthly`, `CalendarEventStore`, navigation** |
| 26-30 | `26_bar_graph.py` … `30_image_view.py` | BarGraph, Tree, Logger, Image widgets |

---

## 🏗️ Project Structure

```
pyratatui/
├── Cargo.toml                  # Rust package (v0.2.1)
├── pyproject.toml              # Python package metadata
│
├── src/                        # Rust → Python bindings (PyO3)
│   ├── lib.rs                  # Extension module entry point
│   ├── terminal/               # Terminal, Frame, KeyEvent
│   ├── widgets/                # Block, Paragraph, List, Table, Monthly, …
│   ├── style/                  # Color, Modifier, Style
│   ├── text/                   # Span, Line, Text
│   ├── layout/                 # Rect, Constraint, Direction, Layout
│   ├── effects/                # TachyonFX bindings
│   ├── popups/                 # Popup, PopupState, KnownSizeWrapper
│   ├── textarea/               # TextArea, CursorMove, Scrolling
│   ├── scrollview/             # ScrollView, ScrollViewState
│   └── qrcode/                 # QrCodeWidget, QrColors
│
├── python/pyratatui/
│   ├── __init__.py             # Python re-exports
│   ├── __init__.pyi            # Complete type stubs
│   ├── async_terminal.py       # AsyncTerminal
│   └── helpers.py              # run_app, run_app_async
│
├── examples/                   # 30 numbered examples (01–30)
├── docs/                       # MkDocs Material documentation
├── scripts/
│   └── build.sh / build.ps1
│   └── format.sh / format.ps1
└── tests/
    ├── python/                 # pytest
    └── rust/                   # cargo test
```

---

## 🤝 Contributing

```bash
git clone https://github.com/pyratatui/pyratatui.git
cd pyratatui
python -m venv .venv && source .venv/bin/activate
pip install maturin pytest pytest-asyncio ruff mypy
maturin develop
pytest tests/python/
cargo test
```

**Lint & format:**

```bash
ruff check . && ruff format .
cargo fmt && cargo clippy -- -D warnings
```

All PRs welcome — bug fixes, new widget bindings, documentation, and examples.

---

## 📄 License

MIT — see [LICENSE](LICENSE)

<div align="center">

<br/>

Built with [ratatui](https://ratatui.rs) 🐀 and [PyO3](https://pyo3.rs) 🦀

<br/>

*If pyratatui saves you time, consider giving the repo a ⭐*

</div>
