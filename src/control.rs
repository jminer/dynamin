
use zaffre::{Point2, Size2};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Invisible,
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
	size: Size2<f64>
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

