use pyo3::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    symbols::Marker,
    widgets::{
        canvas::{
            Canvas as RatatuiCanvas, Map as RatatuiMap, MapResolution as RatatuiMapResolution,
        },
        Widget,
    },
};

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MapResolution {
    inner: RatatuiMapResolution,
}

impl MapResolution {
    fn to_ratatui(self) -> RatatuiMapResolution {
        self.inner
    }
}

#[pymethods]
impl MapResolution {
    #[classattr]
    #[allow(non_snake_case)]
    pub fn Low() -> Self {
        Self {
            inner: RatatuiMapResolution::Low,
        }
    }

    #[classattr]
    #[allow(non_snake_case)]
    pub fn High() -> Self {
        Self {
            inner: RatatuiMapResolution::High,
        }
    }

    fn __repr__(&self) -> String {
        match self.inner {
            RatatuiMapResolution::Low => "MapResolution.Low".to_string(),
            RatatuiMapResolution::High => "MapResolution.High".to_string(),
        }
    }
}

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct Map {
    resolution: MapResolution,
}

#[pymethods]
impl Map {
    #[new]
    #[pyo3(signature = (resolution=None))]
    pub fn new(resolution: Option<&MapResolution>) -> Self {
        Self {
            resolution: resolution.copied().unwrap_or_else(MapResolution::Low),
        }
    }

    pub fn resolution(&self, resolution: &MapResolution) -> Self {
        let mut next = self.clone();
        next.resolution = *resolution;
        next
    }

    fn __repr__(&self) -> String {
        format!("Map(resolution={})", self.resolution.__repr__())
    }
}

impl Widget for &Map {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let resolution = self.resolution.to_ratatui();
        let marker = match resolution {
            RatatuiMapResolution::Low => Marker::Dot,
            RatatuiMapResolution::High => Marker::Braille,
        };

        RatatuiCanvas::default()
            .marker(marker)
            .x_bounds([-180.0, 180.0])
            .y_bounds([-90.0, 90.0])
            .paint(|ctx| {
                ctx.draw(&RatatuiMap {
                    resolution,
                    color: Color::White,
                });
            })
            .render(area, buf);
    }
}

pub fn register_map_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Map>()?;
    m.add_class::<MapResolution>()?;
    Ok(())
}
