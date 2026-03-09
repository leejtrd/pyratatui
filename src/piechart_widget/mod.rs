use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use tui_piechart::{
    LegendAlignment as RLegendAlignment, LegendLayout as RLegendLayout,
    LegendPosition as RLegendPosition, PieChart as RPieChart, PieSlice as RPieSlice,
    Resolution as RResolution,
};

use crate::style::Color;
use crate::style::Style;
use crate::widgets::Block;

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct PieData {
    label: String,
    value: f64,
    color: Color,
}

impl PieData {
    fn to_ratatui(&self) -> RPieSlice<'_> {
        RPieSlice::new(self.label.as_str(), self.value, self.color.inner)
    }
}

#[pymethods]
impl PieData {
    #[new]
    pub fn new(label: String, value: f64, color: &Color) -> Self {
        Self {
            label,
            value,
            color: color.clone(),
        }
    }

    #[getter]
    pub fn label(&self) -> &str {
        &self.label
    }

    #[getter]
    pub fn value(&self) -> f64 {
        self.value
    }

    fn __repr__(&self) -> String {
        format!("PieData(label={:?}, value={})", self.label, self.value)
    }
}

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct PieStyle {
    style: Option<Style>,
    show_legend: bool,
    show_percentages: bool,
    pie_char: Option<char>,
    legend_marker: Option<String>,
    resolution: String,
    legend_position: String,
    legend_layout: String,
    legend_alignment: String,
}

impl PieStyle {
    fn apply<'a>(&'a self, mut chart: RPieChart<'a>) -> PyResult<RPieChart<'a>> {
        if let Some(ref style) = self.style {
            chart = chart.style(style.inner);
        }
        chart = chart
            .show_legend(self.show_legend)
            .show_percentages(self.show_percentages)
            .resolution(parse_resolution(&self.resolution)?)
            .legend_position(parse_legend_position(&self.legend_position)?)
            .legend_layout(parse_legend_layout(&self.legend_layout)?)
            .legend_alignment(parse_legend_alignment(&self.legend_alignment)?);
        if let Some(pie_char) = self.pie_char {
            chart = chart.pie_char(pie_char);
        }
        if let Some(ref marker) = self.legend_marker {
            chart = chart.legend_marker(marker.as_str());
        }
        Ok(chart)
    }
}

#[pymethods]
impl PieStyle {
    #[new]
    pub fn new() -> Self {
        Self {
            style: None,
            show_legend: true,
            show_percentages: true,
            pie_char: None,
            legend_marker: None,
            resolution: "standard".to_string(),
            legend_position: "right".to_string(),
            legend_layout: "vertical".to_string(),
            legend_alignment: "left".to_string(),
        }
    }

    pub fn style(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.style = Some(style.clone());
        next
    }

    pub fn show_legend(&self, show: bool) -> Self {
        let mut next = self.clone();
        next.show_legend = show;
        next
    }

    pub fn show_percentages(&self, show: bool) -> Self {
        let mut next = self.clone();
        next.show_percentages = show;
        next
    }

    pub fn pie_char(&self, value: &str) -> PyResult<Self> {
        let mut chars = value.chars();
        let first = chars
            .next()
            .ok_or_else(|| PyValueError::new_err("pie_char value cannot be empty"))?;
        if chars.next().is_some() {
            return Err(PyValueError::new_err(
                "pie_char value must contain exactly one character",
            ));
        }
        let mut next = self.clone();
        next.pie_char = Some(first);
        Ok(next)
    }

    pub fn legend_marker(&self, marker: String) -> Self {
        let mut next = self.clone();
        next.legend_marker = Some(marker);
        next
    }

    pub fn resolution(&self, mode: &str) -> PyResult<Self> {
        validate_resolution(mode)?;
        let mut next = self.clone();
        next.resolution = normalize(mode);
        Ok(next)
    }

    pub fn legend_position(&self, position: &str) -> PyResult<Self> {
        validate_legend_position(position)?;
        let mut next = self.clone();
        next.legend_position = normalize(position);
        Ok(next)
    }

    pub fn legend_layout(&self, layout: &str) -> PyResult<Self> {
        validate_legend_layout(layout)?;
        let mut next = self.clone();
        next.legend_layout = normalize(layout);
        Ok(next)
    }

