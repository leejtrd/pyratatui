"""Throbber widget demo.

Keys:
  space   start/stop
  +       speed up
  -       slow down
  q       quit
"""

from __future__ import annotations

from pyratatui import (
    Block,
    Color,
    Constraint,
    Direction,
    Layout,
    Paragraph,
    Style,
    Terminal,
    Throbber,
)


def main() -> None:
    throbber = Throbber("Loading data...")
    throbber.set_throbber_style(Style().fg(Color.cyan()).bold())
    throbber.set_style(Style().fg(Color.white()))
    throbber.set_set("clock")

    speed = 1

    with Terminal() as term:
        while True:
            running = throbber.is_running

            def ui(frame, _running=running, _speed=speed):
                areas = (
                    Layout()
                    .direction(Direction.Vertical)
                    .constraints(
                        [
                            Constraint.fill(1),
                            Constraint.length(3),
                            Constraint.length(3),
                            Constraint.fill(1),
                        ]
                    )
                    .split(frame.area)
                )
                frame.render_widget(
                    Block().bordered().title(" Throbber "),
                    areas[1],
                )
                frame.render_widget(throbber, areas[1].inner(1, 1))
                frame.render_widget(
                    Paragraph.from_string(
                        f"running={_running}  speed={_speed}  [space] toggle  [+/-] speed  [q] quit"
                    ).style(Style().fg(Color.dark_gray())),
                    areas[2],
                )

            term.draw(ui)
            event = term.poll_event(timeout_ms=80)
            if not event:
                continue
            if event.code == "q":
                break
            if event.code == " ":
                if throbber.is_running:
                    throbber.stop()
                else:
                    throbber.start()
            elif event.code in {"+", "="}:
                speed += 1
                throbber.set_speed(speed)
            elif event.code == "-":
                speed = max(1, speed - 1)
                throbber.set_speed(speed)


if __name__ == "__main__":
    main()
