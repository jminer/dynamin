
extern crate dynamin2d;

use dynamin2d::{Point, Size};
use super::generic_backend::GenericWindowBackend;
use super::backend::WindowBackend;
use super::{Control, Visibility};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WindowBorderStyle {
    None,
    Normal,
    Tool,
}

pub struct Window<B: GenericWindowBackend = WindowBackend> {
    backend: B,
}

impl Window {
    pub fn new() -> Window {
        Window { backend: WindowBackend::new() }
    }

    pub fn set_text(&mut self, text: &str) {
        self.backend.set_text(text);
    }
}

impl Control for Window {
    fn visibility(&self) -> Visibility {
        self.backend.visibility()
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.backend.set_visibility(visibility)
    }

    fn location(&self) -> Point {
        unimplemented!();
    }

    fn set_location(&mut self, location: &Point) {
    }

    fn size(&self) -> Size {
        unimplemented!();
    }

    fn set_size(&mut self, size: &Size) {
    }

    fn repaint_later(&self) {
    }
}
