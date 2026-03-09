use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tui_checkbox::{
    Checkbox as RCheckbox, HorizontalAlignment as RHorizontalAlignment,
    LabelPosition as RLabelPosition, VerticalAlignment as RVerticalAlignment,
};

use crate::style::Style;
use crate::widgets::Block;

#[pyclass(module = "pyratatui", from_py_object)]
#[allow(clippy::struct_field_names)]
#[derive(Clone, Debug)]
pub struct Checkbox {
    label: String,
    checked: bool,
    block: Option<Block>,
    style: Option<Style>,
    checkbox_style: Option<Style>,
    label_style: Option<Style>,
    checked_symbol: Option<String>,
    unchecked_symbol: Option<String>,
    label_position: String,
    horizontal_alignment: String,
    vertical_alignment: String,
}

impl Checkbox {
    pub(crate) fn to_ratatui(&self) -> PyResult<RCheckbox<'_>> {
        let mut checkbox = RCheckbox::new(self.label.as_str(), self.checked)
            .label_position(parse_label_position(&self.label_position)?)
            .horizontal_alignment(parse_horizontal_alignment(&self.horizontal_alignment)?)
            .vertical_alignment(parse_vertical_alignment(&self.vertical_alignment)?);

        if let Some(ref block) = self.block {
            checkbox = checkbox.block(block.to_ratatui());
        }
        if let Some(ref style) = self.style {
            checkbox = checkbox.style(style.inner);
        }
        if let Some(ref style) = self.checkbox_style {
            checkbox = checkbox.checkbox_style(style.inner);
        }
        if let Some(ref style) = self.label_style {
            checkbox = checkbox.label_style(style.inner);
        }
        if let Some(ref symbol) = self.checked_symbol {
            checkbox = checkbox.checked_symbol(symbol.as_str());
        }
        if let Some(ref symbol) = self.unchecked_symbol {
            checkbox = checkbox.unchecked_symbol(symbol.as_str());
        }
        Ok(checkbox)
    }
}

#[pymethods]
impl Checkbox {
    #[classattr]
    #[allow(non_snake_case)]
    pub fn Checked() -> bool {
        true
    }

    #[classattr]
    #[allow(non_snake_case)]
    pub fn Unchecked() -> bool {
        false
    }

    #[new]
    #[pyo3(signature = (label, checked=false))]
    pub fn new(label: String, checked: bool) -> Self {
        Self {
            label,
            checked,
            block: None,
            style: None,
            checkbox_style: None,
            label_style: None,
            checked_symbol: None,
            unchecked_symbol: None,
            label_position: "right".to_string(),
            horizontal_alignment: "left".to_string(),
            vertical_alignment: "top".to_string(),
        }
    }

    pub fn set_label(&self, label: String) -> Self {
        let mut next = self.clone();
        next.label = label;
        next
    }

    pub fn set_checked(&self, checked: bool) -> Self {
        let mut next = self.clone();
        next.checked = checked;
        next
    }

    pub fn checked(&self) -> Self {
        self.set_checked(true)
    }

    pub fn unchecked(&self) -> Self {
        self.set_checked(false)
    }

    pub fn toggle(&self) -> Self {
        self.set_checked(!self.checked)
    }

    #[getter]
    pub fn is_checked(&self) -> bool {
        self.checked
    }

    pub fn block(&self, block: &Block) -> Self {
        let mut next = self.clone();
        next.block = Some(block.clone());
        next
    }

    pub fn style(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.style = Some(style.clone());
        next
    }

    pub fn checkbox_style(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.checkbox_style = Some(style.clone());
        next
    }

    pub fn label_style(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.label_style = Some(style.clone());
        next
    }

    pub fn checked_symbol(&self, symbol: String) -> Self {
        let mut next = self.clone();
        next.checked_symbol = Some(symbol);
        next
    }

    pub fn unchecked_symbol(&self, symbol: String) -> Self {
        let mut next = self.clone();
        next.unchecked_symbol = Some(symbol);
        next
    }

    pub fn label_position(&self, position: &str) -> PyResult<Self> {
        validate_label_position(position)?;
        let mut next = self.clone();
        next.label_position = normalize(position);
        Ok(next)
    }

    pub fn horizontal_alignment(&self, alignment: &str) -> PyResult<Self> {
        validate_horizontal_alignment(alignment)?;
        let mut next = self.clone();
        next.horizontal_alignment = normalize(alignment);
        Ok(next)
    }

    pub fn vertical_alignment(&self, alignment: &str) -> PyResult<Self> {
        validate_vertical_alignment(alignment)?;
        let mut next = self.clone();
        next.vertical_alignment = normalize(alignment);
        Ok(next)
    }

    fn __repr__(&self) -> String {
        format!("Checkbox(label={:?}, checked={})", self.label, self.checked)
    }
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}

fn parse_label_position(value: &str) -> PyResult<RLabelPosition> {
    match normalize(value).as_str() {
        "right" => Ok(RLabelPosition::Right),
        "left" => Ok(RLabelPosition::Left),
        "top" => Ok(RLabelPosition::Top),
        "bottom" => Ok(RLabelPosition::Bottom),
        _ => Err(PyValueError::new_err(
            "Invalid label_position. Use 'right', 'left', 'top', or 'bottom'",
        )),
    }
}

fn validate_label_position(value: &str) -> PyResult<()> {
    parse_label_position(value).map(|_| ())
}

fn parse_horizontal_alignment(value: &str) -> PyResult<RHorizontalAlignment> {
    match normalize(value).as_str() {
        "left" => Ok(RHorizontalAlignment::Left),
        "center" => Ok(RHorizontalAlignment::Center),
        "right" => Ok(RHorizontalAlignment::Right),
        _ => Err(PyValueError::new_err(
            "Invalid horizontal_alignment. Use 'left', 'center', or 'right'",
        )),
    }
}

fn validate_horizontal_alignment(value: &str) -> PyResult<()> {
    parse_horizontal_alignment(value).map(|_| ())
}

fn parse_vertical_alignment(value: &str) -> PyResult<RVerticalAlignment> {
    match normalize(value).as_str() {
        "top" => Ok(RVerticalAlignment::Top),
        "center" => Ok(RVerticalAlignment::Center),
        "bottom" => Ok(RVerticalAlignment::Bottom),
        _ => Err(PyValueError::new_err(
            "Invalid vertical_alignment. Use 'top', 'center', or 'bottom'",
        )),
    }
}

fn validate_vertical_alignment(value: &str) -> PyResult<()> {
    parse_vertical_alignment(value).map(|_| ())
}

pub fn register_checkbox_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Checkbox>()?;
    Ok(())
}
