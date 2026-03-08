#!/usr/bin/env python3
"""Button widget demo."""

from pyratatui import Button, Terminal


def main() -> None:
    button = Button("Submit").focus()
    pressed = False

    with Terminal() as term:
        while True:
            label = "Submitted!" if pressed else "Submit"
            ui_button = button.set_label(label)
            term.draw(
                lambda frame, widget=ui_button: frame.render_widget(widget, frame.area)
            )

            event = term.poll_event(timeout_ms=100)
            if event is None:
                continue
            if event.code in {"q", "Esc"}:
                break
            if event.code == "Tab":
                button = button.unfocus() if button.is_focused() else button.focus()
                continue
            if button.handle_key(event.code, event.ctrl, event.alt, event.shift):
                pressed = not pressed


if __name__ == "__main__":
    main()
