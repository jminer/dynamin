
use zaffre::{Point2, Size2};
use super::generic_backend::GenericWindowBackend;
use super::backend::WindowBackend;
use super::{Control, Visibility};

// TODO: screenshots of border styles
/// The style of border around a window.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum WindowBorderStyle {
    /// The window has no border around it.
    None,
    /// The window has a normal title bar and border around it.
    Normal,
    /// The window has a border suited for a non-modal auxiliary window.
    ///
    /// On Windows, the title bar is smaller, there is no minimize or maximize buttons, and the
    /// close button is smaller. On macOS, the corners may be square instead of rounded.
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

    fn resizable(&self) -> bool {
        self.backend.resizable()
    }

    fn set_resizable(&mut self, resizable: bool) {
        self.backend.set_resizable(resizable);
    }
}

impl Control for Window {
    fn visibility(&self) -> Visibility {
        self.backend.visibility()
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.backend.set_visibility(visibility)
    }

    fn location(&self) -> Point2<f64> {
        unimplemented!();
    }

    fn set_location(&mut self, location: &Point2<f64>) {
    }

    fn size(&self) -> Size2<f64> {
        unimplemented!();
    }

    fn set_size(&mut self, size: &Size2<f64>) {
    }

    fn repaint_later(&self) {
    }
}
