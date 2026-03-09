# Menu Widget

`Menu` provides nested dropdown menus via `tui-menu`.

## Types

- `Menu` – widget style/configuration
- `MenuItem` – leaf/group node
- `MenuState` – mutable navigation + event state
- `MenuEvent` – emitted selection events

## Building Menus

```python
from pyratatui import MenuItem, MenuState

items = [
    MenuItem.item("File", "file"),
    MenuItem.group("Edit", [
        MenuItem.item("Copy", "copy"),
        MenuItem.item("Paste", "paste"),
    ]),
]
state = MenuState(items)
state.activate()
```

## Rendering

```python
from pyratatui import Menu

menu = Menu().dropdown_width(24)
frame.render_stateful_menu(menu, area, state)
```

## Input Handling

```python
events = state.handle_key(ev.code)
for e in events:
    if e.kind == "Selected":
        print("selected:", e.data)
```

Supported keys in `handle_key`:

- `Up`
- `Down`
- `Left`
- `Right`
- `Enter`
- `Esc`
