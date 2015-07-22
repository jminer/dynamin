
extern crate dynamin2d;

use dynamin2d::{Point, Size};
use super::Visibility;

//pub trait GenericCursorBackend {

//}

pub trait GenericWindowBackend {
	fn new() -> Self;
    //fn location(&self) -> Point;
    //fn set_location(&mut self, location: &Point);
    //fn size(&self) -> Size;
    //fn set_size(&mut self, size: &Size);
    fn set_text(&mut self, text: &str);

    fn visibility(&self) -> Visibility;

    fn set_visibility(&mut self, visibility: Visibility);
}
