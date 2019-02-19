
extern crate event_loop;
extern crate dynamin;
extern crate zaffre;

use event_loop::EventLoop;
use dynamin::{Button, Control, Visibility, Window};
use zaffre::Point2;

fn main() {
    let mut win = Window::new();
    win.set_text("Dynamin Tester");
    win.set_visibility(Visibility::Visible);

    let b = Button::new();

    EventLoop::current().run();
}

