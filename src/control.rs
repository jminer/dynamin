
extern crate dynamin2d;

use dynamin2d::{Point, Size};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Visibility {
    Visible,
    Invisible,
    Gone,
}

pub trait Control {
    fn visibility(&self) -> Visibility;
    fn set_visibility(&mut self, visibility: Visibility);

    fn location(&self) -> Point;
    fn set_location(&mut self, location: &Point);

    fn size(&self) -> Size;
    fn set_size(&mut self, size: &Size);

    fn repaint_later(&self);
}

pub struct SubControl {
    visibility: Visibility,
	location: Point<f64>,
	size: Size<f64>
}

impl Control for SubControl {
    fn visibility(&self) -> Visibility {
        self.visibility
    }

    fn set_visibility(&mut self, visibility: Visibility) {
        self.visibility = visibility;
    }

    fn location(&self) -> Point {
        self.location
    }

	fn set_location(&mut self, location: &Point) {
		self.location = *location;
		self.repaint_later();
	}

    fn size(&self) -> Size {
        self.size
    }

	fn set_size(&mut self, size: &Size) {
		self.size = *size;
		self.repaint_later();
	}

    fn repaint_later(&self) {
    }
}

