"""Checkbox widget demo.

Keys:
  space/enter  toggle
  q            quit
"""

from __future__ import annotations

from pyratatui import (
    Block,
    Checkbox,
    Color,
    Constraint,
    Direction,
    Layout,
    Paragraph,
    Style,
    Terminal,
)


def main() -> None:
    checkbox = (
        Checkbox("Enable alerts", Checkbox.Unchecked)
        .checkbox_style(Style().fg(Color.cyan()).bold())
        .label_style(Style().fg(Color.white()))
        .checked_symbol("[x] ")
        .unchecked_symbol("[ ] ")
    )

    with Terminal() as term:
        while True:
            checked = checkbox.is_checked

            def ui(frame, _checked=checked, _checkbox=checkbox):
                rows = (
                    Layout()
                    .direction(Direction.Vertical)
                    .constraints(
                        [Constraint.fill(1), Constraint.length(3), Constraint.length(3)]
                    )
                    .split(frame.area)
                )
                frame.render_widget(
                    _checkbox.block(Block().bordered().title(" Settings ")),
                    rows[1],
                )
                frame.render_widget(
                    Paragraph.from_string(
                        f"checked={_checked}  [space/enter] toggle  [q] quit"
                    ).style(Style().fg(Color.dark_gray())),
                    rows[2],
                )

            term.draw(ui)
            event = term.poll_event(timeout_ms=80)
            if not event:
                continue
            if event.code == "q":
                break
            if event.code in {" ", "Enter"}:
                checkbox = checkbox.toggle()


if __name__ == "__main__":
    main()
