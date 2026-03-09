# Chart Widget

`Chart` binds the built-in `ratatui::widgets::Chart` API for cartesian plots.

## Types

- `Chart`
- `Dataset`
- `Axis`
- `GraphType` (`Scatter`, `Line`, `Bar`)
- `Marker`
- `LegendPosition`

## Multi-Dataset Example

```python
from pyratatui import Axis, Chart, Dataset, GraphType, Marker

line_points = [(0.0, 1.0), (1.0, 3.0), (2.0, 2.0), (3.0, 5.0)]
scatter_points = [(0.5, 1.4), (1.5, 2.3), (2.5, 3.1)]
bar_points = [(0.0, 1.0), (1.0, 2.2), (2.0, 1.8), (3.0, 2.9)]

datasets = [
    Dataset(line_points).name("Line").graph_type(GraphType.Line).marker(Marker.Braille),
    Dataset(scatter_points).name("Scatter").graph_type(GraphType.Scatter).marker(Marker.Dot),
    Dataset(bar_points).name("Bars").graph_type(GraphType.Bar).marker(Marker.Bar),
]

chart = (
    Chart(datasets)
    .x_axis(Axis().title("X").bounds(0.0, 3.0).labels(["0", "1.5", "3"]))
    .y_axis(Axis().title("Y").bounds(0.0, 6.0).labels(["0", "3", "6"]))
)
```

Render:

```python
frame.render_widget(chart, area)
```

## Axis Notes

- `Axis.bounds(min, max)` defines visible range.
- `Axis.labels([...])` sets tick labels.
- `Axis.labels_alignment("left" | "center" | "right")` controls label alignment.

## Legend

Use `chart.legend_position(...)` with:

- `LegendPosition.Top`
- `LegendPosition.TopRight`
- `LegendPosition.TopLeft`
- `LegendPosition.Left`
- `LegendPosition.Right`
- `LegendPosition.Bottom`
- `LegendPosition.BottomRight`
- `LegendPosition.BottomLeft`

Set `None` to hide the legend:

```python
chart = chart.legend_position(None)
```
