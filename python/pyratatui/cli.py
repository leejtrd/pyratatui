#!/usr/bin/env python3
"""PyRatatui CLI."""

from __future__ import annotations

from pathlib import Path

import typer

app = typer.Typer(
    name="pyratatui",
    help="PyRatatui project utilities",
)

MAIN_TEMPLATE = '''#!/usr/bin/env python3
"""Minimal hello world PyRatatui app."""

from pyratatui import Block, Color, Paragraph, Style, Terminal


def main():
    """Run the application."""
    with Terminal() as term:
        while True:

            def ui(frame):
                frame.render_widget(
                    Paragraph.from_string("Hello, pyratatui! 🐀  Press q to quit.")
                    .block(Block().bordered().title("Hello World"))
                    .style(Style().fg(Color.cyan())),
                    frame.area,
                )

            term.draw(ui)
            ev = term.poll_event(timeout_ms=100)
            if ev and ev.code == "q":
                break


if __name__ == "__main__":
    main()
'''

README_TEMPLATE = """# {project_name}

A PyRatatui terminal user interface application.

Generated with `pyratatui init`.

## Setup

```bash
# Create virtual environment
python -m venv venv
source venv/bin/activate  # or: venv\\Scripts\\activate on Windows

# Install dependencies
pip install -r requirements.txt
```

## Run

```bash
python main.py
```

## Development

Edit `main.py` to customize your TUI application. See [PyRatatui documentation](https://docs.astral.sh/pyratatui/) for examples and API reference.

### Quick Start

```python
from pyratatui import Terminal, Paragraph

with Terminal() as term:
    term.draw(lambda frame: frame.render_widget(
        Paragraph.from_string("Your content here"),
        frame.area,
    ))
```

## Features

PyRatatui provides:
- Async-ready terminal framework
- Rich widgets (List, Table, Gauge, Sparkline, etc.)
- TachyonFX effects system
- Professional styling and colors
- Keyboard event handling
- Cross-platform support (Windows, macOS, Linux)
"""


@app.command()
def init(
    project_name: str = typer.Argument(..., help="Name of the new PyRatatui project"),
    verbose: bool = typer.Option(False, "--verbose", "-v", help="Show created files"),
) -> None:
    """Create a new PyRatatui project scaffold.

    Example:
        pyratatui init my-tui-app
    """
    project_path = Path(project_name)
    if project_path.exists():
        typer.echo(
            typer.style(
                f"Error: directory '{project_name}' already exists.",
                fg=typer.colors.RED,
            ),
            err=True,
        )
        raise typer.Exit(1)

    try:
        project_path.mkdir(parents=True)

        # ✅ UPDATED: Create files without version pinning for pyratatui
        files: dict[Path, str] = {
            project_path / "main.py": MAIN_TEMPLATE,
            project_path / "requirements.txt": "pyratatui\n",
            project_path
            / "README.md": README_TEMPLATE.format(project_name=project_name),
        }

        for file_path, content in files.items():
            file_path.write_text(content, encoding="utf-8")
            if verbose:
                typer.echo(f"created {file_path}")

        typer.echo(
            typer.style(f"✓ Created project '{project_name}'.", fg=typer.colors.GREEN)
        )
        typer.echo("\nNext steps:")
        typer.echo(f"  cd {project_name}")
        typer.echo("  pip install -r requirements.txt")
        typer.echo("  python main.py")
    except OSError as exc:
        typer.echo(typer.style(f"Error: {exc}", fg=typer.colors.RED), err=True)
        raise typer.Exit(1) from exc


@app.command()
def version() -> None:
    """Show PyRatatui version.

    Example:
        pyratatui version
    """
    try:
        import pyratatui

        typer.echo(f"PyRatatui {pyratatui.__version__}")
    except ImportError as exc:
        typer.echo("PyRatatui is not installed", err=True)
        raise typer.Exit(1) from exc


def main() -> None:
    """CLI entrypoint."""
    app()


if __name__ == "__main__":
    main()
