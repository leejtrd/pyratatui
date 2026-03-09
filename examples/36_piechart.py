"""Pie chart widget demo."""

from __future__ import annotations

from pyratatui import Block, Color, PieChart, PieData, PieStyle, Style, Terminal


def main() -> None:
    data = [
        PieData("Rust", 44.0, Color.red()),
        PieData("Python", 28.0, Color.yellow()),
        PieData("Go", 18.0, Color.cyan()),
        PieData("Other", 10.0, Color.green()),
    ]
    style = (
        PieStyle()
        .style(Style().fg(Color.white()))
        .show_legend(True)
        .show_percentages(True)
        .resolution("braille")
        .legend_position("right")
        .legend_layout("vertical")
        .legend_alignment("left")
    )
    chart = (
        PieChart(data)
        .block(Block().bordered().title(" Language Share "))
        .pie_style(style)
    )

    with Terminal() as term:
        while True:
            term.draw(lambda frame: frame.render_widget(chart, frame.area))
            event = term.poll_event(timeout_ms=120)
            if event and event.code in {"q", "Esc"}:
                break


if __name__ == "__main__":
    main()
