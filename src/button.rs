/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::any::Any;
use std::ops::Deref;
use std::rc::Rc;

use crate::control::{Control, SubControl, SubControlData, SubControlRef, SubControlEvent};

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

#[non_exhaustive]
pub enum ButtonEvent {
    Clicked,
}

impl ButtonData {
    pub fn new() -> Self {
        ButtonData {
            sub_control: SubControlData::new(),
        }
    }

    fn on_event(&self, event: &mut Any) {
        match event.downcast_mut::<SubControlEvent>() {
            Some(SubControlEvent::MouseDown) => {
            }
            Some(SubControlEvent::MouseUp) => {
                let mut clicked_event = ButtonEvent::Clicked;
                self.sub_control.event_handlers().send(&mut clicked_event);
            }
            Some(SubControlEvent::MouseMoved) => {
            }
            _ => {}
        }
    }
}
