
use std::ops::Deref;
use std::rc::Rc;

use zaffre::{Point2, Size2};

use control::{Control, SubControlData, SubControlEvent, Visibility};

struct Button(Rc<ButtonData>);

struct ButtonData {
    control: SubControlData,
}

enum ButtonEvent {
    SubControl(SubControlEvent),
    Clicked,
}

type ButtonEventHandler = FnMut(&mut ButtonEvent);

impl From<Button> for Rc<Control> {
    fn from(self_: Button) -> Self {
        self_.0 as Rc<Control>
    }
}

impl Control for ButtonData {
    fn visibility(&self) -> Visibility { self.control.visibility() }
    fn set_visibility(&self, visibility: Visibility) { self.control.set_visibility(visibility) }

    fn location(&self) -> Point2<f64> { self.control.location() }
    fn set_location(&self, location: &Point2<f64>) { self.control.set_location(location) }

    fn size(&self) -> Size2<f64> { self.control.size() }
    fn set_size(&self, size: &Size2<f64>) { self.control.set_size(size) }

    fn repaint_later(&self) { self.repaint_later() }
}

impl Button {
    fn new() -> Self {
        Button(Rc::new(ButtonData {
            control: SubControlData::new()
        }))
    }
}
