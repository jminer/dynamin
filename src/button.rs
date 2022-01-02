/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

use std::any::Any;
use std::ops::Deref;
use std::rc::Rc;
use std::slice::SliceIndex;

use zaffre::{Brush, Color, PathBuf, Point2, Size2, StrokeStyle};

use crate::control::{Control, PaintingEvent, SubControl, SubControlData, SubControlEvent, SubControlRef, MouseUpEvent};
use crate::event_vec::{EventRoute, EventHandler};

// TODO: generate with a proc macro
// start proc macro generated
#[derive(Clone)]
pub struct Button(Rc<ButtonData>);

impl Button {
    pub fn new() -> Self {
        SubControl::register_handle(Button(Rc::new(ButtonData::new())))
    }
}

impl Default for Button {
    fn default() -> Self {
        Self::new()
    }
}

impl Deref for Button {
    type Target = Rc<ButtonData>;
    fn deref(&self) -> &Rc<ButtonData> {
        &self.0
    }
}

impl From<Button> for Rc<dyn Control> {
    fn from(self_: Button) -> Self {
        self_.0 as Rc<dyn Control>
    }
}

impl From<Button> for Rc<dyn EventHandler> {
    fn from(self_: Button) -> Self {
        self_.0 as Rc<dyn EventHandler>
    }
}
// end proc macro generated


//#[dynamin::control]
pub struct ButtonData {
    sub_control: SubControlData,
}

impl SubControlRef for ButtonData {
    fn sub_control_ref(&self) -> &SubControlData { &self.sub_control }
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ButtonEvent {
    Clicked,
}

#[derive(Debug)]
#[non_exhaustive]
pub struct ClickEvent;

impl EventHandler for ButtonData {
    fn on_event(&self, route: &mut EventRoute) {
        if let Some(MouseUpEvent { .. }) = route.event.downcast_mut() {
            let mut event = ClickEvent;
            self.event_handlers().send(&mut event);
        }

        if let Some(PaintingEvent { painter }) = route.event.downcast_mut() {
            let size = self.sub_control.size();
            let size = Size2::new(size.width as f32, size.height as f32);

            let mut path = PathBuf::new();
            path.move_to(Point2::new(0.5f32, 0.5f32));
            path.line_to(Point2::new(size.width - 0.5, 0.5f32));
            path.line_to(Point2::new(size.width - 0.5, size.height - 0.5));
            path.line_to(Point2::new(0.5f32, size.height - 0.5));
            path.close();

            painter.stroke_path(&mut path.path_iter(),
                &Brush::Solid(Color::from_rgba(128, 128, 128, 255)),
                &StrokeStyle::with_width(1.0));
        }
        match route.event.downcast_mut::<SubControlEvent>() {
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

impl ButtonData {
    pub fn new() -> Self {
        ButtonData {
            sub_control: SubControlData::new(),
        }
    }

    // Convenience method to add an event handler that is called for `ClickEvent`s.
    pub fn on_click_event<F>(&self, mut handler: F)
        where F: for<'a> FnMut(&'a mut EventRoute) + 'static
    {
        self.event_handlers().add(move |mut route| {
            if let Some(_) = route.event.downcast_mut::<ClickEvent>() {
                handler(&mut route);
            }
        });
    }
}
