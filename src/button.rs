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

use zaffre::text::{FormattedString, TextLayout, TextRectFramer};
use zaffre::{font, Brush, Color, PathBuf, Point2, Rect, Size2, StrokeStyle};

use crate::control::{
    Control, MouseUpEvent, PaintingEvent, SubControl, SubControlData, SubControlRef,
};

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

            painter.stroke_path(
                &mut path.path_iter(),
                &Brush::Solid(Color::from_rgba(128, 128, 128, 255)),
                &StrokeStyle::with_width(1.0),
            );

            let font_family = font::get_family("DejaVu Sans")
                .or_else(|| font::get_family("DejaVu Sans"))
                .or_else(|| font::get_family("Helvetica"))
                .or_else(|| font::get_family("Arial"))
                .expect("couldn't find font");
            let font = font_family.get_styles()[0].get_font(20.0);

            // painter.draw_glyphs(
            //     &[41, 42], &[Point2::new(0.0, 0.0), Point2::new(10.0, 0.0)], Point2::new(5.0, 5.0),
            //     &font, &Brush::Solid(Color::from_rgba(160, 50, 255, 255)));

            let mut text = FormattedString::new();
            text.clear_and_set_text_from(&"First".to_owned());
            text.set_initial_font(font);
            let mut layout = TextLayout::new();
            layout.set_text(text);
            layout.layout(&mut TextRectFramer::new(Rect::new(
                5.0, 5.0, 50_000.0, 30.0,
            )));
            layout.draw(&mut **painter);
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
