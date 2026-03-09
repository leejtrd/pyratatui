# Throbber Widget

`Throbber` is an animated spinner widget powered by `throbber-widgets-tui`.

## API

```python
from pyratatui import Throbber

th = Throbber("Loading")
th.start()
th.stop()
th.set_speed(2)
th.set_style(...)
th.set_throbber_style(...)
th.set_set("clock")
```

## Example

```python
from pyratatui import Block, Color, Style, Terminal, Throbber

th = Throbber("Syncing...")
th.set_set("clock")
th.set_throbber_style(Style().fg(Color.cyan()).bold())

with Terminal() as term:
    while True:
        def ui(frame):
            block = Block().bordered().title(" Throbber ")
            frame.render_widget(block, frame.area)
            frame.render_widget(th, block.inner(frame.area))
        term.draw(ui)
        ev = term.poll_event(timeout_ms=80)
        if ev and ev.code == "q":
            break
```

## Symbol Sets

Use `set_set(name)` with:

- `ascii`
- `box_drawing`
- `arrow`
- `double_arrow`
- `vertical_block`
- `horizontal_block`
- `quadrant_block`
- `quadrant_block_crack`
- `white_square`
- `white_circle`
- `black_circle`
- `clock`
- `braille_one`
- `braille_six`
- `braille_six_double`
- `braille_eight`
- `braille_eight_double`
- `ogham_a`
- `ogham_b`
- `ogham_c`
- `parenthesis`
- `canadian`
