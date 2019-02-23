
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
        match event {
            WindowEvent::Closing => {
                println!("closing");
                std::process::exit(0);
            }
        }
    });

    let b = Button::new();

    EventLoop::current().run();
}

