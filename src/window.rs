
use std::rc::Rc;

use zaffre::{Point2, Size2};
use super::generic_backend::GenericWindowBackend;
use super::backend::WindowBackend;
use super::{Control, Visibility};
use super::control::SubControl;

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

pub struct Window(Rc<WindowData>);

pub struct WindowData<B: GenericWindowBackend = WindowBackend> {
    backend: B,
    contents: Option<SubControl>,
}

impl Window {
    pub fn new() -> Window {
        Window(Rc::new(WindowData { backend: WindowBackend::new(), contents: None }))
    }

    pub fn set_text(&self, text: &str) {
        self.0.backend.set_text(text);
    }

    fn resizable(&self) -> bool {
        self.0.backend.resizable()
    }

    fn set_resizable(&self, resizable: bool) {
        self.0.backend.set_resizable(resizable);
    }
}

impl Control for Window {
    fn visibility(&self) -> Visibility {
        self.0.backend.visibility()
    }

    fn set_visibility(&self, visibility: Visibility) {
        self.0.backend.set_visibility(visibility)
    }

    fn location(&self) -> Point2<f64> {
        unimplemented!();
    }

    /// Depending on the platform, the location may not be updated immediately (getting the location
    /// just after setting it may return the previous value). When it is updated, it may not be
    /// what it was set to. Most platforms have limits on where windows can be positioned. For
    /// example, the top edge of a window can't be off screen.
    fn set_location(&self, location: &Point2<f64>) {
    }

    fn size(&self) -> Size2<f64> {
        unimplemented!();
    }

    fn set_size(&self, size: &Size2<f64>) {
    }

    fn repaint_later(&self) {
    }
}
