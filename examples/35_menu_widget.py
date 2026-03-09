"""Menu widget demo.

Keys:
  arrows  navigate
  enter   select
  esc     reset menu
  q       quit
"""

from __future__ import annotations

from pyratatui import (
    Block,
    Color,
    Constraint,
    Direction,
    Layout,
    Menu,
    MenuItem,
    MenuState,
    Paragraph,
    Style,
    Terminal,
)


def build_menu() -> list[MenuItem]:
    return [
        MenuItem.item("File", "file"),
        MenuItem.group(
            "Edit",
            [
                MenuItem.item("Copy", "copy"),
                MenuItem.item("Paste", "paste"),
            ],
        ),
        MenuItem.group(
            "View",
            [
                MenuItem.item("Toggle Sidebar", "toggle_sidebar"),
                MenuItem.item("Zoom In", "zoom_in"),
                MenuItem.item("Zoom Out", "zoom_out"),
            ],
        ),
    ]


def main() -> None:
    menu = (
        Menu()
        .highlight(Style().fg(Color.black()).bg(Color.cyan()))
        .dropdown_width(24)
        .dropdown_style(Style().bg(Color.dark_gray()))
    )
    state = MenuState(build_menu())
    state.activate()
    selected = "none"

    with Terminal() as term:
        while True:
            current_selected = selected

            def ui(frame, _selected=current_selected):
                rows = (
                    Layout()
                    .direction(Direction.Vertical)
                    .constraints(
                        [Constraint.length(1), Constraint.fill(1), Constraint.length(3)]
                    )
                    .split(frame.area)
                )
                frame.render_stateful_menu(menu, rows[0], state)
                frame.render_widget(
                    Block().bordered().title(" Menu Output "),
                    rows[1],
                )
                frame.render_widget(
                    Paragraph.from_string(f"Selected action: {_selected}"),
                    rows[1].inner(1, 1),
                )
                frame.render_widget(
                    Paragraph.from_string(
                        "Arrows navigate, Enter select, Esc reset, q quit"
                    ).style(Style().fg(Color.dark_gray())),
                    rows[2],
                )

            term.draw(ui)
            event = term.poll_event(timeout_ms=60)
            if not event:
                continue
            if event.code == "q":
                break

            for item in state.handle_key(event.code):
                if item.kind == "Selected" and item.data is not None:
                    selected = item.data


if __name__ == "__main__":
    main()
