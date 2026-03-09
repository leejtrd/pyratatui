use std::cell::RefCell;

use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget, widgets::Widget};
use throbber_widgets_tui::{
    Set as RSet, Throbber as RThrobber, ThrobberState as RThrobberState, ARROW as RARROW,
    ASCII as RASCII, BLACK_CIRCLE as RBLACK_CIRCLE, BOX_DRAWING as RBOX_DRAWING,
    BRAILLE_EIGHT as RBRAILLE_EIGHT, BRAILLE_EIGHT_DOUBLE as RBRAILLE_EIGHT_DOUBLE,
    BRAILLE_ONE as RBRAILLE_ONE, BRAILLE_SIX as RBRAILLE_SIX,
    BRAILLE_SIX_DOUBLE as RBRAILLE_SIX_DOUBLE, CANADIAN as RCANADIAN, CLOCK as RCLOCK,
    DOUBLE_ARROW as RDOUBLE_ARROW, HORIZONTAL_BLOCK as RHORIZONTAL_BLOCK, OGHAM_A as ROGHAM_A,
    OGHAM_B as ROGHAM_B, OGHAM_C as ROGHAM_C, PARENTHESIS as RPARENTHESIS,
    QUADRANT_BLOCK as RQUADRANT_BLOCK, QUADRANT_BLOCK_CRACK as RQUADRANT_BLOCK_CRACK,
    VERTICAL_BLOCK as RVERTICAL_BLOCK, WHITE_CIRCLE as RWHITE_CIRCLE,
    WHITE_SQUARE as RWHITE_SQUARE,
};

use crate::style::Style;

#[pyclass(module = "pyratatui", unsendable)]
#[allow(clippy::struct_field_names)]
#[derive(Debug)]
pub struct Throbber {
    label: Option<String>,
    style: Option<Style>,
    throbber_style: Option<Style>,
    set_name: String,
    running: bool,
    speed: i8,
    state: RefCell<RThrobberState>,
}

impl Throbber {
    fn to_ratatui(&self) -> RThrobber<'static> {
        let mut widget = RThrobber::default().throbber_set(set_by_name(&self.set_name));
        if let Some(ref label) = self.label {
            widget = widget.label(label.clone());
        }
        if let Some(ref style) = self.style {
            widget = widget.style(style.inner);
        }
        if let Some(ref style) = self.throbber_style {
            widget = widget.throbber_style(style.inner);
        }
        widget
    }
}

#[pymethods]
impl Throbber {
    #[new]
    #[pyo3(signature = (label=None))]
    pub fn new(label: Option<String>) -> Self {
        Self {
            label,
            style: None,
            throbber_style: None,
            set_name: "braille_six".to_string(),
            running: true,
            speed: 1,
            state: RefCell::new(RThrobberState::default()),
        }
    }

    pub fn start(&mut self) {
        self.running = true;
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn set_speed(&mut self, speed: i8) {
        self.speed = speed;
    }

    pub fn set_style(&mut self, style: &Style) {
        self.style = Some(style.clone());
    }

    pub fn set_throbber_style(&mut self, style: &Style) {
        self.throbber_style = Some(style.clone());
    }

    #[pyo3(signature = (label=None))]
    pub fn set_label(&mut self, label: Option<String>) {
        self.label = label;
    }

    pub fn set_set(&mut self, name: &str) -> PyResult<()> {
        let normalized = normalize_set_name(name);
        if !is_valid_set_name(&normalized) {
            return Err(PyValueError::new_err(format!(
                "Unknown throbber set: {name}. \
                 Valid values: ascii, box_drawing, arrow, double_arrow, vertical_block, \
                 horizontal_block, quadrant_block, quadrant_block_crack, white_square, \
                 white_circle, black_circle, clock, braille_one, braille_six, \
                 braille_six_double, braille_eight, braille_eight_double, ogham_a, \
                 ogham_b, ogham_c, parenthesis, canadian"
            )));
        }
        self.set_name = normalized;
        Ok(())
    }

    #[getter]
    pub fn is_running(&self) -> bool {
        self.running
    }

    #[getter]
    pub fn speed(&self) -> i8 {
        self.speed
    }

    fn __repr__(&self) -> String {
        format!(
            "Throbber(label={:?}, running={}, speed={}, set={:?})",
            self.label, self.running, self.speed, self.set_name
        )
    }
}

impl Widget for &Throbber {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = self.state.borrow_mut();
        if self.running {
            state.calc_step(self.speed);
        }
        StatefulWidget::render(self.to_ratatui(), area, buf, &mut state);
    }
}

fn normalize_set_name(name: &str) -> String {
    name.trim().to_lowercase()
}

fn is_valid_set_name(name: &str) -> bool {
    matches!(
        name,
        "ascii"
            | "box_drawing"
            | "arrow"
            | "double_arrow"
            | "vertical_block"
            | "horizontal_block"
            | "quadrant_block"
            | "quadrant_block_crack"
            | "white_square"
            | "white_circle"
            | "black_circle"
            | "clock"
            | "braille_one"
            | "braille_six"
            | "braille_six_double"
            | "braille_eight"
            | "braille_eight_double"
            | "ogham_a"
            | "ogham_b"
            | "ogham_c"
            | "parenthesis"
            | "canadian"
    )
}

fn set_by_name(name: &str) -> RSet {
    match name {
        "ascii" => RASCII.clone(),
        "box_drawing" => RBOX_DRAWING.clone(),
        "arrow" => RARROW.clone(),
        "double_arrow" => RDOUBLE_ARROW.clone(),
        "vertical_block" => RVERTICAL_BLOCK.clone(),
        "horizontal_block" => RHORIZONTAL_BLOCK.clone(),
        "quadrant_block" => RQUADRANT_BLOCK.clone(),
        "quadrant_block_crack" => RQUADRANT_BLOCK_CRACK.clone(),
        "white_square" => RWHITE_SQUARE.clone(),
        "white_circle" => RWHITE_CIRCLE.clone(),
        "black_circle" => RBLACK_CIRCLE.clone(),
        "clock" => RCLOCK.clone(),
        "braille_one" => RBRAILLE_ONE.clone(),
        "braille_six_double" => RBRAILLE_SIX_DOUBLE.clone(),
        "braille_eight" => RBRAILLE_EIGHT.clone(),
        "braille_eight_double" => RBRAILLE_EIGHT_DOUBLE.clone(),
        "ogham_a" => ROGHAM_A.clone(),
        "ogham_b" => ROGHAM_B.clone(),
        "ogham_c" => ROGHAM_C.clone(),
        "parenthesis" => RPARENTHESIS.clone(),
        "canadian" => RCANADIAN.clone(),
        _ => RBRAILLE_SIX.clone(),
    }
}

pub fn register_throbber_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Throbber>()?;
    Ok(())
}
