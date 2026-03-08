# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.2.2] — 2026-03-08

### Summary

`0.2.2` adds three new widgets and a new CLI project scaffold command:
`Canvas`, `Map`, `Button`, and `pyratatui init`.

---

### Added

#### 🎨 Canvas Widget

New drawing widget backed by `ratatui::widgets::canvas::Canvas`.

```python
from pyratatui import Canvas, Terminal

canvas = Canvas(width=60, height=20)
canvas.draw_line(0, 0, 20, 10)
canvas.draw_point(5, 5)
canvas.draw_rect(10, 5, 20, 6)

with Terminal() as term:
    term.draw(lambda frame: frame.render_widget(canvas, frame.area))
```

Features:
- `Canvas.draw_line(x1, y1, x2, y2)` — Draw line primitive
- `Canvas.draw_point(x, y)` — Draw point primitive
- `Canvas.draw_rect(x, y, w, h)` — Draw rectangles
- `Canvas.clear()` — Remove all shapes

#### 🗺️ Map Widget

World map widget backed by `ratatui::widgets::canvas::Map`.

```python
from pyratatui import Map, MapResolution, Terminal

map_widget = Map(resolution=MapResolution.High)

with Terminal() as term:
    term.draw(lambda frame: frame.render_widget(map_widget, frame.area))
```

Features:
- `MapResolution.Low`
- `MapResolution.High`
- `Map(resolution=...)`

#### 🔘 Button Widget

Interactive button widget with focus-aware rendering.

```python
from pyratatui import Button, Terminal

button = Button("Save")

with Terminal() as term:
    term.draw(lambda frame: frame.render_widget(button, frame.area))
```

Features:
- `Button.focus()` / `Button.unfocus()` — Manage focus state
- `Button.is_focused()` — Query focus state
- `Button.set_label(new_label)` — Update button text
- `Button.handle_key(code, ctrl=False, alt=False, shift=False)` — keyboard interaction helper

#### 🚀 CLI Tool

New `pyratatui` command-line tool for project initialization.

```bash
pyratatui init my_app
cd my_app
pip install -r requirements.txt
python main.py
```

Creates a structured project with:
- `main.py` — Immediately runnable starter UI
- `app.py` — App module scaffold
- `requirements.txt` — Dependencies
- `README.md` — Project documentation

Usage: `pyratatui init <project_name>`

#### 📝 Examples

New examples for all widgets:
- `31_canvas_drawing.py` — Drawing primitives and shapes
- `32_map_widget.py` — World map rendering
- `33_button_widget.py` — Focus + keyboard button demo

### Documentation

- Updated `README.md` with 0.2.2 feature summary and screenshots
- Updated docs pages to include the 0.2.2 additions

---

## [0.2.1] — 2026-03-07

### Summary

`0.2.1` is a major widget integration release for terminal-only use.
Five new third-party widget crates have been integrated:
**tui-bar-graph** (gradient bar graphs), **tui-tree-widget** (interactive tree views),
**tui-markdown** (Markdown → Text conversion), **tui-logger** (real-time log viewer),
and **ratatui-image** (terminal image display). The web/WASM/Ratzilla code has been
**fully removed** — pyratatui is now a pure terminal TUI library. All 30 examples
are numbered sequentially.

---

### Added

#### 📊 BarGraph Widget (`tui-bar-graph`)

Colorful gradient bar graph widget using the `colorgrad` crate for gradients.

```python
from pyratatui import BarGraph, BarGraphStyle, BarColorMode

graph = (
    BarGraph([0.1, 0.4, 0.9, 0.6, 0.3])
    .bar_style(BarGraphStyle.Braille)
    .color_mode(BarColorMode.VerticalGradient)
    .gradient("viridis")
)
frame.render_widget(graph, area)
```

#### 🌲 Tree Widget (`tui-tree-widget`)

Interactive hierarchical tree view with keyboard navigation.

```python
from pyratatui import Tree, TreeItem, TreeState

items = [
    TreeItem("Documents", [TreeItem("report.txt"), TreeItem("notes.md")]),
    TreeItem("Downloads"),
]
tree  = Tree(items).block(Block().bordered().title(" Files "))
state = TreeState()
state.select([0])
frame.render_stateful_tree(tree, area, state)
```

#### 📝 Markdown Renderer (`tui-markdown`)

Convert Markdown text to a styled `Text` object for terminal rendering.

```python
from pyratatui import markdown_to_text, Paragraph

text = markdown_to_text("# Hello\n\n**bold** and *italic*")
para = Paragraph(text).wrap(True)
frame.render_widget(para, area)
```

