use pyo3::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    widgets::{Borders, Paragraph, Widget},
};

use crate::widgets::Block;

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct Button {
    label: String,
    focused: bool,
    block: Option<Block>,
    borders: bool,
}

#[pymethods]
impl Button {
    #[new]
    #[pyo3(signature = (label))]
    pub fn new(label: String) -> Self {
        Self {
            label,
            focused: false,
            block: None,
            borders: true,
        }
    }

    #[getter]
    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn set_label(&self, label: String) -> Self {
        let mut next = self.clone();
        next.label = label;
        next
    }

    pub fn is_focused(&self) -> bool {
        self.focused
    }

    pub fn focus(&self) -> Self {
        self.set_focused(true)
    }

    pub fn unfocus(&self) -> Self {
        self.set_focused(false)
    }

    pub fn set_focused(&self, focused: bool) -> Self {
        let mut next = self.clone();
        next.focused = focused;
        next
    }

    pub fn block(&self, block: &Block) -> Self {
        let mut next = self.clone();
        next.block = Some(block.clone());
        next
    }

    #[pyo3(signature = (enabled=true))]
    pub fn borders(&self, enabled: bool) -> Self {
        let mut next = self.clone();
        next.borders = enabled;
        next
    }

    #[pyo3(signature = (code, ctrl=false, alt=false, shift=false))]
    pub fn handle_key(&self, code: &str, ctrl: bool, alt: bool, shift: bool) -> bool {
        if !self.focused || ctrl || alt || shift {
            return false;
        }
        matches!(code, "Enter" | " " | "Space")
    }

    fn __repr__(&self) -> String {
        format!("Button(label={:?}, focused={})", self.label, self.focused)
    }
}

impl Widget for &Button {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut block = self
            .block
            .as_ref()
            .map(Block::to_ratatui)
            .unwrap_or_default();

        if self.borders {
            block = block.borders(Borders::ALL);
        }

        if self.focused {
            block = block.border_style(Style::default().fg(Color::Yellow));
        }

        let mut paragraph = Paragraph::new(self.label.as_str())
            .alignment(Alignment::Center)
            .block(block);

        if self.focused {
            paragraph = paragraph.style(Style::default().add_modifier(Modifier::BOLD));
        }

        paragraph.render(area, buf);
    }
}

pub fn register_button_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Button>()?;
    Ok(())
}
