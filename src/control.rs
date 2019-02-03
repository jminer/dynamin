
use std::cell::Cell;
use std::ops::Deref;
use std::rc::Rc;

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

pub struct SubControl(Rc<Deref<Target = SubControlData>>);

pub struct SubControlData {
    visibility: Cell<Visibility>,
    location: Cell<Point2<f64>>,
    size: Cell<Size2<f64>>,
    //children: Cell<Vec<AnySubControl>>,
    //parent: Cell<Option<Control>>,
}

impl Deref for SubControlData {
    type Target = SubControlData;
    fn deref(&self) -> &SubControlData {
        &self
    }
}

impl SubControl {
    pub fn new() -> Self {
        SubControl(Rc::new(SubControlData {
            visibility: Cell::new(Visibility::Visible),
            location: Cell::new(Point2::new(0.0, 0.0)),
            size: Cell::new(Size2::new(50.0, 50.0)),
        }) as Rc<Deref<Target = SubControlData>>)
    }
}

impl Control for SubControl {
    fn visibility(&self) -> Visibility {
        self.0.visibility.get()
    }

    fn set_visibility(&self, visibility: Visibility) {
        self.0.visibility.set(visibility);
    }

    fn location(&self) -> Point2<f64> {
        self.0.location.get()
    }

    fn set_location(&self, location: &Point2<f64>) {
        self.0.location.set(*location);
        self.repaint_later();
    }

    fn size(&self) -> Size2<f64> {
        self.0.size.get()
    }

    fn set_size(&self, size: &Size2<f64>) {
        self.0.size.set(*size);
        self.repaint_later();
    }

    fn repaint_later(&self) {
    }
}