#### 📜 Logger Widget (`tui-logger`)

Real-time scrollable log viewer with level filtering.

```python
from pyratatui import TuiLoggerWidget, TuiWidgetState, init_logger, log_message

init_logger("debug")
log_message("info", "Application started")

widget = TuiLoggerWidget().block(Block().bordered())
state  = TuiWidgetState()
frame.render_stateful_logger(widget, area, state)
```

#### 🖼 Image Widget (`ratatui-image`)

Display PNG/JPEG images using unicode halfblocks or native graphics protocols.

```python
from pyratatui import ImagePicker, ImageWidget

picker = ImagePicker.halfblocks()
state  = picker.load("./photo.png")
widget = ImageWidget()
frame.render_stateful_image(widget, area, state)
```

#### 📅 Calendar Widget

Three new Python types for the built-in ratatui calendar widget:
`CalendarDate`, `CalendarEventStore`, `Monthly`.

---

### Changed

- 30 examples numbered `01_hello_world.py` through `30_image_view.py`
- `tests/python/test_pyratatui.py` expanded with tests for all new widgets
- `docs/` updated to reflect terminal-only operation and all new widgets

---

### Added

#### 📅 Calendar Widget

Three new Python types expose ratatui's built-in calendar widget:

**`CalendarDate`** — wraps `time::Date`. Represents a calendar day.

```python
from pyratatui import CalendarDate

today = CalendarDate.today()                    # UTC today
d     = CalendarDate.from_ymd(2024, 3, 15)     # March 15, 2024
print(d.year, d.month, d.day)                   # 2024 3 15
```

- `from_ymd(year, month, day)` raises `ValueError` for invalid dates.
- Implements `__hash__` and `__eq__` for use as dict/set keys.

**`CalendarEventStore`** — maps `CalendarDate` keys to `Style` values.
Implements ratatui's `DateStyler` trait.

```python
from pyratatui import CalendarEventStore, Style, Color

store = CalendarEventStore()
store.add(CalendarDate.from_ymd(2024, 12, 25), Style().fg(Color.red()).bold())
store.add_today(Style().fg(Color.green()).bold())

# Factory with today pre-highlighted
store = CalendarEventStore.today_highlighted(Style().fg(Color.cyan()).bold())
```

**`Monthly`** — the calendar widget. Fully composable builder.

```python
from pyratatui import Monthly, Block, Style, Color

cal = (
    Monthly(CalendarDate.today(), store)
    .block(Block().bordered().title(" My Calendar "))
    .show_month_header(Style().bold().fg(Color.cyan()))
    .show_weekdays_header(Style().italic().fg(Color.white()))
    .show_surrounding(Style().dim())
    .default_style(Style().fg(Color.white()))
)
frame.render_widget(cal, area)
```

Builder methods: `.block()`, `.default_style()`, `.show_surrounding()`,
`.show_month_header()`, `.show_weekdays_header()`. All return a new instance.

**New files:**
- `src/widgets/calendar.rs` — Rust bindings
- `docs/reference/calendar.md` — reference documentation
- `examples/25_calendar.py` — interactive demo (month/year nav, event styling)

**Modified files:**
- `Cargo.toml` — added `time = { version = "0.3", features = ["local-offset"] }`
- `src/widgets/mod.rs` — `calendar` module declared; types re-exported
- `src/terminal/mod.rs` — `Monthly` added to `render_widget()` dispatch
- `python/pyratatui/__init__.py` — `CalendarDate`, `CalendarEventStore`, `Monthly` exported
- `python/pyratatui/__init__.pyi` — full typed stubs for all three classes

---

### Changed

- `Cargo.toml` — version `0.2.0` → `0.2.1`
- `pyproject.toml` — version `0.2.0` → `0.2.1`; description updated
- `src/widgets/mod.rs` — `pub mod calendar` declared; `CalendarDate`,
  `CalendarEventStore`, `Monthly` added to `pub use` re-exports
- `src/terminal/mod.rs` — `Monthly` imported; `try_widget!(Monthly, to_ratatui)`
  inserted in `render_widget()` dispatch
- `mkdocs.yml` — Calendar and Web TUI entries added under *API Reference*;
  `site_description` updated to reference ratatui 0.30
- `README.md` — fully rewritten: ratatui 0.30, all widgets, calendar, web
  module, updated examples table (01–26), corrected code snippets
