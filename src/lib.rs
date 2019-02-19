#[allow(dead_code)]

extern crate smallvec;
extern crate zaffre;

pub use self::button::{Button};
pub use self::control::{Control, set_tab_order, Visibility};
pub use self::window::{Window, WindowBorderStyle};

mod bitfield;
mod button;
mod control;
mod window;

mod generic_backend;

#[cfg(windows)]
#[path = "windows_backend/mod.rs"]
pub mod backend;
#[cfg(unix)]
#[path = "gtk_backend/mod.rs"]
pub mod backend;