    pub fn legend_alignment(&self, alignment: &str) -> PyResult<Self> {
        validate_legend_alignment(alignment)?;
        let mut next = self.clone();
        next.legend_alignment = normalize(alignment);
        Ok(next)
    }

    fn __repr__(&self) -> String {
        format!(
            "PieStyle(legend={}, percentages={}, resolution={:?})",
            self.show_legend, self.show_percentages, self.resolution
        )
    }
}

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct PieChart {
    data: Vec<PieData>,
    block: Option<Block>,
    pie_style: PieStyle,
}

impl PieChart {
    pub(crate) fn to_ratatui<'a>(&'a self) -> PyResult<RPieChart<'a>> {
        let slices: Vec<RPieSlice<'a>> = self.data.iter().map(PieData::to_ratatui).collect();
        let mut chart = RPieChart::new(slices);
        if let Some(ref block) = self.block {
            chart = chart.block(block.to_ratatui());
        }
        self.pie_style.apply(chart)
    }
}

#[pymethods]
impl PieChart {
    #[new]
    pub fn new(data: Vec<PieData>) -> Self {
        Self {
            data,
            block: None,
            pie_style: PieStyle::new(),
        }
    }

    pub fn data(&self, data: Vec<PieData>) -> Self {
        let mut next = self.clone();
        next.data = data;
        next
    }

    pub fn add_data(&self, data: &PieData) -> Self {
        let mut next = self.clone();
        next.data.push(data.clone());
        next
    }

    pub fn block(&self, block: &Block) -> Self {
        let mut next = self.clone();
        next.block = Some(block.clone());
        next
    }

    pub fn pie_style(&self, style: &PieStyle) -> Self {
        let mut next = self.clone();
        next.pie_style = style.clone();
        next
    }

    fn __repr__(&self) -> String {
        format!("PieChart(slices={})", self.data.len())
    }
}

fn normalize(value: &str) -> String {
    value.trim().to_lowercase()
}

fn parse_resolution(value: &str) -> PyResult<RResolution> {
    match normalize(value).as_str() {
        "standard" => Ok(RResolution::Standard),
        "braille" => Ok(RResolution::Braille),
        _ => Err(PyValueError::new_err(
            "Invalid resolution. Use 'standard' or 'braille'",
        )),
    }
}

fn validate_resolution(value: &str) -> PyResult<()> {
    parse_resolution(value).map(|_| ())
}

fn parse_legend_position(value: &str) -> PyResult<RLegendPosition> {
    match normalize(value).as_str() {
        "left" => Ok(RLegendPosition::Left),
        "right" => Ok(RLegendPosition::Right),
        "top" => Ok(RLegendPosition::Top),
        "bottom" => Ok(RLegendPosition::Bottom),
        _ => Err(PyValueError::new_err(
            "Invalid legend_position. Use 'left', 'right', 'top', or 'bottom'",
        )),
    }
}

fn validate_legend_position(value: &str) -> PyResult<()> {
    parse_legend_position(value).map(|_| ())
}

fn parse_legend_layout(value: &str) -> PyResult<RLegendLayout> {
    match normalize(value).as_str() {
        "vertical" => Ok(RLegendLayout::Vertical),
        "horizontal" => Ok(RLegendLayout::Horizontal),
        _ => Err(PyValueError::new_err(
            "Invalid legend_layout. Use 'vertical' or 'horizontal'",
        )),
    }
}

fn validate_legend_layout(value: &str) -> PyResult<()> {
    parse_legend_layout(value).map(|_| ())
}

fn parse_legend_alignment(value: &str) -> PyResult<RLegendAlignment> {
    match normalize(value).as_str() {
        "left" => Ok(RLegendAlignment::Left),
        "center" => Ok(RLegendAlignment::Center),
        "right" => Ok(RLegendAlignment::Right),
        _ => Err(PyValueError::new_err(
            "Invalid legend_alignment. Use 'left', 'center', or 'right'",
        )),
    }
}

fn validate_legend_alignment(value: &str) -> PyResult<()> {
    parse_legend_alignment(value).map(|_| ())
}

pub fn register_piechart_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PieData>()?;
    m.add_class::<PieStyle>()?;
    m.add_class::<PieChart>()?;
    Ok(())
}
