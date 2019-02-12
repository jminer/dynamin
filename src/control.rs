
use std::cell::Cell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use zaffre::{Point2, Size2};

/// Whether a control is visible or affects layout.
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    /// The control is painted and can be interacted with, as long as its parent is visible as well.
    Visible,
    /// The control isn't painted and can't receive focus or be interacted with, but it still takes
    /// up space in the layout.
    Invisible,
    /// The control is invisible and doesn't take up space in the layout. This state is about the
    /// same as removing the control from its parent.
    Gone,
}

pub trait Control {
    fn visibility(&self) -> Visibility;
    fn set_visibility(&self, visibility: Visibility);

    fn location(&self) -> Point2<f64>;
    fn set_location(&self, location: &Point2<f64>);

    fn size(&self) -> Size2<f64>;
    fn set_size(&self, size: &Size2<f64>);

    fn repaint_later(&self);
}

struct ChildrenVec {
    control: Cell<Option<Weak<Control>>>,
    vec: Cell<Vec<Rc<Control>>>,
}

pub struct SubControl(Rc<SubControlData>);

pub struct SubControlData {
    visibility: Cell<Visibility>,
    location: Cell<Point2<f64>>,
    size: Cell<Size2<f64>>,
    children: Cell<Vec<Rc<Control>>>,
    parent: Cell<Option<Weak<Control>>>,
    event_handlers: Cell<Vec<Box<SubControlEventHandler>>>,
    //draw_commands: Cell<Vec<DrawCommand>>,
    //elastic: Cell<(bool, bool)>,
}

pub enum SubControlEvent {
    Moved,
    Resized,
    MouseEntered,
    MouseLeft,
    MouseDown,
    MouseUp,
    MouseMoved,
    KeyDown,
    KeyUp,
}

type SubControlEventHandler = FnMut(&mut SubControlEvent);

impl SubControl {
    pub fn new() -> Self {
        SubControl(Rc::new(SubControlData::new()))
    }
}

impl From<SubControl> for Rc<Control> {
    fn from(self_: SubControl) -> Self {
        self_.0 as Rc<Control>
    }
}

impl Control for SubControlData {
    fn visibility(&self) -> Visibility {
        self.visibility.get()
    }

    fn set_visibility(&self, visibility: Visibility) {
        self.visibility.set(visibility);
    }

    fn location(&self) -> Point2<f64> {
        self.location.get()
    }

    fn set_location(&self, location: &Point2<f64>) {
        self.location.set(*location);
        self.repaint_later();
    }

    fn size(&self) -> Size2<f64> {
        self.size.get()
    }

    fn set_size(&self, size: &Size2<f64>) {
        self.size.set(*size);
        self.repaint_later();
    }

    fn repaint_later(&self) {
    }
}

impl SubControlData {
    pub fn new() -> Self {
        SubControlData {
            visibility: Cell::new(Visibility::Visible),
            location: Cell::new(Point2::new(0.0, 0.0)),
            size: Cell::new(Size2::new(50.0, 50.0)),
            children: Cell::new(vec![]),
            parent: Cell::new(None),
            event_handlers: Cell::new(vec![]),
        }
    }
}
