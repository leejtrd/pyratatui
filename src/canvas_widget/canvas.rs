use pyo3::prelude::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Color,
    widgets::{
        canvas::{Canvas as RatatuiCanvas, Line, Points, Rectangle},
        Widget,
    },
};

#[derive(Clone, Debug)]
enum CanvasShape {
    Line { x1: f64, y1: f64, x2: f64, y2: f64 },
    Point { x: f64, y: f64 },
    Rect { x: f64, y: f64, w: f64, h: f64 },
}

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug, Default)]
pub struct Canvas {
    width: u16,
    height: u16,
    shapes: Vec<CanvasShape>,
}

#[pymethods]
impl Canvas {
    #[new]
    #[pyo3(signature = (width, height))]
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            shapes: Vec::new(),
        }
    }

    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.shapes.push(CanvasShape::Line { x1, y1, x2, y2 });
    }

    pub fn draw_point(&mut self, x: f64, y: f64) {
        self.shapes.push(CanvasShape::Point { x, y });
    }

    pub fn draw_rect(&mut self, x: f64, y: f64, w: f64, h: f64) {
        self.shapes.push(CanvasShape::Rect { x, y, w, h });
    }

    pub fn clear(&mut self) {
        self.shapes.clear();
    }

    #[getter]
    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    fn __repr__(&self) -> String {
        format!(
            "Canvas(width={}, height={}, shapes={})",
            self.width,
            self.height,
            self.shapes.len()
        )
    }
}

impl Widget for &Canvas {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let x_bounds = [0.0, f64::from(self.width.max(1))];
        let y_bounds = [0.0, f64::from(self.height.max(1))];

        RatatuiCanvas::default()
            .x_bounds(x_bounds)
            .y_bounds(y_bounds)
            .paint(|ctx| {
                for shape in &self.shapes {
                    match shape {
                        CanvasShape::Line { x1, y1, x2, y2 } => {
                            ctx.draw(&Line {
                                x1: *x1,
                                y1: *y1,
                                x2: *x2,
                                y2: *y2,
                                color: Color::White,
                            });
                        }
                        CanvasShape::Point { x, y } => {
                            let coords = [(*x, *y)];
                            ctx.draw(&Points {
                                coords: &coords,
                                color: Color::White,
                            });
                        }
                        CanvasShape::Rect { x, y, w, h } => {
                            ctx.draw(&Rectangle {
                                x: *x,
                                y: *y,
                                width: *w,
                                height: *h,
                                color: Color::White,
                            });
                        }
                    }
                }
            })
            .render(area, buf);
    }
}

pub fn register_canvas_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Canvas>()?;
    Ok(())
}
