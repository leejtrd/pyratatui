use pyo3::prelude::*;
use tui_menu::{
    Menu as RMenu, MenuEvent as RMenuEvent, MenuItem as RMenuItem, MenuState as RMenuState,
};

use crate::style::Style;

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct MenuItem {
    name: String,
    data: Option<String>,
    children: Vec<MenuItem>,
}

impl MenuItem {
    fn to_ratatui(&self) -> RMenuItem<String> {
        if self.children.is_empty() {
            let data = self.data.clone().unwrap_or_else(|| self.name.clone());
            RMenuItem::item(self.name.clone(), data)
        } else {
            let children = self.children.iter().map(MenuItem::to_ratatui).collect();
            RMenuItem::group(self.name.clone(), children)
        }
    }
}

#[pymethods]
impl MenuItem {
    #[new]
    #[pyo3(signature = (name, data=None, children=None))]
    pub fn new(name: String, data: Option<String>, children: Option<Vec<MenuItem>>) -> Self {
        Self {
            name,
            data,
            children: children.unwrap_or_default(),
        }
    }

    #[staticmethod]
    pub fn item(name: String, data: String) -> Self {
        Self {
            name,
            data: Some(data),
            children: Vec::new(),
        }
    }

    #[staticmethod]
    pub fn group(name: String, children: Vec<MenuItem>) -> Self {
        Self {
            name,
            data: None,
            children,
        }
    }

    pub fn with_child(&self, child: MenuItem) -> MenuItem {
        let mut next = self.clone();
        next.children.push(child);
        next
    }

    #[getter]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[getter]
    pub fn data(&self) -> Option<String> {
        self.data.clone()
    }

    #[getter]
    pub fn children(&self) -> Vec<MenuItem> {
        self.children.clone()
    }

    pub fn is_group(&self) -> bool {
        !self.children.is_empty()
    }

    fn __repr__(&self) -> String {
        format!(
            "MenuItem(name={:?}, children={})",
            self.name,
            self.children.len()
        )
    }
}

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct MenuEvent {
    #[pyo3(get)]
    pub kind: String,
    #[pyo3(get)]
    pub data: Option<String>,
}

impl MenuEvent {
    fn from_ratatui(event: RMenuEvent<String>) -> Self {
        match event {
            RMenuEvent::Selected(data) => Self {
                kind: "Selected".to_string(),
                data: Some(data),
            },
        }
    }
}

#[pymethods]
impl MenuEvent {
    #[staticmethod]
    pub fn selected(data: String) -> Self {
        Self {
            kind: "Selected".to_string(),
            data: Some(data),
        }
    }

    fn __repr__(&self) -> String {
        format!("MenuEvent(kind={:?}, data={:?})", self.kind, self.data)
    }
}

#[pyclass(module = "pyratatui", from_py_object)]
#[derive(Clone, Debug)]
pub struct Menu {
    default_style: Option<Style>,
    highlight_style: Option<Style>,
    dropdown_width: Option<u16>,
    dropdown_style: Option<Style>,
}

impl Menu {
    pub(crate) fn to_ratatui(&self) -> RMenu<String> {
        let mut menu = RMenu::new();
        if let Some(ref style) = self.default_style {
            menu = menu.default_style(style.inner);
        }
        if let Some(ref style) = self.highlight_style {
            menu = menu.highlight(style.inner);
        }
        if let Some(width) = self.dropdown_width {
            menu = menu.dropdown_width(width);
        }
        if let Some(ref style) = self.dropdown_style {
            menu = menu.dropdown_style(style.inner);
        }
        menu
    }
}

#[pymethods]
impl Menu {
    #[new]
    pub fn new() -> Self {
        Self {
            default_style: None,
            highlight_style: None,
            dropdown_width: None,
            dropdown_style: None,
        }
    }

    pub fn default_style(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.default_style = Some(style.clone());
        next
    }

    pub fn highlight(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.highlight_style = Some(style.clone());
        next
    }

    pub fn dropdown_width(&self, width: u16) -> Self {
        let mut next = self.clone();
        next.dropdown_width = Some(width);
        next
    }

    pub fn dropdown_style(&self, style: &Style) -> Self {
        let mut next = self.clone();
        next.dropdown_style = Some(style.clone());
        next
    }

    fn __repr__(&self) -> String {
        format!("Menu(dropdown_width={:?})", self.dropdown_width)
    }
}

#[pyclass(module = "pyratatui", unsendable)]
pub struct MenuState {
    pub(crate) inner: RMenuState<String>,
}

#[pymethods]
impl MenuState {
    #[new]
    pub fn new(items: Vec<MenuItem>) -> Self {
        let rat_items = items.into_iter().map(|item| item.to_ratatui()).collect();
        Self {
            inner: RMenuState::new(rat_items),
        }
    }

    pub fn activate(&mut self) {
        self.inner.activate();
    }

    pub fn is_active(&self) -> bool {
        self.inner.is_active()
    }

    pub fn up(&mut self) {
        self.inner.up();
    }

    pub fn down(&mut self) {
        self.inner.down();
    }

    pub fn left(&mut self) {
        self.inner.left();
    }

    pub fn right(&mut self) {
        self.inner.right();
    }

    pub fn select(&mut self) {
        self.inner.select();
    }

    pub fn push(&mut self) -> bool {
        self.inner.push().is_some()
    }

    pub fn pop(&mut self) {
        self.inner.pop();
    }

    pub fn reset(&mut self) {
        self.inner.reset();
    }

    pub fn drain_events(&mut self) -> Vec<MenuEvent> {
        self.inner
            .drain_events()
            .map(MenuEvent::from_ratatui)
            .collect()
    }

    pub fn handle_key(&mut self, code: &str) -> Vec<MenuEvent> {
        match code {
            "Up" => self.inner.up(),
            "Down" => self.inner.down(),
            "Left" => self.inner.left(),
            "Right" => self.inner.right(),
            "Enter" => self.inner.select(),
            "Esc" => self.inner.reset(),
            _ => {}
        }
        self.drain_events()
    }

    fn __repr__(&self) -> String {
        format!("MenuState(active={})", self.inner.is_active())
    }
}

pub fn register_menu_widget(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Menu>()?;
    m.add_class::<MenuItem>()?;
    m.add_class::<MenuState>()?;
    m.add_class::<MenuEvent>()?;
    Ok(())
}