- `docs/index.md` — version badges corrected (0.29 → 0.30 / 0.2.0 → 0.2.1);
  feature table updated; architecture diagram updated
- `docs/installation.md` — verify snippet updated to expect `"0.2.1"` / `"0.30"`
- `docs/reference/api_surface.md` — Calendar and web module rows added to
  Core Types, Widgets, and Terminal & Frame tables
- `docs/examples/minimal_examples.md` — Calendar and Web counter added as
  examples 25 and 26; stale `Layout.default()` calls fixed to `Layout()`
- `docs/examples/advanced_examples.md` — Calendar advanced section added;
  outdated `Table(rows, widths)` constructor corrected
- `docs/faq.md` — web module Q&A section added; ratatui 0.29 references
  updated to 0.30

---

### Fixed

- `docs/index.md` — stale ratatui 0.29 badge and version strings
- `docs/installation.md` — `__ratatui_version__` verify showed `"0.29"`, now `"0.30"`
- `docs/reference/api_surface.md` — missing `Popup`, `TextArea`, `ScrollView`,
  `QrCodeWidget` rows in widget table; all now present
- `docs/examples/minimal_examples.md` — `Layout.default()` (non-existent)
  replaced with `Layout()` in inline snippets
- `docs/examples/advanced_examples.md` — `Table(rows, widths)` constructor
  replaced with `Table(rows).column_widths(widths)` throughout
- `README.md` — `Constraint.Length` / `Constraint.Min` (uppercase 0.1.x API)
  replaced with lowercase `Constraint.length` / `Constraint.min`

---

## [0.2.0] — 2026-03-06

### Summary

`0.2.0` upgrades the native backend to **ratatui 0.30** and resolves all
compile-time errors and deprecation warnings introduced by the upgrade.
Adds `Block.inner(area)` and fixes all example API mismatches.

### Changed (breaking — Rust internals only)

- **ratatui 0.30** is now the native backend (`ratatui-core 0.1`, `ratatui-widgets 0.3`).
- `Terminal::area()` returns `Rect` converted from ratatui 0.30's `Size` type.
- `Frame::apply_effect` no longer requires `tachyonfx::Shader` in scope.
- `LineGauge::line_set()` uses `filled_symbol` / `unfilled_symbol` internally.
- `Table::row_highlight_style()` replaces deprecated `highlight_style()`.
- `Table::to_ratatui()` returns `RTable<'_>` (lifetime-aware) instead of `'static`.

### Added

- **`Block.inner(area: Rect) -> Rect`** — compute inner area after borders/padding.
- `ratatui_compat` internal bridge for tui-textarea 0.7 (ratatui 0.29) types.
- Examples renumbered sequentially (01–24) with no gaps.

### Fixed

- Example 06, 10 — `Table(rows, widths, header=…)` → `Table(rows).column_widths(…).header(…)`
- Example 15 — `Layout.default()` → `Layout()`; `Constraint.Min` → `Constraint.min`
- Examples 16, 17 — `Block.inner(area)` now available; `Layout.default()` fixed
- 18 compile errors from ratatui 0.29 → 0.30 resolved
- 54 deprecation warnings resolved (PyO3 0.28 API, tui-scrollview 0.6.2, etc.)

---

## [0.1.3] — 2026-03-06

### Summary

`0.1.3` adds four major third-party widget integrations:

| New module | Backing crate | What it adds |
|---|---|---|
| `Popup` family | `tui-popup` 0.7 | Centered floating dialogs with drag/scroll |
| `TextArea` family | `tui-textarea` 0.7 | Full multi-line text editor |
| `ScrollView` family | `tui-scrollview` 0.6 | Scrollable viewport |
| `QrCodeWidget` family | `tui-qrcode` / native | Terminal QR code renderer |

### Added

- `Popup`, `PopupState`, `KnownSizeWrapper` — floating dialog widgets
- `TextArea`, `CursorMove`, `Scrolling` — rich multi-line editor with Emacs bindings
- `ScrollView`, `ScrollViewState` — scrollable content viewport
- `QrCodeWidget`, `QrColors` — native Unicode half-block QR renderer
- `Frame` methods: `render_popup`, `render_stateful_popup`, `render_textarea`,
  `render_stateful_scrollview`, `render_qrcode`
- Examples 11–17 (popups, textarea, scrollview, qrcode)
- Reference docs: `popups.md`, `textarea.md`, `scrollview.md`, `qrcode.md`

---

## [0.1.2] — Earlier

_(Changelog entries for 0.1.2 and earlier are tracked in git history.)_
