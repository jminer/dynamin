
#![feature(trait_alias)]
#[allow(dead_code)]

pub use button::{Button};
pub use control::{ChildrenVec, Control, set_tab_order, Visibility};
pub use event_vec::EventHandlerVec;
pub use window::{Window, WindowData, WindowBorderStyle, WindowEvent};

mod bitfield;
mod button;
mod control;
mod event_vec;
mod window;

mod generic_backend;

#[cfg(windows)]
#[path = "windows_backend/mod.rs"]
pub mod backend;
#[cfg(unix)]
#[path = "gtk_backend/mod.rs"]
pub mod backend;

