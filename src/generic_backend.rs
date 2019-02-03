
use zaffre::{Point2, Size2};
use super::Visibility;

//pub trait GenericCursorBackend {

//}

pub trait GenericWindowBackend {
    fn new() -> Self;
    //fn location(&self) -> Point2<f64>;
    //fn set_location(&mut self, location: &Point2<f64>);
    //fn size(&self) -> Size2<f64>;
    //fn set_size(&mut self, size: &Size2<f64>);
    fn set_text(&self, text: &str);

    fn visibility(&self) -> Visibility;

    fn set_visibility(&self, visibility: Visibility);

    fn set_location(&self, location: &Point2<f64>);

    fn resizable(&self) -> bool;

    fn set_resizable(&self, resizable: bool);
}
