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
use dynamin::{Button, Control, Visibility, Window, WindowEvent};
use zaffre::Point2;

fn main() {
    let mut win = Window::new();
    win.set_text("Dynamin Tester");
    win.set_visibility(Visibility::Visible);
    win.event_handlers().add(|event| {
        match event.downcast_mut::<WindowEvent>() {
            Some(WindowEvent::Closing) => {
                println!("closing");
                std::process::exit(0);
            }
            _ => {}
        }
    });

    let b = Button::new();

    EventLoop::current().run();
}

