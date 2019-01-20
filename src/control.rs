
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
    fn set_visibility(&mut self, visibility: Visibility);

    fn location(&self) -> Point2<f64>;
    fn set_location(&mut self, location: &Point2<f64>);

    fn size(&self) -> Size2<f64>;
    fn set_size(&mut self, size: &Size2<f64>);

    fn repaint_later(&self);
}

pub struct SubControl {
    visibility: Visibility,
    location: Point2<f64>,
    size: Size2<f64>,
}

impl Control for SubControl {
    fn visibility(&self) -> Visibility {
        self.visibility
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    fn location(&self) -> Point2<f64> {
        self.location
    }

    fn set_location(&mut self, location: &Point2<f64>) {
        self.location = *location;
        self.repaint_later();
    }

    fn size(&self) -> Size2<f64> {
        self.size
    }

    fn set_size(&mut self, size: &Size2<f64>) {
        self.size = *size;
        self.repaint_later();
    }

    fn repaint_later(&self) {
    }
}

