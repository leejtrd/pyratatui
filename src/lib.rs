// pyratatui/src/lib.rs
// Root module — wires all sub-modules into the Python extension module.

use pyo3::prelude::*;

mod backend;
mod bar_graph;
mod buffer;
mod button_widget;
mod canvas_widget;
mod chart_widget;
mod checkbox_widget;
mod effects;
mod errors;
mod image_widget;
mod layout;
mod logger;
mod map_widget;
mod markdown;
mod menu_widget;
mod piechart_widget;
mod popups;
mod prompts;
mod qrcode;
mod scrollview;
mod style;
mod terminal;
mod text;
mod textarea;
mod throbber_widget;
mod tree_widget;
mod widgets;

#[pymodule]
fn _pyratatui(py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    errors::register_errors(py, m)?;

    style::register_style(py, m)?;
    text::register_text(py, m)?;
    layout::register_layout(py, m)?;
    buffer::register_buffer(py, m)?;
    widgets::register_widgets(py, m)?;
    backend::register_backend(py, m)?;
    terminal::register_terminal(py, m)?;
    effects::register_effects(py, m)?;
    prompts::register_prompts(py, m)?;
    popups::register_popups(py, m)?;
    textarea::register_textarea(py, m)?;
    scrollview::register_scrollview(py, m)?;
    qrcode::register_qrcode(py, m)?;

    // ── Existing widget integrations ────────────────────────────────────────────
    bar_graph::register_bar_graph(py, m)?;
    tree_widget::register_tree_widget(py, m)?;
    markdown::register_markdown(py, m)?;
    logger::register_logger(py, m)?;
    image_widget::register_image_widget(py, m)?;

    // ── New v0.2.2 widgets ──────────────────────────────────────────────────────
    canvas_widget::register_canvas_widget(py, m)?;
    map_widget::register_map_widget(py, m)?;
    button_widget::register_button_widget(py, m)?;

    // ── New v0.2.4 widgets ──────────────────────────────────────────────────────
    throbber_widget::register_throbber_widget(py, m)?;
    menu_widget::register_menu_widget(py, m)?;
    piechart_widget::register_piechart_widget(py, m)?;
    checkbox_widget::register_checkbox_widget(py, m)?;
    chart_widget::register_chart_widget(py, m)?;

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__ratatui_version__", "0.30")?;

    Ok(())
}
