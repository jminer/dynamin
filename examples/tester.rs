
extern crate event_loop;
extern crate dynamin;
extern crate dynamin2d;

use event_loop::EventLoop;
use dynamin::{Control, Visibility, Window};
use dynamin2d::Point;

fn main() {
    let mut win = Window::new();
    win.set_text("Dynamin Tester");
    win.set_visibility(Visibility::Visible);

    EventLoop::current().run();
}

