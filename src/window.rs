
use std::cell::{RefCell};
use std::ops::Deref;
use std::rc::{Weak, Rc};

use zaffre::{Point2, Size2};

use crate::generic_backend::GenericWindowBackend;
use crate::backend::WindowBackend;
use crate::{ChildrenVec, Control, EventHandlerVec, Visibility};
use crate::control::PrivControl;

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

#[derive(Clone)]
pub struct Window(Rc<WindowData>);

impl Deref for Window {
    type Target = Rc<WindowData>;
    fn deref(&self) -> &Rc<WindowData> {
        &self.0
    }
}

// Can't implement `Clone` without cloning the native handle.
pub struct WindowData<B: GenericWindowBackend = WindowBackend> {
    backend: B,
    children: RefCell<ChildrenVec>,
    event_handlers: EventHandlerVec<WindowEvent>,
}

pub enum WindowEvent {
    // Triggered when the user clicks the close button on the window.
    Closing,
}

impl Window {
    pub fn new() -> Window {
        let handle = Window(Rc::new(WindowData {
            backend: WindowBackend::new(),
            children: RefCell::new(ChildrenVec::new()),
            event_handlers: EventHandlerVec::new(),
        }));
        handle.0.backend.set_window(Rc::downgrade(&handle.0));
        let control_handle = handle.0.clone() as Rc<Control>;
        control_handle.children().borrow_mut().control = Some(Rc::downgrade(&control_handle));
        handle
    }
}

impl<B: GenericWindowBackend> WindowData<B> {
    pub fn set_child(&self, child: Rc<Control>) {
        let mut children = self.children.borrow_mut();
        children.clear();
        children.push(child);
    }

    pub fn set_text(&self, text: &str) {
        self.backend.set_text(text);
    }

    fn resizable(&self) -> bool {
        self.backend.resizable()
    }

    fn set_resizable(&self, resizable: bool) {
        self.backend.set_resizable(resizable);
    }

    pub fn event_handlers(&self) -> &EventHandlerVec<WindowEvent> {
        &self.event_handlers
    }
}

impl PrivControl for WindowData {
    fn set_parent(&self, _parent: Weak<Control>) {
        panic!("a window does not have a parent")
    }
}

impl Control for WindowData {
    fn visibility(&self) -> Visibility {
        self.backend.visibility()
    }

    fn set_visibility(&self, visibility: Visibility) {
        self.backend.set_visibility(visibility)
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

    fn tab_index(&self) -> u16 { panic!("a window does not have a tab index") }

    fn set_tab_index(&self, _tab_index: u16) { panic!("a window does not have a tab index") }

    fn set_size(&self, size: &Size2<f64>) {
    }

    fn children(&self) -> &RefCell<ChildrenVec> {
        &self.children
    }

    fn repaint_later(&self) {
    }
}
