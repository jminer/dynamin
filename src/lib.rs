#[allow(dead_code)]

extern crate smallvec;
extern crate zaffre;

pub use self::control::{Control, Visibility};
pub use self::window::{Window, WindowBorderStyle};

pub mod button;
pub mod control;
pub mod window;

mod generic_backend;

#[cfg(windows)]
#[path = "windows_backend/mod.rs"]
pub mod backend;
#[cfg(unix)]
#[path = "gtk_backend/mod.rs"]
pub mod backend;

