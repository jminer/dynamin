/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/.
 *
 */

extern crate event_loop;
extern crate dynamin;
extern crate zaffre;

use event_loop::EventLoop;
use dynamin::{Button, Control, SubControl, Visibility, Window, WindowEvent, ClickEvent, MouseEnteredEvent, MouseLeftEvent};
use zaffre::{Point2, Size2};

fn main() {
    let mut win = Window::new();
    win.set_text("Dynamin Tester");
    win.set_visibility(Visibility::Visible);
    win.event_handlers().add(|route| {
        match route.event.downcast_mut::<WindowEvent>() {
            Some(WindowEvent::Closing) => {
                println!("closing");
                std::process::exit(0);
            }
            _ => {}
        }
    });

    let parent = SubControl::new();
    parent.set_size(&Size2::<f64>::new(200.0, 200.0));

    let b = Button::new();
    b.set_location(&Point2::<f64>::new(50.0, 50.0));
    b.set_size(&Size2::<f64>::new(75.0, 23.0));
    b.on_click_event(|_| {
        println!("clicked 1");
    });
    parent.children().borrow_mut().push(b.clone());

    let b = Button::new();
    b.set_location(&Point2::<f64>::new(50.0, 90.0));
    b.set_size(&Size2::<f64>::new(75.0, 23.0));
    b.on_click_event(|_| {
        println!("clicked 2");
    });
    b.event_handlers().add(|route| {
        if let Some(_) = route.event.downcast_mut::<MouseEnteredEvent>() {
            println!("entered 2");
        }
        if let Some(_) = route.event.downcast_mut::<MouseLeftEvent>() {
            println!("left 2");
        }
    });
    parent.children().borrow_mut().push(b.clone());

    win.set_child(parent.into());

    EventLoop::current().run();
}

