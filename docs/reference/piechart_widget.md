# PieChart Widget

`PieChart` renders pie charts with legend and percentage controls via `tui-piechart`.

## Types

- `PieChart`
- `PieData`
- `PieStyle`

## Quickstart

```python
from pyratatui import Color, PieChart, PieData, PieStyle

data = [
    PieData("Rust", 45.0, Color.red()),
    PieData("Python", 30.0, Color.yellow()),
    PieData("Go", 25.0, Color.cyan()),
]

style = (
    PieStyle()
    .show_legend(True)
    .show_percentages(True)
    .resolution("braille")
    .legend_position("right")
    .legend_layout("vertical")
    .legend_alignment("left")
)

chart = PieChart(data).pie_style(style)
```

Render:

```python
frame.render_widget(chart, area)
```

## `PieStyle` Options

- `show_legend(bool)`
- `show_percentages(bool)`
- `resolution("standard" | "braille")`
- `legend_position("left" | "right" | "top" | "bottom")`
- `legend_layout("vertical" | "horizontal")`
- `legend_alignment("left" | "center" | "right")`
- `pie_char(single_character)`
- `legend_marker(str)`
