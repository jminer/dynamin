
use std::ops::Deref;
use std::rc::Rc;

use zaffre::{Point2, Size2};

use control::{Control, SubControl, SubControlData, SubControlRef, SubControlEvent, Visibility};

#[derive(Clone)]
pub struct Button(Rc<ButtonData>);

impl Button {
    pub fn new() -> Self {
        SubControl::register_handle(Button(Rc::new(ButtonData::new())))
    }
}

impl Deref for Button {
    type Target = Rc<ButtonData>;
    fn deref(&self) -> &Rc<ButtonData> {
        &self.0
    }
}

impl From<Button> for Rc<Control> {
    fn from(self_: Button) -> Self {
        self_.0 as Rc<Control>
    }
}


pub struct ButtonData {
    sub_control: SubControlData,
}

impl SubControlRef for ButtonData {
    fn sub_control_ref(&self) -> &SubControlData { &self.sub_control }
}

enum ButtonEvent {
    SubControl(SubControlEvent),
    Clicked,
}

type ButtonEventHandler = FnMut(&mut ButtonEvent);

impl ButtonData {
    pub fn new() -> Self {
        ButtonData {
            sub_control: SubControlData::new(),
        }
    }
}