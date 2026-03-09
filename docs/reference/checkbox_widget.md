# Checkbox Widget

`Checkbox` is a configurable checkbox widget backed by `tui-checkbox`.

## States

- `Checkbox.Checked`
- `Checkbox.Unchecked`

## API

```python
from pyratatui import Checkbox

cb = Checkbox("Enable alerts", Checkbox.Unchecked)
cb = cb.toggle()
cb = cb.set_checked(True)
cb = cb.checked()
cb = cb.unchecked()
```

## Styling

```python
from pyratatui import Color, Style

cb = (
    Checkbox("Accept terms")
    .checkbox_style(Style().fg(Color.green()).bold())
    .label_style(Style().fg(Color.white()))
    .checked_symbol("[x] ")
    .unchecked_symbol("[ ] ")
    .label_position("right")
    .horizontal_alignment("left")
    .vertical_alignment("top")
)
```

Render:

```python
frame.render_widget(cb, area)
```
